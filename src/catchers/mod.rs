use rocket::Rocket;
use super::responder::ApiResponse;
use rocket::http::Status;

#[catch(500)]
fn internal_error() -> ApiResponse {
    ApiResponse { 
        json: json!({"status": 500, "message": "Internal Server Error"}),
        status: Status::InternalServerError
    }
}

#[catch(404)]
fn not_found() -> ApiResponse {
    ApiResponse {
        json: json!({"status": 404, "message": "Not Found"}),
        status: Status::NotFound
    }
}

#[catch(400)]
fn bad_request() -> ApiResponse {
    ApiResponse {
        json: json!({"status": 400, "message": "Bad Request"}),
        status: Status::BadRequest
    }
}

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.register(catchers![not_found, internal_error, bad_request])
}