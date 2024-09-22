use crate::protocol::protocol_core::Message;

pub trait ConnectionListener {
    fn on_downstream_message(self: &Self, m: &dyn Message) -> bool;
    fn on_upstream_message(self: &Self, m: &dyn Message) -> bool;
}


pub struct PrintListener {

}

impl ConnectionListener for PrintListener {
    fn on_downstream_message(self: &Self, m: &dyn Message) -> bool {
        println!("Downstream message: Topic {:?} Payload {:?}", m.get_topic_dyn(), m.to_json());
        true
    }

    fn on_upstream_message(self: &Self, m: &dyn Message) -> bool {
        println!("Upstream message: Topic {:?} Payload {:?}", m.get_topic_dyn(), m.to_json());
        true
    }
}