#![feature(decl_macro)]
#![feature(use_extern_macros)]

#[macro_use]
extern crate tapioca;

infer_api!(httpbin, "https://raw.githubusercontent.com/OJFord/tapioca/master/tests/schemata/httpbin.yml");

#[macro_export]
macro_rules! infer_test_api {
    (httpbin) => {
        use tapioca_testutil::Response;
        use tapioca_testutil::httpbin;
    }
}
