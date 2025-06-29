use std::{
    sync::{
        Arc,
        mpsc::{Receiver, Sender},
    },
    thread::JoinHandle,
};

use common::traits::Renderer;

use crate::{
    app::{AppEngine, EngineError, GraphicState},
    threading::engine_thread::engine_thread_main,
};

mod engine_thread;

pub fn spawn_engine_thread<
    R: Renderer + Send + Sync + 'static,
    A: AppEngine<GraphicState<R>> + Send + Sync + 'static,
>(
    app_engine: A,
    graphics_context: GraphicState<R>,
) -> (
    Result<JoinHandle<()>, EngineError>,
    Sender<EngineEvent>,
    Receiver<EngineEvent>,
) {
    let (engine_tx, engine_rx) = std::sync::mpsc::channel::<EngineEvent>();
    let (app_tx, app_rx) = std::sync::mpsc::channel::<EngineEvent>();

    let engine_handle = std::thread::Builder::new()
        .name(String::from("Engine_Thread"))
        .spawn(move || {
            let rx = app_rx;
            let tx = engine_tx;

            let wgpu_context = graphics_context;

            let engine = app_engine;

            engine_thread_main(tx, rx, engine, wgpu_context);
        });

    (
        engine_handle.map_err(|e| EngineError::ThreadFailedToSpawn(e.to_string())),
        app_tx,
        engine_rx,
    )
}

pub enum EngineEvent {
    Quit,
    Tick,
    Init,
}
