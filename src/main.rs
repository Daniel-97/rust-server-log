#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/logs")]
fn get_all_logs() -> [&'static str; 3]{
    return ["log1", "log2", "log3"];
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
    //rocket::ignite().mount("/logs", routes![get_all_logs]).launch();
}
