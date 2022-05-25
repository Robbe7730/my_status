use super::{Module, StatusBlock};

use dbus::blocking::Connection;
use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use dbus::arg::PropMap;
use dbus::arg;

use std::time::Duration;

use async_trait::async_trait;

enum PlaybackStatus {
    Playing,
    Paused,
    Stopped
}

impl From<String> for PlaybackStatus {
    fn from(s: String) -> PlaybackStatus {
        match s.as_str() {
            "Playing" => PlaybackStatus::Playing,
            "Paused" => PlaybackStatus::Paused,
            _ => PlaybackStatus::Stopped,
        }
    }
}

impl PlaybackStatus {
    fn icon(self) -> String {
        match self {
            PlaybackStatus::Playing => "▶".to_string(),
            PlaybackStatus::Paused => "⏸︎".to_string(),
            PlaybackStatus::Stopped => "⏹".to_string(),
        }
    }
}

pub struct PlayingModule {
    conn: Connection,
}

#[async_trait(?Send)]
impl Module for PlayingModule {
    async fn get_blocks(&self) -> Vec<StatusBlock> {
        let playerctl_proxy = self.conn.with_proxy(
            "org.mpris.MediaPlayer2.playerctld",
            "/org/mpris/MediaPlayer2",
            Duration::from_secs(1)
        );

        let players: Vec<String> = playerctl_proxy.get(
            "com.github.altdesktop.playerctld",
            "PlayerNames"
        ).unwrap();

        let mut ret = vec![];

        for player in players {
            ret.push(
                self.status_block_for_player(player)
            );
        }

        ret
    }
}

impl PlayingModule {
    pub fn new() -> Self {
        PlayingModule {
            conn: Connection::new_session().unwrap(),
        }
    }

    fn status_block_for_player(&self, player: String) -> StatusBlock {
        let player_proxy = self.conn.with_proxy(
            &player,
            "/org/mpris/MediaPlayer2",
            Duration::from_secs(1)
        );

        let identity: String = player_proxy.get(
            "org.mpris.MediaPlayer2",
            "Identity"
        ).unwrap();

        let playbackstatus: PlaybackStatus = PlaybackStatus::from(
            player_proxy.get(
                "org.mpris.MediaPlayer2.Player",
                "PlaybackStatus"
            ).unwrap_or(format!("Stopped"))
        );
        
        let metadata: PropMap =  player_proxy.get(
            "org.mpris.MediaPlayer2.Player",
            "Metadata"
        ).unwrap();

        let maybe_title: Option<&String> = arg::prop_cast(&metadata, "xesam:title");
        let maybe_artists: Option<&Vec<String>> = arg::prop_cast(&metadata, "xesam:artist");

        let content;
        
        if let Some(title) = maybe_title {
            if let Some(artists) = maybe_artists {
                if artists.len() == 0 {
                    content = title.to_string();
                } else {
                    content = format!("{} - {}", title, artists.join(","))
                }
            } else {
                content = title.to_string();
            }
        } else {
            content = identity.to_string();
        }

        let display = format!(
            "{} {}",
            playbackstatus.icon(),
            content,
        );

        StatusBlock::new("playing", &display)
            .with_instance(&identity)
    }
}
