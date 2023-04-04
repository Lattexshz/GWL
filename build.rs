use std::env;

fn main() {
    if env::var("WAYLAND_DISPLAY").is_ok() {
        println!("cargo:rustc-cfg=WAYLAND_ENABLED");
    } else if env::var("DISPLAY").is_ok() {
        println!("cargo:rustc-cfg=X11_ENABLED");
    } else {

    }
}