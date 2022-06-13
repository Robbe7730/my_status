mod dummy;
mod battery;
mod datetime;
mod bluetooth;
mod network;
mod playing;
mod module_error;

pub use dummy::DummyModule;
pub use self::battery::BatteryModule;
pub use datetime::DateTimeModule;
pub use bluetooth::BluetoothModule;
pub use network::NetworkModule;
pub use playing::PlayingModule;
pub use module_error::ModuleError;

pub mod status_block;

use async_trait::async_trait;

use status_block::StatusBlock;

use std::error::Error;

type ModuleResult = Result<Vec<StatusBlock>, Box<dyn Error>>;

#[async_trait(?Send)]
pub trait Module {
    async fn get_blocks(&self) -> ModuleResult;
}
