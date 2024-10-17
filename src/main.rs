use crate::abstraction::{Component, Renderer};
use crate::data::Size;
use crate::renderers::SkiaRenderer;

mod abstraction;
mod data;
mod visuals;
mod color;
mod components;
mod renderers;

fn main() {
    const width: f32 = 100f32;
    const height: f32 = 100f32;
    let mut renderer = SkiaRenderer::new(width, height).unwrap();


    renderer.translate(0f32, 3f32);
    let mut scroll_bar = components::ScrollBar::new();
    scroll_bar.max_value = 100f32;
    scroll_bar.value = 0f32;
    scroll_bar.bar_value = 40f32;
    scroll_bar.thickness = 14f32;
    scroll_bar.measure(Size { width, height });
    scroll_bar.arrange(Size { width, height });
    scroll_bar.render(|visual| visual.draw(&mut renderer));
    renderer.translate(0f32, 17f32);

    renderer.translate(0f32, 3f32);
    let mut scroll_bar = components::ScrollBar::new();
    scroll_bar.max_value = 100f32;
    scroll_bar.value = 25f32;
    scroll_bar.bar_value = 40f32;
    scroll_bar.thickness = 14f32;
    scroll_bar.measure(Size { width, height });
    scroll_bar.arrange(Size { width, height });
    scroll_bar.render(|visual| visual.draw(&mut renderer));
    renderer.translate(0f32, 17f32);

    renderer.translate(0f32, 3f32);
    let mut scroll_bar = components::ScrollBar::new();
    scroll_bar.max_value = 100f32;
    scroll_bar.value = 50f32;
    scroll_bar.bar_value = 40f32;
    scroll_bar.thickness = 14f32;
    scroll_bar.measure(Size { width, height });
    scroll_bar.arrange(Size { width, height });
    scroll_bar.render(|visual| visual.draw(&mut renderer));
    renderer.translate(0f32, 17f32);

    renderer.translate(0f32, 3f32);
    let mut scroll_bar = components::ScrollBar::new();
    scroll_bar.max_value = 100f32;
    scroll_bar.value = 75f32;
    scroll_bar.bar_value = 40f32;
    scroll_bar.thickness = 14f32;
    scroll_bar.measure(Size { width, height });
    scroll_bar.arrange(Size { width, height });
    scroll_bar.render(|visual| visual.draw(&mut renderer));
    renderer.translate(0f32, 17f32);

    renderer.translate(0f32, 3f32);
    let mut scroll_bar = components::ScrollBar::new();
    scroll_bar.max_value = 100f32;
    scroll_bar.value = 100f32;
    scroll_bar.bar_value = 40f32;
    scroll_bar.thickness = 14f32;
    scroll_bar.measure(Size { width, height });
    scroll_bar.arrange(Size { width, height });
    scroll_bar.render(|visual| visual.draw(&mut renderer));
    renderer.translate(0f32, 17f32);


    unsafe { renderer.save(std::path::Path::new("output.png".into())); }
}