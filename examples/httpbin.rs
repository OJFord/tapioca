#![feature(associated_consts)]
#![feature(use_extern_macros)]

#[macro_use]
#[allow(plugin_as_library)]
extern crate tapioca;

infer_api!(httpbin, "https://raw.githubusercontent.com/OJFord/tapioca/master/tests/schemata/httpbin.yml");


fn main() {
    match httpbin::ip::get() {
        Ok(response) => match response.body() {
            httpbin::ip::get::OkBody::Status200(body) => println!("Your IP is {}", body.origin),
            _ => panic!(),
        },
        _ => println!("Failed to find IP address"),
    }

    let query = httpbin::post::post::QueryParams {
        echo: Some("echo me!".into()),
    };
    match httpbin::post::post(query) {
        Ok(response) => match response.body() {
            httpbin::post::post::OkBody::Status200(_) => assert!(true),
            _ => panic!(),
        },
        _ => panic!(),
    }
}
