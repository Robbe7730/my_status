mod modules;
mod start_iterator;
mod config;
mod status_line_iterator;

use start_iterator::StartIterator;
use status_line_iterator::StatusLineIterator;

use std::time::Duration;
use std::thread::sleep;

fn main() {
    let start_iter = StartIterator::new();
    let status_line_iter = StatusLineIterator::new().map(|x| format!(
        "{},",
        serde_json::to_string(&x).unwrap()
    ));
    let lines_iter = start_iter.chain(status_line_iter);

    for line in lines_iter {
        println!("{}", line);
        sleep(Duration::from_secs(1));
    }
}
