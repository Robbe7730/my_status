extern crate serde;
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate systemstat;
extern crate blurz;

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

fn status(modules_vec: &Vec<Box<StatusAble>>) -> String {
    let mut statusses: Vec<Option<Status>> = modules_vec.into_iter().map(|module| module.get_status()).collect();
    statusses.retain(|ref x| x.is_some());
    json!(&statusses).to_string()
}

fn main() {
    let modules_vec: Vec<Box<StatusAble>> = vec![
        Box::new(Playing()),
        Box::new(Network()),
        Box::new(Bluetooth()),
        Box::new(Volume()),
        Box::new(Battery()),
        Box::new(Date()),
        Box::new(Time()),
    ];
    let header: String = header();
    println!("{}", header);
    println!("[");
    loop {
        let status: String = status(&modules_vec);
        println!("{},", status);
        sleep(Duration::from_millis(1000));
    }
}
