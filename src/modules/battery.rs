pub mod battery {
    use utils::traits::StatusAble;
    use utils::structs::Status;
    
    use systemstat::{System, Platform};
    use std::{io, fs, str};
    use std::io::Read;

    pub struct Battery();

    impl StatusAble for Battery {
        fn get_status(&self) -> Option<Status> {
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

}