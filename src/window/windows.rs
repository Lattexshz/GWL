use crate::window::{ControlFlow, IWindow, WindowBuildAction, WindowEvent};
use once_cell::sync::{Lazy, OnceCell};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::cell::RefCell;
use std::ffi::{c_int, c_void, OsStr};
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use winapi::shared::minwindef::{HMODULE, LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::{HBRUSH, HWND, HWND__, POINT};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::wingdi::CreateSolidBrush;
use winapi::um::winuser::*;

pub struct WindowHandle {
    pub hwnd: HWND,
    pub hinstance: HMODULE,
}

pub struct RawWindow {
    hwnd: HWND,
    hinstance: HMODULE,

    msg: i32,
}

impl IWindow for RawWindow {
    fn new(
        title: String,
        width: u32,
        height: u32,
        x: i32,
        y: i32,
        mut build_action: Box<dyn WindowBuildAction>,
    ) -> Self {
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

            let mut msg = 0;

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
                &mut msg as *mut i32 as _,
            );

            let handle = WindowHandle { hwnd, hinstance };

            build_action.window_created(&handle);

            Self {
                hwnd,
                hinstance,
                msg,
            }
        }
    }

    fn run<F>(&self, callback: F)
    where
        F: Fn(WindowEvent, &mut ControlFlow),
    {
        let mut message = unsafe { core::mem::zeroed() };

        let mut control_flow = ControlFlow::Listen;

        unsafe {
            while GetMessageW(&mut message, std::ptr::null_mut(), 0, 0) != 0 {
                match control_flow {
                    ControlFlow::Listen => {
                        DispatchMessageW(&message);
                        let proc_message = MSG.borrow();

                        match proc_message.message {
                            WM_PAINT => {
                                callback(WindowEvent::Expose, &mut control_flow);
                            }

                            WM_DESTROY => {
                                callback(WindowEvent::CloseRequested, &mut control_flow);
                            }

                            _ => {
                                match message.message {
                                    WM_KEYDOWN => {
                                        callback(WindowEvent::KeyDown(message.wParam as u32),&mut control_flow);
                                    }

                                    WM_KEYUP => {
                                        callback(WindowEvent::KeyUp(message.wParam as u32),&mut control_flow);
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    ControlFlow::Exit(code) => {
                        PostQuitMessage(code as c_int);
                    }
                }
            }
        }
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

static mut MSG: RefCell<Lazy<MSG>> = RefCell::new(Lazy::new(|| unsafe { std::mem::zeroed() }));

extern "system" fn wndproc(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    unsafe {
        match Msg {
            WM_PAINT => {
                set_msg(Msg, wParam, lParam);
                0
            }
            WM_DESTROY => {
                set_msg(Msg, wParam, lParam);
                0
            }
            _ => DefWindowProcW(hWnd, Msg, wParam, lParam),
        }
    }
}

unsafe fn set_msg(Msg: UINT, wParam: WPARAM, lParam: LPARAM) {
    MSG.borrow_mut().message = Msg;
    MSG.borrow_mut().wParam = wParam;
    MSG.borrow_mut().lParam = lParam;
}
