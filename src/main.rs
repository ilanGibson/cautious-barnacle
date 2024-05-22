use rocket::fs::NamedFile;
use std::path::Path;
use std::path::PathBuf;

mod other_routes;
mod game_compare_answers;


#[macro_use] extern crate rocket;

#[get("/<file..>", rank = 2)] // Define a catch-all route for serving static files
async fn serve_static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[get("/")]
async fn serve_initial_page() -> Option<NamedFile> {
    NamedFile::open("static/html/initial_page.html").await.ok()
}

#[tokio::main]
// #[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![serve_initial_page, serve_static_files])
        .mount("/", other_routes::routes())
        .mount("/", game_compare_answers::routes())
        .launch()
        .await?;
    Ok(())
}