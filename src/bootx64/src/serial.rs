use super::asm;

pub struct Serial {
    port: asm::Port,
}

pub const COM1: Serial = Serial {
    port: 0x03f8,
};

impl Serial {
    pub fn put_char(&self, byte: u8) {
        asm::outb(self.port, byte);
    }
}

