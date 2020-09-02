extern crate serde_json;
extern crate serde;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum MessageUniqueId {
SetMotorPwmUniqueId,
MotorGoToPositionUniqueId,
EnableModeUpdatesUniqueId,
MotorPositionUpdateUniqueId,
RequestBatteryStatusUniqueId,
BatteryStatusUniqueId,
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

#[derive(Serialize, Deserialize)]
pub struct MotorGoToPosition {
    pub pwm: i8,
    pub port: u8,
    pub max_power: u8,
    pub target_angle: i32
}

impl Message for MotorGoToPosition {
    #[inline]
    fn get_unique_id_dyn(&self) -> MessageUniqueId {
        MessageUniqueId::MotorGoToPositionUniqueId
    }
    fn to_json(&self) -> std::result::Result<std::string::String, serde_json::Error> {
        serde_json::to_string(&self)
    }
    fn get_topic_dyn(&self) -> std::string::String {
        return "brickcontrol/motor/go_to_position".to_string();
    }
}

impl StaticMessageInfo for MotorGoToPosition {
    #[inline]
    fn get_unique_id() -> MessageUniqueId {
        MessageUniqueId::MotorGoToPositionUniqueId
    }
    fn get_topic() -> std::string::String {
        return "brickcontrol/motor/go_to_position".to_string();
    }
}

#[derive(Serialize, Deserialize)]
pub struct EnableModeUpdates {
    pub port: u8,
    pub mode: u8,
    pub notifications_enabled: u8,
    pub delta: u32
}

impl Message for EnableModeUpdates {
    #[inline]
    fn get_unique_id_dyn(&self) -> MessageUniqueId {
        MessageUniqueId::EnableModeUpdatesUniqueId
    }
    fn to_json(&self) -> std::result::Result<std::string::String, serde_json::Error> {
        serde_json::to_string(&self)
    }
    fn get_topic_dyn(&self) -> std::string::String {
        return "brickcontrol/generic/set_mode_update".to_string();
    }
}

impl StaticMessageInfo for EnableModeUpdates {
    #[inline]
    fn get_unique_id() -> MessageUniqueId {
        MessageUniqueId::EnableModeUpdatesUniqueId
    }
    fn get_topic() -> std::string::String {
        return "brickcontrol/generic/set_mode_update".to_string();
    }
}

#[derive(Serialize, Deserialize)]
pub struct MotorPositionUpdate {
    pub position: i32,
    pub port: u8
}

impl Message for MotorPositionUpdate {
    #[inline]
    fn get_unique_id_dyn(&self) -> MessageUniqueId {
        MessageUniqueId::MotorPositionUpdateUniqueId
    }
    fn to_json(&self) -> std::result::Result<std::string::String, serde_json::Error> {
        serde_json::to_string(&self)
    }
    fn get_topic_dyn(&self) -> std::string::String {
        return "brickcontrol/motor/output/position_update".to_string();
    }
}

impl StaticMessageInfo for MotorPositionUpdate {
    #[inline]
    fn get_unique_id() -> MessageUniqueId {
        MessageUniqueId::MotorPositionUpdateUniqueId
    }
    fn get_topic() -> std::string::String {
        return "brickcontrol/motor/output/position_update".to_string();
    }
}

#[derive(Serialize, Deserialize)]
pub struct RequestBatteryStatus {

}

impl Message for RequestBatteryStatus {
    #[inline]
    fn get_unique_id_dyn(&self) -> MessageUniqueId {
        MessageUniqueId::RequestBatteryStatusUniqueId
    }
    fn to_json(&self) -> std::result::Result<std::string::String, serde_json::Error> {
        serde_json::to_string(&self)
    }
    fn get_topic_dyn(&self) -> std::string::String {
        return "brickcontrol/battery/request_status".to_string();
    }
}

impl StaticMessageInfo for RequestBatteryStatus {
    #[inline]
    fn get_unique_id() -> MessageUniqueId {
        MessageUniqueId::RequestBatteryStatusUniqueId
    }
    fn get_topic() -> std::string::String {
        return "brickcontrol/battery/request_status".to_string();
    }
}

#[derive(Serialize, Deserialize)]
pub struct BatteryStatus {
    pub charging_state: u8
}

impl Message for BatteryStatus {
    #[inline]
    fn get_unique_id_dyn(&self) -> MessageUniqueId {
        MessageUniqueId::BatteryStatusUniqueId
    }
    fn to_json(&self) -> std::result::Result<std::string::String, serde_json::Error> {
        serde_json::to_string(&self)
    }
    fn get_topic_dyn(&self) -> std::string::String {
        return "brickcontrol/battery/status".to_string();
    }
}

impl StaticMessageInfo for BatteryStatus {
    #[inline]
    fn get_unique_id() -> MessageUniqueId {
        MessageUniqueId::BatteryStatusUniqueId
    }
    fn get_topic() -> std::string::String {
        return "brickcontrol/battery/status".to_string();
    }
}
