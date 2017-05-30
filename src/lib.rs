#![feature(plugin_registrar)]
#![feature(rustc_private)]

extern crate reqwest;
extern crate rustc_plugin;
extern crate syntax;

pub use reqwest::{Client, Url};

use rustc_plugin::Registry;
use syntax::feature_gate::AttributeType;

#[macro_export]
macro_rules! infer_api {
    ($name:ident, $url:expr) => {
        #[macro_use]
        extern crate tapioca_derive;

        mod $name {
            #[derive(Schema)]
            #[SchemaURL = $url]
            struct _Anchor;
        }
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_attribute("SchemaURL".to_owned(), AttributeType::Whitelisted);
}
