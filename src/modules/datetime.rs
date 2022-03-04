use super::{Module, StatusBlock};
use chrono::Local;

pub struct DateTimeModule {}

impl Module for DateTimeModule {
    fn get_blocks(&self) -> Vec<StatusBlock> {
        let now = Local::now();
        vec![
            StatusBlock::new(
                "datetime", 
                &now.format("%a %e %b %Y").to_string()
            ).with_instance("date"),
            StatusBlock::new(
                "datetime", 
                &now.format("%R").to_string()
            ).with_instance("time")
        ]
    }
}

impl DateTimeModule {
    pub fn new() -> Self {
        Self {}
    }
}
