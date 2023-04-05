use gwl::window::{DefWindowBuildAction, WindowBuilder, WindowEvent};

fn main() {
    let mut action = DefWindowBuildAction;
    let window = WindowBuilder::new(Box::new(&mut action))
        .width(500)
        .height(500)
        .title("English 日本語")
        .build();

    window.show();


    window.run(|event, _control_flow| match event {
        WindowEvent::CloseRequested => {
            std::process::exit(0);
        }
        _ => {}
    })
}
