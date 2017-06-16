#![feature(use_extern_macros)]
extern crate tapioca_testutil;

tapioca_testutil::infer_test_api!(httpbin);

use httpbin::status__code_;

const code: &'static i32 = &200;

fn main() {
    let dummy_created_id = status__code_::ResourceId_code::from_static(code);

    status__code_::delete(&dummy_created_id); //~ mismatched types
    status__code_::get(&dummy_created_id);

    status__code_::delete(dummy_created_id.clone()); //~ no method named `clone`
    status__code_::get(&dummy_created_id);
}
