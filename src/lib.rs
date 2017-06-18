#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate base64;
extern crate reqwest;

pub extern crate serde;
pub extern crate serde_json;

pub use reqwest::header;
pub use reqwest::Body;
pub use reqwest::Client;
pub use reqwest::Url;
pub use serde::Deserialize;

pub mod auth;
pub mod response;
pub mod datatype;
pub mod query;

pub type HeaderResult<H> = Result<H, reqwest::HyperError>;

#[macro_export]
macro_rules! infer_api {
    ($name:ident, $url:expr) => {
        #[macro_use]
        extern crate serde_derive;
        extern crate tapioca_codegen;

        pub use tapioca::response::Response;

        pub mod $name {
            ::tapioca_codegen::infer!($url);
        }
    }
}
