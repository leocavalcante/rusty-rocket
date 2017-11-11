#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/hello/<name>")]
fn index(name: String) -> String {
    format!("Hello {}", name)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
