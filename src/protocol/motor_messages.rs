extern crate serde_json;
extern crate serde;
use std::error::Error;
use serde::{Deserialize, Serialize};
use crate::protocol_core::{Message, StaticMessageInfo};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum BleMessageType{
    HubProperties=0x01,
    PortInformationRequest=0x21,
    PortInputFormatSetup=0x41,
    PortOutputCommand=0x81,
    PortOutputCommandFeedback=0x82,
    PortValue=0x45,
    IOAttached=0x04,
    PortInputFormatAck=0x47
}

pub fn translate_blemessagetype_from_int(input: u32) -> Result<BleMessageType, Box<dyn Error>> {
    match input {
        0x01=> Ok(BleMessageType::HubProperties),
        0x21=> Ok(BleMessageType::PortInformationRequest),
        0x41=> Ok(BleMessageType::PortInputFormatSetup),
        0x81=> Ok(BleMessageType::PortOutputCommand),
        0x82=> Ok(BleMessageType::PortOutputCommandFeedback),
        0x45=> Ok(BleMessageType::PortValue),
        0x04=> Ok(BleMessageType::IOAttached),
        0x47=> Ok(BleMessageType::PortInputFormatAck),
        _ => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "UnknownBleMessageType: ".to_string() + &input.to_string())))
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum Port{
    A=0,
    B=1,
    C=2,
    D=3,
    LED=50,
    TILT=99,
}

pub fn translate_port_from_int(input: u32) -> Result<Port, Box<dyn Error>> {
    match input {
        0=> Ok(Port::A),
        1=> Ok(Port::B),
        2=> Ok(Port::C),
        3=> Ok(Port::D),
        50=> Ok(Port::LED),
        99=> Ok(Port::TILT),
        _ => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "UnknownPort")))
    }
}

#[derive(Serialize, Deserialize)]
pub struct RegisterMotor {
    pub port: Port
}

impl Message for RegisterMotor {
    #[inline]
    fn to_json(&self) -> std::result::Result<std::string::String, serde_json::Error> {
        serde_json::to_string(&self)
    }
    fn get_topic_dyn(&self) -> std::string::String {
        return "brickcontrol/motor/register".to_string();
    }
}

impl StaticMessageInfo for RegisterMotor {
    #[inline]
    fn get_topic() -> std::string::String {
        return "brickcontrol/motor/register".to_string();
    }
}

#[derive(Serialize, Deserialize)]
pub struct SetMotorPwm {
    pub pwm: i8,
    pub port: Port
}

impl Message for SetMotorPwm {
    #[inline]
    fn to_json(&self) -> std::result::Result<std::string::String, serde_json::Error> {
        serde_json::to_string(&self)
    }
    fn get_topic_dyn(&self) -> std::string::String {
        return "brickcontrol/motor/pwm".to_string();
    }
}

impl StaticMessageInfo for SetMotorPwm {
    #[inline]
    fn get_topic() -> std::string::String {
        return "brickcontrol/motor/pwm".to_string();
    }
}

#[derive(Serialize, Deserialize)]
pub struct SetMotorPwmMultiple {
    pub motor_pwms: Vec<SetMotorPwm>
}

impl Message for SetMotorPwmMultiple {
    #[inline]
    fn to_json(&self) -> std::result::Result<std::string::String, serde_json::Error> {
        serde_json::to_string(&self)
    }
    fn get_topic_dyn(&self) -> std::string::String {
        return "brickcontrol/motor/pwm_multiple".to_string();
    }
}

impl StaticMessageInfo for SetMotorPwmMultiple {
    #[inline]
    fn get_topic() -> std::string::String {
        return "brickcontrol/motor/pwm_multiple".to_string();
    }
}

#[derive(Serialize, Deserialize)]
pub struct SetMotorSpeed {
    pub speed: i8,
    pub port: Port,
    pub max_power: u8
}

impl Message for SetMotorSpeed {
    #[inline]
    fn to_json(&self) -> std::result::Result<std::string::String, serde_json::Error> {
        serde_json::to_string(&self)
    }
    fn get_topic_dyn(&self) -> std::string::String {
        return "brickcontrol/motor/set_speed".to_string();
    }
}

impl StaticMessageInfo for SetMotorSpeed {
    #[inline]
    fn get_topic() -> std::string::String {
        return "brickcontrol/motor/set_speed".to_string();
    }
}

#[derive(Serialize, Deserialize)]
pub struct MotorGoToPosition {
    pub speed: i8,
    pub port: Port,
    pub max_power: u8,
    pub target_angle: i32
}

impl Message for MotorGoToPosition {
    #[inline]
    fn to_json(&self) -> std::result::Result<std::string::String, serde_json::Error> {
        serde_json::to_string(&self)
    }
    fn get_topic_dyn(&self) -> std::string::String {
        return "brickcontrol/motor/go_to_position".to_string();
    }
}

impl StaticMessageInfo for MotorGoToPosition {
    #[inline]
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
    fn to_json(&self) -> std::result::Result<std::string::String, serde_json::Error> {
        serde_json::to_string(&self)
    }
    fn get_topic_dyn(&self) -> std::string::String {
        return "brickcontrol/motor/output/command_feedback".to_string();
    }
}

impl StaticMessageInfo for MotorCommandFeedback {
    #[inline]
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
    fn to_json(&self) -> std::result::Result<std::string::String, serde_json::Error> {
        serde_json::to_string(&self)
    }
    fn get_topic_dyn(&self) -> std::string::String {
        return "brickcontrol/generic/set_mode_update".to_string();
    }
}

impl StaticMessageInfo for EnableModeUpdates {
    #[inline]
    fn get_topic() -> std::string::String {
        return "brickcontrol/generic/set_mode_update".to_string();
    }
}

#[derive(Serialize, Deserialize)]
pub struct ModeUpdateAck {
    pub port: Port,
    pub mode: u8,
    pub notifications_enabled: u8,
    pub delta: u32
}

impl Message for ModeUpdateAck {
    #[inline]
    fn to_json(&self) -> std::result::Result<std::string::String, serde_json::Error> {
        serde_json::to_string(&self)
    }
    fn get_topic_dyn(&self) -> std::string::String {
        return "brickcontrol/generic/mode_update_ack".to_string();
    }
}

impl StaticMessageInfo for ModeUpdateAck {
    #[inline]
    fn get_topic() -> std::string::String {
        return "brickcontrol/generic/mode_update_ack".to_string();
    }
}

#[derive(Serialize, Deserialize)]
pub struct MotorPositionUpdate {
    pub position: i32,
    pub port: Port
}

impl Message for MotorPositionUpdate {
    #[inline]
    fn to_json(&self) -> std::result::Result<std::string::String, serde_json::Error> {
        serde_json::to_string(&self)
    }
    fn get_topic_dyn(&self) -> std::string::String {
        return "brickcontrol/motor/output/position_update".to_string();
    }
}

impl StaticMessageInfo for MotorPositionUpdate {
    #[inline]
    fn get_topic() -> std::string::String {
        return "brickcontrol/motor/output/position_update".to_string();
    }
}

#[derive(Serialize, Deserialize)]
pub struct TiltMeasurement {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl Message for TiltMeasurement {
    #[inline]
    fn to_json(&self) -> std::result::Result<std::string::String, serde_json::Error> {
        serde_json::to_string(&self)
    }
    fn get_topic_dyn(&self) -> std::string::String {
        return "brickcontrol/tilt/position_update".to_string();
    }
}

impl StaticMessageInfo for TiltMeasurement {
    #[inline]
    fn get_topic() -> std::string::String {
        return "brickcontrol/tilt/position_update".to_string();
    }
}

#[derive(Serialize, Deserialize)]
pub struct RequestBatteryStatus {

}

impl Message for RequestBatteryStatus {
    #[inline]
    fn to_json(&self) -> std::result::Result<std::string::String, serde_json::Error> {
        serde_json::to_string(&self)
    }
    fn get_topic_dyn(&self) -> std::string::String {
        return "brickcontrol/battery/request_status".to_string();
    }
}

impl StaticMessageInfo for RequestBatteryStatus {
    #[inline]
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
    fn to_json(&self) -> std::result::Result<std::string::String, serde_json::Error> {
        serde_json::to_string(&self)
    }
    fn get_topic_dyn(&self) -> std::string::String {
        return "brickcontrol/battery/status".to_string();
    }
}

impl StaticMessageInfo for BatteryStatus {
    #[inline]
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
    fn to_json(&self) -> std::result::Result<std::string::String, serde_json::Error> {
        serde_json::to_string(&self)
    }
    fn get_topic_dyn(&self) -> std::string::String {
        return "unused".to_string();
    }
}

impl StaticMessageInfo for AttachmentInfo {
    #[inline]
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
    fn to_json(&self) -> std::result::Result<std::string::String, serde_json::Error> {
        serde_json::to_string(&self)
    }
    fn get_topic_dyn(&self) -> std::string::String {
        return "brickcontrol/io/connection_update".to_string();
    }
}

impl StaticMessageInfo for AttachedIo {
    #[inline]
    fn get_topic() -> std::string::String {
        return "brickcontrol/io/connection_update".to_string();
    }
}

#[derive(Serialize, Deserialize)]
pub struct PortInformationRequest {
    pub port_id: u8
}

impl Message for PortInformationRequest {
    #[inline]
    fn to_json(&self) -> std::result::Result<std::string::String, serde_json::Error> {
        serde_json::to_string(&self)
    }
    fn get_topic_dyn(&self) -> std::string::String {
        return "brickcontrol/generic/read_port".to_string();
    }
}

impl StaticMessageInfo for PortInformationRequest {
    #[inline]
    fn get_topic() -> std::string::String {
        return "brickcontrol/generic/read_port".to_string();
    }
}

#[derive(Serialize, Deserialize)]
pub struct SetLedColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

impl Message for SetLedColor {
    // For this to work, you need to set the LED to mode 1 first:
    // eg mosquitto_pub -t "brickcontrol/generic/set_mode_update" -m '{"notifications_enabled": 0, "delta": 0, "mode": 1, "port": "LED"}'
    // Then the LED is off by default! But brick is still connected and new color can be set
    #[inline]
    fn to_json(&self) -> std::result::Result<std::string::String, serde_json::Error> {
        serde_json::to_string(&self)
    }
    fn get_topic_dyn(&self) -> std::string::String {
        return "brickcontrol/led/set_color".to_string();
    }
}

impl StaticMessageInfo for SetLedColor {
    #[inline]
    fn get_topic() -> std::string::String {
        return "brickcontrol/led/set_color".to_string();
    }
}


