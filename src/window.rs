use crate::abstraction::{Component, RenderSurface};
use winit::window::WindowId;
use crate::components;
use crate::data::Size;

pub struct AppWindow {
    pub(crate) id: Option<WindowId>,
    pub(crate) generation: usize,
    pub(crate) render_surface: Option<Box<dyn RenderSurface>>,
}

impl AppWindow {
    pub(crate) fn new(
        id: Option<WindowId>,
        render_surface: Option<Box<dyn RenderSurface>>,
        generation: usize,
    ) -> AppWindow {
        AppWindow {
            id,
            render_surface,
            generation,
        }
    }

    pub fn render(&mut self) {
        const width: f32 = 100f32;
        const height: f32 = 100f32;
        let render_surface = self.render_surface.as_mut().unwrap();
        render_surface.start_rendering();

        render_surface.translate(0f32, 3f32);
        let mut scroll_bar = components::ScrollBar::new();
        scroll_bar.max_value = 100f32;
        scroll_bar.value = 0f32;
        scroll_bar.bar_value = 40f32;
        scroll_bar.thickness = 14f32;
        scroll_bar.measure(Size { width, height });
        scroll_bar.arrange(Size { width, height });
        scroll_bar.render(render_surface.renderer());
        render_surface.translate(0f32, 17f32);

        render_surface.translate(0f32, 3f32);
        let mut scroll_bar = components::ScrollBar::new();
        scroll_bar.max_value = 100f32;
        scroll_bar.value = 25f32;
        scroll_bar.bar_value = 40f32;
        scroll_bar.thickness = 14f32;
        scroll_bar.measure(Size { width, height });
        scroll_bar.arrange(Size { width, height });
        scroll_bar.render(render_surface.renderer());
        render_surface.translate(0f32, 17f32);

        render_surface.translate(0f32, 3f32);
        let mut scroll_bar = components::ScrollBar::new();
        scroll_bar.max_value = 100f32;
        scroll_bar.value = 50f32;
        scroll_bar.bar_value = 40f32;
        scroll_bar.thickness = 14f32;
        scroll_bar.measure(Size { width, height });
        scroll_bar.arrange(Size { width, height });
        scroll_bar.render(render_surface.renderer());
        render_surface.translate(0f32, 17f32);

        render_surface.translate(0f32, 3f32);
        let mut scroll_bar = components::ScrollBar::new();
        scroll_bar.max_value = 100f32;
        scroll_bar.value = 75f32;
        scroll_bar.bar_value = 40f32;
        scroll_bar.thickness = 14f32;
        scroll_bar.measure(Size { width, height });
        scroll_bar.arrange(Size { width, height });
        scroll_bar.render(render_surface.renderer());
        render_surface.translate(0f32, 17f32);

        render_surface.finish_rendering();
    }
}
