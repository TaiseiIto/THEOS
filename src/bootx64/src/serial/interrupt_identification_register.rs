// References
// https://www.lookrs232.com/rs232/iir.htm

use super::super::asm;

pub struct InterruptIdentificationRegister {
    no_pending: bool,
    interrupt: Interrupt,
    timeout: bool,
    enabled_64_byte_fifo: bool,
    fifo: Fifo,
}

const PENDING: u8 = 0x01;
const TIMEOUT: u8 = 0x08;
const ENABLED_64_BYTE_FIFO: u8 = 0x20;

impl InterruptIdentificationRegister {
    pub fn new(
        no_pending: bool,
        interrupt: Interrupt,
        timeout: bool,
        enabled_64_byte_fifo: bool,
        fifo: Fifo,
    ) -> Self {
        Self {
            no_pending,
            interrupt,
            timeout,
            enabled_64_byte_fifo,
            fifo,
        }
    }
}

impl From<asm::Port> for InterruptIdentificationRegister {
    fn from(port: asm::Port) -> Self {
        let interrupt_identification_register: u8 = asm::inb(port);
        interrupt_identification_register.into()
    }
}

impl From<u8> for InterruptIdentificationRegister {
    fn from(byte: u8) -> Self {
        let no_pending: bool = byte & PENDING != 0;
        let interrupt: Interrupt = byte.into();
        let timeout: bool = byte & TIMEOUT != 0;
        let enabled_64_byte_fifo: bool = byte & ENABLED_64_BYTE_FIFO != 0;
        let fifo: Fifo = byte.into();
        Self {
            no_pending,
            interrupt,
            timeout,
            enabled_64_byte_fifo,
            fifo,
        }
    }
}

impl Into<u8> for &InterruptIdentificationRegister {
    fn into(self) -> u8 {
        let no_pending: u8 = match self.no_pending {
            true => PENDING,
            false => 0x00,
        };
        let interrupt: u8 = (&self.interrupt).into();
        let timeout: u8 = match self.timeout {
            true => TIMEOUT,
            false => 0x00,
        };
        let enabled_64_byte_fifo: u8 = match self.enabled_64_byte_fifo {
            true => ENABLED_64_BYTE_FIFO,
            false => 0x00,
        };
        let fifo: u8 = (&self.fifo).into();
        no_pending
        | interrupt
        | timeout
        | enabled_64_byte_fifo
        | fifo
    }
}

pub enum Interrupt {
    ModemStatus,
    TransmitterHoldingRegisterEmpty,
    ReceivedDataAvailable,
    ReceiverLineStatus,
}

const MODEM_STATUS: u8 = 0x00;
const TRANSMITTER_HOLDING_REGISTER_EMPTY: u8 = 0x02;
const RECEIVED_DATA_AVAILABLE: u8 = 0x04;
const RECEIVER_LINE_STATUS: u8 = 0x06;
const INTERRUPT: u8 =
    MODEM_STATUS
    | TRANSMITTER_HOLDING_REGISTER_EMPTY
    | RECEIVED_DATA_AVAILABLE
    | RECEIVER_LINE_STATUS;

impl From<u8> for Interrupt {
    fn from(byte: u8) -> Self {
        match byte & INTERRUPT {
            MODEM_STATUS => Self::ModemStatus,
            TRANSMITTER_HOLDING_REGISTER_EMPTY => Self::TransmitterHoldingRegisterEmpty,
            RECEIVED_DATA_AVAILABLE => Self::ReceivedDataAvailable,
            RECEIVER_LINE_STATUS => Self::ReceiverLineStatus,
            _ => panic!("Can't get serial interrupt identifier."),
        }
    }
}

impl Into<u8> for &Interrupt {
    fn into(self) -> u8 {
        match self {
            Interrupt::ModemStatus => MODEM_STATUS,
            Interrupt::TransmitterHoldingRegisterEmpty => TRANSMITTER_HOLDING_REGISTER_EMPTY,
            Interrupt::ReceivedDataAvailable => RECEIVED_DATA_AVAILABLE,
            Interrupt::ReceiverLineStatus => RECEIVER_LINE_STATUS,
        }
    }
}

pub enum Fifo {
    No,
    Unusable,
    Enabled,
}

const NO_FIFO: u8 = 0x00;
const UNUSABLE_FIFO: u8 = 0x40;
const ENABLED_FIFO: u8 = 0xc0;
const FIFO: u8 =
    NO_FIFO
    | UNUSABLE_FIFO
    | ENABLED_FIFO;

impl From<u8> for Fifo {
    fn from(byte: u8) -> Self {
        match byte & FIFO {
            NO_FIFO => Self::No,
            UNUSABLE_FIFO => Self::Unusable,
            ENABLED_FIFO => Self::Enabled,
            _ => panic!("Can't get serial interrupt identifier."),
        }
    }
}

impl Into<u8> for &Fifo {
    fn into(self) -> u8 {
        match self {
            Fifo::No => NO_FIFO,
            Fifo::Unusable => UNUSABLE_FIFO,
            Fifo::Enabled => ENABLED_FIFO,
        }
    }
}

