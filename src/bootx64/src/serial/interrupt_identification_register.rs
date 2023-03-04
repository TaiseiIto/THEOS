// References
// https://www.lookrs232.com/rs232/iir.htm

use super::super::asm;

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

