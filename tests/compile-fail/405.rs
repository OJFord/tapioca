#![feature(use_extern_macros)]
extern crate tapioca_testutil;

tapioca_testutil::infer_test_api!(httpbin);

fn main() {
    httpbin::post::get(); //~ cannot find function `get`
}
