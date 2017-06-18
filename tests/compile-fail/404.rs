#![feature(use_extern_macros)]
extern crate tapioca_testutil;

tapioca_testutil::infer_test_api!(httpbin);

fn main() {
    let auth = httpbin::ServerAuth::new();

    httpbin::nonexistent_path::get(auth); //~ Could not find `nonexistent_path`
}
