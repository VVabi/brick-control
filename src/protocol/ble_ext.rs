use crate::messages::SetMotorPwm;

pub trait BleSerializationExt {
    fn get_cmd_id(&self) -> u8;
    fn serialize(&self, output: &mut Vec<u8>);
}

impl BleSerializationExt for SetMotorPwm {
    fn get_cmd_id(&self) -> u8 {
        0x81 as u8
    }
    fn serialize(&self, output: &mut Vec<u8>) {
        output.push(self.port);
        output.push(1 as u8);
        output.push(1 as u8);
        output.push(self.pwm as u8);
    }
}
