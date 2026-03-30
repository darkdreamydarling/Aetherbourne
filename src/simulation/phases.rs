/// Defines the ordered phases of a simulation tick.
///
/// Phases enforce deterministic execution order and prevent
/// systems from interfering with each other unpredictably.
///
/// Each phase should operate on a well-defined layer of the sim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Phase {
    // --------------------------------------------------
    // 🌍 World updates (macro scale)
    // --------------------------------------------------

    /// Climate, weather, temperature, global environment changes
    Environment,

    /// Terrain, spatial indexing, chunk updates
    Spatial,

    // --------------------------------------------------
    // 🧬 Entity awareness + thinking
    // --------------------------------------------------

    /// Gather nearby data (vision, sensing, queries)
    Perception,

    /// Decision-making (brains, needs evaluation)
    Cognition,

    // --------------------------------------------------
    // ⚡ Doing things
    // --------------------------------------------------

    /// Movement, eating, attacking, reproduction
    Action,

    /// Entity-to-entity/world interactions
    Interaction,

    // --------------------------------------------------
    // 🧠 State persistence
    // --------------------------------------------------

    /// Memory updates, learning, history recording
    Memory,
}

impl Phase {
    /// Ordered list of all phases in execution order.
    ///
    /// This is the *single source of truth* for your pipeline order.
    pub const ALL: [Phase; 7] = [
        Phase::Environment,
        Phase::Spatial,
        Phase::Perception,
        Phase::Cognition,
        Phase::Action,
        Phase::Interaction,
        Phase::Memory,
    ];

    /// Returns a human-readable name (useful for debug/logging)
    pub fn name(&self) -> &'static str {
        match self {
            Phase::Environment => "Environment",
            Phase::Spatial => "Spatial",
            Phase::Perception => "Perception",
            Phase::Cognition => "Cognition",
            Phase::Action => "Action",
            Phase::Interaction => "Interaction",
            Phase::Memory => "Memory",
        }
    }

    /// Returns true if this phase operates primarily on world-scale data
    pub fn is_world_phase(&self) -> bool {
        matches!(self, Phase::Environment | Phase::Spatial)
    }

    /// Returns true if this phase operates primarily on entities
    pub fn is_entity_phase(&self) -> bool {
        matches!(
            self,
            Phase::Perception
                | Phase::Cognition
                | Phase::Action
                | Phase::Interaction
                | Phase::Memory
        )
    }
}