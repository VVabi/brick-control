extern crate blurz;
use blurz::bluetooth_adapter::BluetoothAdapter;
use blurz::bluetooth_device::BluetoothDevice;
use blurz::bluetooth_session::BluetoothSession;
use blurz::bluetooth_gatt_service::BluetoothGATTService;
use blurz::bluetooth_gatt_characteristic::BluetoothGATTCharacteristic;
use blurz::bluetooth_event::BluetoothEvent;
use crate::library::types::*;
use crate::protocol::protocol_core::Message;
use crate::protocol::*;
use crate::listeners::*;
use std::error::Error;
use std::{thread, time};

pub struct BleBrickDevice {
    session:        blurz::bluetooth_session::BluetoothSession,
    characteristic_path: String,
    listeners:      Vec<Box<dyn ConnectionListener>>
}

fn serialize_ble_cmd(cmd: &dyn ble_ext::BleSerializationExt) -> Vec<u8> {
    let mut ret = vec![0, 0, cmd.get_cmd_id()];
    cmd.serialize(&mut ret);
    ret[0] = ret.len() as u8;
    ret
}

fn parse_response(id: u8, values: &[u8]) -> Result<Box<dyn Message>, Box<dyn Error>> {
    match translate_blemessagetype_from_int(id as u32)? {
        BleMessageType::HubProperties => {
            if values.len() >= 3 && values[0] == 6 { 
                let status = values[2];

                return Ok(Box::new(motor_messages::BatteryStatus { charging_state: status }));

            } else {
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "cannot interpret response" )));
            }
        }
        BleMessageType::PortValue => {
            let port = motor_messages::translate_port_from_int(values[0] as u32)?;
            if (port as u8) < 4 {
                let t = [values[1], values[2], values[3], values[4]];
                let position = i32::from_le_bytes(t);
                return Ok(Box::new(motor_messages::MotorPositionUpdate { position: position , port: port}));
            } else if matches!(port, Port::TILT) {
                let x: i32 = i16::from_le_bytes([values[1], values[2]]) as i32;
                let y: i32 = i16::from_le_bytes([values[3], values[4]]) as i32;
                let z: i32 = i16::from_le_bytes([values[5], values[6]]) as i32;
                return Ok(Box::new(motor_messages::TiltMeasurement { x: x, y: y, z: z}));
            } else {
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "cannot interpret response" )));
            }
        }
        BleMessageType::IOAttached => {
            if values.len() >= 2  { 
                let port = values[0];
                let event = values[1];
                
                let mut message = motor_messages::AttachedIo { port_id: port, event: event, info: Vec::new()};
                if event == 1 && values.len() >= 12 {
                    let type_id = u16::from_le_bytes([values[2], values[3]]) as u32;
                    let hw_rev  = i32::from_le_bytes([values[4], values[5], values[6], values[7]]);
                    let sw_rev  = i32::from_le_bytes([values[8], values[9], values[10], values[11]]);
                    message.info.push(motor_messages::AttachmentInfo {type_id : type_id, hw_rev : hw_rev, sw_rev : sw_rev});
                }

                return Ok(Box::new(message));
            } else {
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "cannot interpret response" )));
            }
        }
        BleMessageType::PortOutputCommandFeedback => {
            if values.len() >= 2  {
                let port        = values[0];
                let flags       = values[1];
                let message     = motor_messages::MotorCommandFeedback { port : motor_messages::translate_port_from_int(port as u32)?, flags: flags};
                return Ok(Box::new(message));
            } else {
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "cannot interpret response" )));
            }        
        }

        BleMessageType::PortInputFormatAck => {
            if values.len() >= 2  {
                let port        = values[0];
                let mode        = values[1];
                let delta_interval = u32::from_le_bytes([values[2], values[3], values[4], values[5]]);
                let notification_enabled = values[6];
                let message     = motor_messages::ModeUpdateAck { port : motor_messages::translate_port_from_int(port as u32)?, 
                    mode: mode, 
                    delta: delta_interval, 
                    notifications_enabled: notification_enabled};
                return Ok(Box::new(message));
            } else {
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "cannot interpret response" )));
            }        
        }
        _ => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "id not found: ".to_string()+&id.to_string())))
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
                        match x {
                            Some(v) => {
                                cnt = cnt+1;

                                if cnt < 10 {
                                    let data = serialize_ble_cmd(&*v);
                                    log::debug!("BLE WRITE: {:?}", data);
                                    characteristic.write_value(data, None).unwrap();
                                } else {
                                    log::error!("too many incoming commands, discarding to keep communication to hub alive");                                
                                }
                            }
                            None => break //No more commands available for now
                        }

                    } 
                    Err(v) => {
                        log::error!("Serialization to BLE command failed: {:?}", v);
                    }
                }
            }
        }
    }
}



pub fn init_ble_communication(mac: Option<&str>) -> Result<BleBrickDevice, Box<dyn Error>> {
    let session = BluetoothSession::create_session(None)?;
    let adapter: BluetoothAdapter = BluetoothAdapter::init(&session)?;

    let device_list = adapter.get_device_list();

    let mut path = "".to_string();
    let mut found = false;
    for d in device_list {
        for x in d {
            let device = BluetoothDevice::new(&session, x.clone());
            let local_name = device.get_name().unwrap_or_default();
            log::info!(
                "{} {:?} {:?}",
                device.get_id(),
                device.get_address()?,
                local_name
            );

            if local_name == "Technic Hub" {
                
                let address = device.get_address().unwrap_or_default();
                
                if let Some(specified_address)= mac {
                    println!("{:?} {:?}", address, specified_address);
                    if address == specified_address {
                        path = x.clone();
                        found = true;
                    }
                } else {
                    path = x.clone();
                    found = true;
                }
            }
        }
    }

    if !found {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Technic Hub not found")));
    }

    let device = BluetoothDevice::new(&session, path.clone());
    device.connect(10000)?;

    // Sometimes (usually on the first connect), the below call to get_gatt_services() returns an empty list without an error.AttachmentInfo
    // Looks like a sort of race condition in the device? Waiting first seems to help a lot at least
    // --------------------------------
    for _ in 0..10 {
        thread::sleep(time::Duration::from_millis(500));
        let res = device.get_gatt_services()?;

        for x in res {
            let service = BluetoothGATTService::new(&session, x.clone());
            let uuid = service.get_uuid().unwrap();
            log::info!("Found uuid {:?}", uuid);
            if uuid == "00001623-1212-efde-1623-785feabcd123" {
                let char_path = x.clone();
                let service = BluetoothGATTService::new(&session, char_path.clone());
                let chars = service.get_gatt_characteristics().unwrap();
                let characteristic_path = chars.first().unwrap().to_string(); //TODO is this correct in all situations?
            
                return Ok(BleBrickDevice {session, characteristic_path, listeners: Vec::new()});
            }
        }
    }

    Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "GATT characteristic not found")))
}