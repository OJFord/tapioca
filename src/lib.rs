#![feature(plugin_registrar)]
#![feature(rustc_private)]

extern crate reqwest;
extern crate rustc_plugin;
extern crate serde_json;
extern crate syntax;

use rustc_plugin::Registry;
use syntax::feature_gate::AttributeType;

mod response;
pub mod datatype;

pub use reqwest::{Client, Url};
pub use response::{Response, ResponseResult};

#[macro_export]
macro_rules! infer_api {
    ($name:ident, $url:expr) => {
        #[macro_use]
        extern crate tapioca_derive;
        #[macro_use]
        extern crate serde_derive;

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
