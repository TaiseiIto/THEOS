// References
// https://wiki.osdev.org/Serial_Ports

mod line_control_register;

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

    pub fn put_char(&self, byte: u8) {
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
        let line_control_register: asm::Port = self.line_control_register();
        let line_control: u8 = asm::inb(line_control_register);
        asm::outb(line_control_register, line_control | line_control_register::DLAB);
        // Set low byte.
        let baud: u16 = (FREQUENCY / baud) as u16;
        let baud_low: u8 = baud as u8;
        let baud_low_register: asm::Port = self.baud_low_register();
        // Set high byte.
        asm::outb(baud_low_register, baud_low);
        let baud_high: u8 = (baud >> 8) as u8;
        let baud_high_register: asm::Port = self.baud_high_register();
        asm::outb(baud_high_register, baud_high);
        // Disable DLAB.
        asm::outb(line_control_register, line_control);
    }
}

