// References
// https://wiki.osdev.org/Serial_Ports

use super::asm;

pub struct Serial {
    port: asm::Port,
}

pub const COM1: Serial = Serial {
    port: 0x03f8,
};

impl Serial {
    pub fn put_char(&self, byte: u8) {
        while !self.can_send() {}
        asm::outb(self.port, byte);
    }

    fn can_send(&self) -> bool {
        let line_status_register: asm::Port = self.line_status_register();
        let line_status: u8 = asm::inb(line_status_register);
        line_status & 0x20 != 0
    }

    fn line_status_register(&self) -> asm::Port {
        self.port + 5
    }
}

