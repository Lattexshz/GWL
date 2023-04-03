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

pub trait WindowBuildAction {
    fn pre_init(&mut self);
    fn window_created(&mut self, handle: &WindowHandle);
}

pub struct DefWindowBuildAction;

impl WindowBuildAction for DefWindowBuildAction {
    fn pre_init(&mut self) {}

    fn window_created(&mut self, handle: &WindowHandle) {}
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
        build_action: Box<dyn WindowBuildAction>,
    ) -> Self;

    fn run<F>(&self, callback: F)
    where
        F: Fn(WindowEvent, &mut ControlFlow);
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
        F: Fn(WindowEvent, &mut ControlFlow),
    {
        self.inner.run(callback);
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
            x: 0,
            y: 0,
            build_action: Box::new(DefWindowBuildAction),
        }
    }
}
