use std::error::Error;
use std::fs;
use std::io::prelude::*;

extern crate hyper;
use self::hyper::client::Client;

extern crate yaml_rust;
use self::yaml_rust::{Yaml, YamlLoader};

type Schema = Yaml;
type SchemaResult = Result<Schema, Box<Error>>;

fn local_copy_location(schema_name: &str) -> String {
    fs::create_dir_all("tapioca-schemata");
    "tapioca-schemata/".to_owned() + schema_name
}

pub(super) fn parse_schema<'a>(schema_name: &'a str) -> SchemaResult {
    let mut file = fs::File::open(local_copy_location(schema_name))?;
    let mut schema_str = String::new();
    file.read_to_string(&mut schema_str)?;

    let docs = YamlLoader::load_from_str(schema_str.as_ref())?;
    Ok(docs[0].clone())
}

pub(super) fn fetch_schema<'a>(schema_name: &'a str, schema_url: &'a str) -> SchemaResult {
    let client = Client::new();
    let mut file = fs::File::create(local_copy_location(schema_name))?;

    let mut buf = String::new();
    client.get(schema_url).send()?
        .read_to_string(&mut buf);

    file.write_all(buf.as_ref())?;
    let docs = YamlLoader::load_from_str(buf.as_ref())?;
    Ok(docs[0].clone())
}
