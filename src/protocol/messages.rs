extern crate serde_json;
extern crate serde;
use std::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum BleMessageType{
    HubProperties=0x01,
    PortInputFormatSetup=0x41,
    PortOutputCommand=0x81,
    PortOutputCommandFeedback=0x82,
    PortValue=0x45,
    HubAttached=0x04,
}

pub fn translate_blemessagetype_from_int(input: u32) -> Result<BleMessageType, Box<dyn Error>> {
    match input {
        0x01=> Ok(BleMessageType::HubProperties),
        0x41=> Ok(BleMessageType::PortInputFormatSetup),
        0x81=> Ok(BleMessageType::PortOutputCommand),
        0x82=> Ok(BleMessageType::PortOutputCommandFeedback),
        0x45=> Ok(BleMessageType::PortValue),
        0x04=> Ok(BleMessageType::HubAttached),
        _ => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "UnknownBleMessageType")))
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum Port{
    A=0,
    B=1,
    C=2,
    D=3,
}

pub fn translate_port_from_int(input: u32) -> Result<Port, Box<dyn Error>> {
    match input {
        0=> Ok(Port::A),
        1=> Ok(Port::B),
        2=> Ok(Port::C),
        3=> Ok(Port::D),
        _ => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "UnknownPort")))
    }
}

#[derive(Debug)]
pub enum MessageUniqueId {
    SetMotorPwmUniqueId,
    SetMotorSpeedUniqueId,
    MotorGoToPositionUniqueId,
    MotorCommandFeedbackUniqueId,
    EnableModeUpdatesUniqueId,
    MotorPositionUpdateUniqueId,
    RequestBatteryStatusUniqueId,
    BatteryStatusUniqueId,
    AttachmentInfoUniqueId,
    AttachedIoUniqueId,
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
    pub port: Port
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
pub struct SetMotorSpeed {
    pub pwm: i8,
    pub port: Port,
    pub max_power: u8
}

impl Message for SetMotorSpeed {
    #[inline]
    fn get_unique_id_dyn(&self) -> MessageUniqueId {
        MessageUniqueId::SetMotorSpeedUniqueId
    }
    fn to_json(&self) -> std::result::Result<std::string::String, serde_json::Error> {
        serde_json::to_string(&self)
    }
    fn get_topic_dyn(&self) -> std::string::String {
        return "brickcontrol/motor/set_speed".to_string();
    }
}

impl StaticMessageInfo for SetMotorSpeed {
    #[inline]
    fn get_unique_id() -> MessageUniqueId {
        MessageUniqueId::SetMotorSpeedUniqueId
    }
    fn get_topic() -> std::string::String {
        return "brickcontrol/motor/set_speed".to_string();
    }
}

#[derive(Serialize, Deserialize)]
pub struct MotorGoToPosition {
    pub pwm: i8,
    pub port: Port,
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
pub struct MotorCommandFeedback {
    pub port: Port,
    pub flags: u8
}

impl Message for MotorCommandFeedback {
    #[inline]
    fn get_unique_id_dyn(&self) -> MessageUniqueId {
        MessageUniqueId::MotorCommandFeedbackUniqueId
    }
    fn to_json(&self) -> std::result::Result<std::string::String, serde_json::Error> {
        serde_json::to_string(&self)
    }
    fn get_topic_dyn(&self) -> std::string::String {
        return "brickcontrol/motor/output/command_feedback".to_string();
    }
}

impl StaticMessageInfo for MotorCommandFeedback {
    #[inline]
    fn get_unique_id() -> MessageUniqueId {
        MessageUniqueId::MotorCommandFeedbackUniqueId
    }
    fn get_topic() -> std::string::String {
        return "brickcontrol/motor/output/command_feedback".to_string();
    }
}

#[derive(Serialize, Deserialize)]
pub struct EnableModeUpdates {
    pub port: Port,
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
    pub port: Port
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

#[derive(Serialize, Deserialize)]
pub struct AttachmentInfo {
    pub type_id: u32,
    pub hw_rev: i32,
    pub sw_rev: i32
}

impl Message for AttachmentInfo {
    #[inline]
    fn get_unique_id_dyn(&self) -> MessageUniqueId {
        MessageUniqueId::AttachmentInfoUniqueId
    }
    fn to_json(&self) -> std::result::Result<std::string::String, serde_json::Error> {
        serde_json::to_string(&self)
    }
    fn get_topic_dyn(&self) -> std::string::String {
        return "unused".to_string();
    }
}

impl StaticMessageInfo for AttachmentInfo {
    #[inline]
    fn get_unique_id() -> MessageUniqueId {
        MessageUniqueId::AttachmentInfoUniqueId
    }
    fn get_topic() -> std::string::String {
        return "unused".to_string();
    }
}

#[derive(Serialize, Deserialize)]
pub struct AttachedIo {
    pub port_id: u8,
    pub event: u8,
    pub info: Vec<AttachmentInfo>
}

impl Message for AttachedIo {
    #[inline]
    fn get_unique_id_dyn(&self) -> MessageUniqueId {
        MessageUniqueId::AttachedIoUniqueId
    }
    fn to_json(&self) -> std::result::Result<std::string::String, serde_json::Error> {
        serde_json::to_string(&self)
    }
    fn get_topic_dyn(&self) -> std::string::String {
        return "brickcontrol/io/connection_update".to_string();
    }
}

impl StaticMessageInfo for AttachedIo {
    #[inline]
    fn get_unique_id() -> MessageUniqueId {
        MessageUniqueId::AttachedIoUniqueId
    }
    fn get_topic() -> std::string::String {
        return "brickcontrol/io/connection_update".to_string();
    }
}


