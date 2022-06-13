use super::{Module, StatusBlock, ModuleResult};

use async_trait::async_trait;

pub struct DummyModule {}

#[async_trait(?Send)]
impl Module for DummyModule {
    async fn get_blocks(&self) -> ModuleResult {
        Ok(vec![
            StatusBlock::new("dummy", "Hello world!")
                .with_instance("hello-world"),
            StatusBlock::new("dummy", "Urgent!")
                .with_instance("urgent")
                .with_urgent(true)
        ])
    }
}

impl DummyModule {
    // This is an example module, it is ok to be unused
    #[allow(dead_code)]
    pub fn new() -> Self {
        DummyModule {}
    }
}
