use crate::abstraction::{Component, Visual};
use crate::data::Size;

pub struct button {
    
}

impl Component for button {
    fn measure(&self, available: Size<f32>) -> Size<f32> {
        todo!()
    }

    fn arrange(&mut self, given: Size<f32>) -> Size<f32> {
        todo!()
    }

    fn render<F>(&self, dispatch: F)
    where
        F: FnMut(&dyn Visual)
    {
        todo!()
    }
}