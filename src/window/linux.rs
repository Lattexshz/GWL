use crate::window::{ControlFlow, IWindow, WindowBuildAction, WindowEvent};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use safex::xlib::*;

pub struct WindowHandle<'a> {
    pub window: &'a Window,
}

pub struct RawWindow {
    window: Window,
    display: Display,
    cmap: ColorMap
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

        let handle = WindowHandle { window: &window };

        build_action.window_created(&handle);

        Self { window,display,cmap }
    }

    fn run<F>(&self, callback: F)
        where
            F: Fn(WindowEvent, &mut ControlFlow),
    {
        let mut control_flow = ControlFlow::Listen;

        self.window.map();

        self.window.run(|event,flow| match control_flow {
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
}

unsafe impl HasRawWindowHandle for RawWindow {
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut handle = raw_window_handle::XlibWindowHandle::empty();
        handle.window = self.window.as_raw();
        RawWindowHandle::Xlib(handle)
    }
}
