use super::super::asm;

pub fn disable_dlab(port: asm::Port) {
    let mut line_control_register: LineControlRegister = port.into();
    if line_control_register.dlab {
        line_control_register.dlab = false;
        let line_control_register: u8 = (&line_control_register).into();
        asm::outb(port, line_control_register);
    }
}

pub fn enable_dlab(port: asm::Port) {
    let mut line_control_register: LineControlRegister = port.into();
    if !line_control_register.dlab {
        line_control_register.dlab = true;
        let line_control_register: u8 = (&line_control_register).into();
        asm::outb(port, line_control_register);
    }
}

pub struct LineControlRegister {
    character_length: CharacterLength,
    stop_bit: StopBit,
    parity: Parity,
    dlab: bool,
}

const DLAB: u8 = 0x80;

impl From<asm::Port> for LineControlRegister {
    fn from(port: asm::Port) -> Self {
        let line_control_register: u8 = asm::inb(port);
        line_control_register.into()
    }
}

impl From<u8> for LineControlRegister {
    fn from(byte: u8) -> Self {
        let character_length: CharacterLength = byte.into();
        let stop_bit: StopBit = byte.into();
        let parity: Parity = byte.into();
        let dlab: bool = match byte & 0x80 {
            0x00 => false,
            DLAB => true,
            _ => panic!("Can't get serial line control register!"),
        };
        Self {
            character_length,
            stop_bit,
            parity,
            dlab,
        }
    }
}

impl Into<u8> for &LineControlRegister {
    fn into(self) -> u8 {
        let character_length: u8 = (&self.character_length).into();
        let stop_bit: u8 = (&self.stop_bit).into();
        let parity: u8 = (&self.parity).into();
        let dlab: u8 = match self.dlab {
            false => 0x00,
            true => DLAB,
        };
        character_length | stop_bit | parity | dlab
    }
}

pub enum CharacterLength {
    Bit5,
    Bit6,
    Bit7,
    Bit8,
}

const CHARACTER_LENGTH_BIT_5: u8 = 0x00;
const CHARACTER_LENGTH_BIT_6: u8 = 0x01;
const CHARACTER_LENGTH_BIT_7: u8 = 0x02;
const CHARACTER_LENGTH_BIT_8: u8 = 0x03;

impl From<u8> for CharacterLength {
    fn from(byte: u8) -> Self {
        match byte & 0x03 {
            CHARACTER_LENGTH_BIT_5 => Self::Bit5,
            CHARACTER_LENGTH_BIT_6 => Self::Bit6,
            CHARACTER_LENGTH_BIT_7 => Self::Bit7,
            CHARACTER_LENGTH_BIT_8 => Self::Bit8,
            _ => panic!("Can't get serial character length!"),
        }
    }
}

impl Into<u8> for &CharacterLength {
    fn into(self) -> u8 {
        match self {
            CharacterLength::Bit5 => CHARACTER_LENGTH_BIT_5,
            CharacterLength::Bit6 => CHARACTER_LENGTH_BIT_6,
            CharacterLength::Bit7 => CHARACTER_LENGTH_BIT_7,
            CharacterLength::Bit8 => CHARACTER_LENGTH_BIT_8,
        }
    }
}

pub enum StopBit {
    Bit1,
    Bit2,
}

const STOP_BIT_1: u8 = 0x00;
const STOP_BIT_2: u8 = 0x04;

impl From<u8> for StopBit {
    fn from(byte: u8) -> Self {
        match byte & 0x04 {
            STOP_BIT_1 => Self::Bit1,
            STOP_BIT_2 => Self::Bit2,
            _ => panic!("Can't get serial stop bit length!"),
        }
    }
}

impl Into<u8> for &StopBit {
    fn into(self) -> u8 {
        match self {
            StopBit::Bit1 => STOP_BIT_1,
            StopBit::Bit2 => STOP_BIT_2,
        }
    }
}

pub enum Parity {
    No,
    Odd,
    Even,
    Mark,
    Space,
}

const NO_PARITY: u8 = 0x00;
const ODD_PARITY: u8 = 0x08;
const EVEN_PARITY: u8 = 0x18;
const MARK_PARITY: u8 = 0x28;
const SPACE_PARITY: u8 = 0x38;

impl From<u8> for Parity {
    fn from(byte: u8) -> Self {
        match byte & 0x38 {
            NO_PARITY => Self::No,
            ODD_PARITY => Self::Odd,
            EVEN_PARITY => Self::Even,
            MARK_PARITY => Self::Mark,
            SPACE_PARITY => Self::Space,
            _ => panic!("Can't get serial parity type!"),
        }
    }
}

impl Into<u8> for &Parity {
    fn into(self) -> u8 {
        match self {
            Parity::No => NO_PARITY,
            Parity::Odd => ODD_PARITY,
            Parity::Even => EVEN_PARITY,
            Parity::Mark => MARK_PARITY,
            Parity::Space => SPACE_PARITY,
        }
    }
}

