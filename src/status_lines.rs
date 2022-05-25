use crate::modules::{Module, self};
use crate::modules::status_block::StatusBlock;

use futures::future::join_all;

pub struct StatusLines {
    modules: Vec<Box<dyn Module>>
}

impl StatusLines {
    pub fn new() -> Self {
        Self {
            modules: vec![
                Box::new(modules::BluetoothModule::new()),
                Box::new(modules::NetworkModule::new()),
                Box::new(modules::BatteryModule::new()),
                Box::new(modules::DateTimeModule::new()),
            ]
        }
    }

    pub async fn next(&mut self) -> Option<Vec<StatusBlock>> {
        Some(
            join_all(
                self.modules.iter()
                    .map(|module| {
                        module.get_blocks()
                    })
            )
            .await
            .into_iter()
            .flatten()
            .collect()
        )
    }
}
