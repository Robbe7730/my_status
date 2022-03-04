mod modules;
mod header;
mod config;
mod status_lines;

use header::Header;
use status_lines::StatusLines;

use std::time::Duration;
use std::thread::sleep;

#[tokio::main]
async fn main() {
    println!("{}", Header::new().get_value().unwrap());
    let mut lines = StatusLines::new();
    loop {
        let line = lines.next().await.unwrap();
        println!("{},", serde_json::to_string(&line).unwrap());
        sleep(Duration::from_secs(1));
    }
}
