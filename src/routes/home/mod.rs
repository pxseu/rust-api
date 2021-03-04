use rocket::Route;
use rocket::http::Status;
use super::super::responder::ApiResponse;

#[get("/v2")]
fn home() -> ApiResponse {
    ApiResponse {
        json: json!({"status": 200, "message": "Welcome to the rust rewrite of the PXSEU_V2 api!"}),
        status: Status::Ok,
    }
}

pub fn routes() -> Vec<Route> {
    routes![home]
}