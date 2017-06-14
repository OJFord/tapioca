use ::inflector::Inflector;
use ::quote::Tokens;
use ::syn::Ident;
use ::yaml_rust::Yaml;

use infer::body;
use infer::params;
use infer::query;
use infer::response;
use infer::TokensResult;

const METHODS: &'static [&'static str] = &[
    "DELETE",
    "GET",
    "HEAD",
    "PATCH",
    "POST",
    "PUT",
];

pub(super) fn valid(method: &str) -> bool {
    METHODS.contains(&method.to_uppercase().as_str())
}

fn fn_ident(method: &str) -> Ident {
    assert!(valid(method), "Invalid method: {}", method);
    Ident::new(method.to_snake_case())
}

fn mod_ident(method: &str) -> Ident {
    assert!(valid(method), "Invalid method: {}", method);
    Ident::new(method.to_snake_case())
}

pub(super) fn infer_v3(method: &str, schema: &Yaml) -> TokensResult {
    let method_fn = fn_ident(&method);
    let method_mod = mod_ident(&method);

    let mut structs: Vec<Tokens> = Vec::new();
    let mut bounds: Vec<Tokens> = Vec::new();
    let mut args: Vec<Tokens> = Vec::new();
    let mut url_transforms: Vec<Tokens> = Vec::new();
    let mut req_transforms: Vec<Tokens> = Vec::new();

    if let Some(parameters) = schema["parameters"].as_vec() {
        let query_parameters = parameters.iter().cloned()
            .filter(|p| p["in"] == Yaml::from_str("query"))
            .collect::<Vec<Yaml>>();

        let path_parameters = parameters.iter().cloned()
            .filter(|p| p["in"] == Yaml::from_str("path"))
            .collect::<Vec<Yaml>>();

        if !query_parameters.is_empty() {
            let (s, b, a, t) = query::infer_v3(&method_mod, &Yaml::Array(query_parameters))?;
            structs.push(s);
            bounds.push(b);
            args.push(a);
            url_transforms.push(t);
        }

        if !path_parameters.is_empty() {
            let (s, b, a, t) = params::infer_v3(&method_mod, &Yaml::Array(path_parameters))?;
            structs.push(s);
            bounds.push(b);
            args.push(a);
            url_transforms.push(t);
        }
    }

    match schema["requestBody"] {
        Yaml::BadValue => (),
        ref schema => {
            let (s, b, a, t) = body::infer_v3(&method_mod, &schema)?;
            structs.push(s);
            bounds.push(b);
            args.push(a);
            req_transforms.push(t);
        }
    }

    structs.push(response::infer_v3(&schema["responses"])?);

    Ok(quote! {
        pub mod #method_mod {
            #[allow(unused_imports)]
            use super::schema_ref;

            #(#structs)*
        }

        #[allow(dead_code)]
        #[allow(unused_mut)]
        pub fn #method_fn<#(#bounds),*>(#(#args),*) -> #method_mod::ResponseResult {
            let mut url = Url::parse(format!("{}{}", self::API_URL, self::API_PATH).as_str())
                .expect("malformed server URL or path");
            #(url#url_transforms;)*

            let client = Client::new().unwrap();
            let request = client.#method_fn(url)
                .header(header::Accept::json())
                #(#req_transforms)*;

            let mut response = request.send().ok();
            <#method_mod::ResponseResult as Response>::from(&mut response.as_mut())
        }
    })
}
