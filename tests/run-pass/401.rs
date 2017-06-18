#![feature(use_extern_macros)]
extern crate tapioca_testutil;

tapioca_testutil::infer_test_api!(httpbin);

use httpbin::basic_auth__user__hunter_2 as basic_auth;
use basic_auth::get::OpAuth::HttpBasic;

static USER: &str = "baz";

fn main() {
    let user_id = basic_auth::ResourceId_user::from_static(USER);
    let auth = HttpBasic((USER.into(), "wrong".into()).into());

    match basic_auth::get(&user_id, auth) {
        Ok(_) => assert!(false),
        Err(response) => match response.body() {
            basic_auth::get::ErrBody::Status401(_) => assert!(true),
            _ => assert!(false),
        },
    }

    let user_id = basic_auth::ResourceId_user::from_static(USER);
    let auth = HttpBasic((USER.into(), "hunter2".into()).into());
    match basic_auth::get(&user_id, auth) {
        Ok(response) => match response.body() {
            basic_auth::get::OkBody::Status200(body) => assert!(body.authenticated),
            _ => assert!(false),
        },
        Err(_) => assert!(false),
    }
}
