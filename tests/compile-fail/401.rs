#![feature(use_extern_macros)]
extern crate tapioca_testutil;

tapioca_testutil::infer_test_api!(httpbin);

fn main() {
    let auth = httpbin::ServerAuth::new();

    httpbin::anything::get(); //~ takes 1 parameter but 0 parameters were supplied
    httpbin::anything::get(auth); // OK
}
