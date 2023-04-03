use gwl::window::{ControlFlow, Window, WindowBuilder, WindowEvent};

fn main() {
    let window = WindowBuilder::new().width(500).height(500).title("English 日本語").build();

    window.show();

    window.run(|event, control_flow| match event {
        WindowEvent::CloseRequested => {
            std::process::exit(0);
        }

        _ => {}
    })
}
