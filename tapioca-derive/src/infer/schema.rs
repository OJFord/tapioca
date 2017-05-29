use ::syn::Ident;
use ::yaml_rust::Yaml;

use infer::path;
use infer::TokensResult;

pub(super) fn infer_v3(api_st: &Ident, schema: &Yaml) -> TokensResult {
    let mut tokens = quote! {
        extern crate tapioca;
        use tapioca::Schema;
    };

    let paths = schema["paths"].clone();
    for (path, path_schema) in paths.as_hash().expect("Paths must be a map.") {
        let path = path.as_str().expect("Path must be a string.");
        tokens.append(path::infer_v3(&api_st, &path, &path_schema)?);
    }

    Ok(tokens)
}
