use crate::abstraction::Renderer;
use crate::color::Color;
use skia_safe as sb;
use skia_safe::gpu::DirectContext;
use skia_safe::{Canvas, ISize, ImageInfo, Surface};

#[derive(Debug)]
pub enum Errors {
    #[cfg(feature = "skia-gl")]
    FailedToCreateOpenGlInterface,
    #[cfg(feature = "skia-gl")]
    FailedToCreateOpenGlContext,
    #[cfg(feature = "skia-gl")]
    FailedToCreateOpenGlSurface,
    FailedToCreateD3DCommandQueue,
    FailedToCreateD3DDevice,
    FailedToFindSuitableDXDHardwareAdapter,
    FailedToCreateDXGIFactory,
    FailedToCreateD3DBackendContext,
    FailedToCreateD3DSurface,
}

#[cfg(feature = "skia-d3d")]
pub struct SkiaRenderer {
    pub factory: windows::Win32::Graphics::Dxgi::IDXGIFactory4,
    pub backend_context: sb::gpu::d3d::BackendContext,
    pub context: DirectContext,
    pub size: ISize,
    pub image_info: ImageInfo,
    pub surface: Surface,
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
    #[cfg(feature = "skia-d3d")]
    pub fn canvas(&mut self) -> &Canvas {
        self.surface.canvas()
    }
    #[cfg(feature = "skia-d3d")]
    pub fn new<'a>(width: f32, height: f32) -> Result<SkiaRenderer, Errors> {
        use skia_safe::{gpu::{self, d3d, Budgeted, Protected}, ImageInfo};
        use windows::Win32::Graphics::{
            Direct3D::D3D_FEATURE_LEVEL_11_0,
            Direct3D12::{
                D3D12CreateDevice, ID3D12CommandQueue, ID3D12Device, D3D12_COMMAND_LIST_TYPE_DIRECT,
                D3D12_COMMAND_QUEUE_DESC, D3D12_COMMAND_QUEUE_FLAG_NONE,
            },
            Dxgi::{CreateDXGIFactory1, IDXGIFactory4},
        };

        let factory = match unsafe { CreateDXGIFactory1::<IDXGIFactory4>() } {
            Ok(d) => d,
            Err(_) => return Err(Errors::FailedToCreateDXGIFactory),
        };

        let adapter = match Self::get_hardware_adapter(&factory) {
            Ok(d) => d,
            Err(_) => return Err(Errors::FailedToFindSuitableDXDHardwareAdapter),
        };

        let mut device: Option<ID3D12Device> = None;
        match unsafe { D3D12CreateDevice(&adapter, D3D_FEATURE_LEVEL_11_0, &mut device) } {
            Ok(_) => {}
            Err(_) => return Err(Errors::FailedToCreateD3DDevice),
        };
        let device = match device {
            None => return Err(Errors::FailedToCreateD3DDevice),
            Some(d) => d,
        };

        let queue = match unsafe {
            device.CreateCommandQueue::<ID3D12CommandQueue>(&D3D12_COMMAND_QUEUE_DESC {
                Flags: D3D12_COMMAND_QUEUE_FLAG_NONE,
                Type: D3D12_COMMAND_LIST_TYPE_DIRECT,
                ..Default::default()
            })
        } {
            Ok(d) => d,
            Err(_) => return Err(Errors::FailedToCreateD3DCommandQueue),
        };

        let backend_context = d3d::BackendContext {
            adapter,
            device,
            queue,
            memory_allocator: None,
            protected_context: Protected::No,
        };

        let mut context = match unsafe { DirectContext::new_d3d(&backend_context, None) } {
            None => return Err(Errors::FailedToCreateD3DBackendContext),
            Some(d) => d,
        };
        let size = ISize::new(width as i32, height as i32);
        let image_info = ImageInfo::new_n32_premul(size, None);
        let surface = match gpu::surfaces::render_target(
            &mut context,
            Budgeted::Yes,
            &image_info,
            None,
            gpu::SurfaceOrigin::TopLeft,
            None,
            false,
            None,
        ) {
            None => return Err(Errors::FailedToCreateD3DSurface),
            Some(d) => d,
        };
        Ok(SkiaRenderer {
            factory,
            backend_context,
            context,
            size,
            image_info,
            surface,
        })
    }
    #[cfg(feature = "skia-d3d")]
    fn get_hardware_adapter(factory: &windows::Win32::Graphics::Dxgi::IDXGIFactory4) -> windows::core::Result<windows::Win32::Graphics::Dxgi::IDXGIAdapter1> {
        use windows::Win32::Graphics::{
            Direct3D::D3D_FEATURE_LEVEL_11_0,
            Direct3D12::{
                D3D12CreateDevice, ID3D12Device
                ,
            },
            Dxgi::{
                DXGI_ADAPTER_FLAG,
                DXGI_ADAPTER_FLAG_NONE, DXGI_ADAPTER_FLAG_SOFTWARE,
            },
        };
        for i in 0.. {
            let adapter = unsafe { factory.EnumAdapters1(i)? };

            let desc = unsafe { adapter.GetDesc1()? };

            if (DXGI_ADAPTER_FLAG(desc.Flags as i32) & DXGI_ADAPTER_FLAG_SOFTWARE)
                != DXGI_ADAPTER_FLAG_NONE
            {
                // Don't select the Basic Render Driver adapter. If you want a
                // software adapter, pass in "/warp" on the command line.
                continue;
            }

            // Check to see whether the adapter supports Direct3D 12, but don't
            // create the actual device yet.
            if unsafe {
                D3D12CreateDevice(
                    &adapter,
                    D3D_FEATURE_LEVEL_11_0,
                    std::ptr::null_mut::<Option<ID3D12Device>>(),
                )
            }
                .is_ok()
            {
                return Ok(adapter);
            }
        }

        unreachable!()
    }
    #[cfg(feature = "skia-gl")]
    pub fn canvas(&mut self) -> &Canvas {
        self.surface.canvas()
    }
    #[cfg(feature = "skia-gl")]
    pub fn new<'a>(width: f32, height: f32) -> Result<SkiaRenderer, Errors> {
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