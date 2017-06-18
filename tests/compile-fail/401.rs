#![feature(use_extern_macros)]
extern crate tapioca_testutil;

tapioca_testutil::infer_test_api!(httpbin);

fn main() {
    let server_auth = httpbin::ServerAuth::new();
    let op_auth = httpbin::basic_auth::OpAuth::HttpBasic::from(("".into(), "".into()));
    let query = httpbin::basic_auth::get::QueryParams { user };

    httpbin::basic_auth::get(&query); //~ takes 2 parameters but 1 parameter was supplied
    httpbin::basic_auth::get(&query, server_auth); //~ mismatched types
    httpbin::basic_auth::get(&query, op_auth); // [OK]
}
