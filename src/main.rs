use rand::Rng;
use rand::distributions::Alphanumeric;
use ring::digest;
use base64::{encode_config, URL_SAFE_NO_PAD};
use rocket::response::Redirect;
use url::Url;

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

async fn sha256(plain: &[u8]) -> Vec<u8> {
    let hashed = digest::digest(&digest::SHA256, plain);
    hashed.as_ref().to_vec()
}

fn base64_encode(input: &[u8]) -> String {
    encode_config(input, URL_SAFE_NO_PAD)
}

async fn generate_code_challenge() -> (String, String) {
    let code_verifier: String = generate_random_string(64);
    let hashed = sha256(code_verifier.as_bytes()).await; // Await the hashed future
    let code_challenge = base64_encode(&hashed); // Await the hashed future and pass it to base64_encode
    (code_challenge, code_verifier)
}

async fn generate_spotify_authorization_url() -> Url {
    let client_id = "72ae29791cf14328b43c1c1c03fa19e8";
    let redirect_uri = "http://localhost:8000/world";
    let scope = "user-read-private user-read-email";
    let mut auth_url = Url::parse("https://accounts.spotify.com/authorize").unwrap();

    let (code_challenge, _code_verifier) = generate_code_challenge().await;

    let params = vec![
        ("response_type", "code"),
        ("client_id", client_id),
        ("scope", scope),
        ("code_challenge_method", "S256"),
        ("code_challenge", code_challenge.as_str()),
        ("redirect_uri", redirect_uri),
    ];

    // Map the params to a query string and set it to the URL
    auth_url.set_query(Some(&params.iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<String>>().join("&")));

    auth_url // Return the URL
}

// async fn generate_spotify_access_token() -> () {
//     // let codeVerifier = 
// }

#[get("/please")]
async fn spotify_authorize() -> Redirect {
    let url = generate_spotify_authorization_url().await;
    Redirect::to(url.to_string())
}



#[get("/world?<code>")]
async fn world(code: String) -> String {
    // let auth_code = code;
    // println!("{}", auth_code);
    format!("{}", code)
    // rocket::fs::NamedFile::open("static/index.html").await.unwrap();
}


// #[tokio::main]
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![world, spotify_authorize])
        .launch()
        .await?;

    Ok(())
}