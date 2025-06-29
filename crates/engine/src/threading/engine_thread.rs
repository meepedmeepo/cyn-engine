use std::sync::mpsc::{Receiver, Sender};

use common::traits::Renderer;

use crate::{
    app::{AppEngine, GraphicState},
    threading::EngineEvent,
};

pub fn engine_thread_main<
    R: Renderer + Send + Sync + 'static,
    A: AppEngine<GraphicState<R>> + Send + Sync + 'static,
>(
    tx: Sender<EngineEvent>,
    rx: Receiver<EngineEvent>,
    engine: A,
    wgpu_context: GraphicState<R>,
) {
    let mut engine = engine;

    while let Ok(app_msg) = rx.recv() {
        match app_msg {
            EngineEvent::Init => engine.init(),
            EngineEvent::Tick => engine.tick(&wgpu_context),
            EngineEvent::Quit => {
                tx.send(EngineEvent::Quit)
                    .expect("Couldn't send QUIT from Engine thread.");
            }
            _ => {}
        }
    }
}
