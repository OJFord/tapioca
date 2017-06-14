#![feature(associated_consts)]
#![feature(use_extern_macros)]

#[macro_use]
#[allow(plugin_as_library)]
extern crate tapioca;

infer_api!(httpbin, "https://raw.githubusercontent.com/OJFord/tapioca/master/tests/schemata/httpbin.yml");

fn main() {
    httpbin::patch::patch(); //~ takes 1 parameter but 0 parameters were supplied

    let req_body = httpbin::patch::patch::RequestBody {
        musthave: None, //~ mismatched types
        ifyouwant: Some(vec!["foo".into(), "bar".into(), "baz".into()]),
    };
    match httpbin::patch::patch(&req_body.clone()) {
        Ok(response) => match response.body() {
            httpbin::patch::patch::OkBody::Status200(res_body) =>
                assert_eq!(res_body.json.musthave, req_body.musthave),
            _ => assert!(false),
        },
        Err(_) => assert!(false),
    }
}
