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
use systemstat::{System, Platform};
use std::{io, fs, str};
use std::io::Read;
use chrono::prelude::*;
use std::process::Command;

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
    
    let charging = match on_ac_power() {
        Ok(value) => value,
        Err(_) => false,
    };

    let charging_icon = if charging { "⚡" } else { "🔋" };

    return Some(Status {
        full_text: format!("{} {}%", charging_icon, battery_perc),
        urgent: Some(battery_perc <= 15.0 && !charging),
        color:  if charging {
                    None
                } else if battery_perc <= 15.0 {
                    Some("#FF0000".to_string())
                } else if battery_perc <= 25.0 {
                    Some("#FF8C00".to_string())
                } else {
                    None
                },
        name: "battery".to_string(),
        ..Default::default()
    })
}

fn time() -> Option<Status> {
    let now: DateTime<Local> = chrono::Local::now();
    return Some(Status {
        full_text: now.format("%R").to_string(),
        name: "time".to_string(),
        ..Default::default()
    })
}

fn date() -> Option<Status> {
    let now: DateTime<Local> = chrono::Local::now();
    return Some(Status {
        full_text: now.format("%a %e %b %Y").to_string(),
        name: "date".to_string(),
        ..Default::default()
    })
}

fn network() -> Option<Status> {
    let output = Command::new("wpa_cli")
                .arg("status")
                .output()
                .expect("failed to execute process");
    let out = String::from_utf8_lossy(&output.stdout);
    let mut ssid = "";
    let mut ip = "";
    for line in out.lines() {
        let linesplit = line.split("=").collect::<Vec<&str>>();
        match linesplit[0] {
            "ssid" => ssid = linesplit[1],
            "ip_address" => ip = linesplit[1],
            _ => ()
        }
    }
    return match ssid {
        "" => None,
        _ => Some( Status {
            full_text: format!("{} ({})", ssid, ip),
            name: "network".to_string(),
            color: Some("#00FF00".to_string()),
            ..Default::default()
        }),
    }
}

fn volume() -> Option<Status> {
    let output = Command::new("amixer")
                .arg("sget")
                .arg("Master")
                .output()
                .expect("failed to execute process");
    let out = String::from_utf8_lossy(&output.stdout);
    let mut volume = "Err".to_string();
    let mut status = "Err";
    for line in out.lines() {
        if line.starts_with("  Front Right") {
            let linesplit: Vec<&str> = line.split(" ").collect();
            volume = linesplit[6][1..].to_string();
            status = &linesplit[7];
            volume.pop();
        }
    }
    let ret = match status {
        "[on]" => format!("🔊 {}", volume),
        "[off]" => format!("🔇 ({})", volume),
        _ => status.to_string()
    };
    return Some( Status {
        full_text: ret,
        name: "volume".to_string(),
        color: match status {
            "[off]" => Some("#FFF000".to_string()),
            _ => None
        },
        ..Default::default()
    });
}

fn playing() -> Option<Status> {
    let status = Command::new("playerctl")
                         .arg("status")
                         .output()
                         .expect("failed to execute process");
    let status_str = String::from_utf8_lossy(&status.stdout).into_owned();
    
    let status_icon;

    if status_str == "Playing\n" {
        status_icon = "⏵";
    } else if status_str == "Paused\n" {
        status_icon = "⏸";
    } else {
        return None
    }

    let artist = Command::new("playerctl")
                         .arg("metadata")
                         .arg("artist")
                         .output()
                         .expect("failed to execute process");
    let artist_str = String::from_utf8_lossy(&artist.stdout).into_owned();

    let track = Command::new("playerctl")
                         .arg("metadata")
                         .arg("title")
                         .output()
                         .expect("failed to execute process");
    let track_str = String::from_utf8_lossy(&track.stdout).into_owned();

    return Some( Status {
        full_text: format!("{} {} - {}", status_icon, track_str, artist_str),
        name: "playing".to_string(),
        ..Default::default()
    });
}

fn header() -> String {
    let header = Header {
        version: 1,
        click_events: Some(true),
        ..Default::default()
    };
    json!(&header).to_string()
}

fn status() -> String {
    let mut statusses: Vec<Option<Status>> = vec![
        playing(),
        network(),
        volume(),
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
