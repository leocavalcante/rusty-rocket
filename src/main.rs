#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate log;
extern crate simple_logger;

extern crate rocket;
extern crate url;

use rocket::http::uri::URI;
use rocket::outcome::Outcome;
use rocket::request;
use rocket::request::FromRequest;
use rocket::request::Request;
use std::collections::HashMap;
use url::form_urlencoded::Parse;
use url::Url;

struct AccessToken(String);

impl<'a, 'r> FromRequest<'a, 'r> for AccessToken {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AccessToken, ()> {
        let uri: &URI = request.uri();
        let uri_str: &str = uri.as_str();
        let url_str: String = format!("{}{}", "fakescheme:/", uri_str);
        let maybe_url: Result<Url, _> = Url::parse(&url_str);

        let url: Url = match maybe_url {
            Ok(u) => u,
            _ => panic!("Error parsing url"),
        };

        let pairs: Parse = url.query_pairs();
        let query_hash: HashMap<String, String> = pairs.into_owned().collect();
        let access_token: String = query_hash.get("access_token").unwrap().to_string();

        Outcome::Success(AccessToken(access_token))
    }
}

#[get("/hello/<name>")]
fn index(access_token: AccessToken, name: String) -> String {
    format!("Hello {} {}", name, access_token.0)
}

fn main() {
    simple_logger::init().unwrap();
    rocket::ignite().mount("/", routes![index]).launch();
}
