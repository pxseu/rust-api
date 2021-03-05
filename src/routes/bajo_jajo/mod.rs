use rand::Rng;
use rocket::Route;
use rocket::http::Status;
use super::super::responder::ApiResponse;

#[get("/v2/bajoJajo?<repeat>")]
fn bajo_jajo(repeat: Option<usize>) -> ApiResponse {
    let local_repeat = if repeat.is_none() {
        let mut rng = rand::thread_rng();
        rng.gen_range(1, 20)
    } else { repeat.unwrap() };

    if  1 <= local_repeat && local_repeat >= 1000000 {
        return ApiResponse {
            json: json!({"status": 400, "message": "The repeat size was too large or too smal. (Should be between 1 and 1,000,000)"}),
            status: Status::BadRequest,
        }
    }

    ApiResponse {
        json: json!({"status": 200, "message": generate_bajo_jajo(local_repeat)}),
        status: Status::Ok,
    }
}

fn generate_bajo_jajo(repeat: usize) -> String {
    "bajo Jajo ".repeat(repeat).trim().to_string()
}

pub fn routes() -> Vec<Route> {
    routes![bajo_jajo]
}