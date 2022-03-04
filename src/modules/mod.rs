mod dummy;
mod battery;

pub use dummy::DummyModule;
pub use self::battery::BatteryModule;

pub mod status_block;

use status_block::StatusBlock;

pub trait Module {
    fn get_blocks(&self) -> Vec<StatusBlock>;
}
