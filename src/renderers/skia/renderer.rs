use crate::abstraction::{RenderSurface, Renderer};
use crate::color::Color;
use glutin::config::{ConfigTemplateBuilder, GlConfig};
use glutin::context::{
    ContextApi, ContextAttributesBuilder, NotCurrentGlContext, PossiblyCurrentContext,
};
use glutin::display::{GetGlDisplay, GlDisplay};
use glutin::surface::{GlSurface, SurfaceAttributesBuilder, WindowSurface};
use skia_safe as sb;
use skia_safe::gpu::ganesh::gl::backend_render_targets;
use skia_safe::gpu::gl::FramebufferInfo;
use skia_safe::gpu::SurfaceOrigin;
use skia_safe::{gpu, Canvas, ColorType, Surface};
use std::convert::TryInto;
use std::error::Error;
use std::ffi::CString;
use std::fmt::{Display, Formatter};
use std::num::NonZeroU32;
use skia_safe::colors::WHITE;
use winit::dpi::{LogicalSize, Size};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::raw_window_handle::HasRawWindowHandle;
use winit::window::{Window, WindowAttributes, WindowId};

#[derive(Debug)]
pub enum Errors {
    FailedToCreateWindow,
    FailedToCreateContextUsingGlutinAndGles(Box<dyn Error>, Box<dyn Error>),
    FailedToCreateGlSurface(Box<dyn Error>),
    FailedToCreateGlContextForSurface(Box<dyn Error>),
    FailedToCreateInterface,
}
impl std::fmt::Display for Errors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Errors::FailedToCreateWindow => write!(f, "Failed to create window"),
            Errors::FailedToCreateContextUsingGlutinAndGles(glutin_err, gles_err) => write!(
                f,
                "Failed to create context with both glutin ({}) and gles ({})",
                glutin_err, gles_err
            ),
            Errors::FailedToCreateGlSurface(err) => {
                write!(f, "Failed to create gl surface ({})", err)
            }
            Errors::FailedToCreateGlContextForSurface(err) => {
                write!(f, "Failed to create gl context for surface ({})", err)
            }
            Errors::FailedToCreateInterface => write!(f, "Failed to create interface"),
        }
    }
}
impl Error for Errors {}
#[cfg(feature = "skia-gl")]
pub struct SkiaRenderer {
    surface: Surface,
    gl_surface: glutin::surface::Surface<WindowSurface>,
    gr_context: skia_safe::gpu::DirectContext,
    gl_context: PossiblyCurrentContext,
    window: Window,
}
#[cfg(feature = "disable")]
pub struct SkiaRenderer {
    pub surface: Surface,
}

impl Renderer for SkiaRenderer {
    fn draw_rectangle(&mut self, x: f32, y: f32, width: f32, height: f32, fill: Color) {
        let rect = sb::Rect::new(x, y, width + x, height + y);
        let color = sb::Color4f::new(
            fill.red() as f32 / 255f32,
            fill.green() as f32 / 255f32,
            fill.blue() as f32 / 255f32,
            fill.alpha() as f32 / 255f32,
        );
        let paint = sb::Paint::new(&color, None);
        self.canvas().draw_rect(&rect, &paint);
    }
    fn translate(&mut self, x: f32, y: f32) {
        let rect = sb::Vector::new(x, y);
        self.canvas().translate(rect);
    }
}

impl RenderSurface for SkiaRenderer {
    fn create(event_loop: &ActiveEventLoop) -> Result<Box<Self>, Box<dyn Error>> {
        let attributes = WindowAttributes::default()
            .with_title("slate-ui")
            .with_inner_size(Size::Logical(LogicalSize::<f64>::new(800f64, 600f64)));
        let template = ConfigTemplateBuilder::new()
            .with_alpha_size(8)
            .with_transparency(true);
        let (window, gl_config) = match glutin_winit::DisplayBuilder::new()
            .with_window_attributes(Some(attributes))
            .build(event_loop, template, |configs| {
                // Find the config with the minimum number of samples. Usually Skia takes care of
                // anti-aliasing and may not be able to create appropriate Surfaces for samples > 0.
                // See https://github.com/rust-skia/rust-skia/issues/782
                // And https://github.com/rust-skia/rust-skia/issues/764
                configs
                    .reduce(|accum, config| {
                        let transparency_check = config.supports_transparency().unwrap_or(false)
                            & !accum.supports_transparency().unwrap_or(false);

                        if transparency_check || config.num_samples() < accum.num_samples() {
                            config
                        } else {
                            accum
                        }
                    })
                    .unwrap()
            }) {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        let window = match window {
            None => return Err(Box::new(Errors::FailedToCreateWindow)),
            Some(d) => d,
        };
        let raw_window_handle = match window.raw_window_handle() {
            Ok(d) => d,
            Err(e) => return Err(Box::new(e)),
        };

        let context_attributes = ContextAttributesBuilder::new().build(Some(raw_window_handle));
        let fallback_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::Gles(None))
            .build(Some(raw_window_handle));
        let not_current_gl_context = unsafe {
            match gl_config
                .display()
                .create_context(&gl_config, &context_attributes)
            {
                Ok(d) => d,
                Err(glutin_err) => match gl_config
                    .display()
                    .create_context(&gl_config, &fallback_context_attributes)
                {
                    Ok(d) => d,
                    Err(gles_error) => {
                        return Err(Box::new(Errors::FailedToCreateContextUsingGlutinAndGles(
                            Box::new(glutin_err),
                            Box::new(gles_error),
                        )))
                    }
                },
            }
        };
        let (width, height): (u32, u32) = window.inner_size().into();
        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_window_handle,
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );
        let gl_surface = unsafe {
            match gl_config
                .display()
                .create_window_surface(&gl_config, &attrs)
            {
                Ok(d) => d,
                Err(err) => return Err(Box::new(Errors::FailedToCreateGlSurface(Box::new(err)))),
            }
        };
        let gl_context = match not_current_gl_context.make_current(&gl_surface) {
            Ok(d) => d,
            Err(err) => {
                return Err(Box::new(Errors::FailedToCreateGlContextForSurface(
                    Box::new(err),
                )))
            }
        };
        gl::load_with(|s| {
            gl_config
                .display()
                .get_proc_address(CString::new(s).unwrap().as_c_str())
        });
        let interface = match sb::gpu::gl::Interface::new_load_with_cstr(|s| {
            gl_config.display().get_proc_address(s)
        }) {
            None => return Err(Box::new(Errors::FailedToCreateInterface)),
            Some(d) => d,
        };
        let mut gr_context = match sb::gpu::direct_contexts::make_gl(interface, None) {
            None => return Err(Box::new(Errors::FailedToCreateWindow)),
            Some(d) => d,
        };

        let fb_info = {
            let mut fboid: ::gl::types::GLint = 0;
            unsafe { ::gl::GetIntegerv(::gl::FRAMEBUFFER_BINDING, &mut fboid) };

            FramebufferInfo {
                fboid: fboid.try_into().unwrap(),
                format: skia_safe::gpu::gl::Format::RGBA8.into(),
                ..Default::default()
            }
        };
        let num_samples = gl_config.num_samples() as usize;
        let stencil_size = gl_config.stencil_size() as usize;
        let size = window.inner_size();
        let size = (
            size.width.try_into().expect("Could not convert width"),
            size.height.try_into().expect("Could not convert height"),
        );
        let backend_render_target =
            backend_render_targets::make_gl(size, num_samples, stencil_size, fb_info);

        let surface = match gpu::surfaces::wrap_backend_render_target(
            &mut gr_context,
            &backend_render_target,
            SurfaceOrigin::BottomLeft,
            ColorType::RGBA8888,
            None,
            None,
        ) {
            None => return Err(Box::new(Errors::FailedToCreateWindow)),
            Some(d) => d,
        };

        Ok(Box::new(SkiaRenderer {
            surface,
            gl_surface,
            gr_context,
            gl_context,
            window,
        }))
    }

    fn window_id(&self) -> WindowId {
        self.window.id()
    }

    fn start_rendering(&mut self) {
        self.canvas().reset_matrix();
        self.canvas().clear(WHITE);
    }

    fn renderer(&mut self) -> &mut dyn Renderer {
        self
    }

    fn finish_rendering(&mut self) {
        self.gr_context.flush_and_submit();
        self.gl_surface.swap_buffers(&self.gl_context).unwrap();
    }
}

impl SkiaRenderer {
    #[cfg(feature = "skia-gl")]
    pub fn canvas(&mut self) -> &Canvas {
        self.surface.canvas()
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

        Ok(SkiaRenderer { surface })
    }

    #[cfg(disable)]
    pub unsafe fn save(&mut self, path: &Path) {
        use std::{fs, io::Write};
        let image = self.surface.image_snapshot();
        let data = image
            .encode(
                &mut self.surface.direct_context(),
                EncodedImageFormat::PNG,
                None,
            )
            .unwrap();
        fs::create_dir_all(path.parent().unwrap()).expect("failed to create directory");
        assert_eq!(path.extension().unwrap(), "png");

        let mut file = fs::File::create(path).expect("failed to create file");
        file.write_all(data.as_bytes())
            .expect("failed to write to file");
    }
}
