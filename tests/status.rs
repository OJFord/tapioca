#![feature(associated_consts)]
#![feature(type_ascription)]
#![feature(use_extern_macros)]
#![allow(plugin_as_library)]

#[macro_use]
extern crate tapioca;

infer_api!(httpbin, "https://raw.githubusercontent.com/OJFord/tapioca/master/tests/schemata/httpbin.yml");

use httpbin::status_resource;

#[test]
fn ok_err_matching() {
    match status_resource::get(200) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    }
    match status_resource::get(400) {
        Ok(_) => assert!(false),
        Err(_) => assert!(true),
    }
}

#[test]
fn status_body_matching() {
    match status_resource::get(200) {
        Ok(response) => match response.body() {
            status_resource::get::OkBody::Status200(_) => assert!(true),
            _ => assert!(false),
        },
        Err(_) => assert!(false),
    }
}