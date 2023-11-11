use {
    core::ops::Add,
    super::uefi::protocols::{
        console_support::graphics_output,
        human_interface_infrastructure::font,
    },
};

pub struct Display<'a> {
    frame_buffer: &'a graphics_output::GraphicsOutput<'a>,
    font: &'a font::Font,
}

impl<'a> Display<'a> {
    const TAB_LENGTH: usize = 4;

    pub fn new(frame_buffer: &'a graphics_output::GraphicsOutput<'a>, font: &'a font::Font) -> Self {
        Self {
            frame_buffer,
            font,
        }
    }

    pub fn write_pixel(&self, coordinates: &Coordinates, color: &Color) {
        let Coordinates {
            x,
            y,
        } = coordinates;
        let Color {
            red,
            green,
            blue,
        } = color;
        self.frame_buffer.write_pixel(*x, *y, *red, *green, *blue);
    }

    pub fn put_char(
        &self,
        coordinates: &Coordinates,
        background_color: &Color,
        foreground_color: &Color,
        character: char) {
            self.font
                .glyph(character)
                .map(|glyph| glyph
                    .image()
                    .iter()
                    .for_each(|(pixel_coordinates, foreground)| {
                        let pixel_coordinates: Coordinates = pixel_coordinates.into();
                        let coordinates = coordinates + &pixel_coordinates;
                        let color: &Color = if *foreground {
                            foreground_color
                        } else {
                            background_color
                        };
                        self.write_pixel(&coordinates, color);
                    }));
    }

    pub fn print(
        &self,
        coordinates: &Coordinates,
        background_color: &Color,
        foreground_color: &Color,
        string: &str) {
        let mut x: usize = 0;
        let mut y: usize = 0;
        string
            .chars()
            .for_each(|character| {
                match character {
                    ' ' => {
                        x += 1;
                    },
                    '\n' => {
                        x = 0;
                        y += 1;
                    },
                    '\t' => {
                        x = (x / Self::TAB_LENGTH + 1) * Self::TAB_LENGTH;
                    },
                    '\r' => {
                        x = 0;
                    },
                    character => {
                        let max_width: u16 = self.font.max_width();
                        let max_width: u32 = max_width as u32;
                        let max_height: u16 = self.font.max_height();
                        let max_height: u32 = max_height as u32;
                        let character_x: u32 = x as u32 * max_width;
                        let character_y: u32 = y as u32 * max_height;
                        let character_coordinates: Coordinates = coordinates + &Coordinates::new(character_x, character_y);
                        self.put_char(&character_coordinates, background_color, foreground_color, character);
                        x += 1;
                    },
                }
            });
    }
}

pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
        }
    }
}

pub struct Coordinates {
    x: u32,
    y: u32,
}

impl Coordinates {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }
}

impl From<&font::Coordinates> for Coordinates {
    fn from(font: &font::Coordinates) -> Self {
        let x: u16 = font.x();
        let x: u32 = x as u32;
        let y: u16 = font.y();
        let y: u32 = y as u32;
        Self::new(x, y)
    }
}

impl Add for &Coordinates {
    type Output = Coordinates;

    fn add(self, other: Self) -> Self::Output {
        let x: u32 = self.x() + other.x();
        let y: u32 = self.y() + other.y();
        Self::Output::new(x, y)
    }
}

