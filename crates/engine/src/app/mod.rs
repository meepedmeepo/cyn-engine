use std::sync::Arc;

mod app_state;

use winit::{
    application::ApplicationHandler,
    error::EventLoopError,
    event::{KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::PhysicalKey,
    window::{Window, WindowAttributes},
};

pub fn init_windowing() -> Result<(), EventLoopError> {
    env_logger::init();

    let event_loop = EventLoop::with_user_event().build()?;

    let mut app = App::new("Cyn-Engine");

    event_loop.run_app(&mut app)?;
    Ok(())
}

pub struct App {
    window_attributes: WindowAttributes,
    window: Option<Arc<Window>>,
}

impl App {
    pub fn new<S>(title: S) -> Self
    where
        S: Into<String>,
    {
        App {
            window: None,
            window_attributes: Window::default_attributes().with_title(title),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(Arc::new(
            event_loop
                .create_window(self.window_attributes.clone())
                .unwrap(),
        ));
    }

    fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: ()) {}

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {}
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: keystate,
                        ..
                    },
                ..
            } => {}
            _ => {}
        }
    }
}
