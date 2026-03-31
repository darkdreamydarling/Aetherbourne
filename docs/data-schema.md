# 📊 Data Schema — Canonical Data Structures

This document defines the complete entity/component/item/interaction data model for Aetherbourne. Every structure shown here has an equivalent Rust struct template. **This is the authoritative source for all data types.**

---

## Core Architecture: Component-Based Entity Model

Entities are composed of components. Each component represents a distinct aspect of state.

```rust
// Core entity container
pub struct Entity {
    pub id: u64,                    // Unique identifier
    pub category: EntityCategory,   // Type of entity
    pub name: String,               // Display name
    pub tags: HashSet<String>,      // Categorical tags (e.g., "ai", "mortal", "flying")
    pub active: bool,               // Simulation active?
}

pub enum EntityCategory {
    Character,      // Sentient or intelligent entity
    Creature,       // Non-sentient fauna
    Flora,          // Trees, plants, fungi
    Structure,      // Building, landmark, inactive object
    Item,           // Carried or dropped object
    Hazard,         // Environmental danger (fire, toxic cloud)
}
```

---

## Attribute System

### Stats (Base Attributes)

Five base stats define capability. Range: 0–100.

```rust
pub struct StatBlock {
    pub might: f32,     // Physical force output (0–100)
    pub lithe: f32,     // Agility & efficiency (0–100)
    pub vigil: f32,     // Awareness & perception (0–100)
    pub grit: f32,      // Resilience & persistence (0–100)
    pub vigor: f32,     // Endurance & energy recovery (0–100)
}

// Derived stats computed from pairs of base stats
pub struct DerivedStats {
    pub prowess: f32,       // (might + lithe) / 2
    pub fortitude: f32,     // (vigor + grit) / 2
    pub tenacity: f32,      // (grit + vigor) / 2
    pub insight: f32,       // (vigil + grit) / 2
    pub focus: f32,         // (vigil + lithe) / 2
}
```

### Skills (Learned Attributes)

Skills represent practice and learning. Range: 0–100.

```rust
pub struct SkillBlock {
    pub melee_combat: f32,       // Close-quarters fighting
    pub archery: f32,            // Ranged weaponry
    pub crafting: f32,           // General crafting ability
    pub smithing: f32,           // Metalworking (specialization)
    pub tailoring: f32,          // Textiles (specialization)
    pub alchemy: f32,            // Potion/reagent crafting
    pub cooking: f32,            // Food preparation
    pub forage: f32,             // Plant/resource gathering
    pub hunting: f32,            // Fauna tracking & harvesting
    pub fishing: f32,            // Water resource extraction
    pub herding: f32,            // Fauna control/domestication
    pub agriculture: f32,        // Flora cultivation
    pub stealth: f32,            // Movement concealment
    pub first_aid: f32,          // Health restoration
    pub spiritual_healing: f32,  // Magic-based healing
    pub leadership: f32,         // Group influence
    pub diplomacy: f32,          // Peaceful negotiation
    pub deception: f32,          // Lying & deception
    pub empathy: f32,            // Social understanding
    pub bartering: f32,          // Trade negotiation
    pub observation: f32,        // Detail perception
    pub reflex: f32,             // Combat evasion
    pub martial_arts: f32,       // Unarmed combat
    pub navigation: f32,         // Pathfinding & orientation
    pub climbing: f32,           // Vertical traversal
    pub spellcraft: f32,         // Magic casting (if available)
    pub tactics: f32,            // Strategic thinking
}
```

---

## Need System

Entities have 9 core needs. Each need ranges from 0–100 (0 = satisfied, 100 = critical).

```rust
pub enum NeedType {
    Hunger,         // Nutrition deficit
    Thirst,         // Hydration deficit
    Fatigue,        // Energy depletion
    Warmth,         // Temperature regulation need
    Comfort,        // Safety & well-being (shelter, low danger)
    Stress,         // Psychological pressure (fear, conflict)
    Social,         // Connection need (isolation penalty)
    Fulfillment,    // Purpose/meaning (low causes depression)
    Waste,          // Hygiene need (poison accumulation)
}

pub struct NeedState {
    pub hunger: f32,        // 0–100
    pub thirst: f32,        // 0–100
    pub fatigue: f32,       // 0–100
    pub warmth: f32,        // 0–100
    pub comfort: f32,       // 0–100
    pub stress: f32,        // 0–100
    pub social: f32,        // 0–100
    pub fulfillment: f32,   // 0–100
    pub waste: f32,         // 0–100
}

// Decay rates (% per tick, typically applied once per simulation tick)
pub struct NeedDecayRates {
    pub hunger_drain: f32,       // Base: 0.5% per tick
    pub thirst_drain: f32,       // Base: 0.6% per tick
    pub fatigue_drain: f32,      // Base: 0.4% per tick (recovers during rest)
    pub warmth_drain: f32,       // Base: 0.3% per tick (varies by environment)
    pub comfort_drain: f32,      // Base: 0.2% per tick
    pub stress_drain: f32,       // Base: 0.25% per tick (reduced by safety/social)
    pub social_drain: f32,       // Base: 0.15% per tick
    pub fulfillment_drain: f32,  // Base: 0.1% per tick
    pub waste_accumulation: f32, // Base: 0.4% per tick
}
```

**Modifiers on decay:**
- Stats: Vigor reduces fatigue drain; Grit reduces stress drain
- Environment: Hot biome increases thirst drain; cold biome increases warmth drain
- Activity: Running increases all drains; resting recovers fatigue
- Lifecycle: Infants drain faster; elders drain slower

---

## Health & Energy

```rust
pub struct Health {
    pub current: f32,       // Current health points (0–max)
    pub max: f32,           // Maximum health (depends on size + vigor)
    pub damage_threshold: f32,  // Minimum damage to register (armor effect)
}

pub struct Energy {
    pub current: f32,       // Current energy (0–max)
    pub max: f32,           // Maximum energy (depends on vigor)
    pub recovery_rate: f32, // Energy per tick at rest
}
```

---

## Item System

### Item Base Definition

```rust
pub struct Item {
    pub id: u64,                    // Unique identifier
    pub name: String,               // Display name
    pub category: ItemCategory,     // Type classification
    pub materials: Vec<(String, u32)>,  // List of (material_name, quantity)
    pub quality: f32,               // 0–100 (affects all properties)
    pub durability: f32,            // 0–100 (100 pristine, 0 broken)
    pub created_tick: u64,          // When created (for aging)
    pub owner_id: Option<u64>,      // Current owner (entity id or structure id)
    pub tags: HashSet<String>,      // Behavioral tags (e.g., "edible", "weapon", "cursed")
}

pub enum ItemCategory {
    Tool,               // Hammer, pickaxe, shovel, etc.
    Weapon,             // Sword, bow, club, etc.
    Armor,              // Helmet, chestplate, shield, etc.
    Consumable,         // Food, drink, potion, etc.
    Material,           // Ore, wood, cloth, etc. (crafting input)
    Container,          // Bag, chest, barrel, etc.
    Decoration,         // Jewelry, art, trophy, etc.
    Currency,           // Coin, trade good, etc.
}
```

### Item Properties

```rust
pub struct ItemProperties {
    // Physical properties
    pub weight: f32,            // kg (affects carrying)
    pub size: f32,              // volume (affects storage)
    pub value_base: f32,        // Base trade value (in currency units)
    
    // Functional properties
    pub effectiveness: f32,     // 0–100 (tool quality, weapon damage scaling)
    pub durability_loss_rate: f32,  // % per use
    pub use_frequency: f32,     // How often tool/weapon is used per action
    
    // Stat modifiers (for equipment)
    pub stat_modifier: Option<StatModifier>,
    
    // Consumable effects
    pub consumable_effect: Option<ConsumableEffect>,
    
    // Environmental effects
    pub degrades_in_moisture: bool,
    pub degrades_in_heat: bool,
    pub degrades_in_cold: bool,
}

pub struct StatModifier {
    pub might: f32,     // Added to stat if equipped
    pub lithe: f32,
    pub vigil: f32,
    pub grit: f32,
    pub vigor: f32,
}

pub struct ConsumableEffect {
    pub hunger_reduction: f32,      // % of max need
    pub thirst_reduction: f32,
    pub fatigue_recovery: f32,
    pub health_recovery: f32,
    pub toxicity: f32,              // Poison damage if > 0
    pub duration_ticks: u32,        // If non-instant effect
}

// Material properties influence item effectiveness
pub struct MaterialProperties {
    pub name: String,
    pub hardness: f32,          // 0–100 (affects tool longevity)
    pub density: f32,           // Affects weight
    pub value_multiplier: f32,  // Rarity/prestige factor (gold 1.5x, copper 0.8x)
    pub durability_multiplier: f32,  // How long before breaking/rusting
    pub rust_rate: f32,         // Decay in moisture (0 for non-metals)
}
```

### Item Durability States

```rust
pub enum DurabilityState {
    Pristine,       // 100–70: Full effectiveness
    Worn,           // 70–30: 70% effectiveness, visual wear
    Damaged,        // 30–1: 40% effectiveness, visible cracks
    Broken,         // 0: 0% effectiveness, unusable (except recycle)
}

impl DurabilityState {
    pub fn effectiveness_multiplier(&self) -> f32 {
        match self {
            DurabilityState::Pristine => 1.0,
            DurabilityState::Worn => 0.7,
            DurabilityState::Damaged => 0.4,
            DurabilityState::Broken => 0.0,
        }
    }
}
```

### Item Modifiers (Procedural Generation)

```rust
pub struct ItemModifier {
    pub name: String,           // e.g., "Sharp", "Rusty", "Legendary"
    pub rarity: Rarity,
    pub stat_bonus: StatModifier,
    pub effectiveness_multiplier: f32,  // Scales effectiveness
    pub durability_penalty: f32,        // Extra wear rate
    pub value_multiplier: f32,
}

pub enum Rarity {
    Common,         // 0 modifiers
    Uncommon,       // 1 modifier (prefix or suffix)
    Rare,           // 2 modifiers (prefix + suffix)
    Legendary,      // 2–3 modifiers (unique item)
}

pub struct ModifiedItem {
    pub base_item: Item,
    pub prefix_modifier: Option<ItemModifier>,   // e.g., "Sharp"
    pub suffix_modifier: Option<ItemModifier>,   // e.g., "of Durability"
    pub rare_modifier: Option<ItemModifier>,     // Unique/mythic property
}
```

---

## Stat System

### Base Stats (0–100 scale)

Each stat represents a capability dimension.

```rust
// Stat contribution to action speed
// ModifiedDuration = BaseDuration × (1 – floor(Stat ÷ 40) × 0.5)
// Tiers: 0-39→no boost, 40-79→50% faster, 80-100→100% faster
pub fn stat_speed_multiplier(stat: f32) -> f32 {
    let tier = (stat / 40.0).floor() as u32;
    1.0 - (tier as f32 * 0.5).min(1.0)  // Cap at 1.0 (can't slow down)
}

// Skill contribution to potency
// ModifiedChange = BaseChange × (1 + floor(Skill ÷ 40) × 0.25)
// Tiers: 0-39→base, 40-79→+25%, 80+→+50%
pub fn skill_potency_multiplier(skill: f32) -> f32 {
    let tier = (skill / 40.0).floor() as u32;
    1.0 + (tier as f32 * 0.25)
}

// Skill contribution to roll bonus
// Roll = d20 + floor(Skill ÷ 20)
pub fn skill_roll_bonus(skill: f32) -> i32 {
    (skill / 20.0).floor() as i32
}
```

---

## Affordable & Interaction System

### Affordance Definition

An affordance is a possible action given entity capability, target properties, and context.

```rust
pub struct Affordance {
    pub id: u64,
    pub name: String,                  // e.g., "Eat", "Craft", "Trade"
    pub affordance_type: AffordanceType,
    pub required_capability: AffordanceCapability,
    pub context_requirements: Vec<ContextRequirement>,
    pub priority_tier: PriorityTier,   // Immediate/High/Medium/Low
    pub salience: f32,                 // 0–100 (how important right now)
    pub success_chance: f32,           // 0–100 (likelihood of success)
}

pub enum AffordanceType {
    Environmental,  // Harvest, climb, shelter, burn
    Creature,       // Hunt, herd, breed, flee, fight
    Sentient,       // Speak, trade, ally, follow, revolt
    Structural,     // Enter, craft, gather, defend
    Social,         // Befriend, intimidate, persuade, lie, bond
}

pub struct AffordanceCapability {
    pub required_stats: Option<StatRequirements>,
    pub required_skills: Option<SkillRequirements>,
    pub energy_cost: f32,
    pub time_cost_ticks: u32,
}

pub struct ContextRequirement {
    pub context_type: String,
    pub required_value: String,
}

pub enum PriorityTier {
    Immediate,  // Life threat (flee danger, heal critical)
    High,       // Unsatisfied need > 75
    Medium,     // Unsatisfied need 50–75
    Low,        // Routine/idle activity
}

// Salience formula for affordance weighting
// Salience = base_importance × (1 + need_alignment) × (1 + threat_level) × (1 + novelty)
// Where:
//   base_importance: 0–1 (inherent weight of affordance type)
//   need_alignment: 0–1 (how well affordance satisfies current top need)
//   threat_level: 0–2 (environmental danger scaling)
//   novelty: 0–0.5 (bonus for unexperienced actions)
```

### Interaction Definition

An interaction is a directed action from source to target with measurable effects.

```rust
pub struct Interaction {
    pub id: u64,
    pub source_entity_id: u64,        // Entity performing action
    pub target_entity_id: u64,        // Entity/object receiving action
    pub interaction_type: InteractionType,
    pub priority: InteractionPriority,
    pub cost: InteractionCost,
    pub context: InteractionContext,
}

pub enum InteractionType {
    Consumption,    // Eat, drink, fuel
    Combat,         // Attack, defend, dodge
    Extraction,     // Mine, harvest, hunt, gather
    Crafting,       // Build, assemble, refine
    Transfer,       // Give, trade, receive
    Growth,         // Plant growth, reproduction
    Decay,          // Aging, rust, rot
    Social,         // Speak, befriend, lie, trade
    Environmental,  // Move, climb, swim, shelter
    Reproduction,   // Mating interaction
}

pub struct InteractionCost {
    pub energy: f32,
    pub time_ticks: u32,
    pub risk_factor: f32,  // Chance to fail or harm self
}

pub struct InteractionContext {
    pub environment_type: String,      // Biome/location
    pub weather_conditions: String,    // Clear, rainy, stormy, etc.
    pub temperature: f32,              // Affects some interactions
    pub proximity_distance: f32,        // Distance between source & target
}

pub enum InteractionPriority {
    Critical,   // Immediate threat (combat, extreme need)
    High,       // Important objective (food, reproduction)
    Normal,     // Routine
    Low,        // Idle activity
}

pub enum InteractionOutcome {
    CriticalSuccess,    // 150% effect
    Success,            // 100% effect
    Failure,            // 75% effect
    CriticalFailure,    // 50% effect
}
```

---

## Component Types

All entity aspects are modeled as components on the entity.

```rust
pub struct TransformComponent {
    pub x: f32,
    pub y: f32,
    pub z: f32,  // Altitude/depth
    pub rotation: f32,
}

pub struct VelocityComponent {
    pub vx: f32,
    pub vy: f32,
    pub direction: f32,  // Facing (0–360°)
}

pub struct SpeciesComponent {
    pub species_id: String,         // e.g., "human", "wolf", "oak"
    pub lifecycle_stage: LifecycleStage,
    pub age_ticks: u64,
}

pub enum LifecycleStage {
    Infant,      // 0–10% of lifespan
    Child,       // 10–25%
    Juvenile,    // 25–50%
    Adult,       // 50–90%
    Elder,       // 90–100%
    Dead,        // End state
}

pub struct StatsComponent {
    pub base_stats: StatBlock,
    pub derived_stats: DerivedStats,
    pub skills: SkillBlock,
}

pub struct HealthComponent {
    pub health: Health,
    pub energy: Energy,
}

pub struct NeedsComponent {
    pub needs: NeedState,
    pub decay_rates: NeedDecayRates,
}

pub struct InventoryComponent {
    pub items: Vec<u64>,            // Item entity IDs
    pub carried_weight: f32,        // Total weight currently carrying
    pub weight_capacity: f32,       // Max before encumbrance
    pub equipment_slots: EquipmentSlots,
}

pub struct EquipmentSlots {
    pub head: Option<u64>,          // Item ID if equipped
    pub torso: Option<u64>,
    pub hands: Option<u64>,
    pub legs: Option<u64>,
    pub feet: Option<u64>,
    pub main_hand: Option<u64>,
    pub off_hand: Option<u64>,
    pub accessories: Vec<u64>,      // Can equip multiple
}

pub struct MemoryComponent {
    pub immediate_memories: Vec<Memory>,     // Last 0–5 min
    pub short_term_memories: Vec<Memory>,    // Last 5 min–1 hr
    pub long_term_memories: Vec<Memory>,     // Permanent
}

pub struct Memory {
    pub event_id: u64,
    pub description: String,
    pub context_tags: Vec<String>,
    pub emotional_weight: f32,      // How impressive/traumatic (-1 to +1)
    pub recorded_tick: u64,
}

pub struct PerceptionComponent {
    pub perceived_entities: Vec<PerceivedEntity>,
    pub perceived_items: Vec<PerceivedItem>,
    pub sensory_ranges: SensoryRanges,
}

pub struct PerceivedEntity {
    pub entity_id: u64,
    pub name: String,
    pub distance: f32,
    pub threat_level: f32,          // 0–1
    pub last_seen_tick: u64,
}

pub struct PerceivedItem {
    pub item_id: u64,
    pub name: String,
    pub usefulness: f32,            // 0–1 (relevance to current needs)
    pub distance: f32,
}

pub struct SensoryRanges {
    pub vision_range: f32,
    pub hearing_range: f32,
    pub smell_range: f32,
}

pub struct BrainComponent {
    pub personality_traits: PersonalityTrait,
    pub beliefs: Vec<Belief>,
    pub intentions: Vec<Intention>,
}

pub struct PersonalityTrait {
    pub aggression: f32,        // 0–1 (tendency to fight)
    pub curiosity: f32,         // 0–1 (tendency to explore)
    pub sociability: f32,       // 0–1 (tendency to group)
    pub caution: f32,           // 0–1 (risk aversion)
    pub empathy: f32,           // 0–1 (care for others)
}

pub struct Belief {
    pub subject: String,
    pub claim: String,
    pub confidence: f32,        // 0–1
    pub formed_tick: u64,
}

pub struct Intention {
    pub goal: String,
    pub priority: PriorityTier,
    pub target: Option<u64>,    // Entity ID if relevant
    pub expiry_tick: u64,
}

pub struct ReproductionComponent {
    pub reproduction_need: f32,          // 0–100
    pub fertile: bool,
    pub pregnancy_stage: Option<u32>,   // Ticks into gestation
    pub partner_id: Option<u64>,
}

pub struct RelationshipComponent {
    pub family_relations: HashMap<u64, Relationship>,    // Entity ID → Relationship
    pub friendships: HashMap<u64, Relationship>,
    pub romantic_relations: HashMap<u64, Relationship>,
}

pub struct Relationship {
    pub relation_type: String,          // "parent", "sibling", "child", etc.
    pub affinity_score: f32,            // -100 to +100
    pub history: Vec<RelationshipEvent>,
}

pub struct RelationshipEvent {
    pub tick: u64,
    pub event: String,
    pub impact: f32,  // How much it changed affinity
}

pub struct MetabolismComponent {
    pub base_metabolism_rate: f32,
    pub current_activity_multiplier: f32,
}

pub struct VisionComponent {
    pub brightness_perception: f32,
    pub color_perception: Vec<(String, f32)>,  // Color → sensitivity
}

pub struct EquipmentComponent {
    pub equipped_items: EquipmentSlots,
    pub encumbrance_level: f32,  // 0–2 (0=none, 1=100%, 2=150%+)
}

pub struct DatatypeComponent {
    pub data_type: String,
    pub value: String,
}
```

---

## Relationship System

```rust
pub enum RelationType {
    // Family
    Parent,
    Child,
    Sibling,
    Grandparent,
    Grandchild,
    Cousin,
    // Social
    Friend,
    Acquaintance,
    Colleague,
    Neighbor,
    // Romantic
    Lover,
    Partner,
    Crush,
    ExLover,
    ExPartner,
    Soulmate,
    // Negative
    Enemy,
    Rival,
    Traitor,
    // Group
    Ally,
    Foe,
}

// Affinity ranges from –100 (mortal enemy) to +100 (soulmate)
// 0 = neutral/unknown
pub fn relationship_description(affinity: f32) -> &'static str {
    match affinity {
        -100..=-75 => "mortal enemy",
        -74..=-50 => "enemy",
        -49..=-25 => "disliked",
        -24..=0 => "neutral",
        1..=25 => "acquaintance",
        26..=50 => "friend",
        51..=75 => "close friend",
        76..=100 => "soulmate",
        _ => "unknown",
    }
}
```

---

## Tag System

Tags enable flexible categorization without hardcoding filters.

```rust
pub enum EntityTag {
    // Being type
    Ai,
    Mortal,
    Immortal,
    Ghost,
    // Capability
    Flying,
    Swimming,
    Climbing,
    Burrowing,
    // Behavior
    Aggressive,
    Docile,
    Territorial,
    Migratory,
    // Group
    Predator,
    Prey,
    Pack,
    Solitary,
    // Social
    Civilized,
    Tribal,
    Feral,
    // Special
    Cursed,
    Blessed,
    Diseased,
    Immune,
    Custom(String),
}
```

---

## Summary Table: Entity Composition

| Component | Category | Purpose | Example |
|-----------|----------|---------|---------|
| **StatsComponent** | Attributes | Capability foundation | Might 60, Lithe 55 |
| **HealthComponent** | State | Physical vitality | 85/100 HP, 70/100 energy |
| **NeedsComponent** | State | Drive satisfaction | Hunger 40%, Thirst 60% |
| **InventoryComponent** | Capability | Item carrying | 23/50 kg |
| **MemoryComponent** | State | Experience recording | "Found river" (long-term) |
| **PerceptionComponent** | State | Sensory input | Sees 30m, hears 50m |
| **BrainComponent** | Behavior | Decision-making | Beliefs, intentions, personality |
| **RelationshipComponent** | State | Social bonds | 12 relationships tracked |
| **ReproductionComponent** | State | Breeding | Need 75%, fertile=true |
| **TransformComponent** | State | Position | (100, 200, 0) |
| **VelocityComponent** | State | Movement | Heading NE, speed 2.5 |
| **SpeciesComponent** | Attributes | Biology | Human, Adult, age 8500 ticks |

---

## Next Document Dependencies

- ✅ **data-schema.md** (this file) — defines all types
- ⏳ **systems-math.md** — will reference these types, define formulas
- ⏳ **time-system.md** — will reference tick mechanics, calendar
- ⏳ **affordance-system.md** — will reference Affordance struct, detection logic
