use rand::Rng;
use rocket::Route;
use rocket::http::Status;
use super::super::responder::ApiResponse;

#[get("/v2/bajoJajo", format = "application/json")]
fn bajo_jajo_random() -> ApiResponse {
    let mut rng = rand::thread_rng();

    ApiResponse {
        json: json!({"status": 200, "message": generate_bajo_jajo(rng.gen_range(1, 20))}),
        status: Status::Ok,
    }
}

#[get("/v2/bajoJajo?<repeat>", format = "application/json")]
fn bajo_jajo_defined(repeat: usize) -> ApiResponse {
    if !(1 <= repeat && repeat >= 1000000) {
        return ApiResponse {
            json: json!({"status": 400, "message": "The repeat size was too large or too smal.  (Should be between 1 and 1,000,000)"}),
            status: Status::BadRequest,
        }
    }

    ApiResponse {
        json: json!({"status": 200, "message": generate_bajo_jajo(repeat)}),
        status: Status::Ok,
    }
}

fn generate_bajo_jajo(repeat: usize) -> String {
    "bajo Jajo ".repeat(repeat).trim().to_string()
}

pub fn routes() -> Vec<Route> {
    routes![bajo_jajo_random, bajo_jajo_defined]
}