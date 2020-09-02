use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::collections::VecDeque;
use std::error::Error;
use crate::mqtt_wrapper::mqtt_thread::MqttStrMessage;
use crate::protocol::Message;
use crate::protocol::messages::*;
use crate::protocol::BleSerializationExt;
use crate::library::types::*;
use std::collections::HashMap;

pub struct MqttMessenger<'a> {
    pub tx: &'a Sender<MqttStrMessage>,
    pub rx: &'a Receiver<MqttStrMessage>,
    msg_list: HashMap<String, VecDeque<MqttStrMessage>>
}

impl<'a> MqttMessenger<'a> {
    pub fn new(txi: &'a Sender<MqttStrMessage>, rxi: &'a Receiver<MqttStrMessage>) -> MqttMessenger<'a> {
        Self {tx: txi, rx: rxi, msg_list: HashMap::new()}
    }

    fn push_msg(self: &mut Self, msg: MqttStrMessage) {
        let list = self.msg_list.entry(msg.topic.clone()).or_insert(VecDeque::new()); //no clue why cloning is required...
        list.push_back(msg);
    }
}

fn publish_message(m: &dyn Message, tx: &Sender<MqttStrMessage>) -> Result<(), Box<dyn Error>> {
    let result = m.to_json()?;
    tx.send(MqttStrMessage { topic: m.get_topic_dyn(), payload: result })?;
    Ok(())
}



impl Messenger for MqttMessenger<'_> {
    fn publish_message(self: &Self, m: &dyn Message) -> Result<(), Box<dyn Error>> {
        publish_message(m, self.tx)?;
        Ok(())
    }
    fn receive_message(self: &mut Self, topic: &str) -> Vec<std::string::String> {
        //this still could be more efficient...
        loop {
            let x = self.rx.try_recv();
            match { x } {
                Ok(msg) => self.push_msg(msg),
                Err(_m) => break
            }
        }

        let list = self.msg_list.entry(topic.to_string()).or_insert(VecDeque::new());

        let mut ret = Vec::new();

        for msg in list {
            ret.push(msg.payload.clone());
        }

        ret
    }

    fn receive_ble_message(self: &mut Self) -> Result<Option<Box<dyn BleSerializationExt>>, Box<dyn Error>> {
        let x = self.rx.try_recv();

        match x {
            Ok(v) => {
                log::debug!("Incoming message; Topic: {} Payload: {}", v.topic, v.payload);
                if v.topic == SetMotorPwm::get_topic() {
                    let meas = serde_json::from_str::<SetMotorPwm>(&v.payload)?;
                    return Ok(Some(Box::new(meas)));
                } else if v.topic == MotorGoToPosition::get_topic() {
                    let meas = serde_json::from_str::<MotorGoToPosition>(&v.payload)?;
                    return Ok(Some(Box::new(meas)));
                } else if v.topic == RequestBatteryStatus::get_topic() {
                    let meas = serde_json::from_str::<RequestBatteryStatus>(&v.payload)?;
                    return Ok(Some(Box::new(meas)));
                } else if v.topic == EnableModeUpdates::get_topic() {
                    let meas = serde_json::from_str::<EnableModeUpdates>(&v.payload)?;
                    return Ok(Some(Box::new(meas)));
                } else {
                    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Unknown topic")))
                }
            }
            Err(_e) => return Ok(None)
        }
    }
}