#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;

use rocket::outcome::Outcome;
use rocket::request::FromRequest;
use rocket::request::Request;
use rocket::request;
use rocket::request::FromForm;

#[derive(FromForm)]
struct QueryString {
    accessToken: String,
}

struct AccessToken(String);

impl<'a, 'r> FromRequest<'a, 'r> for AccessToken {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AccessToken, ()> {
        let accessToken = // HELP!;
        Outcome::Success(AccessToken(accessToken.to_string()))
    }
}

#[get("/hello/<name>?<query_string>")]
fn index(name: String, query_string: QueryString) -> String {
    format!("Hello {} {}", name, query_string.accessToken)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
