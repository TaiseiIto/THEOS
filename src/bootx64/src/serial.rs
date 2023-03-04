// References
// https://wiki.osdev.org/Serial_Ports

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
        let serial = Self {
            port,
        };
        // Disable all interrupts.
        let interrupt_enable_register: asm::Port = serial.interrupt_enable_register();
        let disable_all_interrupts: u8 = 0;
        asm::outb(interrupt_enable_register, disable_all_interrupts);
        // Set baud.
        serial.set_baud(baud);
        serial
    }

    pub fn put_byte(&self, byte: u8) {
        while !self.can_send() {}
        asm::outb(self.port, byte);
    }

    fn can_send(&self) -> bool {
        let line_status_register: asm::Port = self.line_status_register();
        let line_status: u8 = asm::inb(line_status_register);
        line_status & 0x20 != 0
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
        // Enable DLAB.
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
        // Disable DLAB.
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

