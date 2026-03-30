// sim context: dt, rng, tick, global refs

pub struct SimContext {
    pub tick: u64,
    pub dt: f32,

    // Core state
    pub ecs: crate::ecs::world::EcsWorld,
    pub world crate::world::world::WorldState,

    // Utilities
    pub rng: rand::rngs::ThreadRng,
}