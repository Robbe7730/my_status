mod dummy;
mod battery;
mod datetime;
mod bluetooth;

pub use dummy::DummyModule;
pub use self::battery::BatteryModule;
pub use datetime::DateTimeModule;
pub use bluetooth::BluetoothModule;

pub mod status_block;

use async_trait::async_trait;

use status_block::StatusBlock;

#[async_trait(?Send)]
pub trait Module {
    async fn get_blocks(&self) -> Vec<StatusBlock>;
}
