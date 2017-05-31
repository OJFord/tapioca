use ::quote::Tokens;
use ::yaml_rust::Yaml;

use infer::path;
use infer::TokensResult;

pub(super) fn infer_v3(schema: &Yaml) -> TokensResult {
    let paths = schema["paths"].clone();
    let path_impls: Vec<Tokens> = paths.as_hash()
        .expect("Paths must be a map.")
        .iter()
        .map(|(path, path_schema)| path::infer_v3(
            path.as_str().expect("Path must be a string."), &path_schema
        ).unwrap())
        .collect();

    let api_url = schema["servers"][0]["url"].as_str()
        .expect("Must have at least one server URL.");

    let schema_ref_struct_defs: Vec<Tokens> = Vec::new();

    Ok(quote! {
        #[macro_use]
        use tapioca::serde_derive;

        #[allow(plugin_as_library)]
        extern crate tapioca;

        use tapioca::serde;
        use tapioca::serde_json;

        mod schema_ref {
            #(#schema_ref_struct_defs)*
        }

        const API_URL: &'static str = #api_url;

        #(#path_impls)*
    })
}
