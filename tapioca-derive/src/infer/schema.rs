use ::quote::Tokens;
use ::syn::Ident;
use ::yaml_rust::Yaml;

use infer::path;
use infer::TokensResult;

pub(super) fn infer_v3(api_st: &Ident, schema: &Yaml) -> TokensResult {
    let paths = schema["paths"].clone();
    let path_impls: Vec<Tokens> = paths.as_hash().expect("Paths must be a map.")
        .iter()
        .map(|(path, path_schema)| path::infer_v3(
            &api_st, path.as_str().expect("Path must be a string."), &path_schema
        ).unwrap())
        .collect();

    Ok(quote! {
        #[allow(plugin_as_library)]
        extern crate tapioca;
        use tapioca::traits::*;

        #(#path_impls)*
    })
}
