// References
// https://www.lookrs232.com/rs232/lsr.htm

use super::super::asm;

pub struct LineStatusRegister {
    data_ready: bool,
    overrun_error: bool,
    parity_error: bool,
    framing_error: bool,
    break_interrupt: bool,
    empty_transmitter_holding_register: bool,
    empty_data_holging_registers: bool,
    error_in_received_fifo: bool,
}

const DATA_READY: u8 = 0x01;
const OVERRUN_ERROR: u8 = 0x02;
const PARITY_ERROR: u8 = 0x04;
const FRAMING_ERROR: u8 = 0x08;
const BREAK_INDICATOR: u8 = 0x10;
const TRANSMITTER_HOLDING_REGISTER_EMPTY: u8 = 0x20;
const TRANSMITTER_EMPTY: u8 = 0x40;
const IMPENDING_ERROR: u8 = 0x80;

impl LineStatusRegister {
    pub fn empty_transmitter_holding_register(&self) -> bool {
        self.empty_transmitter_holding_register
    }
}

impl From<asm::Port> for LineStatusRegister {
    fn from(port: asm::Port) -> Self {
        let line_status_register: u8 = asm::inb(port);
        line_status_register.into()
    }
}

impl From<u8> for LineStatusRegister {
    fn from(byte: u8) -> Self {
        let data_ready: bool = byte & DATA_READY != 0;
        let overrun_error: bool = byte & OVERRUN_ERROR != 0;
        let parity_error: bool = byte & PARITY_ERROR != 0;
        let framing_error: bool = byte & FRAMING_ERROR != 0;
        let break_interrupt: bool = byte & BREAK_INDICATOR != 0;
        let empty_transmitter_holding_register: bool = byte & TRANSMITTER_HOLDING_REGISTER_EMPTY != 0;
        let empty_data_holging_registers: bool = byte & TRANSMITTER_EMPTY != 0;
        let error_in_received_fifo: bool = byte & IMPENDING_ERROR != 0;
        Self {
            data_ready,
            overrun_error,
            parity_error,
            framing_error,
            break_interrupt,
            empty_transmitter_holding_register,
            empty_data_holging_registers,
            error_in_received_fifo,
        }
    }
}

impl Into<u8> for &LineStatusRegister {
    fn into(self) -> u8 {
        let data_ready: u8 = match self.data_ready {
            true => DATA_READY,
            false => 0x00,
        };
        let overrun_error: u8 = match self.overrun_error {
            true => OVERRUN_ERROR,
            false => 0x00,
        };
        let parity_error: u8 = match self.parity_error {
            true => PARITY_ERROR,
            false => 0x00,
        };
        let framing_error: u8 = match self.framing_error {
            true => FRAMING_ERROR,
            false => 0x00,
        };
        let break_interrupt: u8 = match self.break_interrupt {
            true => BREAK_INDICATOR,
            false => 0x00,
        };
        let empty_transmitter_holding_register: u8 = match self.empty_transmitter_holding_register {
            true => TRANSMITTER_HOLDING_REGISTER_EMPTY,
            false => 0x00,
        };
        let empty_data_holging_registers: u8 = match self.empty_data_holging_registers {
            true => TRANSMITTER_EMPTY,
            false => 0x00,
        };
        let error_in_received_fifo: u8 = match self.error_in_received_fifo {
            true => IMPENDING_ERROR,
            false => 0x00,
        };
        data_ready
        | overrun_error
        | parity_error
        | framing_error
        | break_interrupt
        | empty_transmitter_holding_register
        | empty_data_holging_registers
        | error_in_received_fifo
    }
}

