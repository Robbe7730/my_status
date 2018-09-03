pub mod date {
    extern crate chrono;

    use chrono::prelude::{DateTime, Local};
    use utils::structs::Status;
    use utils::traits::StatusAble;
    use std::default::Default;

    pub struct Date();

    impl StatusAble for Date {
        fn get_status(&self) -> Option<Status> {
            let now: DateTime<Local> = Local::now();
            return Some(Status {
                full_text: now.format("%a %e %b %Y").to_string(),
                name: "date".to_string(),
                ..Default::default()
            })  
        }
    }
}