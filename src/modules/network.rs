pub mod network {
    use std::process::Command;

    use utils::structs::Status;
    use utils::traits::StatusAble;
    use std::default::Default;

    pub struct Network();

    impl StatusAble for Network {
        fn get_status(&self) -> Option<Status> {
            let output = Command::new("wpa_cli")
                        .arg("status")
                        .arg("-iwlo1")
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
    }
}
