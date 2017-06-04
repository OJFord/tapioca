#![feature(plugin_registrar)]
#![feature(rustc_private)]

extern crate reqwest;
extern crate rustc_plugin;
extern crate syntax;

pub extern crate serde;
pub extern crate serde_json;

use rustc_plugin::Registry;
use syntax::feature_gate::AttributeType;

pub use reqwest::{Client, Url};
pub use serde::Deserialize;

pub mod response;
pub mod datatype;

#[macro_export]
macro_rules! infer_api {
    ($name:ident, $url:expr) => {
        #[macro_use]
        extern crate tapioca_derive;
        #[macro_use]
        extern crate serde_derive;

        use tapioca::response::Response;

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
