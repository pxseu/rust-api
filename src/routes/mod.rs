use rocket::Route;
use rocket::http::Status;
use super::responder::ApiResponse;

mod home;
mod bajo_jajo;
mod send_message;

#[get("/")]
fn index_page() -> ApiResponse {
    ApiResponse {
        json: json!({"message": "Welcome to the rewrite of the api!. It currently only works with request with the header `Accept` set to `application/json`.",
            "routes": [
                "/v2/bajoJajo",
                "/v2/sendMessage"
            ],"status": 200}),
        status: Status::Ok,
    }
}

pub fn routes() -> Vec<Route> {
    [
        routes![index_page],
        home::routes(),
        bajo_jajo::routes(),
        send_message::routes()
    ].concat()
}