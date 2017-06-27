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

## Getting started

In order to start using tapioca in your project, the first step is to locate the OAS schema for the API you wish to use. Let's assume it's at `https://example.org/schema.yml`. Then, add the latest version to your `Cargo.toml` as usual, and import tapioca with macros:
```rust
#[macro_use]
extern crate tapioca;
```
and invoke the `infer_api` macro to build a client for the API:
```rust
infer_api!(example, "https://example.org/schema.yml");
```

The macro expands at compile-time, building a typed client in-place; (almost) all the code it generates will be located under a module named `example`, or whatever we specify in the first argument. The only exception is two crates (which must be loaded at the root level) which are needed to be externed inside your crate in order to use their macros - at least for now, the Rust's macro system is seeing a lot of change, and this may be improved. These are `serde_derive` and `tapicoa_codegen`; consequently, they also need to be in your `Cargo.toml`, but any other crates used (not for macros) by tapicoa will _not_ need this treatment, or pollute your project's namespace.

## Accessing the client

The module built by `infer_api` contains modules with the names of each of the paths available on an API, and each of those contains a function for each of the HTTP methods valid for that resource. For example, to `GET /foobars`, the function ident is:
```rust
example::foobars::get
```

In order to call this function, we might need to supply some arguments for [authentication](#authentication), [query parameters](#query-parameters), [request body](#request-bodies), et al. - the types for these are located inside a module of the same name as the function, for example:
```rust
example::foobars::get::QueryParams
```

## Authentication

Before we make a request, we need to introduce authentication - currently, authentication must be specified for every request, even if null.

Authentication requirements in an OAS schema are specified at two levels: server-wide, and operation specific - `GET /foobars` can have different requirements to other operations, which may just inherit from the server requirement. Thus we have two `enum`s of acceptable authentication schemes:
```rust
example::ServerAuth
example::foobars::get::OpAuth
```
which must be used depends on whether the operation `examples::foobars::get` overrides the server-wide authentication requirement - but the type-checker will tell us if we get it wrong.

If there's no authentication required at all, we can just use:
```rust
example::ServerAuth::new();
```

If it's HTTP Basic, then (depending on whether it's a server or operation requirement):
```rust
example::ServerAuth::Basic(username: String, password:String);
example::foobars::get::OpAuth::Basic(username: String, password:String);
```

If it's a custom header:
```rust
example::ServerAuth::ApiKey(api_key: String);
example::foobars::get::OpAuth::ApiKey(api_key: String);
```

Though note that the variant identifier, e.g. `Basic` or `ApiKey`, depends on the name used in the OAS schema. This is because there may be multiple definitions of the same type.

## Making a request

Now that we've seen how to construct an authentication argument, we can actually `GET` some `foobars`!

```rust
let auth = examples::ServerAuth::new();
let response = examples::foobars::get(&auth);
```

`response` is actually a `Result<Response, Response>`: if the response status code is an error, we get an `Err(response)`, otherwise it's an `Ok(response)`. This means we can use `response.is_ok`, `response.is_err`, and pattern matching:
```rust
match examples::foobars::get(&auth) {
    Ok(response) => foobar_handler(response),
    Err(response) => err_handler(response),
}
```

We can use further pattern matching in each of these handlers, in order to respond differently to different status codes:
```rust
fn foobar_handler(response: Response) {
    match response.body {
        OkBody::Status200(body) => {
            for foobar in body.the_foobars {
                println!("Foobar {} is named {}", foobar.id, foobar.name);
            }
        },
        OkBody::UnspecifiedCode(body)
        | OkBody::MalformedJson(body) => something_else(),
    }
}
```
where we always have `UnspecifiedCode` (one not in the schema) and `MalformedJson` (invalid JSON, or did not match schema) as well as a `StatusXXX` for each of the possibilities specified in the schema. `err_handler` would look similar, with `ErrBody::Status403`, etc.

## Request bodies

Say this `example::foobars` collection also supports `POST`ing new `foobar`s, we can supply the request body to create one like this:
```rust
let body = example::foobars::post::RequestBody {
    name: "Foobarry".into(),
    age: 12,
    email: None,
};
```

The structure and field types of the body is fully defined by the schema, and may include:
  - `i32`, `i64`
  - `bool`
  - `String`
  - `Option<_>`
  - `Vec<_>`
  - further `struct`s

## Query parameters

Query parameters are supplied much like [request bodies](#request-bodies):
```rust
let query = example::foobars::get::QueryParams {
    age: 34,
};
```

## Path parameters

Path parameters are slightly different. Because of the need to distinguish `example::foobars::get` from a `GET` on a single resource in that collection, the name of the path parameter is encoded in the path module name, for example:
```rust
example::foobars__id_::get
```

If the API specifies two resource identifiers in a row, this would be `foobars__id1___id2_`. This gets ugly, and may be changed in a future version.

Path parameters can be constructed from the response, for example when creating a new resource and the server generated its ID, or from a static reference:
```rust
static provisioned_id = "fea3c8e91baa1";

fn main() {
    let auth = example::ServerAuth::new();
    let resource = examples::foobars__id_::Resource_id::from_static(provisioned_id);

    example::foobars__id_::get(&resource, &auth);
}
```
