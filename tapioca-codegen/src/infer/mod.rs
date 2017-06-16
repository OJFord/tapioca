use std::error::Error;
use ::quote::Tokens;
use ::yaml_rust::Yaml;

mod auth;
mod body;
mod datatype;
mod method;
mod query;
mod params;
mod path;
mod response;
mod schema;

const SCHEMA_VERSION_KEY: &'static str = "openapi";

type InferResult<T> = Result<T, Box<Error + Send + Sync>>;
type MaybeTokens = Option<Tokens>;
type StructBoundArgImpl = InferResult<(MaybeTokens, MaybeTokens, MaybeTokens, MaybeTokens)>;
type TokensResult = InferResult<Tokens>;

pub(super) fn infer_schema(schema: &Yaml) -> TokensResult {
    match schema[SCHEMA_VERSION_KEY].as_str() {
        None => Err(From::from("Unspecified schema version.")),
        Some("3.0.0") => schema::infer_v3(&schema),
        Some(version) => Err(From::from(format!("Unsupported schema version: {}", version))),
    }
}
