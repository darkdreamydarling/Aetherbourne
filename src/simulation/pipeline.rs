use crate::simulation::phases::Phase;
use crate::simulation::sim_context::SimContext;

// Import system entry points (stub as you build them)
use crate::systems::{
    environment,
    geology,
    biology,
    behavior,
    civilization,
    events,
    meta,
};

/// Runs the full simulation pipeline for a single tick.
///
/// This enforces strict phase ordering.
/// Each phase calls its relevant systems.
pub fn run_pipeline(ctx: &mut SimContext) {
    for phase in Phase::ALL {
        if ctx.debug.logging {
            println!("[Tick {}] Phase: {}", ctx.tick, phase.name());
        }

        run_phase(phase, ctx);
    }
}

/// Dispatch systems based on phase
fn run_phase(phase: Phase, ctx: &mut SimContext) {
    match phase {
        // --------------------------------------------------
        // 🌍 World phases
        // --------------------------------------------------
        Phase::Environment => {
            environment::climate::run(ctx);
            environment::weather::run(ctx);
            environment::biome::run(ctx);
        }

        Phase::Spatial => {
            geology::minerals::run(ctx);
            // Later: spatial indexing, chunk updates, etc.
        }

        // --------------------------------------------------
        // 🧬 Entity thinking
        // --------------------------------------------------
        Phase::Perception => {
            behavior::needs::run(ctx);
            // Later: sensing, LOS, spatial queries
        }

        Phase::Cognition => {
            behavior::decision::run(ctx);
            behavior::planner::run(ctx);
        }

        // --------------------------------------------------
        // ⚡ Actions
        // --------------------------------------------------
        Phase::Action => {
            biology::fauna::movement::run(ctx);
            biology::fauna::metabolism::run(ctx);
            biology::fauna::reproduction::run(ctx);

            biology::flora::growth::run(ctx);
        }

        Phase::Interaction => {
            biology::ecosystem::run(ctx);
            civilization::economy::run(ctx);
        }

        // --------------------------------------------------
        // 🧠 Memory / persistence
        // --------------------------------------------------
        Phase::Memory => {
            events::event_queue::run(ctx);
            events::event_chain::run(ctx);

            // future: memory system
        }
    }
}