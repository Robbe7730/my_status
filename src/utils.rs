pub mod traits {
    use utils::structs::*;

    pub trait StatusAble {
        fn get_status(&self) -> Option<Status>;
    }
}

pub mod structs {
    #[derive(Serialize, Deserialize, Default)]
    pub struct Header {
        pub version: u8,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub stop_signal: Option<u8>,
        
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cont_signal: Option<u8>,
        
        #[serde(skip_serializing_if = "Option::is_none")]
        pub click_events: Option<bool>,
    }

    #[derive(Serialize, Deserialize, Default, Clone)]
    pub struct Status {
        pub full_text: String,

        pub name: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub short_text: Option<String>,
        
        #[serde(skip_serializing_if = "Option::is_none")]
        pub color: Option<String>,
        
        #[serde(skip_serializing_if = "Option::is_none")]
        pub background: Option<String>,
        
        #[serde(skip_serializing_if = "Option::is_none")]
        pub border: Option<String>,
        
        #[serde(skip_serializing_if = "Option::is_none")]
        pub min_width: Option<u32>,
        
        #[serde(skip_serializing_if = "Option::is_none")]
        pub align: Option<String>,
        
        #[serde(skip_serializing_if = "Option::is_none")]
        pub instance: Option<String>,
        
        #[serde(skip_serializing_if = "Option::is_none")]
        pub urgent: Option<bool>,
        
        #[serde(skip_serializing_if = "Option::is_none")]
        pub separator: Option<bool>,
        
        #[serde(skip_serializing_if = "Option::is_none")]
        pub separator_block_width: Option<u32>,
        
        #[serde(skip_serializing_if = "Option::is_none")]
        pub markup: Option<String>,
    }
}