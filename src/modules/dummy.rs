use super::{Module, StatusBlock};

use async_trait::async_trait;

pub struct DummyModule {}

#[async_trait(?Send)]
impl Module for DummyModule {
    async fn get_blocks(&self) -> Vec<StatusBlock> {
        vec![
            StatusBlock::new("dummy", "Hello world!")
                .with_instance("hello-world"),
            StatusBlock::new("dummy", "Urgent!")
                .with_instance("urgent")
                .with_urgent(true)
        ]
    }
}

impl DummyModule {
    pub fn new() -> Self {
        DummyModule {}
    }
}
