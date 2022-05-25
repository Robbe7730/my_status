use super::{Module, StatusBlock};

use socket2::{Socket, Domain, Type, SockAddr};

use std::io::Read;
use std::process::id;
use std::str;

use async_trait::async_trait;

pub struct NetworkModule {
    sockets: Vec<Socket>,
}

// Found in hostad's src/common/defs.h
#[derive(Debug)]
pub enum WpaState {
    Disconnected,
    InterfaceDisabled,
    Inactive,
    Scanning,
    Authenticating,
    Associating,
    Associated,
    FourWayHandshake,
    GroupHanshake,
    Completed,

    Unknown
}

impl From<&str> for WpaState {
    fn from(value: &str) -> WpaState {
        match value {
            "DISCONNECTED" => WpaState::Disconnected,
            "INTERFACE_DISABLED" => WpaState::InterfaceDisabled,
            "INACTIVE" => WpaState::Inactive,
            "SCANNING" => WpaState::Scanning,
            "AUTHENTICATING" => WpaState::Authenticating,
            "ASSOCIATING" => WpaState::Associating,
            "ASSOCIATED" => WpaState::Associated,
            "4WAY_HANDSHAKE" => WpaState::FourWayHandshake,
            "GROUP_HANDSHAKE" => WpaState::GroupHanshake,
            "COMPLETED" => WpaState::Completed,
            _ => WpaState::Unknown
        }
    }
}

impl WpaState {
    pub fn icon(&self) -> String {
        format!("{}", match self {
            WpaState::Completed => "📶",
            _ => "?",
        })
    }
}

#[derive(Debug)]
pub struct WpaData {
    state: WpaState,
    ssid: Option<String>,
    ip_addr: Option<String>,
}

impl WpaData {
    fn to_string(&self) -> String {
        let mut ret = format!(
            "{}",
            self.ssid.as_ref().unwrap_or(&format!("no ssid"))
        );

        if let Some(ip) = &self.ip_addr {
            ret.push(' ');
            ret.push('(');
            ret.push_str(&ip);
            ret.push(')');
        }

        ret
    }
}

#[async_trait(?Send)]
impl Module for NetworkModule {
    async fn get_blocks(&self) -> Vec<StatusBlock> {
        let mut ret = vec![];

        for (i, mut socket) in self.sockets.iter().enumerate() {
            socket.send(b"STATUS").unwrap();
            
            let mut response = String::new();

            let mut buf = vec![0; 1024];
            socket.read(&mut buf).unwrap();
            buf.retain(|x| *x != 0);
            response.push_str(str::from_utf8(&buf).unwrap());

            let mut data = WpaData {
                state: WpaState::Unknown,
                ssid: None,
                ip_addr: None,
            };

            for line in response.lines() {
                let mut line_split = line.split("=");
                let key = line_split.next().unwrap();
                let maybe_value = line_split.next();

                if maybe_value.is_none() {
                    eprintln!("No value for {}", key);
                    return vec![];
                }

                let value = maybe_value.unwrap();

                match key {
                    "wpa_state" => data.state = WpaState::from(value),
                    "ssid" => data.ssid = Some(value.to_string()),
                    "ip_address" => data.ip_addr = Some(value.to_string()),
                    _ => ()
                }
            }
            ret.push(
                StatusBlock::new(
                    &format!("network-{}", i),
                    &data.to_string(),
                ).with_color(match data.state {
                    WpaState::Completed => "#00ff00",
                    WpaState::Unknown | WpaState::Disconnected => "#ff0000",
                    _ => "#ff7f00",
                })
            );
        }

        ret
    }
}

impl NetworkModule {
    pub fn new() -> Self {
        let socket_path = "/var/run/wpa_supplicant/wlo1";

        let socket = Socket::new(Domain::UNIX, Type::DGRAM, None).unwrap();
        let addr = SockAddr::unix(format!("/tmp/status-network-{}", id())).unwrap();

        socket.bind(&addr).unwrap();

        socket.connect(&SockAddr::unix(socket_path).unwrap()).unwrap();

        Self {
            sockets: vec![socket],
        }
    }
}
