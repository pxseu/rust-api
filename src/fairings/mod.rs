use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;

pub struct ServerName();

impl Fairing for ServerName {
    fn info(&self) -> Info {
        Info {
            name: "Change the server name",
            kind: Kind::Response
        }
    }

    fn on_response(&self, _request: &Request, response: &mut Response) {
        response.set_header(Header::new("Server", "pxseu_api_v2"));
    }
}

