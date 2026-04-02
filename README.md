# 🌌 Aetherbourne — Unified Simulation Design

A unified simulation framework where **reality emerges from interconnected systems**, not scripts.

## Core Principle:

```
Reality → Perception → Memory → Belief → Intent → Attention → Decision → Action → Feedback
```

---

## Project Philosophy

* **Observer-centric simulation**: The world is simulated from the perspective of observers, not omnisciently.
* **Emergence over scripting**: Behaviors and events arise from systems, not hardcoded scripts.
* **Cause → Effect → Feedback Loops**: Every action produces consequences that feed back into the system.

---

## Core Mental Model

* **Biology defines capability**
* **Perception defines reality**
* **Needs drive decisions**
* **Memory shapes behavior**
* **Beliefs bias interpretation**
* **Intent drives action**
* **Systems create emergence**

---

## Core Simulation Layers

### 1. Reality Layer
The reality layer encompasses seasons, biomes, and the flow of entropy. It includes minerals and flora, as well as fauna and man-made or natural structures. All of these exist within a defined spatial system.

### 2. Biology Layer
The biology layer defines species and their characteristics. It includes base stats and derived stats, tracks needs and energy, and details morphology and traits.

### 3. Mind Layer
The mind layer governs perception, which represents each entity's subjective reality. It stores memory, which decays over time, and maintains beliefs that create persistent biases. Personality is modeled using a waterfall approach, and an attention system regulates focus.

### 4. Intent Layer

```
Intent = f(Needs + Personality + Memory + Beliefs)
```

### 5. Action Layer

* Movement, Combat, Crafting, Interaction, Survival, Communication

---

## 🔁 Core Simulation Loop

1. Environment updates
2. Geology updates
3. Flora grows/decays
4. **Perception updates** (subjective world)
5. **Attention filtering** (top N relevance)
6. **Intent generation** (desire from needs + personality)
7. **Decision scoring** (choose highest-priority action)
8. Actions execute
9. Interactions resolve
10. Memory updates (new experiences recorded)
11. Beliefs update (persistent interpretations shift)
12. Ecosystem balances (populations regulate)
13. Civilization updates (culture/expansion/evolution)
14. Events trigger (notable occurrences broadcast)

---

# 📚 Documentation Map

Organized by relevance and dependency order.

## Foundation (Core Types & Formulas)

* [Data Schema](docs/data-schema.md) — Canonical data structures, all types/components defined
* [Systems Math](docs/systems-math.md) — Centralized formulas, constants, calculations used everywhere

## World & Environment

* [World Systems](docs/world-systems.md) — Biomes, minerals, flora, weather, structures, generation pipeline
* [Spatial System](docs/spatial-system.md) — Spatial partitioning, world layout, coordinate systems
* [Time System](docs/time-system.md) — Tick simulation, update schedules, integration
* [Time Calendar](docs/time-calendar.md) — Seasons, months, calendar system
* [Weather](docs/weather.md) — Weather patterns, climate effects, environmental conditions
* [Entities](docs/entities.md) — Character framework, lifecycle, morphology, relationships
* [Evolution Genetics](docs/evolution-genetics.md) — Environmental pressure, DNA, niche formation, population dynamics

## Behavior & Cognition

* [Behavior Systems](docs/behavior-systems.md) — Perception, memory, belief, intent, attention, decision-making pipeline
* [Affordance System](docs/affordance-system.md) — Detection algorithm, salience scoring, opportunities
* [Affordances](docs/affordances.md) — Interactive possibilities, perception of action opportunities
* [Action System](docs/action-system.md) — Utility scoring, execution, interruptions

## Mechanics & Interactions

* [Items](docs/items.md) — Crafting, inventory, trading, lifecycle, properties
* [Equipment System](docs/equipment-system.md) — Slots, stat modifiers, durability, encumbrance
* [Resource Flow System](docs/resource-flow-system.md) — Scarcity index, dynamic pricing, logistics
* [Interaction Engine](docs/interaction-engine.md) — Priority resolution, outcomes, chains, deception
* [Actions](docs/actions.md) — Action tables, stat→speed/skill→potency mechanics

## Reference & Planning

* [Civilization Culture](docs/civilization-culture.md) — Settlements, roles, culture formation, biology feedback
* [File Structure](docs/file_structure.md) — Project layout, file organization, codebase structure
* [Implementation Summary](docs/IMPLEMENTATION_SUMMARY.md) — Progress overview, phase status
* [Integration Map](docs/integration-map.md) — System dependencies, verification checklists

---

# 🔧 Implementation Phases

## Phase 1: Foundation (Spatial + Survival)
* Spatial system & partitioning
* Basic needs (hunger, energy)
* Perception & memory basics
* Consumption interaction
* Simple decision-making

## Phase 2: Behavior & Agency (Decision Refinement)
* Advanced decision scoring
* Affordances system
* Movement mechanics
* Initial personality integration

## Phase 3: Social & Culture (Group Dynamics)
* Social memory & bonding
* Role emergence
* Settlement formation
* Trade basics
* Civilization scaffold

## Phase 4+: Advanced Systems (Evolution & Emergence)
* Full personality system
* Advanced memory decay
* Evolution & DNA system
* Culture feedback into biology
* Complex civilization dynamics

---

# 🏗️ Technical Architecture

**SimContext** contains:
* ECS (Entity-Component-System) core
* WorldState (biome, resources, entities)
* RNG (deterministic random)
* Time (calendar, seasons, weather)

All systems operate on `&mut SimContext` and are sequentially layered in the simulation loop.

---

# 🤝 Contributing

When adding features:
1. Ensure they fit the information flow (`Reality → Perception → ... → Feedback`)
2. Avoid hardcoded behaviors—use stats and formulas instead
3. Remember observer-centric design (entities perceive imperfectly)
4. Document in appropriate `/docs/` file with examples
5. Update this README's Documentation Map if adding new major systems

---

# 📖 Quick Start

1. **Learn the core loop:** This README's Core Simulation Loop section
2. **Pick a system to explore:** Use the Documentation Map above
3. **Check examples:** Each doc includes worked examples and scenarios


