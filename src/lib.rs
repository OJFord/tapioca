#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![feature(plugin_registrar)]
#![feature(rustc_private)]

extern crate reqwest;
extern crate rustc_plugin;
extern crate syntax;

pub extern crate serde;
pub extern crate serde_json;

use rustc_plugin::Registry;

pub use reqwest::header;
pub use reqwest::Client;
pub use reqwest::Url;
pub use serde::Deserialize;

pub mod response;
pub mod datatype;

#[macro_export]
macro_rules! infer_api {
    ($name:ident, $url:expr) => {
        #[macro_use]
        extern crate serde_derive;
        extern crate tapioca_codegen;

        use tapioca::response::Response;

        mod $name {
            ::tapioca_codegen::infer!($url);
        }
    }
}

#[plugin_registrar]
pub fn plugin_registrar(_: &mut Registry) {
}
