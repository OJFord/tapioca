#![feature(use_extern_macros)]
extern crate tapioca_testutil;

tapioca_testutil::infer_test_api!(httpbin);

fn main() {
    let auth = httpbin::ServerAuth::new();
    let body = httpbin::patch::patch::RequestBody {
        musthave: "foobar".into(),
        ifyouwant: None,
    };

    match httpbin::patch::patch(&body, auth) {
        Ok(response) => match response.body() {
            httpbin::patch::patch::OkBody::Status200(body)
                => assert_eq!(body.headers.content_length, Some("38".into())),
            _ => assert!(false),
        },
        Err(_) => assert!(false),
    }
}
