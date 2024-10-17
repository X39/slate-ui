use crate::abstraction::{Renderer, Visual};
use crate::color::Color;

// #[visual]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub fill: Color,
}

impl Visual for Rectangle {
    fn draw(&self, renderer: &mut dyn Renderer) {
        renderer.draw_rectangle(self.x, self.y, self.width, self.height, self.fill);
    }
}