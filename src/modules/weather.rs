pub mod weather {
    use utils::structs::Status;
    use utils::traits::StatusAble;

    pub struct Weather();

    impl StatusAble for Weather {
        fn get_status(&self) -> Option<Status> {
            return Some(Status {
                full_text: "YEET".to_string(),
                name: "weather".to_string(),
                ..Default::default()
            })  
        }
    }
}
