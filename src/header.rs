use crate::config::StatusBarConfig;

pub struct Header {
    config: Option<StatusBarConfig>
}

impl Header {
    pub fn new() -> Self {
        Self {
            config: Some(StatusBarConfig::new(1, None, None, None))
        }
    }

    pub fn get_value(&mut self) -> Option<String> {
        if let Some(config) = &self.config {
            let ret = format!(
                "{}\n[",
                serde_json::to_string(&config).unwrap()
            );
            self.config = None;
            Some(ret)
        } else {
            None
        }
    }
}

