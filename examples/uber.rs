#![feature(associated_consts)]
#![feature(use_extern_macros)]
#![allow(plugin_as_library)]

#[macro_use]
extern crate tapioca;

infer_api!(uber, "https://raw.githubusercontent.com/OAI/OpenAPI-Specification/OpenAPI.next/examples/v3.0/uber.yaml");

fn main() {
    use uber::products::get::{QueryParams, OkBody, ErrBody};

    let query_params = QueryParams {
        latitude: 10.3,
        longitude: 237.8,
    };

    match uber::products::get(query_params) {
        Ok(result) => match result.body() {
            OkBody::Status200(body) => {
                let list = body.products
                    .unwrap_or_else(|| vec![]);

                if !list.is_empty() {
                    let first = &list[0];
                    let default = String::from("Unknown");
                    let first_id = first.product_id
                        .as_ref()
                        .unwrap_or(&default);

                    println!("First product: {}", first_id);
                } else {
                    println!("No products!");
                }
            },
            OkBody::UnspecifiedCode(body) =>
                println!("Grr.. the server returned something not in its schema: {}", body),
            OkBody::MalformedJSON(body) => println!("Bad response: {}", body),
        },
        Err(result) => match result.body() {
            ErrBody::UnspecifiedCode(body) => {
                let message = body.message
                    .unwrap_or_else(|| String::from("[None given]"));

                println!("Error message: {}", message);
            },
            ErrBody::NetworkFailure() => println!("Request failed!"),
            ErrBody::MalformedJSON(body) => println!("Bad response: {}", body),
        },
    }
}
