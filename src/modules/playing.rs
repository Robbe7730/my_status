pub mod playing {
    use utils::structs::Status;
    use utils::traits::StatusAble;
    use std::default::Default;
    use mpris::{PlayerFinder, PlaybackStatus, LoopStatus};

    pub struct Playing();


    impl StatusAble for Playing {
        fn get_status(&self) -> Option<Status> {
            let player_finder = PlayerFinder::new().unwrap();

            let result_player = player_finder.find_active();
            let player;

            if result_player.is_err() {
                // eprintln!("result_player is err");
                return None;
            } else {
                player = result_player.unwrap();
            }

            let result_current_track_metadata = player.get_metadata();
            let current_track_metadata;

            if result_current_track_metadata.is_err() {
                // eprintln!("result_current_track_metadata is err");
                return None;
            } else {
                current_track_metadata = result_current_track_metadata.unwrap();
            }

            let title = current_track_metadata.title().unwrap_or("unknown title");
            let artists = current_track_metadata.artists().unwrap_or(vec!["unknown artist"]);
            let artist_str = artists.join(",");

            if (title == "unknown title" || title == "") && (artist_str == "unknown artist" || artist_str == "") {
                return None;
            }

            let result_playing = player.get_playback_status();
            let playing;

            if result_playing.is_err() {
                // eprintln!("result_playing is err");
                return None;
            } else {
                playing = result_playing.unwrap();
            }

            let playing_str = match playing {
                PlaybackStatus::Playing => "▶",
                PlaybackStatus::Paused => "⏸",
                PlaybackStatus::Stopped => "■",
            };

            let shuffle_str;
            if player.get_shuffle().unwrap_or(false) {
                shuffle_str = "🔀 ";
            } else {
                shuffle_str = "";
            }

            let loop_str = match player.get_loop_status().unwrap_or(LoopStatus::None) {
                LoopStatus::None => "",
                LoopStatus::Track => "🔂 ",
                LoopStatus::Playlist => "🔁 ",
            };

            return Some( Status {
                full_text: format!("{}{}{} {} - {}", loop_str, shuffle_str, playing_str, title, artist_str),
                name: "playing".to_string(),
                ..Default::default()
            });
        }
    }
}
