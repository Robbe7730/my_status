use serde::Serialize;

#[derive(Serialize)]
pub struct StatusBarConfig {
    version: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_signal: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cont_signal: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    click_events: Option<bool>,
}

impl StatusBarConfig {
    pub fn new(
        version: u8,
        stop_signal: Option<u8>,
        cont_signal: Option<u8>,
        click_events: Option<bool>
    ) -> StatusBarConfig {
        Self {
            version,
            stop_signal,
            cont_signal,
            click_events
        }
    }
}
