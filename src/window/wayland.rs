use crate::window::{ControlFlow, IWindow, WindowBuildAction, WindowEvent};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use safex::xlib::*;

pub struct WindowHandle {

}

pub struct WindowInstance {

}

pub struct RawWindow {

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
        todo!()
    }

    fn run<F>(&self, mut callback: F)
        where
            F: FnMut(WindowEvent, &mut ControlFlow),
    {

    }

    fn get_instance(&self) -> WindowInstance {
        todo!()
    }

    fn set_window_title(&self, title: &str) {
    }

    fn set_window_border_width(&self, border_width: u32) {}

    fn set_undecorated(&self, b: bool) {

    }

    fn show(&self) {
    }

    fn hide(&self) {
    }

    fn get_window_pos(&self) -> (u32, u32) {
        (0,0)
    }

    fn get_window_size(&self) -> (u32, u32) {
        (0,0)
    }
}

unsafe impl HasRawWindowHandle for RawWindow {
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut handle = raw_window_handle::WaylandWindowHandle::empty();
        RawWindowHandle::Wayland(handle)
    }
}
