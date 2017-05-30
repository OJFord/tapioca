#![recursion_limit="128"]
extern crate inflector;
extern crate proc_macro;
#[macro_use] extern crate quote;
extern crate reqwest;
extern crate syn;
extern crate yaml_rust;

use proc_macro::TokenStream;
use syn::{Lit, MetaItem};

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

    impl_schema(ast.ident, &schema_url).parse().unwrap()
}

fn impl_schema(ident: syn::Ident, schema_url: &str) -> quote::Tokens {
    let schema_name = ident.as_ref();
    let schema = match parse::parse_schema(schema_name) {
        Ok(s) => s,
        Err(_) => {
            match parse::fetch_schema(schema_name, schema_url) {
                Ok(s) => s,
                Err(e) => panic!("{} schema not found: {}", ident, e.description()),
            }
        }
    };

    match infer::infer_schema(&ident, &schema) {
        Ok(tokens) => tokens,
        Err(error) => panic!("Failed to infer schema: {}", error),
    }
}
