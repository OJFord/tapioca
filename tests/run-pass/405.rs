#![feature(use_extern_macros)]
extern crate tapioca_testutil;

tapioca_testutil::infer_test_api!(httpbin);

fn main() {
    let auth = httpbin::ServerAuth::new();

    let query = httpbin::post::post::QueryParams {
        echo: Some("foobar".into()),
    };

    match httpbin::post::post(query, auth) {
        Ok(response) => assert!(true),
        Err(response) => assert!(false),
    }
}
