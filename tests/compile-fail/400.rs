#![feature(use_extern_macros)]
extern crate tapioca_testutil;

tapioca_testutil::infer_test_api!(httpbin);

fn main() {
    let auth = httpbin::ServerAuth::new();

    httpbin::patch::patch(auth); //~ takes 2 parameters but 1 parameter was supplied

    let req_body = httpbin::patch::patch::RequestBody {
        musthave: None, //~ mismatched types
        ifyouwant: Some(vec!["foo".into(), "bar".into(), "baz".into()]),
    };
    match httpbin::patch::patch(&req_body, auth) {
        Ok(response) => match response.body() {
            httpbin::patch::patch::OkBody::Status200(res_body) =>
                assert_eq!(res_body.json.musthave, req_body.musthave),
            _ => assert!(false),
        },
        Err(_) => assert!(false),
    }
}
