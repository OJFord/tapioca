#![feature(proc_macro)]
#![recursion_limit="256"]

extern crate inflector;
extern crate proc_macro;
#[macro_use] extern crate quote;
extern crate regex;
extern crate reqwest;
extern crate syn;
extern crate yaml_rust;

use proc_macro::TokenStream;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

mod parse;
mod infer;

#[proc_macro]
pub fn infer(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let schema_url = source.split('"')
        .nth(1).expect("Expected a quoted URL");

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
