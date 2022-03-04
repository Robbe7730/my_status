use super::{Module, StatusBlock};

use std::sync::{Arc, Mutex};

use async_trait::async_trait;

use btleplug::platform::{Manager, Adapter};
use btleplug::api::{Manager as _, Central, CentralEvent, Peripheral};

use uuid::Uuid;

use futures::StreamExt;

pub struct BluetoothDevice {
    name: Option<String>,
    mac: String,
    icon: Option<String>,
}

pub struct BluetoothModule {
    devices: Arc<Mutex<Vec<BluetoothDevice>>>,
}

#[async_trait(?Send)]
impl Module for BluetoothModule {
    async fn get_blocks(&self) -> Vec<StatusBlock> {

        let mut ret = vec![];
        let devices = self.devices.lock().unwrap();
        for device in devices.iter() {
            let mut display_name = device.name.as_ref().unwrap_or(&device.mac).to_string();

            if let Some(icon) = &device.icon {
                display_name = format!("{} {}", icon, display_name);
            }

            ret.push(
                StatusBlock::new("bluetooth", &display_name)
                    .with_instance(&device.mac)
            );
        }

        ret
    }
}

impl BluetoothModule {
    pub fn new() -> Self {
        let devices = Arc::new(Mutex::new(vec![]));
        let ret = Self {
            devices: devices.clone(),
        };

        tokio::spawn(async move {
            let manager = Manager::new().await.unwrap();
            let adapters = manager.adapters().await.unwrap();
            let central = adapters.into_iter().nth(0).unwrap();
            let mut events = central.events().await.unwrap();

            while let Some(event) = events.next().await {
                match event {
                    CentralEvent::DeviceConnected(_) |
                    CentralEvent::DeviceDisconnected(_) => {
                        Self::get_devices(devices.clone(), &central).await;
                    }
                    _ => {}
                }
            }
        });

        ret
    }

    pub async fn get_devices(devices: Arc<Mutex<Vec<BluetoothDevice>>>, central: &Adapter) {
        let heartrate_uuid = Uuid::parse_str("0000180d-0000-1000-8000-00805f9b34fb").unwrap();
        let mut new_devices = vec![];
        for peripheral in central.peripherals().await.unwrap() {
            if peripheral.is_connected().await.unwrap() {
                let properties = peripheral.properties().await.unwrap().unwrap();

                let mut icon = None;

                peripheral.discover_services().await.unwrap();

                for service in peripheral.services() {
                    if service.uuid == heartrate_uuid {
                        icon = Some("💓".to_string());
                    }
                }

                new_devices.push(BluetoothDevice {
                    name: properties.local_name,
                    mac: peripheral.address().to_string(),
                    icon,
                });
            }
        }
        let mut devices_lock = devices.lock().unwrap();
        *devices_lock = new_devices;
    }
}
