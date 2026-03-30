use crate::simulation::sim_context::SimContext;
use crate::world::region::Region;

/// Spatial query utilities.
///
/// PURPOSE:
/// - Centralize all spatial lookups
/// - Prevent duplicate logic across systems
/// - Act as the future bridge to spatial indexing (grids, trees, etc.)
///
/// RULES:
/// - Prefer read-only access
/// - No simulation logic
/// - No side effects unless explicitly required
pub struct SpatialQueries;

impl SpatialQueries {
    // --------------------------------------------------
    // 🌍 REGION ACCESS
    // --------------------------------------------------

    /// Get immutable region at grid position
    pub fn region(
        ctx: &SimContext,
        x: i32,
        y: i32,
    ) -> Option<&Region> {
        ctx.world.get_region(x, y)
    }

    /// Get mutable region at grid position
    pub fn region_mut(
        ctx: &mut SimContext,
        x: i32,
        y: i32,
    ) -> Option<&mut Region> {
        ctx.world.get_region_mut(x, y)
    }

    /// Get region using world-space position
    pub fn region_from_world_pos(
        ctx: &SimContext,
        x: f32,
        y: f32,
    ) -> Option<&Region> {
        let (rx, ry) = Self::world_to_region(x, y);
        Self::region(ctx, rx, ry)
    }

    // --------------------------------------------------
    // 🗺 COORDINATE HELPERS
    // --------------------------------------------------

    /// Convert world-space position to region coordinates
    pub fn world_to_region(x: f32, y: f32) -> (i32, i32) {
        (x.floor() as i32, y.floor() as i32)
    }

    /// Clamp coordinates to valid world bounds
    pub fn clamp_to_world(
        ctx: &SimContext,
        x: i32,
        y: i32,
    ) -> (i32, i32) {
        let max_x = ctx.world.width() as i32 - 1;
        let max_y = ctx.world.height() as i32 - 1;

        (x.clamp(0, max_x), y.clamp(0, max_y))
    }

    // --------------------------------------------------
    // 🌱 ENVIRONMENT LOOKUPS
    // --------------------------------------------------

    pub fn biome_at(ctx: &SimContext, x: f32, y: f32) -> Option<&str> {
        Self::region_from_world_pos(ctx, x, y)
            .map(|r| r.biome.as_str())
    }

    pub fn temperature_at(ctx: &SimContext, x: f32, y: f32) -> Option<f32> {
        Self::region_from_world_pos(ctx, x, y)
            .map(|r| r.temperature)
    }

    // --------------------------------------------------
    // 🧬 ENTITY QUERIES (STUBS → replace later with spatial index)
    // --------------------------------------------------

    /// Get nearby entity IDs within a radius
    ///
    /// NOTE:
    /// Currently a stub. Replace with spatial partitioning later.
    pub fn nearby_entities(
        _ctx: &SimContext,
        _x: f32,
        _y: f32,
        _radius: f32,
    ) -> Vec<u32> {
        Vec::new()
    }

    /// Count entities near a position
    pub fn count_nearby_entities(
        ctx: &SimContext,
        x: f32,
        y: f32,
        radius: f32,
    ) -> usize {
        Self::nearby_entities(ctx, x, y, radius).len()
    }

    // --------------------------------------------------
    // 📏 DISTANCE HELPERS
    // --------------------------------------------------

    /// Squared distance (avoid sqrt for performance)
    pub fn distance_squared(
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
    ) -> f32 {
        let dx = x2 - x1;
        let dy = y2 - y1;
        dx * dx + dy * dy
    }

    /// Check if two points are within a radius
    pub fn within_radius(
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        radius: f32,
    ) -> bool {
        Self::distance_squared(x1, y1, x2, y2) <= radius * radius
    }
}