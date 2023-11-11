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

