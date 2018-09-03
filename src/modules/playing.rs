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
    }
}