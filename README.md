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

## World & Environment

* [World Systems](docs/world-systems.md) — Biomes, minerals, flora, weather, structures, generation pipeline
* [Spatial System](docs/spatial-system.md) — Spatial partitioning, world layout, coordinate systems
* [Time & Calendar](docs/time-calendar.md) — Time simulation, seasons, calendar system
* [Weather](docs/weather.md) — Weather patterns, climate effects, environmental conditions
* [Entities & Biology](docs/entities.md) — Character framework, lifecycle, morphology, relationships
* [Evolution & Genetics](docs/evolution-genetics.md) — Environmental pressure, DNA, niche formation, population dynamics

## Behavior & Cognition

* [Behavior Systems](docs/behavior-systems.md) — Perception, memory, belief, intent, attention, decision-making pipeline
* [Affordances](docs/affordances.md) — Interactive possibilities, perception of opportunities, action opportunities

## Civilization & Culture

* [Civilization & Culture](docs/civilization-culture.md) — Settlements, roles, culture formation, feedback into biology

## Mechanics & Reference

* [Items System](docs/items.md) — Crafting, inventory, trading, lifecycle, properties
* [Actions Reference](docs/actions.md) — Action tables, stat→speed, skill→potency formulas, mechanics
* [Interaction Engine](docs/interaction-engine.md) — Interaction systems, resolution, mechanics
* [File Structure](docs/file_structure.md) — Project layout, file organization, codebase structure

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

1. **Understand the philosophy:** Read [Philosophy & Mental Model](docs/philosophy.md)
2. **Learn the core loop:** This README's Core Simulation Loop section
3. **Pick a system to explore:** Use the Documentation Map above
4. **Check examples:** Each doc includes worked examples and scenarios
