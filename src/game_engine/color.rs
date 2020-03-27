#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r, g, b, a }
    }

    pub fn as_array_rgba(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn as_array_rgb(&self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }
}