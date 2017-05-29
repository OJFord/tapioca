use std::error::Error;
use ::inflector::Inflector;
use ::syn::Ident;
use ::quote::Tokens;
use ::yaml_rust::Yaml;

const SCHEMA_VERSION_KEY: &'static str = "openapi";

type TokensResult = Result<Tokens, Box<Error + Send + Sync>>;

pub(super) fn infer_schema(name: &Ident, schema: &Yaml) -> TokensResult {
    match schema[SCHEMA_VERSION_KEY].as_str() {
        None => Err(From::from("Unspecified schema version.")),
        Some("3.0.0") => infer_v3_schema(&name, &schema),
        Some(version) => Err(From::from(format!("Unsupported schema version: {}", version))),
    }
}

fn infer_v3_schema(api_st: &Ident, schema: &Yaml) -> TokensResult {
    let mut tokens = quote! {
        extern crate tapioca;
        use tapioca::Schema;

        impl Schema for #api_st {
            fn get(&self) {
                //!TODO: Replace for compile_error! macro
                //! when rust-lang/rust#40872 implemented
                panic!("GET not allowed for /")
            }
        }
    };

    let paths = schema["paths"].clone();
    for (path, path_schema) in paths.as_hash().expect("Paths must be a map.") {
        let path = path.as_str().expect("Path must be a string.");
        tokens.append(infer_v3_path(&api_st, &path, &path_schema)?);
    }

    Ok(tokens)
}

fn path_struct_ident(api_st: &Ident, path: &str) -> Ident {
    Ident::new(format!("{}{}", api_st, path.replace('/', " ").to_class_case()))
}

fn path_fn_ident(path: &str) -> Ident {
    Ident::new(path.replace('/', "").to_lowercase())
}

fn infer_v3_path(api_st: &Ident, path: &str, schema: &Yaml) -> TokensResult {
    let path_st = path_struct_ident(api_st, path);
    let path_fn = path_fn_ident(path);

    let tokens = quote! {
        struct #path_st;

        impl #api_st {
            fn #path_fn() -> #path_st {
                #path_st
            }
        }

        impl Schema for #path_st {
            fn get(&self) {
                panic!("Not implemented!")
            }
        }
    };

    Ok(tokens)
}
