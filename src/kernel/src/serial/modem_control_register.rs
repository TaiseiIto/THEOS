// References
// https://www.lookrs232.com/rs232/mcr.htm

use super::super::asm;

pub struct ModemControlRegister {
    force_data_terminal_ready: bool,
    force_request_to_send: bool,
    aux_output_1: bool,
    aux_output_2: bool,
    loop_back_mode: bool,
    autoflow_control_enabled: bool,
}

const FORCE_DATA_TERMINAL_READY: u8 = 0x01;
const FORCE_REQUEST_TO_SEND: u8 = 0x02;
const AUX_OUTPUT_1: u8 = 0x04;
const AUX_OUTPUT_2: u8 = 0x08;
const LOOP_BACK_MODE: u8 = 0x10;
const AUTOFLOW_CONTROL_ENABLED: u8 = 0x20;

impl ModemControlRegister {
    pub fn new(
        force_data_terminal_ready: bool,
        force_request_to_send: bool,
        aux_output_1: bool,
        aux_output_2: bool,
        loop_back_mode: bool,
        autoflow_control_enabled: bool,
    ) -> Self {
        Self {
            force_data_terminal_ready,
            force_request_to_send,
            aux_output_1,
            aux_output_2,
            loop_back_mode,
            autoflow_control_enabled,
        }
    }
}

impl From<asm::Port> for ModemControlRegister {
    fn from(port: asm::Port) -> Self {
        let modem_control_register: u8 = asm::inb(port);
        modem_control_register.into()
    }
}

impl From<u8> for ModemControlRegister {
    fn from(byte: u8) -> Self {
        let force_data_terminal_ready: bool = byte & FORCE_DATA_TERMINAL_READY != 0;
        let force_request_to_send: bool = byte & FORCE_REQUEST_TO_SEND != 0;
        let aux_output_1: bool = byte & AUX_OUTPUT_1 != 0;
        let aux_output_2: bool = byte & AUX_OUTPUT_2 != 0;
        let loop_back_mode: bool = byte & LOOP_BACK_MODE != 0;
        let autoflow_control_enabled: bool = byte & AUTOFLOW_CONTROL_ENABLED != 0;
        Self {
            force_data_terminal_ready,
            force_request_to_send,
            aux_output_1,
            aux_output_2,
            loop_back_mode,
            autoflow_control_enabled,
        }
    }
}

impl Into<u8> for &ModemControlRegister {
    fn into(self) -> u8 {
        let force_data_terminal_ready: u8 = match self.force_data_terminal_ready {
            true => FORCE_DATA_TERMINAL_READY,
            false => 0x00,
        };
        let force_request_to_send: u8 = match self.force_request_to_send {
            true => FORCE_REQUEST_TO_SEND,
            false => 0x00,
        };
        let aux_output_1: u8 = match self.aux_output_1 {
            true => AUX_OUTPUT_1,
            false => 0x00,
        };
        let aux_output_2: u8 = match self.aux_output_2 {
            true => AUX_OUTPUT_2,
            false => 0x00,
        };
        let loop_back_mode: u8 = match self.loop_back_mode {
            true => LOOP_BACK_MODE,
            false => 0x00,
        };
        let autoflow_control_enabled: u8 = match self.autoflow_control_enabled {
            true => AUTOFLOW_CONTROL_ENABLED,
            false => 0x00,
        };
        force_data_terminal_ready
        | force_request_to_send
        | aux_output_1
        | aux_output_2
        | loop_back_mode
        | autoflow_control_enabled
    }
}

