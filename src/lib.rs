use std::env::current_dir;
use std::path::Path;
use relative_path::RelativePath;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Icon},
    platform::{
        windows::WindowBuilderExtWindows,
        
    },
};

pub fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();

    let root = current_dir().unwrap();
    let path = RelativePath::new("resources/sockenginelogo.ico").to_path(&root);
    let icon = load_icon(path.as_path());

    let _window = WindowBuilder::new()
        .with_title("Sock Engine")
        .with_window_icon(Some(icon.clone()))
        .with_taskbar_icon(Some(icon.clone()))
        .build(&event_loop)
        .unwrap();
    
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

    // placeholder cause im too lazy to do error handling currently
    assert_eq!(result.is_err(), false);
}

fn load_icon(path: &Path) -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
