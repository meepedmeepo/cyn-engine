use std::sync::Arc;

mod app_engine;
pub use app_engine::*;

use common::traits::Renderer;
use winit::{
    application::ApplicationHandler,
    error::EventLoopError,
    event::{KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::PhysicalKey,
    window::{Window, WindowAttributes},
};

pub fn init_windowing<R: Renderer>(
    app_engine: Box<dyn AppEngine<GraphicState<R>>>,
) -> Result<(), EventLoopError> {
    env_logger::init();

    let event_loop = EventLoop::<()>::with_user_event().build()?;
    let mut app = App::new("Cyn-Engine", app_engine);

    event_loop.run_app(&mut app)?;
    Ok(())
}

///Please Reconsider all your life choices in regards to the awful soup of generics and dynamic dispatch.
pub struct App<R: Renderer> {
    window_attributes: WindowAttributes,
    event_loop: EventLoop<()>,
    window: Option<Arc<Window>>,
    app_engine: Box<dyn AppEngine<GraphicState<R>>>,
}

///This feels very wrong and smells atrocious. Please kindly reconsider :)
impl<R> App<R>
where
    R: Renderer,
{
    pub fn new<S>(title: S, app_engine: Box<dyn AppEngine<GraphicState<R>>>) -> Self
    where
        S: Into<String>,
    {
        let event_loop = EventLoop::with_user_event().build().unwrap();
        App {
            event_loop,
            window: None,
            window_attributes: Window::default_attributes().with_title(title),
            app_engine,
        }
    }

    pub fn run(&mut self) {}
}

impl<R: Renderer> ApplicationHandler<()> for App<R> {
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
