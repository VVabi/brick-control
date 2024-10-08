use crate::motor_messages::*;

pub trait BleSerializationExt {
    fn get_cmd_id(&self) -> u8;
    fn serialize(&self, output: &mut Vec<u8>);
}

impl BleSerializationExt for SetMotorPwm {
    fn get_cmd_id(&self) -> u8 {
        BleMessageType::PortOutputCommand as u8
    }
    fn serialize(&self, output: &mut Vec<u8>) {
        output.push(self.port as u8);
        output.push(1 as u8);
        output.push(1 as u8);
        output.push(self.pwm as u8);
    }
}

impl BleSerializationExt for MotorGoToPosition {
    fn get_cmd_id(&self) -> u8 {
        BleMessageType::PortOutputCommand as u8
    }
    fn serialize(&self, output: &mut Vec<u8>) {
        output.push(self.port as u8);
        output.push(1 as u8);
        output.push(0x0D as u8);
        output.extend_from_slice(&self.target_angle.to_le_bytes());
        output.push(self.speed as u8);
        output.push(self.max_power as u8);
        output.push(126 as u8);
        output.push(0 as u8);
    }
}

impl BleSerializationExt for EnableModeUpdates {
    fn get_cmd_id(&self) -> u8 {
        BleMessageType::PortInputFormatSetup as u8
    }
    fn serialize(&self, output: &mut Vec<u8>) {
        output.push(self.port as u8);
        output.push(self.mode);
        output.extend_from_slice(&self.delta.to_le_bytes());
        output.push(self.notifications_enabled);
    }
}

impl BleSerializationExt for RequestBatteryStatus {
    fn get_cmd_id(&self) -> u8 {
        BleMessageType::HubProperties as u8
    }

    fn serialize(&self, output: &mut Vec<u8>) {
        output.push(0x06);
        output.push(0x05);
    }
}

impl BleSerializationExt for SetMotorSpeed {
    fn get_cmd_id(&self) -> u8 {
        BleMessageType::PortOutputCommand as u8
    }

    fn serialize(&self, output: &mut Vec<u8>) {
        output.push(self.port as u8);
        output.push(1 as u8);
        output.push(0x07 as u8);
        output.push(self.speed as u8);
        output.push(self.max_power);
        output.push(0);
    }
}

impl BleSerializationExt for PortInformationRequest {
    fn get_cmd_id(&self) -> u8 {
        BleMessageType::PortInformationRequest as u8
    }

    fn serialize(&self, output: &mut Vec<u8>) {
        output.push(self.port_id);
        output.push(0x01);
    }
}

impl BleSerializationExt for SetLedColor {
    fn get_cmd_id(&self) -> u8 {
        BleMessageType::PortOutputCommand as u8
    }

    fn serialize(&self, output: &mut Vec<u8>) {
        output.push(50);
        output.push(0x11);
        output.push(0x51);
        output.push(1);
        output.push(self.red);
        output.push(self.green);
        output.push(self.blue);
    }
}