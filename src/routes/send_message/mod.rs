use rocket::Route;
use rocket::http::Status;
use rocket_contrib::json::{Json};
use super::super::responder::ApiResponse;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiSendMessage {
    pub message: String,
}

#[post("/v2/sendMessage", data = "<data>")]
fn send_message(data: Json<ApiSendMessage>) -> ApiResponse {
    if data.message.trim() == "" {
        return ApiResponse {
            json: json!({"status": 400, "message":"The message cannot be an emtpy string."}),
            status: Status::BadRequest
        };
    }

    let webhook_url = env::var("WEBHOOK");

    if webhook_url.is_err() {
        return ApiResponse {
            json: json!({"status": 500, "message":"I forgot to set the env."}),
            status: Status::InternalServerError
        };
    }

    let response = ureq::post(&webhook_url.unwrap())
    .send_json(ureq::json!({
        "username": "test", "content": "hello"
    }));

    if response.status() == 429 {
        return ApiResponse {
            json: json!({"status": 503, "message":"The backend webhook is getting rate limited. Please try again later."}),
            status: Status::ServiceUnavailable
        };
    }

    ApiResponse {
        json: json!({"status": 200, "message": "Sent."}),
        status: Status::Ok,
    }
}

pub fn routes() -> Vec<Route> {
    routes![send_message]
}