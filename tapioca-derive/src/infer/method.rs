use ::syn::Ident;
use ::yaml_rust::Yaml;

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

    Ok(quote! {
        fn #method_fn(&self) {
            panic!("Not implemented!")
        }
    })
}
