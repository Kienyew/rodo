use std::time::{SystemTime, UNIX_EPOCH};

/// Return the time since UTC 1st January 1970 00:00 in milliseconds
pub fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}
