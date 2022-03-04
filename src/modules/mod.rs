mod dummy;

pub use dummy::DummyModule;

pub mod status_block;

use status_block::StatusBlock;

pub trait Module {
    fn get_blocks(&self) -> Vec<StatusBlock>;
}
