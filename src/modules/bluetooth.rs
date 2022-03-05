use super::{Module, StatusBlock};

use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use async_trait::async_trait;

use btleplug::platform::{Manager, PeripheralId};
use btleplug::api::{Manager as _, Central, CentralEvent, Peripheral};

use futures::StreamExt;

pub enum BluetoothDeviceAttribute {
    BatteryLevel(Option<u8>),
}

pub struct BluetoothDevice {
    name: Option<String>,
    mac: String,
    icon: Option<String>,
    attributes: Vec<BluetoothDeviceAttribute>,
}

pub struct BluetoothModule {
    devices: Arc<Mutex<HashMap<PeripheralId, BluetoothDevice>>>,
}

#[async_trait(?Send)]
impl Module for BluetoothModule {
    async fn get_blocks(&self) -> Vec<StatusBlock> {

        let mut ret = vec![];
        let devices = self.devices.lock().unwrap();
        for device in devices.values() {
            let mut display_name = device.name.as_ref().unwrap_or(&device.mac).to_string();

            if let Some(icon) = &device.icon {
                display_name = format!("{} {}", icon, display_name);
            }

            let attributes = device.attributes.iter().map(|attribute| {
                match attribute {
                    BluetoothDeviceAttribute::BatteryLevel(None) => format!("🔋 ??%"),
                    BluetoothDeviceAttribute::BatteryLevel(Some(r)) => format!("🔋 {}%", r),
                }
            }).collect::<Vec<String>>().join(", ");

            if attributes.len() > 0 {
                display_name = format!("{} ({})", display_name, attributes);
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
        let devices = Arc::new(Mutex::new(HashMap::new()));
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
                    CentralEvent::DeviceConnected(id) => {
                        let peripheral = central.peripheral(&id).await.unwrap();

                        peripheral.discover_services().await.unwrap();

                        let properties = peripheral.properties().await.unwrap().unwrap();

                        let mut attributes = vec![];

                        for characteristic in peripheral.characteristics() {
                            let attribute = match characteristic.uuid.as_u128() {
                                0x00002a19_0000_1000_8000_00805f9b34fb =>
                                    Some(BluetoothDeviceAttribute::BatteryLevel(
                                        peripheral.read(&characteristic).await.ok().map(|x| x[0])
                                    )),
                                _ => None
                            };

                            if let Some(attr) = attribute {
                                attributes.push(attr);
                            }
                        }

                        let device = BluetoothDevice {
                            name: properties.local_name,
                            mac: properties.address.to_string(),
                            icon: None,
                            attributes,
                        };

                        devices.clone().lock().unwrap().insert(id, device);
                    }
                    CentralEvent::DeviceDisconnected(id) => {
                        devices.clone().lock().unwrap().remove(&id);
                    }
                    _ => {}
                }
            }
        });

        ret
    }
}
