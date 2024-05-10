use rand::Rng;
use rand::distributions::Alphanumeric;
use rocket::response::Redirect;
use rocket::http::CookieJar;

#[macro_use] extern crate rocket;


fn generate_random_string(length: usize) -> String {
    let rng = rand::thread_rng();
    let random_string: String = rng
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    random_string
}


async fn generate_code_challenge() -> String {
    let code_verifier: String = generate_random_string(64);

    code_verifier
}


#[get("/world")]
async fn world(cookies: &CookieJar<'_>) -> Redirect {
    let code = generate_code_challenge().await;
    cookies.add(("message", code));
    Redirect::to(uri!(index))
}

#[get("/index")]
fn index(cookies: &CookieJar<'_>) -> Option<String> {
    cookies.get("message").map(|crumb| format!("Message: {}", crumb.value()))
}

// #[tokio::main]
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![world, index])
        .launch()
        .await?;

    Ok(())
}