use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_random_port() -> u16 {
    let now = SystemTime::now().duration_since(UNIX_EPOCH);
    match now {
        Ok(duration) => {
            let millis = duration.as_millis() as u16;
            8080 + (millis % 1000)
        }
        Err(_) => 8080,
    }
}
