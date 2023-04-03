//! # GWL - Generic Windowing Library
//! GWL focuses on high versatility, one example of which is the ability to execute custom code during window creation to make changes to window handles. Windows can also be created from raw window handles.
//! Of course, it is also possible to use the window as-is without making any changes.
//! See the example for more details.


#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "linux")]
pub use self::linux::*;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "macos")]
pub use self::macos::*;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use self::windows::*;

/// Trait to build a structure to supplement the events that occur during window creation.
pub trait WindowBuildAction {
    /// It is called first when WindowBuilder::build() is executed.
    fn pre_init(&mut self);
    /// Optional: By passing a pre-prepared WindowHandle,
    /// you can replace the window handle stored in the Window structure.
    fn override_window_handle(&mut self) -> Option<WindowHandle> {None}
    /// window is created. This event is ignored when Some is **passed** in ```override_window_handle(&mut self) -> Option<WindowHandle>```
    fn window_created(&mut self, handle: &WindowInstance);
}

/// Default WindowBuildAction
pub struct DefWindowBuildAction;

impl WindowBuildAction for DefWindowBuildAction {
    fn pre_init(&mut self) {}

    fn window_created(&mut self, handle: &WindowInstance) {}
}

pub enum WindowEvent {
    Expose,

    KeyDown(u32),
    KeyUp(u32),

    CloseRequested,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ControlFlow {
    Listen,
    Exit(u32),
}

pub trait IWindow {
    fn new(
        title: String,
        width: u32,
        height: u32,
        x: i32,
        y: i32,
        border_width: u32,
        build_action: Box<dyn WindowBuildAction>,
    ) -> Self;

    fn run<F>(&self, callback: F)
    where
        F: FnMut(WindowEvent, &mut ControlFlow);

    fn get_instance(&self) -> WindowInstance;
}

pub struct Window {
    inner: RawWindow,
}

impl Window {
    pub fn new(raw: RawWindow) -> Self {
        Self { inner: raw }
    }

    pub fn run<F>(&self, callback: F)
    where
        F: FnMut(WindowEvent, &mut ControlFlow),
    {
        self.inner.run(callback);
    }

    pub fn get_instance(&self) -> WindowInstance {
        self.inner.get_instance()
    }
}

unsafe impl HasRawWindowHandle for Window {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.inner.raw_window_handle()
    }
}

pub struct WindowBuilder {
    title: String,
    width: u32,
    height: u32,
    border_width: u32,
    x: i32,
    y: i32,

    build_action: Box<dyn WindowBuildAction>,
}

impl WindowBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_owned();
        self
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn x(mut self, x: i32) -> Self {
        self.y = x;
        self
    }

    pub fn y(mut self, y: i32) -> Self {
        self.y = y;
        self
    }

    pub fn border_width(mut self, width: u32) -> Self {
        self.border_width = width;
        self
    }

    pub fn build_action(mut self, action: Box<dyn WindowBuildAction>) -> Self {
        self.build_action = action;
        self
    }

    pub fn build(mut self) -> Window {
        Window::new(RawWindow::new(
            self.title,
            self.width,
            self.height,
            self.x,
            self.y,
            self.border_width,
            self.build_action,
        ))
    }
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self {
            title: "".to_string(),
            width: 100,
            height: 100,
            border_width: 0,
            x: 0,
            y: 0,
            build_action: Box::new(DefWindowBuildAction),
        }
    }
}
