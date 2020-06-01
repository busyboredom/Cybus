// Window management with Winit.

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

/// Create a window.
pub fn make_window() {
    // Make an event loop.
    let event_loop = EventLoop::new();

    // Make a window builder.
    let builder = WindowBuilder::new();
    let window = builder.build(&event_loop).unwrap();

    // Run the event loop.
    event_loop.run(move |event, _, control_flow| {
        // Run the control flow continuously
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("Closing");
                *control_flow = ControlFlow::Exit
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {}
            _ => (),
        }
    });
}
