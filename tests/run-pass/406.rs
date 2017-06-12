#![feature(associated_consts)]
#![feature(use_extern_macros)]
#![allow(plugin_as_library)]

#[macro_use]
extern crate tapioca;

infer_api!(httpbin, "https://raw.githubusercontent.com/OJFord/tapioca/master/tests/schemata/httpbin.yml");

fn main() {
    match httpbin::anything::get() {
        Ok(response) => match response.body() {
            httpbin::anything::get::OkBody::Status200(body) => assert!(
                body.headers.accept.contains("application/json")
            ),
            _ => assert!(false),
        },
        Err(_) => assert!(false),
    }
}
