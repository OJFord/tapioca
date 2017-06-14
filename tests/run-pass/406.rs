#![feature(use_extern_macros)]
extern crate tapioca_testutil;

tapioca_testutil::infer_test_api!(httpbin);

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
