#![feature(associated_consts)]
#![feature(use_extern_macros)]

#[macro_use]
#[allow(plugin_as_library)]
extern crate tapioca;

infer_api!(httpbin, "https://raw.githubusercontent.com/OJFord/tapioca/master/tests/schemata/httpbin.yml");

fn main() {
    let query = httpbin::post::post::QueryParams {
        echo: Some("foobar".into()),
    };

    match httpbin::post::post(query) {
        Ok(response) => assert!(true),
        Err(response) => assert!(false),
    }
}
