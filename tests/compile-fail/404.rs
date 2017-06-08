#![feature(associated_consts)]
#![feature(use_extern_macros)]

#[macro_use]
#[allow(plugin_as_library)]
extern crate tapioca;

infer_api!(httpbin, "https://raw.githubusercontent.com/OJFord/tapioca/master/tests/schemata/httpbin.yml");

fn main() {
    httpbin::nonexistent_path::get(); //~ Could not find `nonexistent_path`
}
