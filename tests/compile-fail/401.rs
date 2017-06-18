#![feature(use_extern_macros)]
extern crate tapioca_testutil;

tapioca_testutil::infer_test_api!(httpbin);

use httpbin::basic_auth__user__hunter_2 as basic_auth;
use basic_auth::get::OpAuth::HttpBasic;

static USER: &str = "baz";

fn main() {
    let server_auth = httpbin::ServerAuth::new();
    let user_id = basic_auth::ResourceId_user::from_static(USER);
    let op_auth = HttpBasic((USER.into(), "".into()).into());

    basic_auth::get(&user_id); //~ takes 2 parameters but 1 parameter was supplied
    basic_auth::get(&user_id, server_auth); //~ mismatched types
    basic_auth::get(&user_id, op_auth); // [OK]
}
