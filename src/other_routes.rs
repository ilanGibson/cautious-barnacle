use rocket::fs::NamedFile;
use rocket::Route;

#[get("/game")]
async fn serve_game_page() -> Option<NamedFile> {
    NamedFile::open("static/html/game_page_test.html").await.ok()
}


pub fn routes() -> Vec<Route> {
    routes![serve_game_page]
}