#![recursion_limit="256"]
extern crate inflector;
extern crate proc_macro;
#[macro_use] extern crate quote;
extern crate regex;
extern crate reqwest;
extern crate syn;
extern crate yaml_rust;

use proc_macro::TokenStream;
use syn::{Lit, MetaItem};

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

mod parse;
mod infer;

#[proc_macro_derive(Schema, attributes(schema_options))]
pub fn derive_schema(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input(&input.to_string()).unwrap();

    let schema_url = &ast.attrs.iter()
        .find(|a| a.name() == "SchemaURL")
        .map(|a| match a.value {
            MetaItem::NameValue(_, Lit::Str(ref value, _)) => value,
            _ => panic!("Bad schema_option"),
        })
        .expect("Schema URL malformed or not given.");

    impl_schema(&schema_url).parse().unwrap()
}

fn impl_schema(schema_url: &str) -> quote::Tokens {
    let mut url_hasher = DefaultHasher::new();
    schema_url.hash(&mut url_hasher);

    let schema_fname = format!("{}.yml", url_hasher.finish());
    let schema = match parse::parse_schema(&schema_fname) {
        Ok(s) => s,
        Err(_) => {
            match parse::fetch_schema(&schema_fname, &schema_url) {
                Ok(s) => s,
                Err(e) => panic!("Unable to find schema: {}", e.description()),
            }
        }
    };

    match infer::infer_schema(&schema) {
        Ok(tokens) => tokens,
        Err(error) => panic!("Failed to infer schema: {}", error),
    }
}
