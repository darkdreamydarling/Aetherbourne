# 🌌 Aetherbourne — Unified Simulation Design

A unified simulation framework where **reality emerges from interconnected systems**, not scripts.

**Core Principle:**

```
Reality → Perception → Memory → Belief → Intent → Attention → Decision → Action → Feedback
```

---

# 🎯 Quick Reference

## Base Stats (0–10)

| Stat | Definition |
|------|-----------|
| **Might** | Force output |
| **Lithe** | Agility & efficiency |
| **Vigil** | Awareness & perception |
| **Grit** | Resilience & persistence |
| **Vigor** | Endurance & recovery |

**Derived Formula:** `(A + B) / 2` optional `+ (A * B) * 0.05`

---

## Core Needs (0–100)

| Need | Drain Rate | Purpose |
|------|-----------|---------|
| Hunger | 1.0 | Nutrition |
| Thirst | 1.0 | Hydration |
| Warmth | 1.0 | Temperature regulation |
| Comfort | 0.75 | Safety & wellbeing |
| Health | 0.5 | Vitality |
| Reproduction | — | Breeding |
| Social | 0.5 | Connection |
| Fulfillment | 0.5 | Purpose |
| Waste | 1.0 | Hygiene |

---

# 🔁 Core Simulation Loop

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

## Foundation & Philosophy

* [Philosophy & Mental Model](docs/philosophy.md) — Project principles, simulation layers, core concept

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

# 🧭 Core Principles

## Observer-Centric Simulation
The world is simulated from each entity's subjective perspective, not omnisciently. Entities perceive limited information shaped by their senses and experience.

## Emergence Over Scripting
Behaviors and events arise from systems interacting, not hardcoded scripts. A creature's actions emerge from its personality, memories, needs, and environment.

## Cause → Effect → Feedback Loops
Every action produces consequences that feed back into the system. Reproduction shapes evolution; culture shapes genetics; behavior shapes environment.

---

# 🧠 Core Mental Model

* **Biology defines capability** — Stats limit what entities can do
* **Perception defines reality** — Entities act on what they perceive, not objective truth
* **Needs drive decisions** — Unsatisfied needs create intent and bias attention
* **Memory shapes behavior** — Past experiences consolidate into beliefs
* **Beliefs bias interpretation** — Entities interpret new events through existing beliefs
* **Intent drives action** — Current goals prioritize which actions to take
* **Systems create emergence** — Individual behaviors aggregate into civilization, culture, evolution

---

# 🧩 Simulation Layers

**5-Layer Model:**

1. **Reality Layer** — Objective world state (biomes, weather, resources, terrain)
2. **Biology Layer** — Species characteristics (stats, needs, morphology, traits, DNA)
3. **Mind Layer** — Subjective experience (perception, memory, beliefs, personality)
4. **Intent Layer** — Motivation and desire (needs drive goals, filtered by personality)
5. **Action Layer** — Behavior execution (movement, interaction, communication, crafting)

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
