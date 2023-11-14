extern crate crypto;

use chrono::{Duration, Utc};
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1::Sha1;
use std::fmt::Write;

pub fn time_in_seconds() -> i64 {
    let expiration_time = Utc::now() + Duration::hours(1);
    // Convert expiration time to epoch time (in seconds)
    println!("{}", expiration_time.timestamp());
    expiration_time.timestamp()
}

pub fn gen_signature(token: String, expire_time: i64) -> String {
    let private_key = "private_Ji6qXhiZ1ziZpvYb5xcaj/NhxTU=";

    let mut hmac = Hmac::new(Sha1::new(), private_key.as_bytes());
    let data = format!("{}{}", token, expire_time);
    hmac.input(data.as_bytes());

    let result = hmac.result();
    let code = result.code();

    let mut hex_digest = String::with_capacity(code.len() * 2);
    for byte in code.iter() {
        write!(&mut hex_digest, "{:02x}", byte).expect("Failed to write to String");
    }

    hex_digest
}
