#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate dotenv;

use dotenv::dotenv;

mod routes;
mod responder;
mod catchers;

fn main() {
    dotenv().ok();

    rocket::ignite()
        .mount("/", routes::routes())
        .register(catchers::catchers())
        .launch();
}
