extern crate serde;
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize, Default)]
struct Header {
    version: u8,

    #[serde(skip_serializing_if = "Option::is_none")]
    stop_signal: Option<u8>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    cont_signal: Option<u8>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    click_events: Option<bool>,
}

#[derive(Serialize, Deserialize, Default)]
struct Status {
    full_text: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    short_text: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    background: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    border: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    min_width: Option<u32>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    align: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    urgent: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    separator: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    separator_block_width: Option<u32>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    markup: Option<String>,
}

fn header() -> String {
    let header = Header {
        version: 1,
        ..Default::default()
    };
    json!(&header).to_string()
}

fn status() -> String{
    let status1 = Status {
        full_text: "Hello World".to_string(),
        color: Some("#FFFF00".to_string()),
        ..Default::default()
    };
    let status2 = Status {
        full_text: "Lorem ipsum".to_string(),
        ..Default::default()
    };
    let statuses = [status1, status2];
    json!(&statuses).to_string()
}

fn main() {
    let header: String = header();
    println!("{}", header);
    println!("[");
    loop {
        let status: String = status();
        println!("{},", status);
    }
}
