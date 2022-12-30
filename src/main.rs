#![feature(proc_macro_hygiene, decl_macro)]

use rocket::response::content;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/logs")]
fn get_all_logs() -> content::Json<&'static str>{
    content::Json("[\"log1\",\"log2\",\"log3\"]")
}

fn main() {
    rocket::ignite().mount("/", routes![get_all_logs]).launch();
    //rocket::ignite().mount("/logs", routes![get_all_logs]).launch();
}
