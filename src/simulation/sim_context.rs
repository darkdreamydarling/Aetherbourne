use rand::rngs::ThreadRng;

use crate::ecs::world::EcsWorld;
use crate::world::world::WorldState;

/// Central simulation context.
/// 
/// - Single source of truth for all simulation state
/// - Passed to every system as `&mut SimContext`
/// - Contains NO simulation logic
pub struct SimContext {
    // --------------------------------------------------
    // ⏱ Time
    // --------------------------------------------------
    pub tick: u64,
    pub delta_time: f32,

    // --------------------------------------------------
    // 🌍 Core State
    // --------------------------------------------------
    pub ecs: EcsWorld,
    pub world: WorldState,

    // --------------------------------------------------
    // 🎲 Utilities
    // --------------------------------------------------
    pub rng: ThreadRng,

    // --------------------------------------------------
    // ⚙️ Simulation Settings / Budgets
    // --------------------------------------------------
    pub budgets: SimBudgets,

    // --------------------------------------------------
    // 🧪 Debug / Control Flags
    // --------------------------------------------------
    pub debug: SimDebug,
}

/// Performance + complexity limits.
/// Prevents runaway simulation cost.
pub struct SimBudgets {
    /// Max interactions processed per tick
    pub max_interactions: usize,

    /// Max entities processed per tick
    pub max_entities: usize,

    /// Perception radius cap (global clamp)
    pub perception_radius: f32,

    /// Max memory entries per entity
    pub memory_capacity: usize,
}

impl Default for SimBudgets {
    fn default() -> Self {
        Self {
            max_interactions: 10_000,
            max_entities: 50_000,
            perception_radius: 50.0,
            memory_capacity: 64,
        }
    }
}

/// Debug + runtime control flags.
/// Safe to mutate during runtime.
pub struct SimDebug {
    /// Pause simulation updates
    pub paused: bool,

    /// Run exactly one tick (when paused)
    pub step: bool,

    /// Enable verbose logging
    pub logging: bool,

    /// Enable decision tracing
    pub trace_decisions: bool,
}

impl Default for SimDebug {
    fn default() -> Self {
        Self {
            paused: false,
            step: false,
            logging: false,
            trace_decisions: false,
        }
    }
}

impl SimContext {
    /// Create a new simulation context
    pub fn new(ecs: EcsWorld, world: WorldState) -> Self {
        Self {
            tick: 0,
            delta_time: 0.0,
            ecs,
            world,
            rng: rand::thread_rng(),
            budgets: SimBudgets::default(),
            debug: SimDebug::default(),
        }
    }

    /// Advance simulation time (no logic, just bookkeeping)
    pub fn advance_time(&mut self, delta_time: f32) {
        self.delta_time = delta_time;
        self.tick += 1;
    }
}