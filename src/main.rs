use rocket::fs::NamedFile;
use std::path::Path;
use std::path::PathBuf;


#[macro_use] extern crate rocket;

#[get("/<file..>", rank = 2)] // Define a catch-all route for serving static files
async fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").await.ok()
}

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

// #[tokio::main]
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // let static_files = StaticFiles::from("./static/script.js");
    let _rocket = rocket::build()
        .mount("/", routes![world, index, static_files])
        .launch()
        .await?;

    Ok(())
}