mod dummy;
mod battery;
mod datetime;

pub use dummy::DummyModule;
pub use self::battery::BatteryModule;
pub use datetime::DateTimeModule;

pub mod status_block;

use status_block::StatusBlock;

pub trait Module {
    fn get_blocks(&self) -> Vec<StatusBlock>;
}
