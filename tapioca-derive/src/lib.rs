extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

use proc_macro::TokenStream;
use syn::{Lit, MetaItem};

mod parse;

#[proc_macro_derive(Schema, attributes(schema_options))]
pub fn derive_infer_schema(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input(&input.to_string()).unwrap();

    let schema_url = &ast.attrs.iter()
        .find(|a| a.name() == "SchemaURL")
        .map(|a| match a.value {
            MetaItem::NameValue(_, Lit::Str(ref value, _)) => value,
            _ => panic!("Bad schema_option"),
        })
        .expect("Schema URL malformed or not given.");

    infer_schema(ast.ident, &schema_url).parse().unwrap()
}

fn infer_schema(ident: syn::Ident, schema_url: &str) -> quote::Tokens {
    quote! {
        trait Schema {
            fn test();
        }

        impl Schema for #ident {
            fn test() {
                println!("OK! {}", #schema_url);
            }
        }
    }
}
