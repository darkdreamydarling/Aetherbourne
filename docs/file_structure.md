# Project File & Folder Structure

```
aetherbourne/
├── Cargo.toml
├── README.md
├── assets/
│   ├── data/
│   │   ├── biomes/
│   │   ├── flora/
│   │   ├── fauna/
│   │   ├── minerals/
│   │   ├── structures/
│   │   ├── items/
│   │   ├── events/
│   │   └── culture/
│   └── graphics/
│       ├── palettes/
│       ├── templates/
│       └── generated/
├── docs/
│   ├── architecture.md
│   ├── pipeline.md
│   ├── systems.md
│   └── file_structure.md  # <--- this file
├── src/
│   ├── main.rs
│   ├── app/
│   │   ├── mod.rs
│   │   ├── app.rs
│   │   ├── input.rs
│   │   ├── camera.rs
│   │   ├── time.rs
│   │   └── selection.rs
│   ├── simulation/
│   │   ├── mod.rs
│   │   ├── tick.rs
│   │   ├── pipeline.rs
│   │   ├── phases.rs
│   │   ├── sim_context.rs
│   │   ├── access.rs
│   │   ├── snapshot.rs
│   │   └── debug.rs
│   ├── world/
│   │   ├── mod.rs
│   │   ├── world.rs
│   │   ├── region.rs
│   │   ├── chunk.rs
│   │   ├── spatial_index.rs
│   │   ├── memory.rs
│   │   └── queries.rs
│   ├── ecs/
│   │   ├── mod.rs
│   │   ├── world.rs
│   │   ├── components/
│   │   │   ├── mod.rs
│   │   │   ├── transform.rs
│   │   │   ├── velocity.rs
│   │   │   ├── energy.rs
│   │   │   ├── metabolism.rs
│   │   │   ├── health.rs
│   │   │   ├── species.rs
│   │   │   ├── reproduction.rs
│   │   │   ├── needs.rs
│   │   │   ├── memory.rs
│   │   │   ├── perception.rs
│   │   │   └── brain.rs
│   ├── systems/
│   │   ├── mod.rs
│   │   ├── spatial/
│   │   │   ├── mod.rs
│   │   │   ├── partition.rs
│   │   │   ├── queries.rs
│   │   │   └── pathfinding.rs
│   │   ├── environment/
│   │   │   ├── mod.rs
│   │   │   ├── climate.rs
│   │   │   ├── weather.rs
│   │   │   └── biome.rs
│   │   ├── geology/
│   │   │   ├── mod.rs
│   │   │   └── minerals.rs
│   │   ├── biology/
│   │   │   ├── mod.rs
│   │   │   ├── flora.rs
│   │   │   ├── fauna.rs
│   │   │   ├── ecosystem.rs
│   │   │   └── evolution.rs
│   │   ├── cognition/
│   │   │   ├── mod.rs
│   │   │   ├── perception.rs
│   │   │   ├── memory.rs
│   │   │   ├── needs.rs
│   │   │   ├── decision.rs
│   │   │   └── planning.rs
│   │   ├── action/
│   │   │   ├── mod.rs
│   │   │   ├── movement.rs
│   │   │   ├── consumption.rs
│   │   │   ├── interaction.rs
│   │   │   └── combat.rs
│   │   ├── interaction/
│   │   │   ├── mod.rs
│   │   │   ├── affordances.rs
│   │   │   ├── rules.rs
│   │   │   └── resolver.rs
│   │   ├── civilization/
│   │   │   ├── mod.rs
│   │   │   ├── settlement.rs
│   │   │   ├── economy.rs
│   │   │   └── expansion.rs
│   │   ├── events/
│   │   │   ├── mod.rs
│   │   │   ├── event.rs
│   │   │   └── queue.rs
│   │   └── meta/
│   │       ├── mod.rs
│   │       ├── danger.rs
│   │       └── resources.rs
│   ├── generation/
│   │   ├── mod.rs
│   │   ├── noise.rs
│   │   ├── world_gen.rs
│   │   ├── biome_gen.rs
│   │   ├── mineral_gen.rs
│   │   ├── flora_gen.rs
│   │   └── fauna_gen.rs
│   ├── data/
│   │   ├── mod.rs
│   │   ├── loaders.rs
│   │   └── registry.rs
│   ├── rendering/
│   │   ├── mod.rs
│   │   ├── renderer.rs
│   │   ├── camera_view.rs
│   │   ├── snapshot.rs
│   │   ├── terrain/
│   │   ├── sprites/
│   │   └── overlays/
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── ui.rs
│   │   ├── panel.rs
│   │   ├── text.rs
│   │   ├── inspector/
│   │   └── overlays/
│   ├── debug/
│   │   ├── mod.rs
│   │   ├── logging.rs
│   │   ├── metrics.rs
│   │   ├── tracing.rs
│   │   └── debug_draw.rs
│   └── utils/
│       ├── mod.rs
│       ├── random.rs
│       ├── math.rs
│       ├── color.rs
│       └── timing.rs
```
