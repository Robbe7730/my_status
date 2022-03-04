use super::{Module, StatusBlock};

use std::sync::{Arc, Mutex};

use async_trait::async_trait;

use btleplug::platform::{Manager, Adapter};
use btleplug::api::{Manager as _, Central, CentralEvent, Peripheral};

use futures::StreamExt;

pub struct BluetoothDevice {
    name: Option<String>,
    mac: String,
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
            let display_name = device.name.as_ref().unwrap_or(&device.mac);
            ret.push(StatusBlock::new("bluetooth", &display_name));
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
        let mut new_devices = vec![];
        for peripheral in central.peripherals().await.unwrap() {
            if peripheral.is_connected().await.unwrap() {
                let properties = peripheral.properties().await.unwrap().unwrap();
                new_devices.push(BluetoothDevice {
                    name: properties.local_name,
                    mac: peripheral.address().to_string(),
                });
            }
        }
        let mut devices_lock = devices.lock().unwrap();
        *devices_lock = new_devices;
    }
}
