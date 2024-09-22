use crate::protocol::protocol_core::Message;
use crate::protocol::*;
use crate::protocol_core::*;
use std::collections::HashMap;

pub trait ConnectionListener {
    fn on_downstream_message(self: &mut Self, m: &dyn Message) -> (bool, Option<Box<dyn Message>>);
    fn on_upstream_message(self: &mut Self, m: &dyn Message) -> (bool, Option<Box<dyn Message>>);
}


pub struct PrintListener {

}

impl ConnectionListener for PrintListener {
    fn on_downstream_message(self: &mut Self, m: &dyn Message) -> (bool, Option<Box<dyn Message>>) {
        println!("Downstream message: Topic {:?} Payload {:?}", m.get_topic_dyn(), m.to_json());
        (true, None)
    }

    fn on_upstream_message(self: &mut Self, m: &dyn Message) -> (bool, Option<Box<dyn Message>>) {
        println!("Upstream message: Topic {:?} Payload {:?}", m.get_topic_dyn(), m.to_json());
        (true, None)
    }
}


pub struct AttachedIOListener {
    attached_ios: HashMap<u8, motor_messages::AttachedIo>,
}

impl AttachedIOListener {
    pub fn new() -> AttachedIOListener {
        AttachedIOListener {
            attached_ios: HashMap::new(),
        }
    }
}

impl ConnectionListener for AttachedIOListener {
    fn on_downstream_message(self: &mut Self, m: &dyn Message) -> (bool, Option<Box<dyn Message>>) {
        println!("Downstream message");
        if m.get_topic_dyn() == motor_messages::RequestAttachedIos::get_topic() {
            let mut ios: Vec<AttachedIo> = Vec::new();
            for (_key, value) in self.attached_ios.iter() {
                if value.event == 0 {
                    println!("Skipping event 0");
                    continue
                }
                
                let attached_io = serde_json::from_str::<motor_messages::AttachedIo>(&value.to_json().unwrap()).unwrap();
                ios.push(attached_io);
            }
            let attached_ios = motor_messages::AttachedIos {
                attached_devices: ios,
            };

            return (false, Some(Box::new(attached_ios)));
        }
        (true, None)
    }

    fn on_upstream_message(self: &mut Self, m: &dyn Message) -> (bool, Option<Box<dyn Message>>) {
        if m.get_topic_dyn() == motor_messages::AttachedIo::get_topic() {
            let payload = m.to_json().unwrap(); //TODO quite ugly
            
            let attached_io: AttachedIo = serde_json::from_str::<motor_messages::AttachedIo>(&payload).unwrap();
            self.attached_ios.insert(attached_io.port_id, attached_io);

            println!("Attached IOs:");

            for &key in self.attached_ios.keys() {
                println!("Port {:?} {:?}", key, self.attached_ios.get(&key).unwrap().to_json().unwrap());
            }
        }

        (true, None)
    }
}



