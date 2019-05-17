pub mod playing {
    use std::process::Command;

    use utils::structs::Status;
    use utils::traits::StatusAble;
    use std::default::Default;

    pub struct Playing();

    impl StatusAble for Playing {
        fn get_status(&self) -> Option<Status> {
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
            let mut artist_str = String::from_utf8_lossy(&artist.stdout).into_owned();

            if artist_str == "" {
                artist_str = "Unknown Artist".to_string()
            }

            artist_str = artist_str.trim_end_matches("\n").to_string();

            let track = Command::new("playerctl")
                                .arg("metadata")
                                .arg("title")
                                .output()
                                .expect("failed to execute process");
            let mut track_str = String::from_utf8_lossy(&track.stdout).into_owned();

            if track_str == "" {
                track_str = "Unknown Track".to_string();
            }

            track_str = track_str.trim_end_matches("\n").to_string();

            return Some( Status {
                full_text: format!("{} {} - {}", status_icon, track_str, artist_str),
                name: "playing".to_string(),
                ..Default::default()
            });
        }
    }
}
