use gwl::window::{ControlFlow, Window, WindowBuilder, WindowEvent};

fn main() {
    let window = WindowBuilder::new().build();

    window.run(|event, control_flow| match event {
        WindowEvent::KeyDown(i) => {
            println!("Key downed: {}",i);
        }
        WindowEvent::CloseRequested => {
            std::process::exit(0);
        }

        _ => {}
    })
}
