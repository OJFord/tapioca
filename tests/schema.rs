#![feature(use_extern_macros)]
extern crate tapioca_testutil;

tapioca_testutil::infer_test_api!(httpbin);

#[test]
fn response_ref() {
    use httpbin::anything_ref;

    let auth = httpbin::ServerAuth::new();
    let test_vec: Vec<String> = vec!["foobar".into(), "bazzer".into()];
    let query = anything_ref::get::QueryParams {
        array: test_vec.clone(),
    };

    match anything_ref::get(query, auth) {
        Ok(response) => match response.body() {
            anything_ref::get::OkBody::Status200(body) => assert_eq!(body.args.array, test_vec),
            _ => assert!(false),
        },
        _ => assert!(false),
    }
}

#[test]
fn response_array() {
    use httpbin::anything_array;

    let auth = httpbin::ServerAuth::new();
    let test_vec: Vec<f32> = vec![1.2, 2.3, 4.5];
    let query = anything_array::get::QueryParams {
        array: test_vec.clone(),
    };

    match anything_array::get(query, auth) {
        Ok(response) => match response.body() {
            anything_array::get::OkBody::Status200(body) => assert_eq!(
                body.args.array,
                test_vec.iter().map(ToString::to_string).collect::<Vec<_>>()
            ),
            _ => assert!(false),
        },
        _ => assert!(false),
    }
}

#[test]
fn request() {
    use httpbin::patch;

    let auth = httpbin::ServerAuth::new();
    let req_body = patch::patch::RequestBody {
        musthave: "foobar".into(),
        ifyouwant: Some(vec![]),
    };

    match patch::patch(&req_body, auth) {
        Ok(response) => match response.body() {
            patch::patch::OkBody::Status200(body) => assert_eq!(body.json, req_body),
            _ => assert!(false),
        },
        _ => assert!(false),
    }
}
