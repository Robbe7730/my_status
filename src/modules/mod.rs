mod date;
mod time;
mod battery;
mod network;
mod volume;
mod playing;

pub use modules::date::date::Date as Date;
pub use modules::time::time::Time as Time;
pub use modules::battery::battery::Battery as Battery;
pub use modules::network::network::Network as Network;
pub use modules::volume::volume::Volume as Volume;
pub use modules::playing::playing::Playing as Playing;