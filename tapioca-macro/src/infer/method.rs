use ::inflector::Inflector;
use ::quote::Tokens;
use ::syn::Ident;
use ::yaml_rust::Yaml;

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
    let mut transformations: Vec<Tokens> = Vec::new();

    let query_parameters: Vec<Yaml>;
    if let Some(parameters) = schema["parameters"].as_vec() {
        query_parameters = parameters
            .iter().cloned()
            .filter(|p| p["in"] == Yaml::from_str("query")).collect();

        if query_parameters.len() > 0 {
            let (s, b, a, t) = query::infer_v3(
                &method_mod,
                &Yaml::Array(query_parameters)
            )?;
            structs.push(s);
            bounds.push(b);
            args.push(a);
            transformations.push(t);
        }
    }

    structs.push(response::infer_v3(&schema["responses"])?);

    Ok(quote! {
        pub mod #method_mod {
            use super::schema_ref;
            #(#structs)*
        }

        #[allow(dead_code)]
        pub fn #method_fn<#(#bounds),*>(#(#args),*) -> #method_mod::ResponseResult {
            let mut url = Url::parse(
                format!("{}{}", self::API_URL, self::API_PATH).as_str()
            ).expect("Malformed server URL or path.");

            #(#transformations)*

            let client = Client::new().unwrap();
            let request = client.#method_fn(url)
                .header(header::Accept::json());

            let mut response = request.send().ok();
            <#method_mod::ResponseResult as Response>::from(&mut response.as_mut())
        }
    })
}
