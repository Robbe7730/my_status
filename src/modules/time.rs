pub mod time {
    extern crate chrono;

    use chrono::prelude::{DateTime, Local};
    use utils::structs::Status;
    use utils::traits::StatusAble;
    use std::default::Default;

    pub struct Time();

    impl StatusAble for Time {
        fn get_status(&self) -> Option<Status> {
            let now: DateTime<Local> = Local::now();
            return Some(Status {
                full_text: now.format("%R").to_string(),
                name: "time".to_string(),
                ..Default::default()
            })  
        }
    }
}