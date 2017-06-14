#![feature(use_extern_macros)]
extern crate tapioca_testutil;

tapioca_testutil::infer_test_api!(httpbin);

use httpbin::status__code_;

#[test]
fn ok_err_matching() {
    match status__code_::get(200) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    }
    match status__code_::get(400) {
        Ok(_) => assert!(false),
        Err(_) => assert!(true),
    }
}

#[test]
fn status_body_matching() {
    match status__code_::get(200) {
        Ok(response) => match response.body() {
            status__code_::get::OkBody::Status200(_) => assert!(true),
            _ => assert!(false),
        },
        Err(_) => assert!(false),
    }
}
