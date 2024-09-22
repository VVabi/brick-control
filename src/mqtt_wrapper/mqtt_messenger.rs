use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::collections::VecDeque;
use std::error::Error;
use crate::mqtt_wrapper::mqtt_thread::MqttStrMessage;
use crate::protocol::protocol_core::{Message, StaticMessageInfo};
use crate::protocol::motor_messages::*;
use crate::protocol::BleSerializationExt;
use crate::library::types::*;
use crate::listeners::*;

pub struct MqttMessenger<'a> {
    pub tx: &'a Sender<MqttStrMessage>,
    pub rx: &'a Receiver<MqttStrMessage>,
    listeners: Vec<Box<dyn ConnectionListener>>,
}

impl<'a> MqttMessenger<'a> {
    pub fn new(txi: &'a Sender<MqttStrMessage>, rxi: &'a Receiver<MqttStrMessage>, listeners: Vec<Box<dyn ConnectionListener>> ) -> MqttMessenger<'a> {
        Self {tx: txi, rx: rxi, listeners: listeners}
    }

    fn parse_message(self: &Self, v: &MqttStrMessage) -> Result<Box<dyn Message>, Box<dyn Error>> {
        if v.topic == SetMotorPwm::get_topic() {
            let meas = serde_json::from_str::<SetMotorPwm>(&v.payload)?;
            return Ok(Box::new(meas));
        } else if v.topic == MotorGoToPosition::get_topic() {
            let meas = serde_json::from_str::<MotorGoToPosition>(&v.payload)?;
            return Ok(Box::new(meas));
        } else if v.topic == RequestBatteryStatus::get_topic() {
            let meas = serde_json::from_str::<RequestBatteryStatus>(&v.payload)?;
            return Ok(Box::new(meas));
        } else if v.topic == EnableModeUpdates::get_topic() {
            let meas = serde_json::from_str::<EnableModeUpdates>(&v.payload)?;
            return Ok(Box::new(meas));
        } else if v.topic == SetMotorSpeed::get_topic() {
            let meas = serde_json::from_str::<SetMotorSpeed>(&v.payload)?;
            return Ok(Box::new(meas));    
        } else if v.topic == PortInformationRequest::get_topic() {
            let meas = serde_json::from_str::<PortInformationRequest>(&v.payload)?;
            return Ok(Box::new(meas));
        } else if v.topic == SetLedColor::get_topic() {
            let meas = serde_json::from_str::<SetLedColor>(&v.payload)?;
            return Ok(Box::new(meas));    
        } else {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Unknown topic")))
        }
    }

    fn parse_message_ble(self: &Self, v: &MqttStrMessage) -> Result<Box<dyn BleSerializationExt>, Box<dyn Error>> { //TODO this duplicates a lot of code
        if v.topic == SetMotorPwm::get_topic() {
            let meas = serde_json::from_str::<SetMotorPwm>(&v.payload)?;
            return Ok(Box::new(meas));
        } else if v.topic == MotorGoToPosition::get_topic() {
            let meas = serde_json::from_str::<MotorGoToPosition>(&v.payload)?;
            return Ok(Box::new(meas));
        } else if v.topic == RequestBatteryStatus::get_topic() {
            let meas = serde_json::from_str::<RequestBatteryStatus>(&v.payload)?;
            return Ok(Box::new(meas));
        } else if v.topic == EnableModeUpdates::get_topic() {
            let meas = serde_json::from_str::<EnableModeUpdates>(&v.payload)?;
            return Ok(Box::new(meas));
        } else if v.topic == SetMotorSpeed::get_topic() {
            let meas = serde_json::from_str::<SetMotorSpeed>(&v.payload)?;
            return Ok(Box::new(meas));    
        } else if v.topic == PortInformationRequest::get_topic() {
            let meas = serde_json::from_str::<PortInformationRequest>(&v.payload)?;
            return Ok(Box::new(meas));
        } else if v.topic == SetLedColor::get_topic() {
            let meas = serde_json::from_str::<SetLedColor>(&v.payload)?;
            return Ok(Box::new(meas));    
        } else {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Unknown topic for ble conversion")))
        }
    }
}

impl Messenger for MqttMessenger<'_> {
    fn publish_message(self: &Self, m: &dyn Message) -> Result<(), Box<dyn Error>> {
        let mut publish = true;

        for listener in &self.listeners {
            publish = publish && listener.on_upstream_message(&*m);
        }
    
        if publish {
            let result = m.to_json()?;
            self.tx.send(MqttStrMessage { topic: m.get_topic_dyn(), payload: result })?;
        }
        
        Ok(())
    }   

    fn receive_ble_message(self: &mut Self) -> Result<Option<Box<dyn BleSerializationExt>>, Box<dyn Error>> {
        let x = self.rx.try_recv();
        match x {
            Ok(m) => {
                let message = self.parse_message(&m)?;
                let mut send_downstream = true;
                for listener in &self.listeners {
                    send_downstream = send_downstream && listener.on_downstream_message(&*message);
                }
                if send_downstream {
                    let result = self.parse_message_ble(&m)?;
                    return Ok(Some(result));
                } else {
                    return Ok(None);
                }
            },
            Err(_) => Ok(None),
        }
    }
}