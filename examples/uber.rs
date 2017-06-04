#![feature(associated_consts)]
#![feature(plugin)]
#![plugin(tapioca)]

#[macro_use]
extern crate tapioca;

infer_api!(uber, "https://raw.githubusercontent.com/OAI/OpenAPI-Specification/OpenAPI.next/examples/v3.0/uber.yaml");

fn main() {
    use uber::products;

    let query_params = products::get::QueryParams {
        latitude: 10.3,
        longitude: 237.8,
    };

    match products::get(query_params) {
        Ok(result) => match result.body() {
            products::get::OkBody::Status200(body) => {
                let list = body.products.unwrap_or(vec![]);
                println!("First product: {}", list[0]);
            },
        },
        Err(result) => match result.body() {
            products::get::ErrBody::Status520(body) => {
                println!("Error message: {}", body.message);
            },
            products::get::ErrBody::NetworkFailure() => {
                println!("Request failed!");
            },
        },
    }
}
