extern crate serde;
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

extern crate systemstat;
extern crate chrono;

use std::time::Duration;
use std::thread::sleep;
use systemstat::{System, Platform};
use std::{io, fs, str};
use std::io::Read;
use chrono::prelude::*;
use systemstat::IpAddr::*;

#[derive(Serialize, Deserialize, Default)]
struct Header {
    version: u8,

    #[serde(skip_serializing_if = "Option::is_none")]
    stop_signal: Option<u8>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    cont_signal: Option<u8>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    click_events: Option<bool>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
struct Status {
    full_text: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    short_text: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    background: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    border: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    min_width: Option<u32>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    align: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    urgent: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    separator: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    separator_block_width: Option<u32>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    markup: Option<String>,
}

fn read_file(path: &str) -> io::Result<String> {
    let mut s = String::new();
    fs::File::open(path)
        .and_then(|mut f| f.read_to_string(&mut s))
        .map(|_| s)
}

fn value_from_file<T: str::FromStr>(path: &str) -> io::Result<T> {
    try!(read_file(path))
        .trim_right_matches("\n")
        .parse()
        .and_then(|n| Ok(n))
        .or_else(|_| {
            Err(io::Error::new(io::ErrorKind::Other,
                               format!("File: \"{}\" doesn't contain an int value", &path)))
        })
}

fn on_ac_power() -> io::Result<bool> {
    value_from_file::<i32>("/sys/class/power_supply/AC0/online").map(|v| v == 1)
}

fn battery() -> Option<Status> {
    let sys = System::new();
    let battery_perc = match sys.battery_life() {
        Ok(battery) =>  (battery.remaining_capacity*100.0).round(),
        Err(_) => -1.0,
    };
    
    let charging_icon = match on_ac_power() {
        Ok(value) => if value { "⚡" } else { "🔋" },
        Err(_) => "Err",
    };

    return Some(Status {
        full_text: format!("{} {}%", charging_icon, battery_perc),
        urgent: Some(battery_perc <= 15.0),
        color:  if battery_perc <= 15.0 {
                    Some("#FF0000".to_string())
                } else if battery_perc <= 25.0 {
                    Some("#FF8C00".to_string())
                } else {
                    None
                },
        ..Default::default()
    })
}

fn time() -> Option<Status> {
    let now: DateTime<Local> = chrono::Local::now();
    return Some(Status {
        full_text: now.format("%R").to_string(),
        ..Default::default()
    })
}

fn date() -> Option<Status> {
    let now: DateTime<Local> = chrono::Local::now();
    return Some(Status {
        full_text: now.format("%e %b %Y").to_string(),
        ..Default::default()
    })
}

fn network() -> Option<Status> {
    let sys = System::new();
    return match sys.networks() {
        Ok(interfaces) => match interfaces["wlp3s0"].addrs[0].addr {
                            Empty => None,
                            Unsupported => None,
                            V4(ip) => Some(Status {
                                                full_text: ip.to_string(),
                                                color: Some("#00FF00".to_string()),
                                                ..Default::default()
                                            }),
                            V6(ip) => Some(Status {
                                                full_text: ip.to_string(),
                                                color: Some("#00FF00".to_string()),
                                                ..Default::default()
                                            }),
                        }
        Err(_) => None
    }
}

fn header() -> String {
    let header = Header {
        version: 1,
        ..Default::default()
    };
    json!(&header).to_string()
}

fn status() -> String {
    let mut statusses: Vec<Option<Status>> = vec![
        network(),
        battery(),
        date(),
        time(),
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
