use std::env;
use rocket::Route;
use rocket::http::Status;
use rocket::response::Redirect;
use super::super::responder::ApiResponse;
use url::form_urlencoded;

#[get("/v2/spotify/authorize")]
fn redirect_to_auth() -> Result<Redirect, ApiResponse> {
    let client_id = env::var("SPOTIFY_CLIENT_ID");
    let redirect_uri = env::var("SPOTIFY_REDIRECT_URI");
    
    let scopes = "user-read-playback-state";

    if client_id.is_err() || redirect_uri.is_err() {
        return Err(
            ApiResponse {
                json: json!({"status": 500, "message":"I forgot to set the env."}),
                status: Status::InternalServerError
            }
        )
    }

    Ok(Redirect::to(
        form_urlencoded::Serializer::new("https://accounts.spotify.com/authorize?response_type=code".to_string())
            .append_pair("client_id", &client_id.unwrap())
            .append_pair("scope", scopes)
            .append_pair("redirect_uri", &redirect_uri.unwrap())
        .finish()
    ))
}

#[get("/v2/spotify/callback?<code>&<error>")]
fn callback(code: Option<String>, error: Option<String>) -> ApiResponse {
    if error.is_some() {
        return ApiResponse {
            json: json!({"status": 400, "message": format!("Error: {}", error.unwrap())}),
            status: Status::BadRequest,
        }
    } 

    if code.is_none() {
        return ApiResponse {
            json: json!({"status": 400, "message": "Bad."}),
            status: Status::BadRequest,
        }
    }

    ApiResponse {
        json: json!({"status": 200, "message": "Too lazy to implement.", "code": code}),
        status: Status::Ok,
    }
}

#[get("/v2/spotify/playing/<user>")]
fn current_song(user: String) -> ApiResponse {
    ApiResponse {
        json: json!({"status": 200, "message": "Too lazy to implement.", "user": user}),
        status: Status::Ok,
    }
}

pub fn routes() -> Vec<Route> {
    routes![redirect_to_auth, callback, current_song]
}