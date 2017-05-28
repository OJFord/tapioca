use std::error::Error;
use ::syn::Ident;
use ::quote::Tokens;
use ::yaml_rust::Yaml;

const SCHEMA_VERSION_KEY: &'static str = "openapi";

type TokensResult = Result<Tokens, Box<Error + Send + Sync>>;

pub(super) fn infer_schema(name: &Ident, schema: &Yaml) -> TokensResult {
    match schema[SCHEMA_VERSION_KEY].as_str() {
        None => Err(From::from("Unspecified schema version.")),
        Some("3.0.0") => infer_schema_v3(&name, &schema),
        Some(version) => Err(From::from(format!("Unsupported schema version: {}", version))),
    }
}

fn infer_schema_v3(name: &Ident, schema: &Yaml) -> TokensResult {
    Ok(quote! {
        extern crate tapioca;
        use tapioca::Schema;

        struct Resource;

        impl #name {
            fn resource() -> Resource {
                Resource
            }
        }

        impl Schema for #name {
            fn get(&self) {
                //!TODO: Replace for compile_error! macro
                //! when rust-lang/rust#40872 implemented
                panic!("GET not allowed for /")
            }
        }

        impl Schema for Resource {
            fn get(&self) {
                panic!("Not implemented!")
            }
        }
    })
}
