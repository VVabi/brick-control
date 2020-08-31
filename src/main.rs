mod protocol;
mod ble_brick_device;
mod library;
mod mqtt_wrapper;

use crate::protocol::*;
use mqtt_wrapper::mqtt_thread::launch_mqtt;
use mqtt_wrapper::mqtt_messenger::MqttMessenger;





fn main() {
    let subscriptions = vec![messages::SetMotorPwm::get_topic()];
    let (tx, rx) = launch_mqtt("localhost".to_string(), 1883, subscriptions);
    let mut mqtt_messenger = MqttMessenger::new(&tx, &rx);
    let device = ble_brick_device::init_ble_communication();
    device.run_loop(& mut mqtt_messenger);
}
