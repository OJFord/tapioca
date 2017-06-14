#![feature(associated_consts)]
#![feature(use_extern_macros)]

#[macro_use]
#[allow(plugin_as_library)]
extern crate tapioca;

infer_api!(httpbin, "https://raw.githubusercontent.com/OJFord/tapioca/master/tests/schemata/httpbin.yml");

fn main() {
    let req_body = httpbin::patch::patch::RequestBody {
        musthave: "Hello, world!".into(),
        ifyouwant: Some(vec!["foo".into(), "bar".into(), "baz".into()]),
    };
    match httpbin::patch::patch(&req_body) {
        Ok(response) => assert!(response.status_code().is_success()),
        _ => assert!(false),
    }
}
