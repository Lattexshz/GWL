pub mod window;


pub enum Platform {
    Windows,
    Macos,
    XLib,
    Wayland
}

#[cfg(target_os = "windows")]
pub fn get_platform() -> Platform {
    Platform::Windows
}

#[cfg(target_os = "macos")]
pub fn get_platform() -> Platform {
    Platform::Macos
}

#[cfg(linux)]
#[cfg(X11_ENABLED)]
pub fn get_platform() -> Platform {
    Platform::XLib
}

#[cfg(linux)]
#[cfg(WAYLAND_ENABLED)]
pub fn get_platform() -> Platform {
    Platform::Wayland
}