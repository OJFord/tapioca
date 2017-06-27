#![feature(associated_consts)]
#![feature(use_extern_macros)]

#[macro_use]
extern crate tapioca;

infer_api!(httpbin, "https://raw.githubusercontent.com/OJFord/tapioca/master/tests/schemata/httpbin.yml");

use httpbin::basic_auth__user__hunter_2 as basic_auth;
use basic_auth::get::OpAuth::HttpBasic;

static USER: &str = "baz";

fn main() {
    let auth = httpbin::ServerAuth::new();

    match httpbin::ip::get(auth) {
        Ok(response) => match response.body() {
            httpbin::ip::get::OkBody::Status200(body) => println!("Your IP is {}", body.origin),
            _ => println!("httbin.org did something unexpected"),
        },
        Err(_) => println!("httpbin.org errored"),
    }

    let user_id = basic_auth::ResourceId_user::from_static(USER);
    let auth = HttpBasic((USER.into(), "hunter2".into()).into());
    match basic_auth::get(&user_id, auth.into()) {
        Ok(response) => match response.body() {
            basic_auth::get::OkBody::Status200(body) => if body.authenticated {
                println!("User '{}' authenticated OK!", body.user)
            } else {
                println!("Authentication failed for user '{}'!", body.user)
            },
            _ => println!("httbin.org did something unexpected"),
        },
        Err(_) => println!("httpbin.org errored"),
    }
}
