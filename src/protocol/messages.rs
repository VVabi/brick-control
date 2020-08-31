extern crate serde_json;
extern crate serde;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum MessageUniqueId {
SetMotorPwmUniqueId,
}

pub trait Message {
 fn get_unique_id_dyn(&self) -> MessageUniqueId;
 fn to_json(&self) -> std::result::Result<std::string::String, serde_json::Error>;
 fn get_topic_dyn(&self) -> std::string::String;
}

pub trait StaticMessageInfo {
 fn get_unique_id() -> MessageUniqueId;
 fn get_topic() -> std::string::String;
}

#[derive(Serialize, Deserialize)]
pub struct SetMotorPwm {
    pub pwm: i8,
    pub port: u8
}

impl Message for SetMotorPwm {
    #[inline]
    fn get_unique_id_dyn(&self) -> MessageUniqueId {
        MessageUniqueId::SetMotorPwmUniqueId
    }
    fn to_json(&self) -> std::result::Result<std::string::String, serde_json::Error> {
        serde_json::to_string(&self)
    }
    fn get_topic_dyn(&self) -> std::string::String {
        return "brickcontrol/motor/pwm".to_string();
    }
}

impl StaticMessageInfo for SetMotorPwm {
    #[inline]
    fn get_unique_id() -> MessageUniqueId {
        MessageUniqueId::SetMotorPwmUniqueId
    }
    fn get_topic() -> std::string::String {
        return "brickcontrol/motor/pwm".to_string();
    }
}

