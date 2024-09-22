
pub trait Message {
    fn to_json(&self) -> std::result::Result<std::string::String, serde_json::Error>;
    fn get_topic_dyn(&self) -> std::string::String;
}

pub trait StaticMessageInfo {
    fn get_topic() -> std::string::String;
}
