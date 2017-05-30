#![feature(associated_consts)]
#![feature(plugin)]
#![plugin(tapioca)]

#[macro_use]
#[allow(plugin_as_library)]
extern crate tapioca;

infer_api!(uber, "https://raw.githubusercontent.com/OAI/OpenAPI-Specification/OpenAPI.next/examples/v3.0/uber.yaml");

fn main() {
    use uber::products;

    let query_params = products::GetQueryParams {
        latitude: 10.3,
        longitude: 237.8,
    };

    products::get(&query_params);
}
