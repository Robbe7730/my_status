use super::{Module, StatusBlock};
use chrono::Local;

use async_trait::async_trait;

pub struct DateTimeModule {}

#[async_trait(?Send)]
impl Module for DateTimeModule {
    async fn get_blocks(&self) -> Vec<StatusBlock> {
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
