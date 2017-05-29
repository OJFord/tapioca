#![feature(plugin)]
#![plugin(tapioca)]

#[macro_use]
extern crate tapioca_derive;

#[derive(Schema)]
#[SchemaURL = "https://raw.githubusercontent.com/OAI/OpenAPI-Specification/OpenAPI.next/examples/v3.0/uber.yaml"]
struct UberAPI;

fn main() {
    let latlong = UberAPIProductGetQueryParams{
        latitude: 10.3,
        longitude: 237.8,
    };
    UberAPI::products().get(latlong);
}
