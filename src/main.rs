use rand::Rng;
use rand::distributions::Alphanumeric;
use ring::digest;
use base64::{encode_config, URL_SAFE_NO_PAD};

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

// fn request_authorization() {
//     let client_id = "72ae29791cf14328b43c1c1c03fa19e8";
//     let redirect_uri = "http://localhost:8888/callback";

//     let scope = "user-read-private user-read-email";
//     let auth_url = format!("https://accounts.spotify.com/authorize");




// }

#[tokio::main]
async fn main() {
    let code_verifier: String = generate_random_string(64);
    let hashed = sha256(code_verifier.as_bytes()).await; // Await the hashed future
    let code_challenge = base64_encode(&hashed); // Await the hashed future and pass it to base64_encode
    println!("Code Challenge: {}", code_challenge);
}