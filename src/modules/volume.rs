pub mod volume {
    use std::process::Command;

    use utils::structs::Status;
    use utils::traits::StatusAble;
    use std::default::Default;

    pub struct Volume();

    impl StatusAble for Volume {
        fn get_status(&self) -> Option<Status> {
            let output = Command::new("amixer")
                        .arg("sget")
                        .arg("Master")
                        .output()
                        .expect("failed to execute process");
            let out = String::from_utf8_lossy(&output.stdout);
            let mut volume = "Err".to_string();
            let mut status = "Err";
            for line in out.lines() {
                if line.starts_with("  Front") {
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
    }
}
