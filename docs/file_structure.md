# Project File & Folder Structure

```
aetherbourne/
├── Cargo.toml
├── README.md
├── README_old.md
├── OVERVIEW.md
├── notes.md
├── file_struct_creation.py
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
│   ├── actions.md
│   ├── affordance-system.md
│   ├── affordances.md
│   ├── behavior-systems.md
│   ├── civilization-culture.md
│   ├── data-schema.md
│   ├── entities.md
│   ├── evolution-genetics.md
│   ├── file_structure.md  # <--- this file
│   ├── IMPLEMENTATION_SUMMARY.md
│   ├── interaction-engine.md
│   ├── items.md
│   ├── philosophy.md
│   ├── spatial-system.md
│   ├── systems-math.md
│   ├── time-calendar.md
│   ├── time-system.md
│   ├── weather.md
│   └── world-systems.md
└── src/
   ├── main.rs
   ├── app/
   │   ├── mod.rs
   │   ├── app.rs
   │   ├── input.rs
   │   ├── camera.rs
   │   ├── time.rs
   │   └── selection.rs
   ├── simulation/
   │   ├── mod.rs
   │   ├── tick.rs
   │   ├── pipeline.rs
   │   ├── phases.rs
   │   ├── sim_context.rs
   │   ├── access.rs
   │   ├── snapshot.rs
   │   └── debug.rs
   ├── world/
   │   ├── mod.rs
   │   ├── world.rs
   │   ├── region.rs
   │   ├── chunk.rs
   │   ├── spatial_index.rs
   │   ├── memory.rs
   │   └── queries.rs
   ├── ecs/
   │   ├── mod.rs
   │   ├── world.rs
   │   ├── components/
   │   │   ├── mod.rs
   │   │   ├── transform.rs
   │   │   ├── velocity.rs
   │   │   ├── energy.rs
   │   │   ├── metabolism.rs
   │   │   ├── health.rs
   │   │   ├── species.rs
   │   │   ├── reproduction.rs
   │   │   ├── needs.rs
   │   │   ├── memory.rs
   │   │   ├── perception.rs
   │   │   └── brain.rs
   │   └──  systems/
   │       ├── mod.rs
   │       ├── spatial/
   │       │   ├── mod.rs
   │       │   ├── partition.rs
   │       │   ├── queries.rs
   │       │   └── pathfinding.rs
   │       ├── environment/
   │       │   ├── mod.rs
   │       │   ├── climate.rs
   │       │   ├── weather.rs
   │       │   └── biome.rs
   │       ├── geology/
   │       │   ├── mod.rs
   │       │   └── minerals.rs
   │       ├── biology/
   │       │   ├── mod.rs
   │       │   ├── flora/
   │       │   │   ├── decay.rs
   │       │   │   ├── growth.rs
   │       │   │   ├── mod.rs
   │       │   │   ├── spread.rs
   │       │   ├── fauna/
   │       │   │   ├── combat.rs
   │       │   │   ├── consumption.rs
   │       │   │   ├── interaction.rs
   │       │   │   ├── metabolism.rs
   │       │   │   ├── mod.rs
   │       │   │   ├── movement.rs
   │       │   │   ├── reproduction.rs
   │       │   ├── ecosystem.rs
   │       │   └── evolution.rs
   │       ├── cognition/
   │       │   ├── mod.rs
   │       │   ├── perception.rs
   │       │   ├── memory.rs
   │       │   ├── needs.rs
   │       │   ├── decision.rs
   │       │   └── planning.rs
   │       ├── action/
   │       │   ├── mod.rs
   │       │   ├── movement.rs
   │       │   ├── consumption.rs
   │       │   ├── interaction.rs
   │       │   └── combat.rs
   │       ├── interaction/
   │       │   ├── mod.rs
   │       │   ├── affordances/
   │       │   ├── rules/
   │       │   └── resolution/
   │       ├── civilization/
   │       │   ├── mod.rs
   │       │   ├── settlement.rs
   │       │   ├── economy.rs
   │       │   ├── expansion.rs
   │       │   ├── ai/
   │       │   └── logistics/
   │       ├── events/
   │       │   ├── mod.rs
   │       │   ├── event.rs
   │       │   ├── queue.rs
   │       │   ├── dispatch.rs
   │       │   └── handlers/
   │       │       ├── mod.rs
   │       │       ├── environment.rs
   │       │       ├── biology.rs
   │       │       └── civilization.rs
   │       └── meta/
   │           ├── mod.rs
   │           ├── danger.rs
   │           └── resources.rs
   ├── math.rs
   ├── generation/
   │   ├── mod.rs
   │   ├── noise.rs
   │   ├── world_gen.rs
   │   ├── biome_gen.rs
   │   ├── mineral_gen.rs
   │   ├── flora_gen.rs
   │   └── fauna_gen.rs
   ├── data/
   │   ├── mod.rs
   │   ├── loaders.rs
   │   └── registry.rs
   └── rendering/
       ├── mod.rs
       ├── renderer.rs
       ├── camera_view.rs
       ├── snapshot.rs
       ├── interaction/
       │   ├── mod.rs
       │   ├── rules/
       │   ├── resolution/
       │   └── affordances/
       ├── civilization/
       │   ├── mod.rs
       │   ├── settlement.rs
       │   ├── economy.rs
       │   ├── expansion.rs
       │   ├── ai/
       │   └── logistics/
       ├── events/
       │   ├── mod.rs
       │   ├── event.rs
       │   ├── queue.rs
       │   ├── dispatch.rs
       │   └── handlers/
       │       ├── mod.rs
       │       ├── environment.rs
       │       ├── biology.rs
       │       └── civilization.rs
       ├── math.rs
       ├── color.rs
       └── timing.rs
```
