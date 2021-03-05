#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate dotenv;
extern crate url;

use dotenv::dotenv;

mod routes;
mod catchers;
mod fairings;
mod responder;

fn main() {
    dotenv().ok();

    rocket::ignite()
        .attach(fairings::ServerName())
        .mount("/", routes::routes())
        .register(catchers::catchers())
        .launch();
}
