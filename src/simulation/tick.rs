use crate::simulation::pipeline::run_pipeline;
use crate::simulation::sim_context::SimContext;

/// Advances the simulation by one tick.
///
/// Handles:
/// - pause/step logic
/// - time advancement
/// - pipeline execution
pub fn tick(ctx: &mut SimContext, delta_time: f32) {
    // --------------------------------------------------
    // ⏸ Pause handling
    // --------------------------------------------------
    if ctx.debug.paused && !ctx.debug.step {
        return;
    }

    // --------------------------------------------------
    // ⏱ Advance time
    // --------------------------------------------------
    ctx.advance_time(delta_time);

    // --------------------------------------------------
    // 🔁 Run simulation pipeline
    // --------------------------------------------------
    run_pipeline(ctx);

    // --------------------------------------------------
    // 🪜 Step reset
    // --------------------------------------------------
    if ctx.debug.step {
        ctx.debug.step = false;
    }
}