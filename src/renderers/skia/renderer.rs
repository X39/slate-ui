use crate::abstraction::Renderer;
use crate::color::Color;
use skia_safe as sb;
use skia_safe::gpu::DirectContext;
use skia_safe::{Canvas, ISize, ImageInfo, Surface};
use winit::error::EventLoopError;
use winit::platform::windows::EventLoopBuilderExtWindows;
use winit::window::WindowAttributes;

#[derive(Debug)]
pub enum Errors {
    #[cfg(feature = "skia-gl")]
    FailedToCreateOpenGlInterface,
    #[cfg(feature = "skia-gl")]
    FailedToCreateOpenGlContext,
    #[cfg(feature = "skia-gl")]
    FailedToCreateOpenGlSurface,
    Winit(EventLoopError),
}
#[cfg(feature = "skia-gl")]
pub struct SkiaRenderer {
    pub context: DirectContext,
    pub size: ISize,
    pub surface: Surface,
    pub image_info: ImageInfo,
}
#[cfg(disable)]
pub struct SkiaRenderer {
    pub surface: Surface,
}

impl Renderer for SkiaRenderer {
    fn draw_rectangle(&mut self, x: f32, y: f32, width: f32, height: f32, fill: Color) {
        let rect = sb::Rect::new(x, y, width + x, height + y);
        let color = sb::Color4f::new(fill.red() as f32 / 255f32, fill.green() as f32 / 255f32, fill.blue() as f32 / 255f32, fill.alpha() as f32 / 255f32);
        let paint = sb::Paint::new(&color, None);
        self.canvas().draw_rect(&rect, &paint);
    }
    fn translate(&mut self, x: f32, y: f32) {
        let rect = sb::Vector::new(x, y);
        self.canvas().translate(rect);
    }
}

impl SkiaRenderer {
    #[cfg(feature = "skia-gl")]
    pub fn canvas(&mut self) -> &Canvas {
        self.surface.canvas()
    }
    #[cfg(feature = "skia-gl")]
    pub fn new<'a>(width: f32, height: f32) -> Result<SkiaRenderer, Errors> {

        




        ///////////////////////////////
        ///////////////////////////////
        ///////////////////////////////
        ///////////////////////////////
        ///////////////////////////////
        ///////////////////////////////

        use skia_safe::gpu::gl::Interface;
        let interface = match sb::gpu::gl::Interface::new_native() {
            None => return Err(Errors::FailedToCreateOpenGlInterface),
            Some(d) => d,
        };
        let mut context = match sb::gpu::direct_contexts::make_gl(interface, None) {
            None => return Err(Errors::FailedToCreateOpenGlContext),
            Some(d) => d,
        };
        let size = ISize::new(width as i32, height as i32);
        let image_info = ImageInfo::new_n32_premul(size, None);
        let surface = match sb::gpu::surfaces::render_target(
            &mut context,
            sb::gpu::Budgeted::Yes,
            &image_info,
            None,
            sb::gpu::SurfaceOrigin::TopLeft,
            None,
            false,
            None,
        ) {
            None => return Err(Errors::FailedToCreateOpenGlSurface),
            Some(d) => d,
        };
        Ok(SkiaRenderer {
            context,
            size,
            image_info,
            surface,
        })
    }
    #[cfg(disable)]
    pub fn canvas(&mut self) -> &Canvas {
        self.surface.canvas()
    }
    #[cfg(disable)]
    pub fn new<'a>(width: f32, height: f32) -> Result<SkiaRenderer, Errors> {
        let size = ISize::new(width as i32, height as i32);
        use skia_safe::surfaces;
        let surface = surfaces::raster_n32_premul(size).unwrap();

        Ok(SkiaRenderer {
            surface,
        })
    }

    #[cfg(disable)]
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