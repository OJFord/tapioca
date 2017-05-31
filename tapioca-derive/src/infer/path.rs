use ::inflector::Inflector;
use ::syn::Ident;
use ::yaml_rust::Yaml;

use infer::method;
use infer::TokensResult;

fn mod_ident(path: &str) -> Ident {
    Ident::new(path.replace('/', " ").trim().to_snake_case())
}

pub(super) fn infer_v3(path: &str, schema: &Yaml) -> TokensResult {
    let path_mod = mod_ident(path);

    let method_schemata = schema.as_hash().expect("Path must be a map.");
    let mut method_impls = quote!{};

    for (method, method_schema) in method_schemata {
        let method = method.as_str().expect("Method must be a string.");
        method_impls.append(method::infer_v3(&method, &method_schema)?);
    }

    Ok(quote! {
        pub(super) mod #path_mod {
            use ::tapioca;

            use super::schema_ref;
            use super::API_URL;

            const API_PATH: &'static str = #path;

            #method_impls
        }
    })
}
