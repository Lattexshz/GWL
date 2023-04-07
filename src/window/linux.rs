use crate::window::{ControlFlow, IWindow, WindowBuildAction, WindowEvent};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use safex::xlib::*;

pub struct WindowHandle {
    pub window: Window,
    pub display: Display,
}

pub struct WindowInstance<'a> {
    pub window: &'a Window,
    pub display: &'a Display,
}

pub struct RawWindow {
    window: Window,
    display: Display,
    cmap: ColorMap,
}

impl IWindow for RawWindow {
    fn new(
        title: String,
        width: u32,
        height: u32,
        x: i32,
        y: i32,
        border_width: u32,
        mut build_action: Box<dyn WindowBuildAction>,
    ) -> Self {
        build_action.pre_init();

        match build_action.override_window_handle() {
            None => {
                let display = Display::open(None);
                let screen = Screen::default(&display);
                let root = Window::root_window(&display, &screen);

                let cmap = ColorMap::default(&display, &screen);

                let white = Color::from_rgb(&display, &cmap, 65535, 65535, 65535).get_pixel();

                let window = Window::create_simple(
                    &display,
                    &screen,
                    Some(()),
                    Some(root),
                    y,
                    x,
                    width,
                    height,
                    border_width,
                    0,
                    white,
                );

                window.set_window_title(&title);

                let handle = WindowInstance {
                    window: &window,
                    display: &display,
                };

                build_action.window_created(&handle);

                Self {
                    window,
                    display,
                    cmap,
                }
            }

            Some(handle) => {
                let screen = Screen::default(&handle.display);
                let cmap = ColorMap::default(&handle.display, &screen);
                Self {
                    window: handle.window,
                    display: handle.display,
                    cmap,
                }
            }
        }
    }

    fn run<F>(&self, mut callback: F)
    where
        F: FnMut(WindowEvent, &mut ControlFlow),
    {
        let mut control_flow = ControlFlow::Listen;

        self.window.map();

        self.window.run(|event, flow| match control_flow {
            ControlFlow::Listen => match event {
                safex::xlib::WindowEvent::Expose => {
                    callback(WindowEvent::Expose, &mut control_flow);
                }
            },
            ControlFlow::Exit(code) => {
                std::process::exit(code as i32);
            }
        })
    }

    fn get_instance(&self) -> WindowInstance {
        WindowInstance {
            window: &self.window,
            display: &self.display,
        }
    }

    fn set_window_title(&self, title: &str) {
        self.window.set_window_title(title);
    }

    fn set_window_border_width(&self, border_width: u32) {}

    fn set_undecorated(&self, b: bool) {
        match b {
            true => {}
            false => {}
        }
    }

    fn set_minimized(&self, b: bool) {
        match b {
            true => {
                self.window.unmap();
            }
            false => {
                self.window.map();
            }
        }
    }

    fn set_maximized(&self, b: bool) {
        match b {
            true => {
                self.window.unmap();
            }
            false => {
                self.window.map();
            }
        }
    }

    fn show(&self) {
        self.window.map();
    }

    fn hide(&self) {
        self.window.unmap();
    }

    fn get_window_pos(&self) -> (u32, u32) {
        let geometry = self.window.get_geometry();
        (geometry.x, geometry.y)
    }

    fn get_window_size(&self) -> (u32, u32) {
        let geometry = self.window.get_geometry();
        (geometry.width, geometry.height)
    }
}

unsafe impl HasRawWindowHandle for RawWindow {
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut handle = raw_window_handle::XlibWindowHandle::empty();
        handle.window = self.window.as_raw();
        RawWindowHandle::Xlib(handle)
    }
}
