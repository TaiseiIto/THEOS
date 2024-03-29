// References
// https://www.lookrs232.com/rs232/lcr.htm

use super::super::asm;

pub struct LineControlRegister {
    character_length: CharacterLength,
    stop_bit: StopBit,
    parity: Parity,
    set_break_enable: bool,
    divisor_latch_access_bit: bool,
}

const DIVISOR_LATCH_ACCESS_BIT: u8 = 0x80;
const SET_BREAK_ENABLE: u8 = 0x40;

impl LineControlRegister {
    pub fn new(
        character_length: CharacterLength,
        stop_bit: StopBit,
        parity: Parity,
        set_break_enable: bool,
        divisor_latch_access_bit: bool,
    ) -> Self {
        Self {
            character_length,
            stop_bit,
            parity,
            set_break_enable,
            divisor_latch_access_bit,
        }
    }

    pub fn enable_divisor_latch_access_bit(&mut self) {
        self.divisor_latch_access_bit = true;
    }

    pub fn disable_divisor_latch_access_bit(&mut self) {
        self.divisor_latch_access_bit = false;
    }
}

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
        let set_break_enable: bool = match byte & SET_BREAK_ENABLE {
            0x00 => false,
            SET_BREAK_ENABLE => true,
            _ => panic!("Can't get serial line control register!"),
        };
        let divisor_latch_access_bit: bool = match byte & DIVISOR_LATCH_ACCESS_BIT {
            0x00 => false,
            DIVISOR_LATCH_ACCESS_BIT => true,
            _ => panic!("Can't get serial line control register!"),
        };
        Self {
            character_length,
            stop_bit,
            parity,
            set_break_enable,
            divisor_latch_access_bit,
        }
    }
}

impl Into<u8> for &LineControlRegister {
    fn into(self) -> u8 {
        let character_length: u8 = (&self.character_length).into();
        let stop_bit: u8 = (&self.stop_bit).into();
        let parity: u8 = (&self.parity).into();
        let set_break_enable: u8 = match self.set_break_enable {
            false => 0x00,
            true => SET_BREAK_ENABLE,
        };
        let divisor_latch_access_bit: u8 = match self.divisor_latch_access_bit {
            false => 0x00,
            true => DIVISOR_LATCH_ACCESS_BIT,
        };
        character_length
        | stop_bit
        | parity
        | set_break_enable
        | divisor_latch_access_bit
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
const CHARACTER_LENGTH: u8 =
    CHARACTER_LENGTH_BIT_5
    | CHARACTER_LENGTH_BIT_6
    | CHARACTER_LENGTH_BIT_7
    | CHARACTER_LENGTH_BIT_8;

impl From<u8> for CharacterLength {
    fn from(byte: u8) -> Self {
        match byte & CHARACTER_LENGTH {
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
const STOP_BIT: u8 = STOP_BIT_1 | STOP_BIT_2;

impl From<u8> for StopBit {
    fn from(byte: u8) -> Self {
        match byte & STOP_BIT {
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
    High,
    Low,
}

const NO_PARITY: u8 = 0x00;
const ODD_PARITY: u8 = 0x08;
const EVEN_PARITY: u8 = 0x18;
const HIGH_PARITY: u8 = 0x28;
const LOW_PARITY: u8 = 0x38;
const PARITY: u8 =
    NO_PARITY
    | ODD_PARITY
    | EVEN_PARITY
    | HIGH_PARITY
    | LOW_PARITY;

impl From<u8> for Parity {
    fn from(byte: u8) -> Self {
        match byte & PARITY {
            NO_PARITY => Self::No,
            ODD_PARITY => Self::Odd,
            EVEN_PARITY => Self::Even,
            HIGH_PARITY => Self::High,
            LOW_PARITY => Self::Low,
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
            Parity::High => HIGH_PARITY,
            Parity::Low => LOW_PARITY,
        }
    }
}

