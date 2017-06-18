#![feature(use_extern_macros)]
extern crate tapioca_testutil;

tapioca_testutil::infer_test_api!(httpbin);

fn main() {
    let auth = httpbin::ServerAuth::new();

    match httpbin::anything::get(auth) {
        Ok(response) => assert!(true),
        Err(response) => assert!(false),
    }
}
