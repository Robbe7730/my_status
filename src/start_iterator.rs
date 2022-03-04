use crate::config::StatusBarConfig;

pub struct StartIterator {
    config: Option<StatusBarConfig>
}

impl Iterator for StartIterator {
    type Item = String;

    fn next(&mut self) -> Option<String> {
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

impl StartIterator {
    pub fn new() -> Self {
        Self {
            config: Some(StatusBarConfig::new(1, None, None, None))
        }
    }
}

