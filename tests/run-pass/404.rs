#![feature(associated_consts)]
#![feature(use_extern_macros)]

#[macro_use]
extern crate tapioca;

infer_api!(httpbin, "https://raw.githubusercontent.com/OJFord/tapioca/master/tests/schemata/httpbin.yml");

use httpbin::ip;

fn main() {
    match ip::get() {
        Ok(response) => match response.body() {
            ip::get::OkBody::Status200(body) => {
                let ipv4_parts: Vec<&str> = body.origin
                    .split('.').collect();
                assert_eq!(ipv4_parts.len(), 4);
            },
            _ => assert!(false, "This test might be broken"),
        },
        Err(response) => match response.body() {
            ip::get::ErrBody::NetworkFailure() => main(),
            _ => assert!(false, "This test might be broken"),
        },
    }
}
