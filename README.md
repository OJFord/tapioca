Tapioca
=======

_**T**yped **API**s (that **O**llie **C**oshed into an **A**cronym)_

[![Crate](https://img.shields.io/crates/v/tapioca.svg)](https://crates.io/crates/tapioca)
[![Build Status](https://travis-ci.org/OJFord/tapioca.svg?branch=master)](https://travis-ci.org/OJFord/tapioca)

_tapioca_ is an HTTP client for _[rust](https://github.com/rust-lang/rust)_ that
aims to help the compiler help _you_ to access REST+JSON APIs in a type-safer
manner.

It uses the [OpenAPI Initiative's schema specification](https://github.com/OAI/OpenAPI-Specification)
to infer types for path and query parameters, request and response bodies, et
al. and then [serde](serde-rs/json) to de/serialise them.

```rust
infer_api!(service, "https://service.api/schema.yml")
use service::path;

fn main() {
    let auth = service::ServerAuth::new();

    match path::get(&auth) {
        Ok(response) => match response.body() {
            path::OkBody::Status200(body) => println!("Thing is: {}", body.thing),
            path::OkBody::UnspecifiedCode(body) => {
                // We're forced to handle every status code in the schema;
                //  including the possibility that the server replies off-script.
                println!("I don't know what thing is!")
            },
        },
        Err(response) => match response.body() {
            path::ErrBody::Status403(body) => println!("That's not my thing"),
            path::ErrBody::UnspecifiedCode(_)
            | path::ErrBody::MalformedJson(_)
            | path::ErrBody::NetworkFailure() => println!("Something went wrong"),
        },
    }
}
```

So, we can pattern-match responses by status code, and access the JSON response
as a _rust_ type.

_tapioca_ also aims to prevent you from shooting yourself in the foot with an
invalid _sequence_ of requests, such as '`GET` after `DELETE`' on a particular
resource: this is achieved by constructing resource IDs only from responses,
and static values. `DELETE` functions cause the resource ID argument to be
_moved_ (while other methods only _borrow_) preventing it from being further
used.
