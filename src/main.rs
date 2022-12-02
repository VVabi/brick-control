mod protocol;
mod ble_brick_device;
mod library;
mod mqtt_wrapper;

use crate::protocol::*;
use mqtt_wrapper::mqtt_thread::launch_mqtt;
use mqtt_wrapper::mqtt_messenger::MqttMessenger;
use clap::{Arg, App};


fn main() {
    let matches = App::new("brick-control")
        .version("0.1.0")
        .author("")
        .about("Translator from MQTT to LEGO bluetooth hubs")
        .arg(Arg::with_name("prefix")
                 .short("p")
                 .long("prefix")
                 .takes_value(true)
                 .help("Optional prefix: If non-empty, MQTT topics will be $prefix/brick-control/..., otherwise just brick-control/..."))
        .arg(Arg::with_name("mac")
                 .short("m")
                 .long("mac")
                 .takes_value(true)
                 .help("MAC address of LEGO hub to connect to. If left empty, will connect to first available hub."))
        .get_matches();

    let prefix = matches.value_of("prefix").unwrap_or("");
    let mac = matches.value_of("mac");

    env_logger::init();
    let subscriptions = vec![motor_messages::SetMotorPwm::get_topic(), motor_messages::MotorGoToPosition::get_topic(), motor_messages::EnableModeUpdates::get_topic(), motor_messages::RequestBatteryStatus::get_topic(), motor_messages::SetMotorSpeed::get_topic(), motor_messages::PortInformationRequest::get_topic()];
    let (tx, rx) = launch_mqtt("localhost".to_string(), 1883, subscriptions, prefix.to_string());
    let mut mqtt_messenger = MqttMessenger::new(&tx, &rx);
    let device = ble_brick_device::init_ble_communication(mac).unwrap();

    log::info!("Found Technic hub");
    device.run_loop(& mut mqtt_messenger);
}
