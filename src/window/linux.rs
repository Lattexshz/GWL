use std::ffi::{c_int, c_void, OsStr};
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use winapi::shared::minwindef::{HMODULE, LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::HWND;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::*;
use crate::window::{ControlFlow, IWindow, WindowBuildAction, WindowEvent};

pub struct WindowHandle {
    pub hwnd: HWND,
    pub hinstance: HMODULE
}

pub struct RawWindow {
    hwnd: HWND,
    hinstance: HMODULE
}

impl IWindow for RawWindow {
    fn new(title: String,
           width: u32,
           height: u32,
           x: i32,
           y: i32,
           build_action: Box<dyn WindowBuildAction>) -> Self {
        build_action.pre_init();
        let title_wide: Vec<u16> = OsStr::new(&title)
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect();

        unsafe {
            let hinstance = GetModuleHandleW(std::ptr::null());

            let window_class = OsStr::new("window")
                .encode_wide()
                .chain(Some(0).into_iter())
                .collect::<Vec<_>>();

            let wc = WNDCLASSW {
                hCursor: std::ptr::null_mut(),
                hInstance: hinstance,
                lpszClassName: window_class.as_ptr(),
                style: CS_HREDRAW | CS_VREDRAW | CS_OWNDC,
                lpfnWndProc: Some(wndproc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hIcon: std::ptr::null_mut(),
                hbrBackground: std::ptr::null_mut(),
                lpszMenuName: std::ptr::null(),
            };

            RegisterClassW(&wc);

            let hwnd = CreateWindowExW(
                0,
                window_class.as_ptr(),
                title_wide.as_ptr(),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                x,
                y,
                width as c_int,
                height as c_int,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                hinstance,
                std::ptr::null_mut(),
            );

            let handle = WindowHandle {hwnd,hinstance};

            build_action.window_created(&handle);

            Self {
                hwnd,
                hinstance,
            }
        }
    }

    fn run<F>(&self, callback: F) where F: Fn(WindowEvent, &mut ControlFlow) {

    }
}

unsafe impl HasRawWindowHandle for RawWindow {
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut handle = raw_window_handle::Win32WindowHandle::empty();
        handle.hwnd = self.hwnd as *mut c_void;
        handle.hinstance = self.hinstance as *mut c_void;
        RawWindowHandle::Win32(handle)
    }
}
