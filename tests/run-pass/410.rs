#![feature(use_extern_macros)]
extern crate tapioca_testutil;

tapioca_testutil::infer_test_api!(httpbin);

use httpbin::status__code_;

fn main() {
    let dummy_created_id = status__code_::get::__ResourceId_code(200);

    status__code_::get(&dummy_created_id);
    status__code_::delete(dummy_created_id);
}

