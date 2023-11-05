use {
    super::uefi::protocols::console_support::graphics_output,
};

pub struct Display<'a>(&'a graphics_output::GraphicsOutput<'a>);

impl<'a> Display<'a> {
    pub fn new(graphics: &'a graphics_output::GraphicsOutput<'a>) -> Self {
        Self(graphics)
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
        self.0.write_pixel(*x, *y, *red, *green, *blue);
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
}

