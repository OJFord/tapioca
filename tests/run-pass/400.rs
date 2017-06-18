#![feature(use_extern_macros)]
extern crate tapioca_testutil;

tapioca_testutil::infer_test_api!(httpbin);

fn main() {
    let auth = httpbin::ServerAuth::new();

    let req_body = httpbin::patch::patch::RequestBody {
        musthave: "Hello, world!".into(),
        ifyouwant: Some(vec!["foo".into(), "bar".into(), "baz".into()]),
    };

    match httpbin::patch::patch(&req_body, auth) {
        Ok(response) => assert!(response.status_code().is_success()),
        _ => assert!(false),
    }
}
