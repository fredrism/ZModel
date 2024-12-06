use std::collections::HashMap;
use winit::application::ApplicationHandler;
use winit::dpi;
use winit::dpi::PhysicalPosition;
use winit::error::OsError;
use winit::event::{DeviceId, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

struct ZModeler {
    windows: HashMap<WindowId, Window>
}

impl ZModeler {
    fn new() -> ZModeler {
        ZModeler{windows: HashMap::new()}
    }
}

impl ApplicationHandler for ZModeler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut window_attributes = WindowAttributes::default();
        window_attributes.title = String::from("ZModeler");
        let window = event_loop.create_window(window_attributes);

        match window {
            Ok(window) => {
                self.windows.insert(window.id(), window);
            }
            Err(_) => {}
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        let window = self.windows.get(&window_id);

        match event {
            WindowEvent::ActivationTokenDone { .. } => {}
            WindowEvent::Resized(_) => {}
            WindowEvent::Moved(_) => {}
            WindowEvent::CloseRequested => {
                self.windows.remove(&window_id);
            }
            WindowEvent::Destroyed => {
                if self.windows.is_empty() {
                    event_loop.exit();
                }
            }
            WindowEvent::DroppedFile(_) => {}
            WindowEvent::HoveredFile(_) => {}
            WindowEvent::HoveredFileCancelled => {}
            WindowEvent::Focused(_) => {}
            WindowEvent::KeyboardInput { .. } => {}
            WindowEvent::ModifiersChanged(_) => {}
            WindowEvent::Ime(_) => {}
            WindowEvent::CursorMoved {
                device_id,
                position, } => {
                println!("{}, {}", position.x, position.y);
            }
            WindowEvent::CursorEntered { .. } => {}
            WindowEvent::CursorLeft { .. } => {}
            WindowEvent::MouseWheel { .. } => {}
            WindowEvent::MouseInput { .. } => {}
            WindowEvent::PinchGesture { .. } => {}
            WindowEvent::PanGesture { .. } => {}
            WindowEvent::DoubleTapGesture { .. } => {}
            WindowEvent::RotationGesture { .. } => {}
            WindowEvent::TouchpadPressure { .. } => {}
            WindowEvent::AxisMotion { .. } => {}
            WindowEvent::Touch(_) => {}
            WindowEvent::ScaleFactorChanged { .. } => {}
            WindowEvent::ThemeChanged(_) => {}
            WindowEvent::Occluded(_) => {}
            WindowEvent::RedrawRequested => {}
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = ZModeler::new();
    event_loop.run_app(&mut app).unwrap();
}
