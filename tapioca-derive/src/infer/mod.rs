use std::error::Error;
use ::inflector::Inflector;
use ::syn::Ident;
use ::quote::Tokens;
use ::yaml_rust::Yaml;

mod path;
mod schema;

const SCHEMA_VERSION_KEY: &'static str = "openapi";

type TokensResult = Result<Tokens, Box<Error + Send + Sync>>;

pub(super) fn infer_schema(name: &Ident, schema: &Yaml) -> TokensResult {
    match schema[SCHEMA_VERSION_KEY].as_str() {
        None => Err(From::from("Unspecified schema version.")),
        Some("3.0.0") => schema::infer_v3(&name, &schema),
        Some(version) => Err(From::from(format!("Unsupported schema version: {}", version))),
    }
}
