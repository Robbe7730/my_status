use serde::Serialize;

#[allow(dead_code)]
#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AlginValue {
    Left,
    Right,
    Center
}

#[allow(dead_code)]
#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
enum MarkupValue {
    Pango,
    None
}

#[derive(Serialize)]
pub struct StatusBlock {
    full_text: String,
    name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    short_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    border_top: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_right: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_bottom: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_left: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min_width: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    align: Option<AlginValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    urgent: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    separator: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    separator_block_width: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    markup: Option<String>,
}

impl StatusBlock {
    pub fn new(name: &str, full_text: &str) -> Self {
        Self {
            name: name.to_owned(),
            full_text: full_text.to_owned(),
            short_text: None,
            color: None,
            background: None,
            border: None,
            border_top: None,
            border_right: None,
            border_bottom: None,
            border_left: None,
            min_width: None,
            align: None,
            instance: None,
            urgent: None,
            separator: None,
            separator_block_width: None,
            markup: None
        }
    }

    // TODO: Implement these as needed

    pub fn with_urgent(mut self, urgent: bool) -> Self {
        self.urgent = Some(urgent);
        self
    }

    pub fn with_instance(mut self, instance: &str) -> Self {
        self.instance = Some(instance.to_owned());
        self
    }

    pub fn with_color(mut self, color: &str) -> Self {
        self.color = Some(color.to_owned());
        self
    }
}
