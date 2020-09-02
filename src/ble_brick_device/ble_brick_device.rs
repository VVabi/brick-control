extern crate blurz;
use blurz::bluetooth_adapter::BluetoothAdapter;
use blurz::bluetooth_device::BluetoothDevice;
use blurz::bluetooth_session::BluetoothSession;
use blurz::bluetooth_gatt_service::BluetoothGATTService;
use blurz::bluetooth_gatt_characteristic::BluetoothGATTCharacteristic;
use blurz::bluetooth_event::BluetoothEvent;
use crate::library::types::*;
use crate::protocol::*;
use std::error::Error;

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

fn parse_response(id: u8, values: &[u8]) -> Result<Box<dyn Message>, Box<dyn Error>> {
    println!("{}", id);
    match id {
        0x01 => {
            if values.len() >= 3 && values[0] == 6 { 
                let status = values[2];

                return Ok(Box::new(messages::BatteryStatus { charging_state: status }));

            } else {
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "cannot interpret response" )));
            }
        }
        0x45 => {
            if values.len() >= 5  { 
                let t = [values[1], values[2], values[3], values[4]];
                let position = i32::from_le_bytes(t);
                return Ok(Box::new(messages::MotorPositionUpdate { position: position , port: values[0]}));
            } else {
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "cannot interpret response" )));
            }
        }
        _ => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "id not found")))
    }
}


impl BleBrickDevice {
    fn handle_incoming_message(&self, event: &BluetoothEvent, messenger: &mut dyn Messenger) -> Result<(), Box<dyn Error>> {
        match event {
            BluetoothEvent::Value { object_path: _, value } => {
                let len = value.len();
                if len >= 3 {
                    let id = value[2];
                    let res = parse_response(id, &value[3..len])?;
                    messenger.publish_message(&*res)?;
                } else {
                    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Could not parse header, too short")))
                }
            }   
            _ => ()
        }

        Ok(())
    }

    pub fn run_loop(&self, messenger: &mut dyn Messenger) {  
        let characteristic = BluetoothGATTCharacteristic::new(&self.session, self.characteristic_path.clone());
        characteristic.start_notify().unwrap();
        loop {
            for event in self.session.incoming(1).map(BluetoothEvent::from) {
                match event {
                    Some(x) => {
                            log::debug!("BLE RECEIVE {:?}", x);
                            let res = self.handle_incoming_message(&x, messenger);
                            if let Err(e) = res {
                                log::error!("Error in BLE receive: {:?}", e);
                            }
                        }
                        None    => (),    
                    }
                }

            let mut cnt: u32 = 0;

            loop {
                let next = messenger.receive_ble_message();

                match next {
                    Ok(x) => {
                        cnt = cnt+1;

                        if cnt < 5 {
                            let data = serialize_ble_cmd(&*x);
                            log::debug!("BLE WRITE: {:?}", data);
                            characteristic.write_value(data, None).unwrap();
                        } else {
                            log::error!("too many incoming commands, discarding to keep communication to hub alive");
                        }
                    }
                    Err(v) => {
                        if !v {
                            break;
                        }
                    }
                }
            }
        }
    }
}



pub fn init_ble_communication() -> Result<BleBrickDevice, Box<dyn Error>> {
    let session = BluetoothSession::create_session(None)?;
    let adapter: BluetoothAdapter = BluetoothAdapter::init(&session)?;

    let device_list = adapter.get_device_list();

    let mut name = "".to_string();
    let mut found = false;
    for d in device_list {
        for x in d {
            let device = BluetoothDevice::new(&session, x.clone());
            log::info!(
                "{} {:?} {:?}",
                device.get_id(),
                device.get_address()?,
                device.get_name()?
            );

            if device.get_name()? == "Technic Hub" {
                name = x.clone();
                found = true;
            }
        }
    }

    if !found {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Technic Hub not found")));
    }

    let device = BluetoothDevice::new(&session, name.clone());
    let result = device.connect(1000);

    if let Err(_err) = result {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Could not connect")));
    }

    let res = device.get_gatt_services()?;

    for x in res {
        let service = BluetoothGATTService::new(&session, x.clone());
        let uuid = service.get_uuid().unwrap();

        if uuid == "00001623-1212-efde-1623-785feabcd123" {
            let char_path = x.clone();
            let service = BluetoothGATTService::new(&session, char_path.clone());
            let chars = service.get_gatt_characteristics().unwrap();
            let characteristic_path = chars.first().unwrap().to_string(); //TODO is this correct in all situations?
        
            return Ok(BleBrickDevice {session, characteristic_path})
        }
    }

    Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "GATT characteristic not found")))
}