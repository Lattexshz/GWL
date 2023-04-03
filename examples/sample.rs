use gwl::window::{ControlFlow, Window, WindowBuilder, WindowEvent};

fn main() {
    let window = WindowBuilder::new().border_width(200).title("English 日本語").build();

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
