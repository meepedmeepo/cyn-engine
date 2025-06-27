use std::sync::{
    Arc,
    mpsc::{Receiver, Sender},
};

use common::traits::Renderer;

use crate::app::{AppEngine, GraphicState};

pub fn spawn_engine_thread<
    R: Renderer + Send + Sync + 'static,
    A: AppEngine<GraphicState<R>> + Send + Sync + 'static,
>(
    app_engine: A,
    graphics_context: GraphicState<R>,
) -> (Sender<EngineEvent>, Receiver<EngineEvent>) {
    let (engine_tx, engine_rx) = std::sync::mpsc::channel::<EngineEvent>();
    let (app_tx, app_rx) = std::sync::mpsc::channel::<EngineEvent>();

    let engine_handle = std::thread::Builder::new()
        .name(String::from("Engine_Thread"))
        .spawn(move || {
            let rx = app_rx;
            let tx = engine_tx;

            let wgpu_context = graphics_context;

            let mut engine = app_engine;

            while let Ok(app_msg) = rx.recv() {
                engine.tick(&wgpu_context);
            }
        });

    (app_tx, engine_rx)
}

pub enum EngineEvent {
    Quit,
    Tick,
}
