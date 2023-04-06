use crate::window::{ControlFlow, DefWindowBuildAction, IWindow, WindowBuildAction, WindowEvent};
use once_cell::sync::Lazy;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::cell::RefCell;
use std::ffi::{c_int, c_void, OsStr};
use std::mem::size_of;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use winapi::shared::minwindef::{DWORD, HMODULE, LPARAM, LPCVOID, LRESULT, UINT, WPARAM};
use winapi::shared::windef::HWND;
use winapi::um::dwmapi::{DwmExtendFrameIntoClientArea, DwmSetWindowAttribute};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::uxtheme::MARGINS;
use winapi::um::winuser::*;
use winapi::ENUM;

ENUM! {enum DWMWINDOWATTRIBUTE {
      DWMWA_WINDOW_CORNER_PREFERENCE = 33,
}}

ENUM! {enum DWM_WINDOW_CORNER_PREFERENCE {
        DWMWCP_DEFAULT      = 0,
        DWMWCP_DONOTROUND   = 1,
        DWMWCP_ROUND        = 2,
        DWMWCP_ROUNDSMALL   = 3,
}}

pub struct WindowHandle {
    pub hwnd: HWND,
    pub hinstance: HMODULE,
}

pub struct WindowInstance {
    pub hwnd: HWND,
    pub hinstance: HMODULE,
}

pub struct RawWindow {
    hwnd: HWND,
    hinstance: HMODULE,

    border_width: RefCell<u32>,
}

impl IWindow<'_> for RawWindow {
    fn new(
        title: String,
        width: u32,
        height: u32,
        x: i32,
        y: i32,
        border_width: u32,
        mut build_action: Box<&mut dyn WindowBuildAction>,
    ) -> Self {
        build_action.pre_init();
        let title_wide: Vec<u16> = OsStr::new(&title)
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect();

        match build_action.override_window_handle() {
            None => unsafe {
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
                    WS_OVERLAPPEDWINDOW,
                    x,
                    y,
                    width as c_int,
                    height as c_int,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    hinstance,
                    &mut msg as *mut i32 as _,
                );
                let handle = WindowInstance { hwnd, hinstance };
                build_action.window_created(&handle);
                Self {
                    hwnd,
                    hinstance,
                    border_width: RefCell::new(border_width),
                }
            },
            Some(handle) => Self {
                hwnd: handle.hwnd,
                hinstance: handle.hinstance,
                border_width: RefCell::new(border_width),
            },
        }
    }

    fn run<F>(&self, mut callback: F)
    where
        F: FnMut(WindowEvent, &mut ControlFlow),
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
                            WM_CREATE => {
                                let mut margins: MARGINS = std::mem::zeroed();

                                let border_width = *self.border_width.borrow();

                                margins.cxLeftWidth = border_width as c_int; // 8
                                margins.cxRightWidth = border_width as c_int; // 8
                                margins.cyBottomHeight = border_width as c_int; // 20
                                margins.cyTopHeight = border_width as c_int; // 27

                                DwmExtendFrameIntoClientArea(self.hwnd, &margins);
                            }
                            WM_PAINT => {
                                callback(WindowEvent::Expose, &mut control_flow);
                            }

                            WM_DESTROY => {
                                callback(WindowEvent::CloseRequested, &mut control_flow);
                            }

                            _ => {},
                        }

                        match message.message {
                            WM_KEYDOWN => {
                                callback(
                                    WindowEvent::KeyDown(message.wParam as u32),
                                    &mut control_flow,
                                );
                            }

                            WM_KEYUP => {
                                callback(
                                    WindowEvent::KeyUp(message.wParam as u32),
                                    &mut control_flow,
                                );
                            }
                            _ => {}
                        }
                    }
                    ControlFlow::Exit(code) => {
                        PostQuitMessage(code as c_int);
                    }
                }
            }
        }
    }

    fn get_instance(&self) -> WindowInstance {
        WindowInstance {
            hwnd: self.hwnd,
            hinstance: self.hinstance,
        }
    }

    fn set_window_title(&self, title: &str) {
        unsafe {
            let title_wide: Vec<u16> = OsStr::new(title)
                .encode_wide()
                .chain(Some(0).into_iter())
                .collect();

            SetWindowTextW(self.hwnd, title_wide.as_ptr());
        }
    }

    fn set_window_border_width(&self, border_width: u32) {
        *self.border_width.borrow_mut() = border_width;
    }

    fn set_undecorated(&self, b: bool) {
        match b {
            true => unsafe {
                DwmSetWindowAttribute(
                    self.hwnd,
                    DWMWA_WINDOW_CORNER_PREFERENCE,
                    &DWMWCP_ROUND as *const u32 as *const c_void as LPCVOID,
                    size_of::<u32>() as DWORD,
                );
                SetWindowLongW(
                    self.hwnd,
                    GWL_STYLE,
                    (WS_POPUP | WS_BORDER) as winapi::shared::ntdef::LONG,
                );
                DwmSetWindowAttribute(
                    self.hwnd,
                    DWMWA_WINDOW_CORNER_PREFERENCE,
                    DWMWCP_ROUNDSMALL as LPCVOID,
                    size_of::<u32>() as DWORD,
                );
                SetWindowPos(
                    self.hwnd,
                    null_mut(),
                    0,
                    0,
                    0,
                    0,
                    SWP_DRAWFRAME | SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_NOZORDER,
                );
            },
            false => unsafe {
                SetWindowLongW(
                    self.hwnd,
                    GWL_STYLE,
                    WS_OVERLAPPEDWINDOW as winapi::shared::ntdef::LONG,
                );
            },
        }
    }

    fn show(&self) {
        unsafe {
            ShowWindow(self.hwnd, SW_SHOW);
        }
    }

    fn hide(&self) {
        unsafe {
            ShowWindow(self.hwnd, SW_HIDE);
        }
    }

    fn get_window_pos(&self) -> (u32, u32) {
        unsafe {
            let mut rect = std::mem::zeroed();
            GetWindowRect(self.hwnd, &mut rect);
            (rect.left.try_into().unwrap(), rect.top.try_into().unwrap())
        }
    }

    fn get_window_size(&self) -> (u32, u32) {
        unsafe {
            let mut rect = std::mem::zeroed();
            GetWindowRect(self.hwnd, &mut rect);
            (
                (rect.right-rect.left) as u32,
                (rect.bottom-rect.top) as u32,
            )
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
            WM_CREATE => {
                set_msg(Msg, wParam, lParam);
                0
            }
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
