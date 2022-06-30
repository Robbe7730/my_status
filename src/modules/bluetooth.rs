use super::{Module, StatusBlock, ModuleResult, ModuleError};

use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use async_trait::async_trait;

use btleplug::platform::{Manager, PeripheralId};
use btleplug::api::{Manager as _, Central, CentralEvent, Peripheral, ValueNotification};

use futures::stream::StreamExt;

#[derive(Clone)]
pub enum BluetoothDeviceAttribute {
    Heartrate(Option<usize>),
    BatteryLevel(Option<u8>),
}

pub struct BluetoothDevice {
    name: Option<String>,
    mac: String,
    attributes: Vec<BluetoothDeviceAttribute>,
}

impl BluetoothDevice {
    pub fn handle_notification(&mut self, notification: ValueNotification) {
        // The single_match is ok as I intend on adding more attributes
        #[allow(clippy::single_match)]
        match notification.uuid.as_u128() {
            0x00002a37_0000_1000_8000_00805f9b34fb => {
                let flags = notification.value[0];

                let heartrate: usize = if flags & 1 == 0 {
                    notification.value[1].into()
                } else {
                    (u16::from(notification.value[1]) + u16::from(notification.value[2]) * 256).into()
                };
                let mut new_attibutes = vec![];
                for attribute in self.attributes.iter().cloned() {
                    if let BluetoothDeviceAttribute::Heartrate(_) = attribute {
                        new_attibutes.push(BluetoothDeviceAttribute::Heartrate(Some(heartrate)));
                    } else {
                        new_attibutes.push(attribute);
                    }
                }
                self.attributes = new_attibutes;
            }
            _ => {}
        }
    }
}

pub struct BluetoothModule {
    devices: Arc<Mutex<HashMap<PeripheralId, BluetoothDevice>>>,
}

#[async_trait(?Send)]
impl Module for BluetoothModule {
    async fn get_blocks(&self) -> ModuleResult {

        let mut ret = vec![];
        let devices = self.devices.lock().map_err(ModuleError::from)?;
        for device in devices.values() {
            let mut display_name = device.name.as_ref().unwrap_or(&device.mac).to_string();

            let attributes = device.attributes.iter().map(|attribute| {
                match attribute {
                    BluetoothDeviceAttribute::Heartrate(None) => "💓 ??".to_string(),
                    BluetoothDeviceAttribute::Heartrate(Some(r)) => format!("💓 {}", r),
                    BluetoothDeviceAttribute::BatteryLevel(None) => "🔋 ??%".to_string(),
                    BluetoothDeviceAttribute::BatteryLevel(Some(r)) => format!("🔋 {}%", r),
                }
            }).collect::<Vec<String>>().join(", ");

            if !attributes.is_empty() {
                display_name = format!("{} ({})", display_name, attributes);
            }

            ret.push(
                StatusBlock::new("bluetooth", &display_name)
                    .with_instance(&device.mac)
            );
        }

        Ok(ret)
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
            let central = adapters.into_iter().next().unwrap();
            let mut events = central.events().await.unwrap();

            while let Some(event) = events.next().await {
                match event {
                    CentralEvent::DeviceConnected(id) => {
                        let peripheral = central.peripheral(&id).await.unwrap();

                        peripheral.discover_services().await.unwrap();

                        let properties = peripheral.properties().await.unwrap().unwrap();

                        let mut attributes = vec![];
                        let mut should_listen = false;

                        for characteristic in peripheral.characteristics() {
                            let attribute = match characteristic.uuid.as_u128() {
                                0x00002a37_0000_1000_8000_00805f9b34fb => {
                                    peripheral.subscribe(&characteristic).await.unwrap();
                                    should_listen = true;
                                    Some(BluetoothDeviceAttribute::Heartrate(None))
                                }
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

                        if should_listen {
                            let listener_devices = devices.clone();
                            let listener_id = id.clone();
                            tokio::spawn(async move {
                                let mut stream = peripheral.notifications().await.unwrap();
                                while let Some(notification) = stream.next().await {
                                    let mut devices = listener_devices.lock().unwrap();
                                    let device = devices.get_mut(&listener_id).unwrap();
                                    device.handle_notification(notification);
                                }
                            });
                        }

                        let device = BluetoothDevice {
                            name: properties.local_name,
                            mac: properties.address.to_string(),
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
