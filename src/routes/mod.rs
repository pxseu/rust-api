use rocket::Rocket;
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

pub fn mount(rocket: Rocket) -> Rocket {
    let mut _rocket = rocket;
    _rocket = _rocket.mount("/", routes![index_page]);
    _rocket = home::mount(_rocket);
    _rocket = bajo_jajo::mount(_rocket);
    _rocket = send_message::mount(_rocket);
    _rocket
}