use crate::modules::{Module, self};
use crate::modules::status_block::StatusBlock;

pub struct StatusLineIterator {
    modules: Vec<Box<dyn Module>>
}

impl Iterator for StatusLineIterator {
    type Item = Vec<StatusBlock>;

    fn next(&mut self) -> Option<Vec<StatusBlock>> {
        Some(self.modules.iter().map(|module| {
            module.get_blocks()
        }).flatten().collect())
    }
}

impl StatusLineIterator {
    pub fn new() -> Self {
        Self {
            modules: vec![
                Box::new(modules::BatteryModule::new()),
                Box::new(modules::DateTimeModule::new()),
            ]
        }
    }
}
