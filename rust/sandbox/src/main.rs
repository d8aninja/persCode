use std::{path::PathBuf, time::Duration};

fn return_duration(s: u64) -> Duration {
    Duration::new(s, 0)
}

fn main() {
    let d = return_duration(5);
    dbg!(d);
}
