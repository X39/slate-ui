use log::{error, info, log, warn};
use winit::application::ApplicationHandler;
use winit::error::{EventLoopError, OsError};
use winit::event::{DeviceEvent, DeviceId, StartCause, WindowEvent};
use winit::event_loop;
use winit::event_loop::{ActiveEventLoop, EventLoop, EventLoopBuilder};
use winit::window::{Window, WindowAttributes, WindowId};

#[derive(Debug)]
pub enum AppErrors {
    WinitEventLoopError(EventLoopError),
    WinitOsError(OsError),
    MaxWindowCountReached(usize),
    WindowEventIssuedButNoAssociatedWindowCouldBeFound,
}
#[derive(Debug)]
pub enum AppEvents {}

pub struct AppBuilder {}

impl AppBuilder {
    pub fn new() -> Self {
        AppBuilder {}
    }
    pub fn run(self) -> Result<(), AppErrors> {
        let event_loop = match EventLoop::with_user_event().build() {
            Ok(d) => d,
            Err(e) => return Err(AppErrors::WinitEventLoopError(e)),
        };
        let mut app = App::new();
        match event_loop.run_app(&mut app) {
            Ok(_) => match app.last_error {
                None => Ok(()),
                Some(e) => Err(e),
            },
            Err(e) => Err(AppErrors::WinitEventLoopError(e)),
        }
    }
}

pub struct AppWindowId {
    index: usize,
    pub generation: usize,
}
pub struct App {
    windows: Vec<(Option<WindowId>, Option<Window>, usize)>,
    last_error: Option<AppErrors>,
}

impl App {
    pub(crate) fn count_active_windows(&self) -> usize {
        let mut count: usize = 0;
        for window in &self.windows {
            if window.0.is_some() {
                count = count + 1;
            }
        }
        return count;
    }
}

enum AppFindWindowResult {
    Found(AppWindowId),
    FoundButClosing(AppWindowId),
    NotFound,
}

impl App {
    fn find_window(&self, window_id: WindowId) -> AppFindWindowResult {
        for (index, tuple) in self.windows.iter().enumerate() {
            match tuple.0 {
                None => {}
                Some(tuple_window_id) => {
                    if tuple_window_id == window_id {
                        return match tuple.1 {
                            None => AppFindWindowResult::FoundButClosing(AppWindowId {
                                index,
                                generation: tuple.2,
                            }),
                            Some(_) => AppFindWindowResult::Found(AppWindowId {
                                index,
                                generation: tuple.2,
                            }),
                        };
                    }
                }
            }
        }
        AppFindWindowResult::NotFound
    }
}

impl ApplicationHandler<AppEvents> for App {
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        info!("Application received new events with cause {:?}", cause);
        match cause {
            StartCause::ResumeTimeReached { .. } => {}
            StartCause::WaitCancelled { .. } => {}
            StartCause::Poll => {}
            StartCause::Init => match self.create_window(event_loop) {
                Ok(_) => {}
                Err(e) => {
                    error!(
                        "{:?}, failed to create initial window; exiting event loop",
                        e
                    );
                    self.last_error = Some(e);
                    event_loop.exit();
                }
            },
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        info!("Resume event issued, the application will continue");
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: AppEvents) {
        info!("User event received: event {:?}", event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        info!(
            "Window event received: window {:?}, event {:?}",
            window_id, event
        );

        let app_window_id = match self.find_window(window_id) {
            AppFindWindowResult::Found(d) => d,
            AppFindWindowResult::FoundButClosing(d) => {
                match event {
                    WindowEvent::Destroyed => {
                        self.windows[d.index].0 = None;
                        let active_windows = self.count_active_windows();
                        if (active_windows == 0)
                        {
                            info!("Last window was destroyed, stopping event loop");
                            event_loop.exit();
                        }
                    }
                    _ => {}
                }
                return; // Ignore event
            }
            AppFindWindowResult::NotFound => {
                error!("Window event issued but no associated window found");
                self.last_error =
                    Some(AppErrors::WindowEventIssuedButNoAssociatedWindowCouldBeFound);
                event_loop.exit();
                return;
            }
        };
        match event {
            WindowEvent::ActivationTokenDone { .. } => {}
            WindowEvent::Resized(_) => {}
            WindowEvent::Moved(_) => {}
            WindowEvent::CloseRequested => {
                self.windows[app_window_id.index].1 = None;
            }
            WindowEvent::Destroyed => {}
            WindowEvent::DroppedFile(_) => {}
            WindowEvent::HoveredFile(_) => {}
            WindowEvent::HoveredFileCancelled => {}
            WindowEvent::Focused(_) => {}
            WindowEvent::KeyboardInput { .. } => {}
            WindowEvent::ModifiersChanged(_) => {}
            WindowEvent::Ime(_) => {}
            WindowEvent::CursorLeft { .. } => {}
            WindowEvent::CursorMoved { .. } => {}
            WindowEvent::CursorEntered { .. } => {}
            WindowEvent::MouseWheel { .. } => {}
            WindowEvent::MouseInput { .. } => {}
            WindowEvent::PinchGesture { .. } => {}
            WindowEvent::PanGesture { .. } => {}
            WindowEvent::DoubleTapGesture { .. } => {}
            WindowEvent::RotationGesture { .. } => {}
            WindowEvent::TouchpadPressure { .. } => {}
            WindowEvent::AxisMotion { .. } => {}
            WindowEvent::Touch(_) => {}
            WindowEvent::ScaleFactorChanged { .. } => {}
            WindowEvent::ThemeChanged(_) => {}
            WindowEvent::Occluded(_) => {}
            WindowEvent::RedrawRequested => {}
        }
    }

    fn device_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        device_id: DeviceId,
        event: DeviceEvent,
    ) {
        info!(
            "Device event received: device {:?}, event {:?}",
            device_id, event
        );
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        info!("Application is waiting for new events");
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        info!("Suspend event issued, the application will exit");
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        info!("Shutdown event issued, the application will exit");
    }

    fn memory_warning(&mut self, event_loop: &ActiveEventLoop) {
        warn!("Low memory warning was issued, the Application might crash");
    }
}
impl App {
    fn new() -> Self {
        App {
            windows: vec![],
            last_error: None,
        }
    }

    fn create_window(&mut self, event_loop: &ActiveEventLoop) -> Result<AppWindowId, AppErrors> {
        let attributes = WindowAttributes::default().with_title("slate-ui");
        let window = match event_loop.create_window(attributes) {
            Ok(d) => d,
            Err(e) => return Err(AppErrors::WinitOsError(e)),
        };

        // Try to reuse old slot
        let index = self.windows.len();
        for i in 0..index {
            match self.windows[i].0 {
                None => {
                    let generation = self.windows[i].2 + 1;
                    self.windows[i] = (Some(window.id()), Some(window), generation);
                    return Ok(AppWindowId {
                        index: i,
                        generation,
                    });
                }
                Some(_) => {}
            }
        }

        // Create new slot

        // We must check this as push may panic:
        // "Panics if the new capacity exceeds isize::MAX bytes."
        if self.windows.len() + 1 == isize::MAX as usize {
            return Err(AppErrors::MaxWindowCountReached(isize::MAX as usize));
        }
        self.windows.push((Some(window.id()), Some(window), 1));
        Ok(AppWindowId {
            index,
            generation: 1,
        })
    }
}

struct AppWindow {
    index: usize,
}
