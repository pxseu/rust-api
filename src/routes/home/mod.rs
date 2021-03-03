use rocket::Rocket;
use rocket::http::Status;
use super::super::responder::ApiResponse;

#[get("/", format = "application/json")]
fn home() -> ApiResponse {
    ApiResponse {
        json: json!({"status": 200, "message": "Welcome to the rust rewrite of the PXSEU_V2 api!"}),
        status: Status::Ok,
    }
}

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/v2", routes![home])
}