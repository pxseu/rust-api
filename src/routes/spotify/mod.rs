use rocket::Route;
use rocket::http::Status;
use super::super::responder::ApiResponse;
use rocket::response::Redirect;
use std::env;

#[get("/v2/spotify/authorize")]
fn redirect_to_auth() -> Result<Redirect, ApiResponse> {
    let client_id = env::var("SPOTIFY_CLIENT_ID");

    if client_id.is_err() {
        return Err(
            ApiResponse {
                json: json!({"status": 500, "message":"I forgot to set the env."}),
                status: Status::InternalServerError
            }
        )
    }

    Ok(Redirect::to(format!("https://accounts.spotify.com/authorize?response_type=code&client_id={}&scope=user-read-playback-state&redirect_uri=https%3A%2F%2Fapi.pxseu.com%2Fv2%2Fspotify%2Fcallback", client_id.unwrap())))
}

#[get("/v2/spotify/callback?<code>")]
fn callback(code: String) -> ApiResponse {
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