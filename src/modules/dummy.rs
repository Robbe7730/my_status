use super::{Module, StatusBlock};

pub struct DummyModule {}

impl Module for DummyModule {
    fn get_blocks(&self) -> Vec<StatusBlock> {
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
