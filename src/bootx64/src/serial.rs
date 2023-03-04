// References
// https://wiki.osdev.org/Serial_Ports
// https://www.lookrs232.com/

mod interrupt_enable_register;
mod interrupt_identification_register;
mod line_control_register;
mod line_status_register;
mod modem_control_register;

use super::asm;

pub struct Serial {
    port: asm::Port,
}

pub const COM1: asm::Port = 0x03f8;
pub const BAUD: u32 = 9600;
const FREQUENCY: u32 = 115200;

impl Serial {
    pub fn new(port: asm::Port, baud: u32) -> Self {
        // A new serial interface.
        let serial = Self {
            port,
        };

        // Disable all interrupts.
        let disable_all_interrupts = interrupt_enable_register::InterruptEnableRegister::disable_all_interrupts();
        serial.write_interrupt_enable_register(&disable_all_interrupts);

        // Set baud.
        serial.set_baud(baud);

        // Set a line control register.
        let character_length = line_control_register::CharacterLength::Bit8;
        let stop_bit = line_control_register::StopBit::Bit1;
        let parity = line_control_register::Parity::No;
        let set_break_enable = false;
        let divisor_latch_access_bit = false;
        let line_control_register = line_control_register::LineControlRegister::new(
            character_length,
            stop_bit,
            parity,
            set_break_enable,
            divisor_latch_access_bit,
        );
        serial.write_line_control_register(&line_control_register);

        // Enable FIFO
        let no_pending = true;
        let interrupt = interrupt_identification_register::Interrupt::ReceiverLineStatus;
        let timeout = false;
        let enabled_64_byte_fifo = false;
        let fifo = interrupt_identification_register::Fifo::Enabled;
        let interrupt_identification_register = interrupt_identification_register::InterruptIdentificationRegister::new(
            no_pending,
            interrupt,
            timeout,
            enabled_64_byte_fifo,
            fifo,
        );
        serial.write_interrupt_identification_register(&interrupt_identification_register);

        // Set modem
        let force_data_terminal_ready = true;
        let force_request_to_send = true;
        let aux_output_1 = false;
        let aux_output_2 = true;
        let loop_back_mode = false;
        let autoflow_control_enabled = false;
        let modem_control_register = modem_control_register::ModemControlRegister::new(
            force_data_terminal_ready,
            force_request_to_send,
            aux_output_1,
            aux_output_2,
            loop_back_mode,
            autoflow_control_enabled,
        );
        serial.write_modem_control_register(&modem_control_register);

        serial
    }

    pub fn put_byte(&self, byte: u8) {
        while !self.can_send() {}
        asm::outb(self.port, byte);
    }

    fn can_send(&self) -> bool {
        let line_status_register: line_status_register::LineStatusRegister = self.into();
        line_status_register.empty_transmitter_holding_register()
    }

    fn baud_low_register(&self) -> asm::Port {
        self.port
    }

    fn baud_high_register(&self) -> asm::Port {
        self.port + 1
    }

    fn interrupt_enable_register(&self) -> asm::Port {
        self.port + 1
    }

    fn interrupt_identification_register(&self) -> asm::Port {
        self.port + 2
    }

    fn line_control_register(&self) -> asm::Port {
        self.port + 3
    }

    fn modem_control_register(&self) -> asm::Port {
        self.port + 4
    }

    fn line_status_register(&self) -> asm::Port {
        self.port + 5
    }

    fn set_baud(&self, baud: u32) {
        self.enable_divisor_latch_access_bit();

        // Set low byte.
        let baud: u16 = (FREQUENCY / baud) as u16;
        let baud_low: u8 = baud as u8;
        let baud_low_register: asm::Port = self.baud_low_register();
        asm::outb(baud_low_register, baud_low);

        // Set high byte.
        let baud_high: u8 = (baud >> 8) as u8;
        let baud_high_register: asm::Port = self.baud_high_register();
        asm::outb(baud_high_register, baud_high);

        self.disable_divisor_latch_access_bit();
    }

    fn enable_divisor_latch_access_bit(&self) {
        let mut line_control_register: line_control_register::LineControlRegister = self.into();
        line_control_register.enable_divisor_latch_access_bit();
        self.write_line_control_register(&line_control_register);
    }

    fn disable_divisor_latch_access_bit(&self) {
        let mut line_control_register: line_control_register::LineControlRegister = self.into();
        line_control_register.disable_divisor_latch_access_bit();
        self.write_line_control_register(&line_control_register);
    }

    fn write_interrupt_enable_register(&self, interrupt_enable_register: &interrupt_enable_register::InterruptEnableRegister) {
        let port = self.interrupt_enable_register();
        let interrupt_enable_register: u8 = interrupt_enable_register.into();
        asm::outb(port, interrupt_enable_register);
    }

    fn write_interrupt_identification_register(&self, interrupt_identification_register: &interrupt_identification_register::InterruptIdentificationRegister) {
        let port = self.interrupt_identification_register();
        let interrupt_identification_register: u8 = interrupt_identification_register.into();
        asm::outb(port, interrupt_identification_register);
    }

    fn write_line_control_register(&self, line_control_register: &line_control_register::LineControlRegister) {
        let port = self.line_control_register();
        let line_control_register: u8 = line_control_register.into();
        asm::outb(port, line_control_register);
    }

    fn write_modem_control_register(&self, modem_control_register: &modem_control_register::ModemControlRegister) {
        let port = self.modem_control_register();
        let modem_control_register: u8 = modem_control_register.into();
        asm::outb(port, modem_control_register);
    }
}

impl Into<line_control_register::LineControlRegister> for &Serial {
    fn into(self) -> line_control_register::LineControlRegister {
        let port: asm::Port = self.line_control_register();
        port.into()
    }
}

impl Into<line_status_register::LineStatusRegister> for &Serial {
    fn into(self) -> line_status_register::LineStatusRegister {
        let port: asm::Port = self.line_status_register();
        port.into()
    }
}

