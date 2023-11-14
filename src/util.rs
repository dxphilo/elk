use chrono::{Duration, Utc};

pub fn time_in_seconds() -> i64 {
    let expiration_time = Utc::now() + Duration::hours(1);
    // Convert expiration time to epoch time (in seconds)
    println!("{}",expiration_time.timestamp());
    expiration_time.timestamp()
}