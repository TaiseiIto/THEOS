// References
// https://wiki.osdev.org/Serial_Ports
// https://www.lookrs232.com/

mod interrupt_enable_register;
mod line_control_register;
mod line_status_register;

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
        let dlab = false;
        let line_control_register = line_control_register::LineControlRegister::new(
            character_length,
            stop_bit,
            parity,
            dlab,
        );
        serial.write_line_control_register(&line_control_register);

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

    fn line_control_register(&self) -> asm::Port {
        self.port + 3
    }

    fn line_status_register(&self) -> asm::Port {
        self.port + 5
    }

    fn set_baud(&self, baud: u32) {
        self.enable_dlab();

        // Set low byte.
        let baud: u16 = (FREQUENCY / baud) as u16;
        let baud_low: u8 = baud as u8;
        let baud_low_register: asm::Port = self.baud_low_register();
        asm::outb(baud_low_register, baud_low);

        // Set high byte.
        let baud_high: u8 = (baud >> 8) as u8;
        let baud_high_register: asm::Port = self.baud_high_register();
        asm::outb(baud_high_register, baud_high);

        self.disable_dlab();
    }

    fn enable_dlab(&self) {
        let mut line_control_register: line_control_register::LineControlRegister = self.into();
        line_control_register.enable_dlab();
        self.write_line_control_register(&line_control_register);
    }

    fn disable_dlab(&self) {
        let mut line_control_register: line_control_register::LineControlRegister = self.into();
        line_control_register.disable_dlab();
        self.write_line_control_register(&line_control_register);
    }

    fn write_interrupt_enable_register(&self, interrupt_enable_register: &interrupt_enable_register::InterruptEnableRegister) {
        let port = self.interrupt_enable_register();
        let interrupt_enable_register: u8 = interrupt_enable_register.into();
        asm::outb(port, interrupt_enable_register);
    }

    fn write_line_control_register(&self, line_control_register: &line_control_register::LineControlRegister) {
        let port = self.line_control_register();
        let line_control_register: u8 = line_control_register.into();
        asm::outb(port, line_control_register);
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

