use std::error::Error;
use std::fs;
use std::io::prelude::*;
use ::reqwest;
use ::yaml_rust::{Yaml, YamlLoader};

const CACHE_DIR: &'static str = ".tapioca-cache";

type Schema = Yaml;
type SchemaResult = Result<Schema, Box<Error + Send + Sync>>;

fn cache_path(schema_fname: &String) -> String {
    fs::create_dir_all(&CACHE_DIR).unwrap();
    format!("{}/{}", &CACHE_DIR, &schema_fname)
}

fn parse_first_doc(buf: &str) -> SchemaResult {
    let docs = YamlLoader::load_from_str(buf.as_ref())?;
    if docs.len() == 0 {
        Err(From::from("Could not parse YAML."))
    } else {
        Ok(docs[0].clone())
    }
}

pub(super) fn parse_schema(schema_fname: &String) -> SchemaResult {
    let mut file = fs::File::open(cache_path(&schema_fname))?;
    let mut buf = String::new();

    file.read_to_string(&mut buf)?;
    parse_first_doc(&buf)
}

pub(super) fn fetch_schema(schema_fname: &String, schema_url: &str) -> SchemaResult {
    let mut file = fs::File::create(cache_path(&schema_fname))?;
    let mut buf = String::new();

    let mut resp = reqwest::get(schema_url)?;
    if resp.status().is_success() {
        resp.read_to_string(&mut buf)?;
        file.write_all(buf.as_ref())?;

        parse_first_doc(&buf)
    } else {
        Err(From::from(format!("Failed to fetch schema: {}", resp.status())))
    }
}
