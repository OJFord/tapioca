#![feature(plugin)]
#![plugin(tapioca)]

#[macro_use]
extern crate tapioca_derive;

#[derive(Schema)]
#[SchemaURL = "https://raw.githubusercontent.com/OAI/OpenAPI-Specification/OpenAPI.next/examples/v3.0/uber.yaml"]
struct UberAPI;

fn main() {
    UberAPI::products().get()
}
