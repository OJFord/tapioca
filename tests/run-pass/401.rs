#![feature(use_extern_macros)]
extern crate tapioca_testutil;

tapioca_testutil::infer_test_api!(httpbin);

fn main() {
    let user = String::from("Foo");
    let mut auth = httpbin::basic_auth::HttpBasic::from((user, "wrong".into()));
    let query = httpbin::basic_auth::get::QueryParams { user };

    match httpbin::basic_auth::get(&query, auth) {
        Ok(_) => assert!(false),
        Err(response) => match response.body() {
            httpbin::basic_auth::get::ErrBody::Status401() => assert!(true),
            _ => assert!(false),
        },
    }

    auth.password = "hunter2";
    match httpbin::basic_auth::get(&query, auth) {
        Ok(response) => match response.body() {
            httpbin::basic_auth::get::OkBody::Status200(body) => {
                assert!(body.authenticated);
                assert_eq!(body.user, user);
            }
        },
        Err(_) => assert!(false),
    }
}
