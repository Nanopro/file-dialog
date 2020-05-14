use winit::{EventsLoop, WindowBuilder, Event, WindowEvent, ControlFlow, };
use open_dialog::{open_dialog, save_dialog};
use winit::os::windows::WindowExt;


fn main() {
    let mut events_loop = EventsLoop::new();
    let window = WindowBuilder::new().build(&events_loop).unwrap();
    let hwnd = window.get_hwnd();

    events_loop.run_forever(|event|{
        match event{
            Event::WindowEvent {
                event: WindowEvent::CloseRequested, ..
            } => {
                ControlFlow::Break
            },
            _ => {
                let file = open_dialog(hwnd);
                if let Some(file) = file{
                    println!("{}", file);
                }

                ControlFlow::Continue
            }
        }
    });
}