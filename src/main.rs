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
    NamedFile::open("static/html/initial_page.html").await.ok()
}

#[get("/<code>")]
async fn my_handler(code: &str) -> Option<NamedFile> {
    println!("code: {}", code);
    NamedFile::open("static/html/initial_page.html").await.ok()
}

#[get("/world")]
async fn world() -> &'static str {
    "Hello, world!"
}

// #[tokio::main]
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index, static_files, my_handler, world])
        .launch()
        .await?;

    Ok(())
}