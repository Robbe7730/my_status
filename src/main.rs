extern crate serde;
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

extern crate systemstat;
extern crate chrono;
extern crate libpulse_binding;
extern crate wpactrl;

use std::time::Duration;
use std::thread::sleep;

pub mod modules;
pub mod utils;

use utils::structs::*;
use utils::traits::*;
use modules::*;

fn header() -> String {
    let header = Header {
        version: 1,
        click_events: Some(true),
        ..Default::default()
    };
    json!(&header).to_string()
}

fn status() -> String {
    let date: Date = Date();
    let time: Time = Time();
    let battery: Battery = Battery();
    let network: Network = Network();
    let volume: Volume = Volume();
    let playing: Playing = Playing();

    let mut statusses: Vec<Option<Status>> = vec![
        playing.get_status(),
        network.get_status(),
        volume.get_status(),
        battery.get_status(),
        date.get_status(),
        time.get_status(),
    ];
    statusses.retain(|ref x| x.is_some());
    json!(&statusses).to_string()
}

fn main() {
    let header: String = header();
    println!("{}", header);
    println!("[");
    loop {
        let status: String = status();
        println!("{},", status);
        sleep(Duration::from_millis(1000));
    }
}
