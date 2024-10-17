use crate::abstraction::Renderer;
use crate::color::Color;
use skia_safe as sb;
use skia_safe::{EncodedImageFormat, ISize, Surface};
use std::path::Path;

#[derive(Debug)]
pub enum Errors {}
pub struct SkiaRenderer {
    pub surface: Surface,
}

impl Renderer for SkiaRenderer {
    fn draw_rectangle(&mut self, x: f32, y: f32, width: f32, height: f32, fill: Color) {
        let rect = sb::Rect::new(x, y, width + x, height + y);
        let color = sb::Color4f::new(fill.red() as f32 / 255f32, fill.green() as f32 / 255f32, fill.blue() as f32 / 255f32, fill.alpha() as f32 / 255f32);
        let paint = sb::Paint::new(&color, None);
        self.surface.canvas().draw_rect(&rect, &paint);
    }
    fn translate(&mut self, x: f32, y: f32) {
        let rect = sb::Vector::new(x, y);
        self.surface.canvas().translate(rect);
    }
}

impl SkiaRenderer {
    pub fn new<'a>(width: f32, height: f32) -> Result<SkiaRenderer, Errors> {
        let size = ISize::new(width as i32, height as i32);
        use skia_safe::surfaces;
        let surface = surfaces::raster_n32_premul(size).unwrap();

        Ok(SkiaRenderer {
            surface,
        })
    }

    pub unsafe fn save(&mut self, path: &Path) {
        use std::{fs, io::Write};
        let image = self.surface.image_snapshot();
        let data = image
            .encode(&mut self.surface.direct_context(), EncodedImageFormat::PNG, None)
            .unwrap();
        fs::create_dir_all(path.parent().unwrap()).expect("failed to create directory");
        assert_eq!(path.extension().unwrap(), "png");

        let mut file = fs::File::create(path).expect("failed to create file");
        file.write_all(data.as_bytes()).expect("failed to write to file");
    }
}