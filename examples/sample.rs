use GWL::window::{ControlFlow, Window, WindowBuilder, WindowEvent};

fn main() {
    let window = WindowBuilder::new()
        .build();

    window.run(|event,control_flow| {
        match event {

            WindowEvent::KeyUp(code) => {
                if char::from_u32(code).unwrap() == 'E' {
                    *control_flow = ControlFlow::Exit(0);
                }
            }

            _ => {}
        }
    })
}