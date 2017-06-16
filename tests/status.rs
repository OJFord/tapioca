#![feature(use_extern_macros)]
extern crate tapioca_testutil;

tapioca_testutil::infer_test_api!(httpbin);

use httpbin::redirect_to;
use httpbin::redirect_to::get::QueryParams;

#[test]
fn ok_err_matching() {
    let query200 = QueryParams {
        url: "http://httpbin.org/status/200".into(),
    };

    let query400 = QueryParams {
        url: "http://httpbin.org/status/400".into(),
    };

    match redirect_to::get(query200) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    }

    match redirect_to::get(query400) {
        Ok(_) => assert!(false),
        Err(_) => assert!(true),
    }
}

#[test]
fn status_body_matching() {
    let query200 = QueryParams {
        url: "http://httpbin.org/status/200".into(),
    };

    let query400 = QueryParams {
        url: "http://httpbin.org/status/400".into(),
    };

    match redirect_to::get(query200) {
        Ok(response) => match response.body() {
            redirect_to::get::OkBody::Status200(_) => assert!(true),
            _ => assert!(false),
        },
        Err(_) => assert!(false),
    }

    match redirect_to::get(query400) {
        Ok(_) => assert!(false),
        Err(response) => match response.body() {
            redirect_to::get::ErrBody::Status400(_) => assert!(true),
            _ => assert!(false),
        },
    }
}
