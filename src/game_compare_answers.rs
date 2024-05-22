use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::Route;

#[derive(Deserialize, Debug)]
struct CompareRequest {
    guess: String,
    track_name: String,
    mode: i8,
}

#[derive(Serialize)]
struct CompareResponse {
    same: bool,
}

#[post("/compare", data = "<request>")]
fn compare_song_title(request: Json<CompareRequest>) -> Json<CompareResponse> {
    let item1 = &request.guess;
    let item2 = &request.track_name;
    let mode = &request.mode;

    let same = (item1 == item2) && (*mode == 0);

    Json(CompareResponse { same })
}

pub fn routes() -> Vec<Route> {
    routes![compare_song_title]
}