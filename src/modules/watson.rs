pub mod watson {
    use std::process::Command;

    use utils::structs::Status;
    use utils::traits::StatusAble;
    use std::default::Default;

    pub struct Watson();

    impl StatusAble for Watson {
        fn get_status(&self) -> Option<Status> {
            let project_output = Command::new("watson")
                        .arg("status")
                        .arg("-p")
                        .output()
                        .expect("failed to execute process");
            let project_out = String::from_utf8_lossy(&project_output.stdout);
            let project_out_stripped = project_out.strip_suffix("\n")?;
            
            if project_out_stripped == "No project started." {
                return None;
            }

            let time_output = Command::new("watson")
                        .arg("status")
                        .arg("-e")
                        .output()
                        .expect("failed to execute process");
            let time_out = String::from_utf8_lossy(&time_output.stdout);
            let time_out_stripped = time_out.strip_suffix("\n")?;
            return Some( Status {
                full_text: format!("{} ({})", project_out_stripped, time_out_stripped),
                name: "watson".to_string(),
                ..Default::default()
            });
        }
    }
}
