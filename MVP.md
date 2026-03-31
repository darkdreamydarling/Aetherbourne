# 🌌 Aetherbourne — MVP (Minimum Viable Product)

**Focus:** Get a working simulation running. Phase 1 essentials only.

---

# 🧭 Project Philosophy

* **Observer-centric simulation**: The world is simulated from the perspective of observers, not omnisciently.
* **Emergence over scripting**: Behaviors and events arise from systems, not hardcoded scripts.
* **Everything is systemic, not hardcoded**: All elements interact through systems.
* **Cause → Effect → Feedback Loops**: Every action produces consequences that feed back into the system.

---

# 🧠 Core Mental Model (MVP)

* **Biology defines capability** — stats affect what entities can do
* **Perception defines reality** — entities don't see global truth
* **Needs drive decisions** — hunger/energy are primary drivers
* **Memory shapes behavior** — entities remember successes/failures
* **Intent drives action** — entities pursue goals based on needs

---

# 🧩 MVP Simulation Layers

## 1. Reality Layer (Minimal)

* 2D Spatial grid with regions
* Flora (food sources)
* Time system (basic day/night)

## 2. Biology Layer

* Species definitions (just herbivore vs carnivore)
* 5 base stats (Might, Lithe, Vigil, Grit, Vigor)
* Needs (Hunger, Thirst, Energy, Health)
* Size (affects food requirement)

## 3. Mind Layer (Minimal)

* Perception (what can this entity see/sense)
* Memory (locations of food, dangers)
* Simple decision making (pick best action)

## 4. Action Layer

* Movement
* Consume (eat, drink)
* Rest
* Reproduce

---

# 🔁 MVP Simulation Loop

1. Update time
2. Update environment (simple flora growth)
3. For each entity:
   - Perceive nearby world
   - Update internal state (needs decay)
   - Decide action (best unfulfilled need)
   - Execute action
   - Update memory

---

# 🧬 MVP Entity Framework

**Minimal Entity Structure:**

```
Entity {
  // Static
  species: enum { Herbivore, Carnivore, Omnivore },
  size: f32, // affects food requirement & speed
  
  // Dynamic State
  position: (i32, i32),
  energy: f32, // 0 = death
  hunger: f32, // 0-100
  thirst: f32, // 0-100
  health: f32, // 0-100
  
  // Stats
  might: f32,
  lithe: f32,
  vigil: f32,
  grit: f32,
  vigor: f32,
  
  // Memory
  remembered_food: Vec<(position, time_since_seen)>,
  remembered_danger: Vec<(position, time_since_seen)>,
}
```

---

# 📊 MVP Stat System (Simplified)

## Base Stats (0–10 scale)

* **Might** — movement speed, pushing power
* **Lithe** — dodge chance, energy efficiency  
* **Vigil** — perception range, threat detection
* **Grit** — survival resistance, stress handling
* **Vigor** — max energy, recovery rate

---

## Derived Stats (Super Simple)

```
Perception Range = 5 + (Vigil × 2)
Movement Speed = 1 + (Lithe × 0.05)
Energy Recovery = 0.5 + (Vigor × 0.1)
Energy Drain per Action = 1 / (Lithe × 0.5)
```

---

# ⚙️ MVP Needs System

## Core Needs (0–100 urgency scale)

* **Hunger** — decay rate 0.5/tick, death at 100
* **Thirst** — decay rate 0.3/tick, death at 100
* **Energy** — decay 1.0/tick (from movement, thinking), regen from rest, death at 0
* **Health** — decay if injured, restore via rest

---

## Need Priority Scoring

```
Need Score = Urgency × Importance

Decision made on: highest scoring need that entity CAN fulfill
```

---

# 🔋 MVP Energy System

```
Energy(t+1) = Energy(t) − ActionCost + RestRecovery

0 Energy = Death (immediate)
50 Energy = Lethargy (reduced perception, slower movement)
```

---

# 👁️ MVP Perception System

**Each entity tracks:**

* Visible nearby entities (within Perception Range)
* Visible food sources
* Visible threats
* Confidence based on time-since-seen

**Simplification:** Perfect accuracy; enemies are visible as "threat"

---

# 🧠 MVP Memory System

**Individual Memory (Very Simple)**

* Food locations: `(position, time_added, confidence)`
* Danger locations: `(position, type, time_added)`
* Memory decay: confidence -= 5% per tick, forget at 0%

**Capacity:** Max 20 memories (simple limit)

---

# 🎮 MVP Decision System

```
For each possible action:
  score = f(need_urgency, success_likelihood, energy_cost)

Choose action with highest score
If tied, break randomly
If no valid action, wait
```

---

## Priority Order

1. **Survive:** If Health < 30 → Rest
2. **Energy:** If Energy < 20 → Rest
3. **Hunger:** If Hunger > 70 → Find/Eat Food
4. **Thirst:** If Thirst > 70 → Find/Drink Water  
5. **Reproduce:** If Energy > 70 & Health > 70 & nearby mate → Reproduce
6. **Explore:** Random move to reduce boredom

---

# ⚡ MVP Action System (Phase 1)

## Movement

```
Cost: 1 Energy per tile
Success: Always succeeds (move 1 tile in direction)
Affected: Hunger += 0.1, Thirst += 0.05
```

## Consume (Eat/Drink)

```
Cost: 1 Energy
Success: If on food/water tile, consume 1 unit
Result: Hunger -= 30, Thirst -= 20 (for water)
Memory: Remember "food found here"
```

## Rest

```
Cost: 0 Energy
Recovery: Energy += (Vigor × 0.2), Health += (Vigor × 0.1)
Duration: 1 tick
```

## Reproduce

```
Requirement: Nearby compatible species, Energy > 70, Health > 70
Cost: 50 Energy to parent (each)
Result: New entity spawns at parent location
DNA: Offspring stats = average of parents ± small mutation
```

---

# 🗺️ MVP Spatial System

## Tile-Based 2D World

* Grid size: 100x100 tiles initially
* Tile types: Grass, Water, Rock, Tree (food source)
* Flora: Trees have food points (regrow over time)

---

## Perception Model

```
Visible Tile = withinDistance(entity.perceivRange) 
             AND not blockedByTerrain
```

---

# 🧬 MVP Reproduction System

## DNA (Super Minimal)

```
DNA = {
  might: f32,
  lithe: f32,
  vigil: f32,
  grit: f32,
  vigor: f32,
  diet: Herbivore|Carnivore|Omnivore,
}
```

## Inheritance

```
offspring_stat = (parent1_stat + parent2_stat) / 2 
               + random(-0.3, 0.3)  // small mutation
```

---

# 🌱 MVP Ecosystem (Minimal Closure)

## Flora

* Trees spawn randomly, take multiple seasons
* Each tree has "food_points: int" (0-10)
* Consumed by herbivores, regrows slowly (~1 point/tick)
* Spreads seeds: 5% chance new tree per tick

## Herbivores

* Eat trees exclusively
* Optimize for finding food → survival

## Carnivores

* Eat herbivores exclusively
* Must hunt (harder, more risky)
* Larger population pressure

---

# 📋 MVP Minimal Action Table

| Action | Cost | Duration | Effect |
|--------|------|----------|--------|
| Walk | 1 Energy | 1 tick | Move 1 tile, Hunger +0.1 |
| Eat (on food) | 1 Energy | 1 tick | Hunger -30 |
| Drink (on water) | 1 Energy | 1 tick | Thirst -20 |
| Rest | 0 Energy | 1 tick | Energy +vigor×0.2, Health +vigor×0.1 |
| Reproduce | 50 Energy | 1 tick | Create offspring, -50 Energy each parent |
| Wait | 0 Energy | 1 tick | Nothing happens |

---

# 🏗️ Architecture (MVP)

```
SimContext {
  world: Vec<Vec<Tile>>,
  entities: Vec<Entity>,
  time: u32,
  rng: StdRng,
}

System traits:
  - PerceptionSystem
  - NeedsSystem
  - DecisionSystem
  - ActionSystem
  - MemorySystem
```

---

# 🎯 MVP Success Criteria

**A simulation is MVP-ready when:**

1. ✅ Entities spawn and exist in the world
2. ✅ Energy system works (entities die at 0 Energy)
3. ✅ Needs decay and push decision-making
4. ✅ Entities move toward food/water autonomously
5. ✅ Memory system works (entities remember food sources)
6. ✅ Reproduction creates new entities with inherited stats
7. ✅ Observation shows basic ecosystem behavior:
   - Herbivores gather around food
   - Population fluctuates
   - Memory-based foraging improves over time
   - Successful entities reproduce

---

# 🗓️ Implementation Order

1. **World & Spatial** — Create grid, add flora
2. **Entities & Stats** — Define entity struct, spawn a few
3. **Needs System** — Energy, hunger, thirst decay
4. **Perception** — "What can this entity see?"
5. **Memory System** — Remember food locations
6. **Decision System** — Pick best action based on needs
7. **Movement & Consume** — Execute the chosen actions
8. **Reproduction** — Create offspring with inherited traits
9. **Observation Tools** — Visualize/log what's happening
10. **Tuning** — Balance energy costs, decay rates, reproduction

---

# 🏁 What's NOT in MVP

* Complex crafting/items
* Combat system
* Advanced personality
* Weather effects
* Civilization/culture
* Magic systems
* Advanced evolution
* Social memory sharing
* Complex perception (perfect info)
* Belief systems
* Role specialization

**These are Phase 2+ features.**

---

# 🚀 From MVP to Phase 2

Once MVP is stable:

1. Add affordances (trees can be "climbed" or "harvested")
2. Add basic combat (carnivores hunt herbivores)
3. Refine decision system (add planning/intent)
4. Add perception inaccuracy (limited sensing)
5. Add basic belief formation
6. Introduce social memory (sharing info)

---

# 🏁 Core Principle

```
Minimal World + Simple Systems + Feedback Loops = Emergence
```

No hardcoding behavior.  
No scripted events.  
Just let stats, needs, and memory create the dance.
