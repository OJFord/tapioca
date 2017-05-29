use ::syn::Ident;
use ::yaml_rust::Yaml;

use infer::parameter;
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

pub(super) fn infer_v3(path_st: &Ident, method: &str, schema: &Yaml) -> TokensResult {
    let method_fn = fn_ident(method);
    let parameters = schema["parameters"]
        .as_vec().expect("Parameters must be an array.");

    let query_params = parameters.iter()
        .filter(|p| p["in"] == Yaml::from_str("query"))
        .map(|p| parameter::infer_v3(p).unwrap());

    Ok(quote! {
        fn #method_fn(&self, #(#query_params),*) {
            panic!("Not implemented!")
        }
    })
}
