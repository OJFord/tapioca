#![feature(associated_consts)]
#![feature(plugin)]
#![plugin(tapioca)]

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
                    .unwrap_or(vec![]);

                if list.len() > 0 {
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
            OkBody::UnspecifiedCode(body) => println!(
                "Grr.. the server returned something not in its schema: {}",
                body
            )
        },
        Err(result) => match result.body() {
            ErrBody::UnspecifiedCode(body) => {
                let message = body.message
                    .unwrap_or(String::from("[None given]"));

                println!("Error message: {}", message);
            },
            ErrBody::NetworkFailure() => println!("Request failed!"),
        },
    }
}
