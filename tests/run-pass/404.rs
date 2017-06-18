#![feature(use_extern_macros)]
extern crate tapioca_testutil;

tapioca_testutil::infer_test_api!(httpbin);

use httpbin::ip;

fn main() {
    let auth = httpbin::ServerAuth::new();

    match ip::get(auth) {
        Ok(response) => match response.body() {
            ip::get::OkBody::Status200(body) => {
                let ipv4_parts: Vec<&str> = body.origin
                    .split('.').collect();
                assert_eq!(ipv4_parts.len(), 4);
            },
            _ => assert!(false, "This test might be broken"),
        },
        Err(response) => match response.body() {
            ip::get::ErrBody::NetworkFailure() => main(),
            _ => assert!(false, "This test might be broken"),
        },
    }
}
