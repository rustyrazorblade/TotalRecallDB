use std::time::{Duration, SystemTime};

pub struct Row {
    id: u64,
    created_at: SystemTime,
}