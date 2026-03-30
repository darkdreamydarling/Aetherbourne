# Aetherbourne

## Project Philosophy


- **Observer-centric simulation**: The world is simulated from the perspective of observers, not omnisciently.
- **Emergence over scripting**: Behaviors and events arise from systems, not from hardcoded scripts.
- **Everything is systemic, not hardcoded**: All elements interact through systems, avoiding one-off logic.
- **Cause → Effect → Feedback Loops**: Every action has consequences, and feedback loops drive ongoing change.

---

## Universal Entity Framework

All entities (characters, fauna, agents) follow a unified structure:

**Entity =**
- Category
- Attributes (with defined ranges or enums)
- State (dynamic values)
- Behaviors (driven by systems)


### 1. Characters / Fauna / Agents

#### Species Definition
Every character/fauna/agent species is defined by a set of biological and physical attributes, which determine their capabilities, needs, and behaviors in the simulation.

#### Categories
- **Species**
- **Role** (dynamic job assignment)
- **Lifecycle Stage** (Infant, Child, Juvenile, Adult, Elder)
- **Intelligence Tier**

#### Lifecycle Stages
- Infant
- Child
- Juvenile
- Adult
- Elder

Each stage modifies:
- Stats scaling
- Needs intensity
- Behavior
- Capabilities
- Reproduction eligibility

#### Core Biology
- **Size:** Tiny / Small / Medium / Large / Huge / Colossal
- **Mass:** Light → Heavy
- **Lifespan:** Short → Long
- **Metabolism:** Slow → Rapid
- **Activity:** Nocturnal / Diurnal / Crepuscular / Always Active

#### Intelligence & Behavior
- **Intelligence:** Instinctive → Basic → Adaptive → Advanced → Sapient
- **Temperament:** Docile / Neutral / Aggressive / Territorial / Skittish

#### Diet
- Herbivore
- Carnivore
- Omnivore
- Scavenger
- Specialized

#### Social Structure
- Solitary
- Pair-bonded
- Pack
- Herd
- Swarm
- Hierarchical
- Collective

#### Reproduction
- Asexual / Sexual
- **Reproduction Rate:** Rare → Frequent
- **Offspring Count:** Low → High
- **Maturity Age:** Early → Late

#### Parental Care
- None
- Minimal
- Moderate
- Intensive

#### Population Density
- Sparse
- Low
- Moderate
- Dense
- Swarming

#### Territorial Behavior
- **Territorial Range:** Small → Massive
- **Migration Pattern:** None / Seasonal / Reactive / Constant

#### Defense & Combat
- **Skin Covering:** Skin / Fur / Scales / Chitin / Shell / Hybrid
- **Defense Mechanism:** None / Armor / Camouflage / Speed / Venom / Regeneration
- **Attack Strategy:** None / Ambush / Chase / Pack Hunting / Opportunistic


#### Base Stats
- **Might** → strength, damage, carrying capacity, mining effectiveness
- **Lithe** → agility, speed, dexterity, evasion, crafting precision
- **Vigil** → perception, sight range, smell, detection chance
- **Grit** → willpower, stress resistance, persistence, mental resilience
- **Vigor** → endurance, health, stamina, survivability

#### Derived Stats (All Combinations)
- **Prowess** = Might + Lithe → combat execution, attack flow
- **Finesse** = Lithe + Vigil → precision, crafting accuracy, targeting
- **Insight** = Vigil + Grit → awareness, decision quality, pattern recognition
- **Resolve** = Grit + Vigor → persistence, resistance to stress and fatigue
- **Fortitude** = Vigor + Might → durability, damage resistance
- **Precision** = Might + Vigil → strike accuracy, harvesting efficiency
- **Drive** = Might + Grit → motivation, forceful action
- **Guile** = Lithe + Grit → deception, evasiveness, trickery
- **Stamina** = Lithe + Vigor → sustained effort, energy efficiency
- **Vigilance** = Vigil + Vigor → long-duration awareness, fatigue resistance

All stats directly influence:
- Combat
- Crafting
- Harvesting
- Detection
- Movement
- Survival
- Decision scoring

#### Needs (0–100, dynamic decay)
- Hunger, Thirst, Warmth, Comfort/Safety, Health, Reproduction, Social, Fulfillment, Waste
	- Each need: decay rate, urgency curve, competes in decision system

#### Energy System
- Energy = Intake − Action Cost (0 = death)

#### Biological & Physical Attributes
- Morphology: limbs, locomotion
- Core Biology: size, mass, lifespan, metabolism, activity
- Intelligence & Behavior: instinctive → sapient, temperament
- Diet: herbivore, carnivore, omnivore, etc.
- Social Structure: solitary, pack, herd, etc.
- Reproduction: asexual/sexual, rate, offspring count, maturity age, parental care
- Population Density, Territorial Behavior, Migration, Defense & Combat

#### Personality & Cognition (Waterfall Model)
1. Reflex (BIS/BAS): avoidance, reward drive, thresholds, reaction speed
2. Routine (HEXACO): honesty, emotionality, extraversion, agreeableness, conscientiousness, openness
3. Strategy (Dark Triad): machiavellianism, narcissism, psychopathy
4. Purpose (Enneagram): type, core desire/fear
5. Communication (DISC): dominance, influence, steadiness, conscientiousness

#### Action System
- Movement, Interaction, Harvesting, Crafting, Survival, Reproduction, Communication
- Skills (0–100, learned): movement, crafting, communication, harvesting, combat, survival

#### Roles / Jobs
- Dynamic, not exclusive: scout, soldier, medic, cook, crafter, farmer, forager, hunter, miner, gatherer, caretaker
- Each role modifies: decision weights, task priority, movement patterns

---


## Perception System

Entities do not access global truth. Each entity maintains its own:
- **Visible Entities**
- **Known Resources**
- **Known Threats**
- **Confidence Level** (certainty of info)

**Perception is influenced by:**
- Vigil (stat)
- Distance
- Obstruction
- Environment (weather, terrain)
- Memory

---

## Memory System

### 1. Individual Memory
Stores:
- Locations (food, danger, shelter)
- Outcomes (success/failure)
- Social interactions

Properties:
- Decays over time
- Confidence decreases
- Limited capacity (budgeted)

### 2. Shared Memory (Social)
- Information spreads through interaction
- Enables culture, group intelligence, collective behavior

---

## Affordance System

Defines what actions are possible for each entity/object.

**Entity/Object → Affordances**

Examples:
- Tree → harvest, climb
- Rock → carry, use as tool
- Water → drink

**Purpose:**
- Reduces rule complexity
- Improves performance
- Enables dynamic interaction discovery

---

## Spatial System

**Requirements:**
- Position (2D)
- Region partitioning
- Distance-based interaction
- Pathfinding

**Influences:**
- Movement cost
- Resource access
- Territory
- Perception

---

## Civilization System

**Emergence Conditions:**
- Stable food supply
- Social grouping
- Memory sharing

**Features:**
- Settlements
- Role specialization
- Expansion / defense
- Cultural formation

---

## Culture System

**Emerges from:**
- Repeated behaviors
- Shared memory

**Produces:**
- Norms
- Roles
- Traditions
- Communication patterns

---


### Biomes
- Grassland, Highland, Subterranean, Coastal, Ocean, Wetland, Desert, Tundra, Forest, Polar, Volcanic
- **Attributes:**
	- **Moisture:** Level of water availability (Arid, Semi-Arid, Temperate, Humid)
	- **Climate:** General temperature and weather pattern (Frigid, Cool, Warm, Torrid)
	- **Topography:** Terrain shape and elevation (Flat, Undulating, Rugged, Vertical)
	- **Stability:** How often the biome changes (Stable, Seasonal, Shifting, Cataclysmic)
	- **Soil Quality:** Fertility and richness for plant growth (Barren, Poor, Moderate, Fertile, Rich)


### Minerals
- Ore, Gemstone, Aggregate, Organic, Evaporite, Crystalline, Stone
- **Attributes:**
	- **Abundance:** How common or rare the mineral is (Common, Uncommon, Rare, Precious, Unique)
	- **Value:** Economic or practical worth (Low → Extreme)
	- **Purity:** Quality and concentration of the mineral (Crude, Refined, High-Grade, Flawless)
	- **Hardness:** Resistance to scratching or breaking (Soft, Brittle, Firm, Hard, Ultra-Hard)
	- **Reactivity:** Tendency to chemically react (Inert, Stable, Reactive, Volatile, Catalytic)
	- **Formation:** Geological process of creation (Igneous, Sedimentary, Metamorphic, Hydrothermal, Biogenic)
	- **Extraction:** How and where it can be mined (Surface, Shallow, Deep, Hazardous)


### Flora
- Tree, Bush, Flower, Herb, Grain, Fungus, Vine, Aquatic, Moss
- **Attributes:**
	- **Reproductive:** How the plant reproduces (Flowering, Fruiting, Seed, Spore)
	- **Utility:** Uses for characters or ecosystem (Edible, Medicinal, Toxic, Fibrous, Structural)
	- **Growth Cycle:** Lifespan and seasonal pattern (Evergreen, Deciduous, Ephemeral, Perennial)
	- **Size:** Physical scale (Dwarf, Small, Medium, Large, Giant)
	- **Habitat:** Preferred environment (Terrestrial, Aquatic, Marsh, Alpine, Desert)
	- **Ecology:** Role in ecosystem (Passive, Invasive, Symbiotic, Parasitic, Carnivorous)
	- **Yield:** Amount and frequency of resources produced (Sparse, Moderate, Abundant, Seasonal)


### Weather
- Rain, Storm, Wind, Snow, Fog, Drought, Lightning
- **Attributes:**
	- **Intensity:** Strength or severity of the event (Light → Cataclysmic)
	- **Duration:** How long the event lasts (Brief → Persistent)
	- **Predictability:** How regular or random the event is (Stable, Variable, Unpredictable)
	- **Temperature Extreme:** Range of temperature impact (Mild → Severe)


### Structures
- Temple, Fortress, Mine, Tomb, Tower, Road, Monument
- **Attributes:**
	- **Age:** How old the structure is (Recent, Old, Ancient, Primordial)
	- **Condition:** Current state of repair (Intact, Weathered, Ruined, Collapsed)
	- **Origin:** Who or what created it (Natural, Civilized, Lost Civilization, Unknown)
	- **Stability:** Likelihood of remaining standing (Stable → Crumbling)
	- **Value:** Cultural, historical, or material worth (Low → Legendary)


### Items
- Food, Tool, Weapon, Material, Medicine, Textile, Luxury
- **Attributes:**
	- **Quality:** Craftsmanship and effectiveness (Poor → Masterwork)
	- **Durability:** Resistance to wear and breakage (Fragile → Unbreakable)
	- **Weight:** Mass and ease of transport (Light → Heavy)
	- **Value:** Economic or practical worth (Cheap → Exotic)
	- **Rarity:** How often the item is found (Common → Unique)
	- **Effectiveness:** How well it performs its function (Weak → Powerful)

---

## Cross-System Attribute Logic

- All systems interact through attributes.
- Characters evaluate: `DecisionScore = Need × Stat × AttributeValue × PersonalityModifiers × Memory`
- Biomes modify flora growth, survival difficulty.
- Minerals affect resource availability, danger.
- Flora feeds characters and ecosystem.
- Characters affect environment, culture, resource depletion.

### State vs Attribute Distinction
- **Static (Generated):** category, base attributes, DNA
- **Dynamic (Changing):** needs, energy, relationships, health, position, memory

---

## Complete System Overview

### Core Philosophy
- Observer-centric simulation (world exists independently of player)
- Emergence over scripting
- Everything is systemic, not hardcoded
- Cause → Effect → Feedback loops drive everything


### Core Simulation Loop (per tick)
1. Environment updates
2. Geology updates
3. Flora grows/decays
4. **Perception updates** (NEW priority)
5. Characters decide (perceived world)
6. Actions execute
7. Interactions resolve
8. Memory updates
9. Ecosystem balances
10. Civilization updates
11. Events trigger

### Core Architecture
- **SimContext** holds:
	- ECS (entities: characters, flora, etc.)
	- WorldState (regions, biomes)
	- RNG
	- Tick/time
- All systems receive: `&mut SimContext`

#### Separation of Concerns
- `main.rs` → window + loop only
- `simulation/` → pipeline + tick
- `systems/` → pure logic
- `ecs/` → characters/entities
- `world/` → regions/biomes

---

## Character System (Details)

- Characters (not just "agents") have physical, biological, and psychological traits.
- Traits: limbs, locomotion, diet, size, metabolism, lifespan, reproduction, skin/defense/attack
- Base Stats: Brawn, Lithe, Vigil, Grit, Vigor
- Advanced Stats: Rend, Wrath, Mark, Bulwark, Oath, Ward, Omen, Veil, Wisp, Drift

#### Advanced Stat Formulas

| Stat      | Formula             | Purpose                        |
|-----------|---------------------|--------------------------------|
| Rend      | Brawn + Lithe       | Damage output                  |
| Wrath     | Brawn + Grit        | Aggression/attack drive        |
| Mark      | Lithe + Vigil       | Accuracy/targeting             |
| Bulwark   | Vigor + Brawn       | Damage resistance              |
| Oath      | Vigor + Grit        | Endurance/sustain              |
| Ward      | Vigil + Grit        | Danger sense                   |
| Omen      | Vigil + Grit        | Prediction/intuition           |
| Veil      | Lithe + Vigil       | Stealth/concealment            |
| Wisp      | Lithe + Vigil       | Evasion/reaction speed         |
| Drift     | Lithe + Vigor       | Movement efficiency            |

*Stat combos overlap intentionally; same inputs, different purposes.*

#### Example (Rust):

```rust
pub struct Stats {
		pub brawn: f32,
		pub lithe: f32,
		pub vigil: f32,
		pub grit: f32,
		pub vigor: f32,
}

impl Stats {
		pub fn rend(&self) -> f32 { self.brawn + self.lithe }
		pub fn bulwark(&self) -> f32 { self.vigor + self.brawn }
		pub fn omen(&self) -> f32 { self.vigil + self.grit }
		// ...etc
}
```

---

## Personality System (Waterfall Model)

1. Reflex (BIS/BAS): immediate reactions, overrides everything
2. Behavioral (HEXACO): default personality
3. Ethical (Dark Triad): selfish vs cooperative
4. Intentional (Enneagram): long-term goals
5. Communication (DISC): how actions are expressed

---

## Decision & Action Systems

- Needs drive behavior: hunger, thirst, energy, safety, warmth, social, health, reproduction, fulfillment
- Decision formula: `Score = Need × Stats × Personality × Memory × Random`
- Output: character selects action, passed to interaction system
- High-level actions: movement, combat, interaction, harvesting, crafting, survival, communication

---

## Interaction Engine

- Everything runs on rules: Source + Target → Rule → Effect
- Example: Character + Flora → Consume
- Rule structure: conditions, inputs (stats, traits), formula, outputs (state changes)
- Types: consumption, combat, growth, decay, transfer, extraction, social

---

## World, Flora, Mineral, and Ecosystem Systems

- Biomes: defined by moisture, temperature, terrain, stability, soil quality
- Flora: type, utility, growth, yield, ecology
- Minerals: category, hardness, purity, reactivity, value, extraction
- Ecosystem loop: flora feeds characters, characters consume flora, death feeds soil, soil grows flora

---

## Civilization, Social, and Culture Systems

- Characters form groups, settlements, roles/jobs
- Settlement behavior: expand if resources low, fortify if danger high, specialize by environment
- Social system: relationships based on interaction × personality (trust, affinity, resentment)
- Culture: repeated behaviors → traditions, roles, norms, language, writing

---

## Weather, Danger, and Memory Systems

- Weather: rain, storms, drought, etc. affects growth, movement, survival
- Danger: total danger = weather + geological, affects survival, evolution, behavior
- Memory: regions track danger/resource history; characters use memory for decisions

---

## Implementation Model

- All systems access data only through `&mut SimContext`
- ECS stores: characters, flora, entities
- World stores: regions, biomes, large-scale data

---


## Implementation Priorities (Rewritten)

### Phase 1 (MVE)
- Spatial system
- Hunger + energy
- Basic perception
- Consume interaction
- Simple memory

### Phase 2
- Decision refinement
- Affordances
- Multiple needs
- Movement + pathfinding

### Phase 3
- Social memory
- Roles
- Cooperation/competition

### Phase 4+
- Culture
- Advanced personality
- Civilization systems

---

## Mental Model

- Biology defines capability
- **Perception defines reality** ← NEW
- Needs drive decisions
- **Memory shapes behavior** ← NEW
- Systems create emergence

---

## Mental Model

- **Base stats = biology**
- **Advanced stats = usable capabilities**

## Core Simulation Loop (per tick)

1. Environment updates
2. Geology updates
3. Flora grows/decays
4. Characters act (needs + decisions)
5. Interactions happen (rules)
6. Ecosystem balances
7. Civilization updates
8. Events trigger

## Core Architecture

- `SimContext` holds:
	- ECS (entities: characters, flora, etc.)
	- WorldState
	- RNG
	- Tick/time
- All systems receive: `&mut SimContext`
