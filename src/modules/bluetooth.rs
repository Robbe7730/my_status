pub mod bluetooth {
    extern crate blurz;

    use blurz::bluetooth_adapter::BluetoothAdapter as Adapter;
    use blurz::bluetooth_device::BluetoothDevice as Device;
    use blurz::bluetooth_session::BluetoothSession as Session;

    use utils::structs::Status;
    use utils::traits::StatusAble;
    use std::default::Default;

    pub struct Bluetooth();

    impl StatusAble for Bluetooth {
        fn get_status(&self) -> Option<Status> {
            let session = &Session::create_session(None).unwrap();
            let adapter: Adapter = Adapter::init(session).unwrap();
            if !adapter.is_powered().unwrap() {
                return None;
            }
            let mut dev_name = "Not connected".to_string();
            let mut dev_icon = "None".to_string();
            let devices = adapter.get_device_list().unwrap();
            'device_loop: for d in devices {
                let device = Device::new(session, d.clone());
                if device.is_connected().unwrap() {
                    dev_name = device.get_name().unwrap();
                    dev_icon = device.get_icon().unwrap_or("unknown".to_string());
                }
            }

            // https://standards.freedesktop.org/icon-naming-spec/icon-naming-spec-latest.html
            let dev_icon_disp = match dev_icon.as_ref() {
                "audio-card"              => format!("🎧"),
                "audio-headset"              => format!("🎧"),
                "audio-input-microphone	" => format!("🎤"),
                "battery"                 => format!("🔋"),
                "camera-photo"            => format!("📷"),
                "camera-video"            => format!("🎥"),
                "camera-web"              => format!("📷"),
                "computer"                => format!("💻"),
                "drive-harddisk"          => format!("🖴"),
                "drive-optical"           => format!("💿"),
                "drive-removable-media"   => format!("🖴"),
                "input-gaming"            => format!("🎮"),
                "input-keyboard"          => format!("⌨"),
                "input-mouse"             => format!("🖱️"),
                "input-tablet"            => format!("🖵"),
                "media-flash"             => format!("🖴"),
                "media-floppy"            => format!("💾"),
                "media-optical"           => format!("💿"),
                "media-tape"              => format!("🖭"),
                "modem"                   => format!("🖀"),
                "multimedia-player"       => format!("🎛️"),
                "network-wired"           => format!("🌐"),
                "network-wireless"        => format!("📶"),
                "pda"                     => format!("📱"),
                "phone"                   => format!("📞"),
                "printer"                 => format!("🖨"),
                "scanner"                 => format!("🖨"),
                "video-display"           => format!("🖵"),
                "None"                    => format!("✖"),
                "unknown"                 => format!("?"),
                a                         => format!("({})", a),
            };

            return Some(Status {
                full_text: format!("{} {}", dev_icon_disp, dev_name),
                name: "bluetooth".to_string(),
                ..Default::default()
            })  
        }
    }
}
