use std::error::Error;
use std::fs;
use std::io::prelude::*;

extern crate hyper;
use self::hyper::client::Client;

extern crate yaml_rust;
use self::yaml_rust::{Yaml, YamlLoader};

type Schema = Yaml;
type SchemaResult = Result<Schema, Box<Error + Send + Sync>>;

fn local_copy_location(schema_name: &str) -> String {
    fs::create_dir_all("tapioca-schemata");
    format!("tapioca-schemata/{}.yml", schema_name)
}

fn parse_first_doc(name: &str, buf: &str) -> SchemaResult {
    let docs = YamlLoader::load_from_str(buf.as_ref())?;
    if docs.len() == 0 {
        Err(From::from(format!("Could not parse YAML for {}", name)))
    } else {
        Ok(docs[0].clone())
    }
}

pub(super) fn parse_schema(schema_name: &str) -> SchemaResult {
    let mut file = fs::File::open(local_copy_location(schema_name))?;
    let mut buf = String::new();

    file.read_to_string(&mut buf)?;
    parse_first_doc(&schema_name, &buf)
}

pub(super) fn fetch_schema(schema_name: &str, schema_url: &str) -> SchemaResult {
    let mut file = fs::File::create(local_copy_location(schema_name))?;
    let mut buf = String::new();
    client.get(schema_url).send()?
        .read_to_string(&mut buf);

    file.write_all(buf.as_ref())?;
        parse_first_doc(&schema_name, &buf)
}
