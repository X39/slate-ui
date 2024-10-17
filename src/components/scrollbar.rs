use std::cmp::min;
use crate::abstraction::{Component, Visual};
use crate::color::Color;
use crate::data::{Orientation, Size};
use crate::visuals;

pub struct ScrollBar {
    /// The orientation of the scroll bar.
    // #[property]
    pub mode: Orientation,
    /// The thickness of the scroll bar.
    // #[property]
    pub thickness: f32,
    /// The minimum value of the scroll bar
    // #[property]
    pub min_value: f32,
    /// The actual value of the scroll bar
    // #[property]
    pub value: f32,
    /// The maximum value of the scroll bar
    // #[property]
    pub max_value: f32,
    /// The value the bar represents.
    // #[property]
    pub bar_value: f32,

    render_thickness: f32,
    size: Size<f32>,
    bar_length: f32,
    bar_offset: f32,
}
impl Component for ScrollBar {
    fn measure(&self, available: Size<f32>) -> Size<f32> {
        match self.mode {
            Orientation::Horizontal => {
                Size::<f32> {
                    height: self.thickness,
                    width: available.width,
                }
            }
            Orientation::Vertical => {
                Size::<f32> {
                    height: available.height,
                    width: self.thickness,
                }
            }
        }
    }

    fn arrange(&mut self, given: Size<f32>) -> Size<f32> {
        let p_value = (self.value - self.min_value) / (self.max_value - self.min_value);
        let p_bar = f32::min(self.bar_value / (self.max_value - self.min_value), 1.0);
        self.size = match self.mode {
            Orientation::Horizontal => {
                self.render_thickness = f32::min(self.thickness, given.height);
                let bar_total = given.width - self.render_thickness - self.render_thickness;
                self.bar_length = bar_total * p_bar;
                self.bar_offset = (bar_total - self.bar_length) * p_value;
                Size::<f32> {
                    height: self.render_thickness,
                    width: given.width,
                }
            }
            Orientation::Vertical => {
                self.render_thickness = f32::min(self.thickness, given.width);
                let bar_total = given.height - self.render_thickness - self.render_thickness;
                self.bar_length = bar_total * p_bar;
                self.bar_offset = (bar_total - (self.bar_length)) * p_value;
                Size::<f32> {
                    height: given.height,
                    width: self.render_thickness,
                }
            }
        };
        self.size
    }

    fn render<F>(&self, mut dispatch: F)
    where
        F: FnMut(&dyn Visual),
    {
        let thickness = self.render_thickness;
        let bar_length = self.bar_length;
        let bar_offset = self.bar_offset;
        match self.mode {
            Orientation::Horizontal => {
                let length = self.size.width;

                // Thumb Left
                dispatch(&visuals::Rectangle { y: 0f32, x: 0f32, height: thickness, width: thickness, fill: Color::GRAY });

                // Thumb Right
                dispatch(&visuals::Rectangle { y: 0f32, x: length - thickness, height: thickness, width: thickness, fill: Color::GRAY });

                // Scroll Area
                dispatch(&visuals::Rectangle { y: 0f32, x: thickness, height: thickness, width: length - thickness - thickness, fill: Color::LIGHT_GRAY });

                // Scroll Bar
                dispatch(&visuals::Rectangle { y: 0f32, x: bar_offset + thickness, height: thickness, width: bar_length, fill: Color::DARK_GRAY });
            }
            Orientation::Vertical => {
                let length = self.size.height;

                // Thumb Top
                dispatch(&visuals::Rectangle { x: 0f32, y: 0f32, width: thickness, height: thickness, fill: Color::AQUA });

                // Thumb Bottom
                dispatch(&visuals::Rectangle { x: 0f32, y: length - thickness, width: thickness, height: thickness, fill: Color::BEIGE });

                // Scroll Area
                dispatch(&visuals::Rectangle { x: 0f32, y: thickness, width: thickness, height: length - thickness - thickness, fill: Color::GRAY });

                // Scroll Bar
                dispatch(&visuals::Rectangle { x: 0f32, y: bar_offset, width: thickness, height: bar_length, fill: Color::GOLD });
            }
        }
    }
}

impl ScrollBar {
    pub fn new() -> ScrollBar {
        ScrollBar {
            bar_offset: 0f32,
            bar_value: 0f32,
            max_value: 0f32,
            value: 0f32,
            bar_length: 0f32,
            size: Size { width: 0f32, height: 0f32 },
            mode: Orientation::Horizontal,
            thickness: 0f32,
            render_thickness: 0f32,
            min_value: 0f32,
        }
    }
}