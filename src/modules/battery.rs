use super::{Module, StatusBlock};

use battery::{Manager, State};

use async_trait::async_trait;

pub struct BatteryModule {
    manager: Manager,
}

#[async_trait(?Send)]
impl Module for BatteryModule {
    async fn get_blocks(&self) -> Vec<StatusBlock> {
        match self.manager.batteries().unwrap().next() {
            Some(Ok(battery)) => {
                let charging = battery.state() == State::Charging;
                let percentage: f32 = (battery.state_of_charge() * 100.0).into();
                let color = if !charging && (percentage <= 25.0 && percentage > 15.0) {
                    "#ff7f00"
                } else {
                    "#ffffff"
                };
                let icon = if charging { "⚡" } else { "🔋" };
                vec![
                    StatusBlock::new(
                        "battery",
                        &format!("{} {}%", icon, percentage.round())
                    ).with_color(color)
                        .with_urgent(percentage <= 15.0 && !charging)
                ]
            },
            _ => vec![]
        }
    }
}

impl BatteryModule {
    pub fn new() -> Self {
        let manager = Manager::new().unwrap();

        Self {
            manager
        }
    }
}
