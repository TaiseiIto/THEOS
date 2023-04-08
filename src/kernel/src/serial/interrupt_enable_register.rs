// References
// https://www.lookrs232.com/rs232/ier.htm

use super::super::asm;

pub struct InterruptEnableRegister {
    received_data_available: bool,
    transmitter_holding_register_empty: bool,
    receiver_line_status: bool,
    modem_status: bool,
    sleep: bool,
    low_power: bool,
}

const RECEIVED_DATA_AVAILABLE: u8 = 0x01;
const TRANSMITTER_HOLDING_REGISTER_EMPTY: u8 = 0x02;
const RECEIVER_LINE_STATUS: u8 = 0x04;
const MODEM_STATUS: u8 = 0x08;
const SLEEP: u8 = 0x10;
const LOW_POWER: u8 = 0x20;

impl InterruptEnableRegister {
    pub fn disable_all_interrupts() -> Self {
        let received_data_available: bool = false;
        let transmitter_holding_register_empty: bool = false;
        let receiver_line_status: bool = false;
        let modem_status: bool = false;
        let sleep: bool = false;
        let low_power: bool = false;
        Self::new(
            received_data_available,
            transmitter_holding_register_empty,
            receiver_line_status,
            modem_status,
            sleep,
            low_power,
        )
    }

    pub fn new(
        received_data_available: bool,
        transmitter_holding_register_empty: bool,
        receiver_line_status: bool,
        modem_status: bool,
        sleep: bool,
        low_power: bool,
    ) -> Self {
        Self {
            received_data_available,
            transmitter_holding_register_empty,
            receiver_line_status,
            modem_status,
            sleep,
            low_power,
        }
    }
}

impl From<asm::Port> for InterruptEnableRegister {
    fn from(port: asm::Port) -> Self {
        let interrupt_enable_register: u8 = asm::inb(port);
        interrupt_enable_register.into()
    }
}

impl From<u8> for InterruptEnableRegister {
    fn from(byte: u8) -> Self {
        let received_data_available: bool = byte & RECEIVED_DATA_AVAILABLE != 0;
        let transmitter_holding_register_empty: bool = byte & TRANSMITTER_HOLDING_REGISTER_EMPTY != 0;
        let receiver_line_status: bool = byte & RECEIVER_LINE_STATUS != 0;
        let modem_status: bool = byte & MODEM_STATUS != 0;
        let sleep: bool = byte & SLEEP != 0;
        let low_power: bool = byte & LOW_POWER != 0;
        Self {
            received_data_available,
            transmitter_holding_register_empty,
            receiver_line_status,
            modem_status,
            sleep,
            low_power,
        }
    }
}

impl Into<u8> for &InterruptEnableRegister {
    fn into(self) -> u8 {
        let received_data_available: u8 = match self.received_data_available {
            true => RECEIVED_DATA_AVAILABLE,
            false => 0x00,
        };
        let transmitter_holding_register_empty: u8 = match self.transmitter_holding_register_empty {
            true => TRANSMITTER_HOLDING_REGISTER_EMPTY,
            false => 0x00,
        };
        let receiver_line_status: u8 = match self.receiver_line_status {
            true => RECEIVER_LINE_STATUS,
            false => 0x00,
        };
        let modem_status: u8 = match self.modem_status {
            true => MODEM_STATUS,
            false => 0x00,
        };
        let sleep: u8 = match self.sleep {
            true => SLEEP,
            false => 0x00,
        };
        let low_power: u8 = match self.low_power {
            true => LOW_POWER,
            false => 0x00,
        };
        received_data_available
        | transmitter_holding_register_empty
        | receiver_line_status
        | modem_status
        | sleep
        | low_power
    }
}

