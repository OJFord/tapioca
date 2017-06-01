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
    Ident::new(method.to_lowercase())
}

fn query_param_struct_ident(method: &str) -> Ident {
    assert!(valid(method), "Invalid method: {}", method);
    Ident::new(format!("{}QueryParams", method.to_class_case()))
}

fn response_struct_idents(method: &str) -> (Ident, Ident) {
    assert!(valid(method), "Invalid method: {}", method);
    (
        Ident::new(format!("{}OkResponse", method.to_class_case())),
        Ident::new(format!("{}ErrResponse", method.to_class_case())),
    )
}

pub(super) fn infer_v3(method: &str, schema: &Yaml) -> TokensResult {
    let method_fn = fn_ident(&method);

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
                &query_param_struct_ident(&method),
                &Yaml::Array(query_parameters)
            )?;
            structs.push(s);
            bounds.push(b);
            args.push(a);
            transformations.push(t);
        }
    }

    structs.push(
        response::infer_v3(&response_struct_idents(&method), &schema["responses"])?
    );

    Ok(quote! {
        #(#structs)*

        #[allow(dead_code)]
        pub fn #method_fn<#(#bounds),*>(#(#args),*) -> tapioca::ResponseResult {
            let mut url = tapioca::Url::parse(self::API_URL)
                .expect("Malformed server URL or path.");
            url.set_path(self::API_PATH);

            #(#transformations)*

            let client = tapioca::Client::new().unwrap();
            let mut result = client.#method_fn(url).send();
            let response = &mut result.as_mut().ok();
            <tapioca::ResponseResult as tapioca::Response>::from(response)
        }
    })
}
