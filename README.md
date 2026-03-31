# 🌌 Aetherbourne — Unified Simulation Design

---

# 🧭 Project Philosophy

* **Observer-centric simulation**: The world is simulated from the perspective of observers, not omnisciently.
* **Emergence over scripting**: Behaviors and events arise from systems, not hardcoded scripts.
* **Everything is systemic, not hardcoded**: All elements interact through systems.
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

* Biomes, Weather, Time
* Minerals, Flora, Structures
* Spatial system

## 2. Biology Layer

* Species definitions
* Stats & derived stats
* Needs & energy
* Morphology & traits

## 3. Mind Layer

* Perception (subjective reality)
* Memory (decaying)
* Beliefs (persistent bias)
* Personality (waterfall model)
* Attention system

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

* Intelligence: Instinctive → Sapient
* Temperament: Docile / Neutral / Aggressive / Territorial / Skittish

---

## Diet

* Herbivore / Carnivore / Omnivore / Scavenger / Specialized

---

## Social Structure

* Solitary / Pair-bonded / Pack / Herd / Swarm / Hierarchical / Collective

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

* Energy Drain Rate (EDR) = 1.0
* Stress Gain Rate (SGR) = 1.0
* Morale Loss Rate (MLR) = 1.0
* Concentration Drain (CDR) = 1.0

Stats **multiply** these.

---

# 🔋 Energy System

```
Energy = Intake − Action Cost
0 = death
```

---

# 📊 Derived Stat Effects

## 🔴 Physical

**Prowess**

```
Output = 1 + (Prowess × 0.08)
Energy Drain = 1 + (Prowess × 0.05)
```

**Discipline**

```
Energy Spike Reduction = - (Discipline × 0.06)
Waste Chance = - (Discipline × 0.05)
```

**Tenacity**

```
Stress Gain = 1 - (Tenacity × 0.06)
Morale Loss = 1 - (Tenacity × 0.05)
```

**Fortitude**

```
Fatigue Reduction = - (Fortitude × 0.07)
Injury Impact = - (Fortitude × 0.05)
```

---

## 🟡 Efficiency

**Precision**

```
Concentration Drain = 1 - (Precision × 0.07)
Error Chance = - (Precision × 0.06)
```

**Finesse**

```
Stress Spikes = - (Finesse × 0.06)
Stability = + (Finesse × 0.05)
```

**Coordination**

```
Energy Drain = 1 - (Coordination × 0.06)
Consistency = + (Coordination × 0.05)
```

---

## 🔵 Mental

**Insight**

```
Decision Quality = + (Insight × 0.08)
Stress Gain = 1 - (Insight × 0.05)
```

**Attunement**

```
Concentration Regen = + (Attunement × 0.07)
Fatigue Penalty = - (Attunement × 0.05)
```

**Resolve**

```
Energy Regen = + (Resolve × 0.07)
Morale Recovery = + (Resolve × 0.06)
Stress Recovery = + (Resolve × 0.05)
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
* Stamina Drain: 1.0
* Perception Bonus: 1.1
* Duration: 6–24 hours

### Rainy

* Light Level: 0.5
* Temperature Effect: –3.0
* Visibility: 0.4
* Movement Speed: 0.9
* Stamina Drain: 1.2
* Perception Penalty: 0.15
* Fire Extinguish Rate: 0.8
* Plant Watering Rate: 0.3
* Erosion Rate: 0.25

### Stormy

* Light Level: 0.3
* Temperature Effect: –4.0
* Visibility: 0.2
* Movement Speed: 0.8
* Stamina Drain: 1.5
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

**Day Names:** Solis, Lunis, Terris, Aquas, Ignis, Ventis, Umbra, Lux

**Months:**

| Month | Season | Length (days) |
|-------|--------|---|
| Brigara | Winter | 28 |
| Imbrelle | Winter | 32 |
| Florayne | Spring | 30 |
| Lithara | Spring | 26 |
| Heliora | Summer | 31 |
| Aestara | Summer | 29 |
| Mavonel | Summer | 33 |
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
* Need Drain ↑
* Stress ↑
* Reproduction Chance ↓
* Mortality ↑

**If Strain ≤ 0 (well-adapted):**
* Efficiency ↑
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

# 🛠️ Crafting & Item System (Unified)

A unified system where **resources, materials, crafting, and items** form a continuous lifecycle.

## Full Pipeline

**↓ World Resources → Materials → Crafting → Items → Usage → Degradation → Recycling ↓**

* Resources define what exists
* Crafting defines transformation
* Items define gameplay impact
* Degradation creates feedback loops

---

## 2. Item System Overview

Items are **all interactable objects** that agents can carry, use, trade, or manipulate.

They are central to:

* Survival
* Crafting
* Combat
* Economy
* Social systems

Every crafted object becomes an **item with properties, behavior, and lifecycle**.

---

## 3. Item Categories

### 3.1 Tools & Equipment

Used to interact with the world and enable crafting.

* Primitive tools (stone, wood)
* Advanced tools (metal, precision)
* Containers (bags, barrels, chests)
* Light sources (torches, lanterns)

**Integration:** Tools act as **crafting requirements** and influence output quality.

### 3.2 Weapons & Armor

Used for combat and defense.

* Melee weapons (swords, axes, clubs)
* Ranged weapons (bows, slings)
* Armor (clothing, shields, helmets)
* Ammunition (arrows, bolts, thrown items)

**Integration:** Material properties directly affect damage, durability, weight, special effects.

### 3.3 Consumables

Items that are used up.

* Food and drink
* Medicine and potions
* Magical items (scrolls, runes)
* Fuel sources

**Integration:** Influenced by resource tags (edible, poisonous, medicinal) and environmental modifiers.

### 3.4 Materials & Resources

Items that feed back into crafting.

* Raw materials (wood, ore, fiber)
* Processed materials (ingots, cloth)
* Rare components (crystals, essence)
* Construction materials (brick, glass)

**Integration:** Bridge layer between gathering and crafting.

### 3.5 Decorative & Social Items

Non-essential but impactful.

* Jewelry
* Art objects
* Currency
* Gifts

**Integration:** Ties into social systems, reputation, economy.

---

## 4. Item Properties

### 4.1 Physical Properties

* **Weight** → affects carrying capacity
* **Size** → affects storage and visibility
* **Durability** → how long it lasts
* **Value** → economic worth

Influenced by:
* Material
* Crafting quality
* Item type

### 4.2 Functional Properties

* **Usability** → what actions it enables
* **Effectiveness** → performance strength
* **Requirements** → skills or conditions needed
* **Side Effects** → unintended outcomes

*Example:* A cursed item (tag-based) may harm the user; a brittle weapon deals high damage but breaks quickly.

### 4.3 Social Properties

* Faction markers
* Status symbols
* Cultural significance
* Trade value variation

Items exist in **society and narrative**, not just mechanics.

---

## 5. Item Lifecycle

### 5.1 Creation

Items are created through:

1. **Gathering** → raw resources
2. **Processing** → intermediate materials
3. **Assembly** → crafting system
4. **Enhancement** → quality or enchantment

### 5.2 Use & Degradation

* Items activated through actions
* Effects applied (damage, healing, etc.)
* Wear accumulates over time
* Maintenance restores condition

Ties into:
* Durability system
* Environmental effects
* Material traits

### 5.3 Destruction & Recycling

Items transform, not vanish:

* Breakage → damaged or ruined state
* Consumption → item used up
* Loss → dropped, stolen, destroyed
* Recycling → yields materials

Reinforces **closed-loop crafting economy**.

---

## 6. Inventory & Item Management

### 6.1 Inventory System

* Personal inventory (carried items)
* Shared storage (settlements)
* Equipment slots (worn/active items)
* Quick access (hotbar-style usage)

### 6.2 Trading System

* Barter-based exchange
* Dynamic market values
* Cultural preferences
* Illegal/black market systems

**Integration:** Item value and rarity directly affect trade.

---

## 7. Crafting Integration

### 7.1 Recipe Requirements

Items act as:

* Ingredients
* Tools
* Catalysts

### 7.2 Tool Dependency

Certain crafting requires tools:

* Hammer for metalwork
* Loom for textiles
* Alchemy setup for potions

Tool quality affects output.

### 7.3 Material Quality Influence

Input materials determine:

* Output durability
* Performance
* Special traits

### 7.4 Batch Processing

Players can produce multiple items depending on:

* Resources available
* Skill level
* Station capability

---

## 8. Advanced Item Features

### 8.1 Procedural Generation

Items can gain:

* Random modifiers
* Unique traits
* Material-based variations

### 8.2 Unique & Legendary Items

* Fixed identity
* Special abilities
* Lore & history

### 8.3 Environmental Adaptation

Items may:

* Change in different climates
* React to weather
* Gain/lose properties

### 8.4 Aging System

Items evolve over time:

* Improve (seasoned tools)
* Degrade (rust, decay)
* Gain historical value

---

## 9. Social & Cultural Systems

### 9.1 Heirloom System

Items can:

* Be passed down
* Gain value over time
* Carry history

### 9.2 Reputation Items

Certain items affect:

* Status
* Trust
* Influence

### 9.3 Gift Economy

Items can:

* Build relationships
* Repair alliances
* Trigger events

### 9.4 Ritual & Cultural Use

Items may be required for:

* Ceremonies
* Magic
* Traditions

---

## Integration Summary

Full gameplay loop emerges:

```
Gather → Craft → Use → Degrade → Recycle
```

System features:
* Tag-based crafting
* Material-driven properties
* Procedural variation
* Economy & culture
* Historical depth

---

# ⚙️ Action Effect System

## Stat → Speed Modifier

```
ModifiedDuration = BaseDuration × (1 – floor(Stat ÷ 40) × 0.5)
```

* Stat 0–39 → no speed boost
* Stat 40–79 → 50% faster
* Stat 80+ → 100% faster

---

## Skill → Potency & Success

```
ModifiedChange = BaseChange × (1 + floor(Skill ÷ 40) × 0.25)
Roll = d20 + floor(Skill ÷ 20) vs DC 12
```

**Outcomes:**
* Success → 100% effect
* Fail → 75% effect
* Crit Success → 150% effect
* Crit Fail → 50% effect

---

## Example Walkthrough

Character (Focus 60, Cooking 80) uses **Cook**:

1. **Find Cook's Stats**
   - Base Change: +10% Hunger, –2% Fatigue
   - Base Duration: 10 min

2. **Apply Stat → Speed**
   ```
   ModifiedDuration = 10 × (1 – floor(60÷40)×0.5)
                    = 10 × (1 – 1×0.5)
                    = 5 minutes
   ```

3. **Apply Skill → Potency**
   ```
   ModifiedChange = 10% × (1 + floor(80÷40)×0.25)
                  = 10% × (1 + 2×0.25)
                  = 15% Hunger boost
   ```

4. **Resolve Success Roll**
   ```
   Roll = d20 + floor(80÷20) = d20 + 4 vs DC 12
   • Success → +15% in 5 min
   • Fail    → 75% of 15% = +11.25%
   • Crit    → 150% = +22.5%
   ```

---

# 📋 Action Table (Comprehensive)

## Table Legend

| Column | Meaning |
|--------|---------|
| **Action** | In-game activity name |
| **Primary Stat(s)** | Governs action speed |
| **Relevant Skill(s)** | Determines potency & success |
| **Description** | What the action does |
| **Needs Affected** | Which needs change |
| **Base Change** | Raw % before modifiers |
| **Base Duration** | Minutes to full effect |
| **Rate per Tick** | Change per 10-min tick |

---

## Movement

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Walk | Dexterity | — | Move to nearby tile | Fatigue –2%, Thirst –1.5% | –2%, –1.5% | 10 min |
| Run | Stamina, Endurance | — | Move quickly | Fatigue –5%, Thirst –3%, Hunger –1.5% | –5%, –3%, –1.5% | 10 min |
| Jump | Dexterity, Stamina | — | Traverse gaps | Fatigue –3% | –3% | 5 min |
| Climb | Endurance, Strength | — | Ascend/descend | Fatigue –4%, Stress +2% | –4%, +2% | 10 min |
| Crouch | Dexterity | Stealth | Move stealthily | Fatigue –1%, Stress –2% | –1%, –2% | 5 min |
| Carry | Strength | — | Transport heavy objects | Fatigue –6%, Hunger –2% | –6%, –2% | 10 min |

---

## Interaction

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Inspect | Perception | Foraging, Observation | Examine details | Focus –3%, Motivation +2% | –3%, +2% | 10 min |
| Pick Up | Dexterity | — | Collect items | Fatigue –1% | –1% | 2 min |
| Use | Dexterity | — | Interact with tools/doors | Contextual | Varies | Varies |
| Speak | Willpower | Conversation, Charisma | Initiate dialogue | Social +5%, Morale +2% | +5%, +2% | 5 min |
| Trade | Perception, Conviction | Bartering, Negotiation | Exchange goods | Social +3%, Morale +2% | +3%, +2% | 10 min |

---

## Combat

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Attack | Strength, Dexterity, Power | Melee Combat, Archery | Offensive move | Fatigue –8%, Stress +4%, Hunger –2% | –8%, +4%, –2% | 10 min |
| Defend | Strength, Tenacity | Defense | Block/reduce damage | Fatigue –4%, Stress +2% | –4%, +2% | 5 min |
| Dodge | Dexterity, Perception | Reflex, Martial Arts | Evade attack | Fatigue –3%, Stress +3% | –3%, +3% | 5 min |
| Special Ability | Varies | Spellcraft, Tactics | Use power/spell | Mana –10%, Stress +5% | –10%, +5% | 5 min |
| Equip/Swap | Dexterity | — | Change weapons | Fatigue –1% | –1% | 2 min |

---

## Utility

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Craft | Focus, Dexterity | Crafting, Smithing, Tailoring, Alchemy | Create items | Fatigue –6%, Focus –4% | –6%, –4% | 15 min |
| Heal | Vitality, Willpower | First Aid, Spiritual Healing, Alchemy | Restore health | Health +10%, Fatigue –3% | +10%, –3% | 10 min |
| Rest | Vitality | — | Recover stamina | Fatigue +15%, Health +5% | +15%, +5% | 30 min |
| Signal | Willpower, Perception | Tactics, Communication | Call allies | Morale +3%, Focus +2% | +3%, +2% | 5 min |

---

## Social

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Befriend | Willpower, Conviction | Diplomacy, Empathy | Build rapport | Social +10%, Morale +5% | +10%, +5% | 20 min |
| Intimidate | Strength, Conviction | Leadership | Influence via fear | Morale –5%, Stress +5% | –5%, +5% | 10 min |
| Persuade | Willpower, Conviction | Negotiation, Bartering | Convince | Motivation +5%, Morale +3% | +5%, +3% | 10 min |
| Lie | Willpower, Perception | Deception | Deceive | Morale –2%, Stress +4% | –2%, +4% | 5 min |
| Form Bond | Willpower, Conviction | Empathy, Charisma | Close relationship | Social +15%, Motivation +10% | +15%, +10% | 30 min |

---

## Tactical

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Wait | Willpower | — | Delay action | Stress –2% | –2% | 5 min |
| Prepare | Focus, Insight | Tactics | Ready weapon/stance | Focus +5%, Motivation +2% | +5%, +2% | 10 min |
| Distract | Dexterity, Perception | Stealth, Performance | Divert attention | Stress –3%, Morale +2% | –3%, +2% | 5 min |
| Camouflage | Dexterity, Insight | Stealth | Blend in | Stress –4%, Focus +2% | –4%, +2% | 10 min |

---

## Construction

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Build | Strength, Endurance | Masonry, Carpentry, Engineering | Construct structures | Fatigue –7%, Hunger –3% | –7%, –3% | 10 min |
| Repair | Dexterity, Strength | Engineering, Masonry | Fix structures | Fatigue –5%, Focus –2% | –5%, –2% | 10 min |
| Survey | Insight, Perception | Design, Architecture | Plan layouts | Focus –3%, Motivation +2% | –3%, +2% | 10 min |
| Excavate | Strength, Stamina | Earthwork | Dig tunnels | Fatigue –6%, Hunger –2% | –6%, –2% | 10 min |

---

## Resource

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Plant | Dexterity, Focus | Farming | Grow crops | Motivation +3%, Hunger –2% | +3%, –2% | 10 min |
| Harvest | Dexterity, Perception | Farming, Foraging | Collect crops | Hunger +5%, Fatigue –2% | +5%, –2% | 10 min |
| Tame | Willpower, Dexterity | Taming | Train animals | Motivation +5%, Social +5% | +5%, +5% | 15 min |
| Hunt | Perception, Dexterity | Tracking, Archery, Combat | Kill wild animals | Hunger +10%, Fatigue –5%, Stress +3% | +10%, –5%, +3% | 15 min |
| Fish | Dexterity, Perception | Fishing | Catch aquatic resources | Hunger +8%, Fatigue –3% | +8%, –3% | 15 min |
| Mine | Strength, Endurance | Mining | Extract minerals | Fatigue –6%, Hunger –3% | –6%, –3% | 10 min |
| Chop | Strength, Dexterity | Logging | Cut trees | Fatigue –5%, Hunger –2% | –5%, –2% | 10 min |
| Gather | Dexterity, Perception | Foraging | Collect herbs | Hunger +5%, Fatigue –2% | +5%, –2% | 10 min |

---

## Crafting

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Forge | Strength, Focus | Smithing | Create weapons/armor | Fatigue –6%, Focus –3% | –6%, –3% | 15 min |
| Carve | Dexterity, Focus | Carpentry, Crafting | Shape wood | Fatigue –5%, Enjoyment +3% | –5%, +3% | 15 min |
| Weave | Dexterity, Focus | Tailoring, Weaving | Create textiles | Fatigue –4%, Comfort +3% | –4%, +3% | 15 min |
| Tinker | Dexterity, Insight | Crafting, Tinkering | Improve items | Focus –3%, Motivation +2% | –3%, +2% | 10 min |
| Refine | Focus, Dexterity | Tanning, Glasswork, Metallurgy | Process materials | Fatigue –5%, Hunger –2% | –5%, –2% | 15 min |
| Assemble | Dexterity, Focus | Engineering, Crafting | Fit components | Fatigue –4%, Focus –2% | –4%, –2% | 10 min |

---

## Consumable

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Cook | Focus, Dexterity | Cooking | Prepare meals | Hunger +10%, Fatigue –2% | +10%, –2% | 10 min |
| Bake | Focus, Dexterity | Baking | Make baked goods | Hunger +10%, Fatigue –2% | +10%, –2% | 10 min |
| Brew | Focus, Willpower | Brewing, Alchemy | Create drinks | Mana +10%, Hunger +5% | +10%, +5% | 15 min |

---

## Daily Life

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Clean | Dexterity, Willpower | — | Maintain hygiene | Hygiene +15%, Comfort +5% | +15%, +5% | 15 min |
| Organize | Insight, Dexterity | Logistics | Manage inventory | Motivation +5%, Focus –2% | +5%, –2% | 10 min |
| Care | Vitality, Willpower | Healing, Nurturing | Tend to others | Health +10%, Social +5% | +10%, +5% | 15 min |
| Socialize | Willpower, Conviction | Charisma, Conversation | Casual interaction | Social +10%, Morale +5% | +10%, +5% | 20 min |
| Teach | Insight, Willpower | Mentorship, Instruction | Instruct others | Motivation +5%, Focus –2% | +5%, –2% | 15 min |
| Work | Varies | Varies | Job-related tasks | Fatigue –6%, Motivation +4% | –6%, +4% | 15 min |

---

## Warfare

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Strategize | Insight, Willpower | Tactics, Strategy | Plan battles | Stress –5%, Focus –3% | –5%, –3% | 20 min |
| Scout | Perception, Dexterity | Navigation, Tracking | Survey areas | Fatigue –4%, Focus –2% | –4%, –2% | 10 min |

---

## Trade

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Haggle | Conviction, Perception | Bartering, Negotiation | Negotiate deals | Social +5%, Morale +3% | +5%, +3% | 10 min |
| Deliver | Endurance, Dexterity | Logistics | Transport items | Fatigue –5%, Hunger –2% | –5%, –2% | 10 min |
| Manage | Insight, Conviction | Accounting, Leadership | Oversee resources | Motivation +5%, Stress –2% | +5%, –2% | 15 min |

---

## Magic

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Cast Spell | Willpower, Focus | Spellcraft | Use magic | Mana –10%, Stress +2% | –10%, +2% | 5 min |
| Mix | Focus, Dexterity | Alchemy, Herbalism | Brew potions | Mana +10%, Hunger +5% | +10%, +5% | 15 min |
| Enchant | Focus, Willpower | Spellcraft, Runesmithing | Imbue objects | Mana –5%, Motivation +5% | –5%, +5% | 15 min |
| Divine | Insight, Willpower | Divination | Seek insight | Stress –5%, Focus +5% | –5%, +5% | 15 min |

---

## Arts & Culture

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Perform | Dexterity, Conviction | Performance | Sing, dance, entertain | Enjoyment +15%, Morale +5% | +15%, +5% | 30 min |
| Write | Insight, Focus | Writing, Curation | Record knowledge | Motivation +5%, Focus –2% | +5%, –2% | 15 min |
| Study | Insight, Willpower | Scholarship | Gain knowledge | Motivation +5%, Focus –2% | +5%, –2% | 15 min |

---

## Exploration

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Track | Perception, Insight | Tracking | Follow signs | Fatigue –4%, Focus –3% | –4%, –3% | 10 min |
| Navigate | Insight, Dexterity | Navigation | Guide travel | Fatigue –3%, Focus –2% | –3%, –2% | 10 min |
| Watch | Perception, Willpower | Vigilance | Observe surroundings | Stress –3%, Focus +2% | –3%, +2% | 10 min |

---

## Cognitive & Behavioral (Internal)

| Action | Primary Stat(s) | Relevant Skill(s) | Description |
|--------|-----------------|-------------------|-------------|
| **Desire** | Willpower | Motivation, Ambition | Generates needs and goals |
| **Remember** | Insight | Introspection, Scholarship | Logs experiences; influences decisions |
| **Decide** | Insight | Strategy, Tactics | Chooses action from available options |
| **Plan** | Insight | Strategy, Organization | Creates action sequence for goals |
| **Forget** | Willpower | Focus, Introspection | Purges low-value memories |
| **Learn** | Insight, Willpower | Scholarship, Socialization | Acquires knowledge/skills |

---

## Social (Advanced)

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Reproduce | Vitality, Willpower | Nurturing, Empathy | Create offspring (passes DNA) | (Internal) | — | — |
| Flee | Dexterity, Endurance | Reflex, Survival | Escape danger | Fatigue –5%, Stress +4% | –5%, +4% | 10 min |
| Steal | Dexterity, Perception | Stealth, Deception | Take without permission | Fatigue –2%, Stress +4% | –2%, +4% | 5 min |
| Give Item | Willpower, Dexterity | Empathy, Bartering | Hand over object | Social +5%, Morale +2% | +5%, +2% | 2 min |
| Observe | Perception, Insight | Observation, Vigilance | Watch/listen for info | Focus –3%, Motivation +2% | –3%, +2% | 10 min |
| Hide Item | Dexterity, Insight | Stealth, Deception | Conceal objects | Stress –2%, Comfort +3% | –2%, +3% | 10 min |

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
