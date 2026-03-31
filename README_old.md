# 🌌 Aetherbourne — Unified Simulation Design

---

# 🧭 Project Philosophy

* **Observer-centric simulation**: The world is simulated from the perspective of observers, not omnisciently.
* **Emergence over scripting**: Behaviors and events arise from systems, not hardcoded scripts.
* **Cause → Effect → Feedback Loops**: Every action produces consequences that feed back into the system.

---

# 🧠 Core Mental Model

* **Biology defines capability**
* **Perception defines reality**
* **Needs drive decisions**
* **Memory shapes behavior**
* **Beliefs bias interpretation**
* **Intent drives action**
* **Systems create emergence**

---

# 🧩 Core Simulation Layers

## 1. Reality Layer
The reality layer encompasses seasons, biomes, and the flow of entropy. It includes minerals and flora, as well as fauna and man-made or natural structures. All of these exist within a defined spatial system.

## 2. Biology Layer
The biology layer defines species and their characteristics. It includes base stats and derived stats, tracks needs and energy, and details morphology and traits.

## 3. Mind Layer
The mind layer governs perception, which represents each entity’s subjective reality. It stores memory, which decays over time, and maintains beliefs that create persistent biases. Personality is modeled using a waterfall approach, and an attention system regulates focus.

## 4. Intent Layer

```
Intent = f(Needs + Personality + Memory + Beliefs)
```

## 5. Action Layer

* Movement, Combat, Crafting, Interaction, Survival, Communication

---

# 🔁 Core Simulation Loop (Unified)

1. Environment updates
2. Geology updates
3. Flora grows/decays
4. **Perception updates (subjective world)**
5. **Attention filtering (top N relevance)**
6. **Intent generation**
7. **Decision scoring (choose action)**
8. Actions execute
9. Interactions resolve
10. Memory updates
11. Beliefs update
12. Ecosystem balances
13. Civilization updates
14. Events trigger

---

# 🧬 Universal Entity Framework

**Entity =**

* Category
* Attributes (static)
* State (dynamic)
* Behaviors (system-driven)

---

# 🧍 Entities: Characters / Fauna / Agents

## Categories

* Species
* Role (dynamic)
* Lifecycle Stage
* Intelligence Tier

---

## Lifecycle Stages

* Infant
* Child
* Juvenile
* Adult
* Elder

Each stage modifies:

* Stats scaling
* Needs intensity
* Behavior
* Capabilities
* Reproduction eligibility

---

## Core Biology

* Size: Tiny → Colossal
* Mass: Light → Heavy
* Lifespan: Short → Long
* Metabolism: Slow → Rapid
* Activity: Nocturnal / Diurnal / Crepuscular / Always Active

---

## Intelligence & Behavior

* Intelligence: Primordial→ Instinctive → Behavioral → Cognitive → Sentient
* Temperament: Docile / Neutral / Aggressive / Territorial / Skittish

---

## Diet

* Herbivore / Carnivore / Omnivore / Scavenger / Specialized

---

## Social Structure

* Solitary / Pair-bonded / Societal / Swarm

---

## Reproduction

* Asexual / Sexual
* Rate: Rare → Frequent
* Offspring Count: Low → High
* Maturity Age: Early → Late

---

## Parental Care

* None / Minimal / Moderate / Intensive

---

## Population Density

* Sparse → Swarming

---

## Territorial Behavior

* Range: Small → Massive
* Migration: None / Seasonal / Reactive / Constant

---

## Defense & Combat

* Skin: Skin / Fur / Scales / Chitin / Shell / Hybrid
* Defense: Armor / Camouflage / Speed / Venom / Regeneration
* Attack: Ambush / Chase / Pack / Opportunistic

---

# 📊 Stat System (Unified)

## Base Stats (0–10, decimals allowed)

* Might — force output
* Lithe — agility & efficiency
* Vigil — awareness & perception
* Grit — resilience & persistence
* Vigor — endurance & recovery

---

## Derived Stat Formula (Global Rule)

```
Derived = (A + B) / 2
Optional: + (A * B) * 0.05
```

---

## Derived Stats (Unified Set)

### 🔴 Physical

* **Prowess** = Might + Lithe
* **Discipline** = Might + Vigil
* **Tenacity** = Might + Grit
* **Fortitude** = Might + Vigor

---

### 🟡 Efficiency

* **Precision** = Lithe + Vigil
* **Finesse** = Lithe + Grit
* **Coordination** = Lithe + Vigor

---

### 🔵 Mental

* **Insight** = Vigil + Grit
* **Attunement** = Vigil + Vigor
* **Resolve** = Grit + Vigor

---

# ⚙️ Needs System

## Core Needs (0–100)

* Hunger
* Thirst
* Warmth
* Comfort / Safety
* Health
* Reproduction
* Social
* Fulfillment
* Waste

Each has:

* decay rate
* urgency curve
* competition logic

---

## Standard Rates

Base need drain/decay per action cycle:

* Hunger Drain Rate = 1.0
* Thirst Drain Rate = 1.0
* Warmth Loss Rate = 1.0
* Health Decline Rate = 0.5
* Comfort Decline Rate = 0.75
* Social Decline Rate = 0.5
* Fulfillment Decline Rate = 0.5
* Waste Accumulation Rate = 1.0

Stats **multiply** these rates based on relevant derived stat.

---

# 🔋 Energy System

```
Energy = Intake − Action Cost
0 = death
```

---

# 📊 Derived Stat Effects

## 🔴 Physical

**Prowess** (Might + Lithe)

```
Action Output = 1 + (Prowess × 0.08)
Energy Cost Multiplier = 1 + (Prowess × 0.05)
```

**Discipline** (Might + Vigil)

```
Action Precision = 1 + (Discipline × 0.06)
Waste Reduction = Discipline × 0.05
```

**Tenacity** (Might + Grit)

```
Comfort Drain Rate = 1 - (Tenacity × 0.06)
Health Protection = Tenacity × 0.05
```

**Fortitude** (Might + Vigor)

```
Health Recovery = Fortitude × 0.07
Warmth Retention = Fortitude × 0.05
```

---

## 🟡 Efficiency

**Precision** (Lithe + Vigil)

```
Action Success Modifier = 1 + (Precision × 0.07)
Error Reduction = Precision × 0.06
```

**Finesse** (Lithe + Grit)

```
Action Stability = 1 + (Finesse × 0.06)
Smooth Execution Bonus = Finesse × 0.05
```

**Coordination** (Lithe + Vigor)

```
Energy Efficiency = 1 - (Coordination × 0.06)
Stamina Consistency = Coordination × 0.05
```

---

## 🔵 Mental

**Insight** (Vigil + Grit)

```
Decision Quality = 1 + (Insight × 0.08)
Fulfillment from Actions = Insight × 0.05
```

**Attunement** (Vigil + Vigor)

```
Perception Range = 1 + (Attunement × 0.07)
Awareness Duration = Attunement × 0.05
```

**Resolve** (Grit + Vigor)

```
Health Recovery Rate = 1 + (Resolve × 0.07)
Social Influence = Resolve × 0.06
Thirst/Hunger Resistance = Resolve × 0.05
```

---

# 🧠 Personality System (Waterfall)

1. Reflex (BIS/BAS)
2. Behavioral (HEXACO)
3. Ethical (Dark Triad)
4. Intentional (Enneagram)
5. Communication (DISC)

---

# 👁️ Perception System

Entities track:

* Visible entities
* Known resources
* Known threats
* Confidence levels

Influenced by:

* Vigil
* Distance
* Obstruction
* Environment
* Memory

---

# 🧠 Memory System

## Individual Memory

* Locations
* Outcomes
* Social interactions
* Decays over time
* Limited capacity

## Shared Memory

* Spreads via interaction
* Enables culture

---

# 🧠 Belief System (NEW CORE)

```
Memory → Belief → Decision Bias
```

Beliefs:

* Persistent
* Inaccurate possible
* Socially influenced

**Examples:**

* "This forest is dangerous"
* "Red berries are safe"
* "That tribe is hostile"

**Effects:**

* Spread socially across populations
* Create superstition, culture, bias
* Lead to collective misinformation
* Influence future decisions directly

---

# ⏰ Weather System

Dynamic weather affects resource availability, character comfort, travel, and world state.

## Weather Conditions

### Clear

* Light Level: 1.0
* Temperature Effect: 0.0
* Visibility: 1.0
* Movement Speed: 1.0
* Thirst Drain Multiplier: 1.0
* Vigil Bonus: 1.1
* Duration: 6–24 hours

### Rainy

* Light Level: 0.5
* Temperature Effect: –3.0 (Warmth)
* Visibility: 0.4
* Movement Speed: 0.9
* Thirst Drain Multiplier: 1.2
* Vigil Penalty: 0.15
* Fire Extinguish Rate: 0.8
* Plant Watering Rate: 0.3
* Soil Erosion Rate: 0.25

### Stormy

* Light Level: 0.3
* Temperature Effect: –4.0 (Warmth)
* Visibility: 0.2
* Movement Speed: 0.8
* Thirst Drain Multiplier: 1.5
* Accuracy Penalty: 0.25
* Lightning Strike Chance: 0.05
* Wind Effect Strength: 1.5

---

# ⏳ Time System

## World Calendar

**Year Structure:**
* Days per Year: 300
* Months per Year: 10

**Week Structure:**
* Days per Week: 8
* Hours per Day: 24
* Minutes per Hour: 60
* Seconds per Minute: 60

**Day Names:** Solis, Lunis, Terris, Aquas, Ignis, Ventis, Umbranis, Luxis

**Months:**

| Month | Season | Length (days) |
|-------|--------|---|
| Brigara | Winter | 28 |
| Imbrelle | Winter | 32 |
| Florayne | Spring | 30 |
| Lithara | Spring | 26 |
Fulthane
| Heliora | Summer | 31 |
| Aestara | Summer | 29 |
| Mavonel | Summer | 33 |
Vivmora
| Ceresen | Autumn | 27 |
| Yulith | Autumn | 34 |
| Duskael | Winter | 30 |



**Total verification:** 28+32+30+26+31+29+33+27+34+30 = 300 days ✓

---

# 👁️ Physical & Morphological Attributes

## Body Structure

**Body Types:** Ecomorph, Mesomorph, Endomorph

**Body Shapes:** Pear, Inverted Triangle, Apple, Rectangle, Hourglass

**Head Shapes:** Oval, Round, Square, Oblong, Heart, Triangle, Diamond, Base-Down Diamond, Pentagon

## Limbs & Features

**Limb Counts:**
* Legs: 0–4
* Arms: 0–2
* Eyes: 1–3
* Tails: 0–1

## Eyes

**Eye Shapes:** Almond, Round, Monolid, Downturned, Upturned, Hooded, Droopy

**Eye Spacing:** Close, Wide, Normal

**Eye Depth:** Deep, Protruding, Normal

**Iris Colors:** Grey, Blue, Green, Hazel, Brown

## Hair & Skin

**Hair Colors (34 variants):** Starless Night, Midnight Obsidian, Raven, Jet Black, Smoldering Coal, Chestnut, Bloodwine, Auburn, Emberglow, Autumn Rust, Phoenix Flame, Copper, Honey Blonde, Golden Wheat, Ash Brown, Stormcloud Grey, Mothwing Grey, Ashen Fog, Silver, Frosted Pearl, Platinum, Moonlit Ash, Ghostlight White

**Hair Texture:** Coily, Curly, Wavy, Straight

**Skin Tone:** Pale, Light, Medium, Dark, Very Dark

**Skin Undertone:** Yellow, Neutral, Pink

## Face Structure

**Face Shapes:** Oval, Round, Square, Oblong, Heart, Triangle, Diamond, Pentagon

**Body Proportions:**
* Upper Body Width: Wide, Medium, Thin
* Mid Body Width: Wide, Medium, Thin
* Lower Body Width: Wide, Medium, Thin

**Height Range:** 640–1920 pixels

## Emotional & Behavioral

**Emotions:** Neutral, Happy, Sad, Surprised, Disgust, Fear, Anger

---

# 🤝 Relationship System

Entities track nuanced relationships across three axes.

## Family Relations

* Range: –100 to 100
* Types: Parent, Child, Sibling, Grandparent, Grandchild, Cousin, Aunt, Uncle, Niece, Nephew

## Friendships

* Range: –100 to 100
* Types: Friend, Acquaintance, Colleague, Neighbor, Enemy, Rival, Best Friend, Close Friend, Traitor, Ally, Foe

## Romantic Relations

* Range: –100 to 100
* Types: Lover, Partner, Crush, Ex-Lover, Ex-Partner, Soulmate

**⚠️ Note:** Each entity maintains perspective-based views: how they view others (Family/Friends/Romance) AND how they view themselves through those same lenses.

---

# 🎯 Intent System

## Formula

```
Intent = Needs × Personality × Memory × Beliefs
```

## Types

* Immediate (eat, flee)
* Persistent (goals, identity-driven)

---

# 👁️ Attention System

```
AttentionScore =
  NeedUrgency
+ Threat × Distance⁻¹
+ Novelty
+ BeliefBias
```

Only top N processed.

---

# 🎮 Decision System

```
DecisionScore =
  Need × Stat × Personality × Memory × Belief × Random
```

Priority order:

1. Needs
2. Threats
3. Intent
4. Social
5. Exploration

---

# ⚡ Action System

* Movement
* Interaction
* Harvesting
* Crafting
* Survival
* Reproduction
* Communication

Skills (0–100):

* movement, crafting, combat, survival, etc.

---

# 🔗 Interaction Engine

```
Source + Target → Rule → Effect
```

Types:

* Consumption
* Combat
* Growth
* Decay
* Transfer
* Extraction
* Social

---

# 🌍 World Systems

## Biomes

Attributes:

* Moisture
* Climate
* Topography
* Stability
* Soil

## Minerals

* Abundance, Value, Purity, Hardness, Reactivity, Formation, Extraction

## Flora

* Reproduction, Utility, Growth, Size, Habitat, Ecology, Yield

## Weather

* Intensity, Duration, Predictability, Temperature

## Structures

* Age, Condition, Origin, Stability, Value

## Items

* Quality, Durability, Weight, Value, Rarity, Effectiveness

---

# 🌱 Ecosystem Loop

```
Flora → Characters → Death → Soil → Flora
```

---

# 🧬 Evolution System

```
Environment → Pressure → Selection → DNA → Behavior
```

Includes tradeoffs:

* Speed vs energy consumption
* Intelligence vs biological cost
* Specialization vs generalization
* Adaptation constraints

## Environmental Pressure Model

**Survival Difficulty Calculation:**

```
SurvivalDifficulty = f(Biome, Weather, Resources, Danger, Competition)
```

**Entity Impact:**

```
EffectiveNeedDrain = BaseNeedDrain × SurvivalDifficulty × AdaptationFactor
```

---

## Adaptation System (DNA Bridge)

Each species has **adaptation traits** that mitigate environmental pressure.

**Core Adaptations:**
* Heat Resistance (0.0 → 1.0)
* Cold Resistance (0.0 → 1.0)
* Water Efficiency (0.0 → 1.0)
* Oxygen Efficiency / Altitude (0.0 → 1.0)
* Toxic Resistance (0.0 → 1.0)
* Terrain Affinity: Forest, Desert, Mountain, Aquatic, etc. (0.0 → 1.0 per terrain)

---

## Survival Formula (KEY)

```
Strain = EnvironmentPressure - Adaptation
```

**If Strain > 0 (maladapted):**
* Hunger/Thirst Drain ↑
* Health Decline ↑
* Comfort↓
* Fulfillment ↓
* Reproduction Chance ↓
* Mortality ↑

**If Strain ≤ 0 (well-adapted):**
* Energy Efficiency ↑
* Health Recovery ↑
* Reproduction Chance ↑
* Population sustainability

---

## Example: Desert Selection

**Desert Pressure Vector:**
* Heat: 0.9
* Water Scarcity: 0.8
* Food Availability: 0.3

**Creature A (Poorly Adapted):**
* Heat Resistance: 0.2
* Water Efficiency: 0.3
* → Strain: HIGH
* → Dies off

**Creature B (Well Adapted):**
* Heat Resistance: 0.8
* Water Efficiency: 0.7
* → Strain: LOW
* → Thrives & reproduces

👉 **Natural selection without scripting.**

---

## DNA System

Keep it simple initially.

**DNA Structure:**

```
DNA = {
  Base Stats (Might, Lithe, Vigil, Grit, Vigor),
  Adaptations (Heat, Cold, Water, Oxygen, Toxic, Terrain),
  Traits (binary or scalar values)
}
```

---

## Reproduction & Inheritance

**Offspring Trait Calculation:**

```
OffspringTrait = (ParentA_Trait + ParentB_Trait) / 2 + Mutation
```

**Mutation Range:** ± 0.01–0.05 (small random variation)

**Reproduction Condition:**

```
ReproductionChance = Energy × Health × LowStrain × Safety
```

👉 Only well-adapted, healthy creatures in safe conditions reproduce successfully.

---

## Population Feedback Loop

```
1. Environment updates (weather, biome state)
2. Environment applies pressure (heat, cold, scarcity)
3. Entities calculate strain (env vs adaptation)
4. Needs + stress adjusted
5. Decisions made (based on perceived world)
6. Actions executed
7. Survival success/failure
8. Reproduction (based on success)
9. Traits propagate (DNA)
10. Population shifts
11. Ecosystem rebalances
```

---

## Environmental Pressure Modifiers

### Stat Effect Modifiers

**Cold Biome:**
- Lithe ↓ (stiff movement)
- Vigor drain ↑

**Dense Forest:**
- Vigil ↓ (visibility)
- Precision ↓

**High Altitude:**
- Vigor ↓ (oxygen)
- Attunement ↓

### Action Cost Modifiers

```
Movement Cost = BaseCost × Terrain × Weather × Adaptation
```

### Perception Modifiers

**Fog:**
- Visible range ↓
- Confidence ↓

**Storm:**
- Sound masking ↑
- Detection ↓

### Resource Availability Cascade

```
Biome → Flora → Food → Population Size → Predator Behavior
```

**Emergent Outcomes:**
* Migration patterns
* Extinction events
* Territorial conflict
* Niche specialization

---

## Niche Formation

When environment + traits interact, **roles emerge naturally**:

* Fast + Low Stamina → Scouts
* High Vigor + Low Intelligence → Laborers
* High Vigilance → Sentries
* High Precision → Hunters

👉 No role assignment needed—just natural bias.

---

## Environmental Instability Model

**Biome Stability Levels:**

* **Stable:** Consistent conditions; specialization rewarded
* **Seasonal:** Cyclic shifts; generalists + migration useful
* **Shifting:** Unpredictable changes; flexibility essential
* **Cataclysmic:** Frequent disasters; extinction risk high

**Result:** Species evolve strategies (specialization vs. generalization) based on stability

---

# 🏛️ Civilization System

Emerges from:

* Stable food
* Social grouping
* Shared memory

Features:

* Settlements
* Roles
* Expansion / defense
* Culture

---

# 🎭 Culture System

Emerges from:

* repeated behavior
* shared memory

Produces:

* norms
* traditions
* roles
* communication

---

## Cultural Feedback into Biology

Culture doesn't just describe society—it shapes evolution.

**Example Feedback Loops:**

* Warrior culture → population biased toward +Prowess
* Scholarly culture → population biased toward +Insight growth
* Harsh environments + cultural innovation → +Grit selection
* Peaceful societies → lower dominance/aggression traits

**Result:** Over generations, cultural values create measurable stat distributions in a population.

---

# 🧠 Consistency System (Identity)

```
Consistency = similarity(current_action, past_behavior)
```

Agents prefer consistent behavior.

---

# 🧭 Affordance System

Defines possible actions:

* Tree → harvest, climb
* Rock → carry, tool
* Water → drink

---

# 🗺️ Spatial System

* 2D position
* Region partitioning
* Distance interaction
* Pathfinding

---

# 🛠️ Crafting & Item System

See [docs/items.md](docs/items.md) for the complete item system documentation, including:

* Item categories (tools, weapons, consumables, materials, decorative)
* Physical, functional, and social properties
* Item lifecycle (creation, use, degradation, recycling)
* Inventory and trading systems
* Crafting integration and advanced features

---

# ⚙️ Action System

See [docs/actions.md](docs/actions.md) for the complete action reference, including:

* Stat → Speed and Skill → Potency mechanics
* Calculation formulas and worked examples
* Comprehensive action tables organized by category:
  * Movement, Interaction, Combat, Utility, Social
  * Tactical, Construction, Resource Gathering, Crafting
  * Consumables, Daily Life, Warfare, Trade, Magic
  * Arts & Culture, Exploration, Cognitive/Behavioral, Advanced Social

---

# 🏗️ Architecture

**SimContext contains:**

* ECS
* WorldState
* RNG
* Time

All systems use:

```
&mut SimContext
```

---

# 🚀 Implementation Phases

## Phase 1

* Spatial
* Hunger + energy
* Basic perception
* Consume
* Memory

## Phase 2

* Decision refinement
* Affordances
* Movement

## Phase 3

* Social memory
* Roles

## Phase 4+

* Culture
* Civilization
* Advanced personality

---

# 🏁 Final Unified Principle

```
Reality → Perception → Memory → Belief → Intent → Action → Feedback
```
