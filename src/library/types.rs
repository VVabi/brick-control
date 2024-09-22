use std::error::Error;
use crate::protocol::protocol_core::Message;
use crate::protocol::ble_ext::BleSerializationExt;

pub trait Messenger {
    fn publish_message(self: &mut Self, m: &dyn Message) -> Result<(), Box<dyn Error>>;
    fn receive_ble_message(self: &mut Self) -> Result<Option<Box<dyn BleSerializationExt>>, Box<dyn Error>>;
}