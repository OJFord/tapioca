#![feature(associated_consts)]
#![feature(use_extern_macros)]
#![allow(plugin_as_library)]

#[macro_use]
extern crate tapioca;

infer_api!(httpbin, "https://raw.githubusercontent.com/OJFord/tapioca/master/tests/schemata/httpbin.yml");

#[test]
fn response_ref() {
    use httpbin::anything_ref;

    let query = anything_ref::get::QueryParams {
        array: vec!["foobar".to_string()],
    };

    match anything_ref::get(query) {
        Ok(response) => match response.body() {
            anything_ref::get::OkBody::Status200(body) => {
                assert_eq!(body.args.array, vec!["foobar".to_string()]);
            },
            _ => panic!(),
        },
        _ => panic!(),
    }
}

#[test]
fn response_array() {
    use httpbin::anything_array;

    let query = anything_array::get::QueryParams {
        array: vec![1.2, 2.3, 4.5],
    };

    match anything_array::get(query) {
        Ok(response) => match response.body() {
            anything_array::get::OkBody::Status200(body) => {
                assert_eq!(body.args.array, vec![1.2, 2.3, 4.5]);
            },
            _ => panic!(),
        },
        _ => panic!(),
    }
}
