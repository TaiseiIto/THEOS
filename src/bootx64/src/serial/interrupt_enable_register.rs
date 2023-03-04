use super::super::asm;

pub struct InterruptEnableRegister {
    data_available: bool,
    transmitter_empty: bool,
    line_status: bool,
    modem_status: bool,
}

const DATA_AVAILABLE: u8 = 0x01;
const TRANSMITTER_EMPTY: u8 = 0x02;
const LINE_STATUS: u8 = 0x04;
const MODEM_STATUS: u8 = 0x08;

impl InterruptEnableRegister {
    pub fn disable_all_interrupts() -> Self {
        let data_available: bool = false;
        let transmitter_empty: bool = false;
        let line_status: bool = false;
        let modem_status: bool = false;
        Self::new(
            data_available,
            transmitter_empty,
            line_status,
            modem_status,
        )
    }

    pub fn new(
        data_available: bool,
        transmitter_empty: bool,
        line_status: bool,
        modem_status: bool,
    ) -> Self {
        Self {
            data_available,
            transmitter_empty,
            line_status,
            modem_status,
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
        let data_available: bool = byte & DATA_AVAILABLE != 0;
        let transmitter_empty: bool = byte & TRANSMITTER_EMPTY != 0;
        let line_status: bool = byte & LINE_STATUS != 0;
        let modem_status: bool = byte & MODEM_STATUS != 0;
        Self {
            data_available,
            transmitter_empty,
            line_status,
            modem_status,
        }
    }
}

impl Into<u8> for &InterruptEnableRegister {
    fn into(self) -> u8 {
        let data_available: u8 = match self.data_available {
            true => DATA_AVAILABLE,
            false => 0x00,
        };
        let transmitter_empty: u8 = match self.transmitter_empty {
            true => TRANSMITTER_EMPTY,
            false => 0x00,
        };
        let line_status: u8 = match self.line_status {
            true => LINE_STATUS,
            false => 0x00,
        };
        let modem_status: u8 = match self.modem_status {
            true => MODEM_STATUS,
            false => 0x00,
        };
        data_available
        | transmitter_empty
        | line_status
        | modem_status
    }
}

