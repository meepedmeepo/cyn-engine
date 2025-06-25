pub struct DataPipeline {
    world: bevy_ecs::world::World,
    schedule: bevy_ecs::schedule::Schedule,
}

impl DataPipeline {
    pub fn new() -> Self {
        let world = bevy_ecs::world::World::new();
        let schedule = bevy_ecs::schedule::Schedule::default();

        DataPipeline { world, schedule }
    }
}
