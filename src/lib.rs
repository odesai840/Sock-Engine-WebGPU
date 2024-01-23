use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let builder = WindowBuilder::new()
        .with_title("Sock Engine");
    let _window = builder.build(&event_loop).unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let result = event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("Close button pressed! Stopping...");
                elwt.exit();
            },
            _ => ()
        }
    });

    assert_eq!(result.is_err(), false);
}
