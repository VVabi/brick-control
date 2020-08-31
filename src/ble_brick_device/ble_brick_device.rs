extern crate blurz;
use std::thread;
use blurz::bluetooth_adapter::BluetoothAdapter;
use blurz::bluetooth_device::BluetoothDevice;
use blurz::bluetooth_session::BluetoothSession;
use blurz::bluetooth_gatt_service::BluetoothGATTService;
use blurz::bluetooth_gatt_characteristic::BluetoothGATTCharacteristic;
use blurz::bluetooth_event::BluetoothEvent;
use crate::library::types::*;
use crate::protocol::*;

pub struct BleBrickDevice {
    session:        blurz::bluetooth_session::BluetoothSession,
    characteristic_path: String
}

fn serialize_ble_cmd(cmd: &dyn ble_ext::BleSerializationExt) -> Vec<u8> {
    let mut ret = vec![0, 0, cmd.get_cmd_id()];
    cmd.serialize(&mut ret);
    ret[0] = ret.len() as u8;
    ret
}

impl BleBrickDevice {
    pub fn run_loop(&self, messenger: &mut dyn Messenger) {  
        let characteristic = BluetoothGATTCharacteristic::new(&self.session, self.characteristic_path.clone());
        characteristic.start_notify().unwrap();
        loop {
            for event in self.session.incoming(1).map(BluetoothEvent::from) {
                match event {
                    Some(x) => println!("event {:?}", x),
                    None    => println!("None"),
                }
            }

            let list = messenger.receive_message(&messages::SetMotorPwm::get_topic());

            for msg in list {
                println!("{}", msg);
                let meas: messages::SetMotorPwm = serde_json::from_str(&msg).unwrap();
                characteristic.write_value(serialize_ble_cmd(&meas), None);
            }
        }
    }
}


pub fn init_ble_communication() -> BleBrickDevice{
    let session = BluetoothSession::create_session(None).unwrap();
    let adapter: BluetoothAdapter = BluetoothAdapter::init(&session).unwrap();

    let device_list = adapter.get_device_list();

    let mut name = "".to_string();
    for d in device_list {
        for x in d {
            let device = BluetoothDevice::new(&session, x.clone());
            println!(
                "{} {:?} {:?}",
                device.get_id(),
                device.get_address().unwrap(),
                device.get_name().unwrap()
            );

            if device.get_name().unwrap() == "Technic Hub" {
                name = x.clone()
            }
        }
    }

    let device = BluetoothDevice::new(&session, name.clone());
    let result = device.connect(1000);

    result.unwrap();

    let res = device.get_gatt_services().unwrap();

    for x in res {
        println!("{:?}", x);
        let service = BluetoothGATTService::new(&session, x.clone());
        let uuid = service.get_uuid().unwrap();
        println!("{:?}", uuid);
    }

    let char_path = "/org/bluez/hci0/dev_90_84_2B_58_1A_B1/service000c".to_string();
    let service = BluetoothGATTService::new(&session, char_path.clone());
    let chars = service.get_gatt_characteristics().unwrap();
    let characteristic_path = chars.first().unwrap().to_string();

    BleBrickDevice {session, characteristic_path}
}