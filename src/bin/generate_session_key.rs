use actix_web::cookie::Key;
use base64::{Engine, engine::general_purpose::STANDARD as BASE64};

fn main() {
    let key = Key::generate();
    let master_key = key.master();
    let encoded_key = BASE64.encode(master_key);
    println!("Generated session key (base64): {}", encoded_key);
}
