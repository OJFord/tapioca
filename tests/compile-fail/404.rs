#![feature(use_extern_macros)]
extern crate tapioca_testutil;

tapioca_testutil::infer_test_api!(httpbin);

fn main() {
    httpbin::nonexistent_path::get(); //~ Could not find `nonexistent_path`
}
