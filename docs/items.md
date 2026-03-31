# 🛠️ Crafting & Item System (Unified)

A unified system where **resources, materials, crafting, and items** form a continuous lifecycle.

## Full Pipeline

**↓ World Resources → Materials → Crafting → Items → Usage → Degradation → Recycling ↓**

* Resources define what exists
* Crafting defines transformation
* Items define gameplay impact
* Degradation creates feedback loops

---

## Item System Overview

Items are **all interactable objects** that agents can carry, use, trade, or manipulate.

They are central to:

* Survival
* Crafting
* Combat
* Economy
* Social systems

Every crafted object becomes an **item with properties, behavior, and lifecycle**.

---

## Item Categories

### Tools & Equipment

Used to interact with the world and enable crafting.

* Primitive tools (stone, wood)
* Advanced tools (metal, precision)
* Containers (bags, barrels, chests)
* Light sources (torches, lanterns)

**Integration:** Tools act as **crafting requirements** and influence output quality.

### Weapons & Armor

Used for combat and defense.

* Melee weapons (swords, axes, clubs)
* Ranged weapons (bows, slings)
* Armor (clothing, shields, helmets)
* Ammunition (arrows, bolts, thrown items)

**Integration:** Material properties directly affect damage, durability, weight, special effects.

### Consumables

Items that are used up.

* Food and drink
* Medicine and potions
* Magical items (scrolls, runes)
* Fuel sources

**Integration:** Influenced by resource tags (edible, poisonous, medicinal) and environmental modifiers.

### Materials & Resources

Items that feed back into crafting.

* Raw materials (wood, ore, fiber)
* Processed materials (ingots, cloth)
* Rare components (crystals, essence)
* Construction materials (brick, glass)

**Integration:** Bridge layer between gathering and crafting.

### Decorative & Social Items

Non-essential but impactful.

* Jewelry
* Art objects
* Currency
* Gifts

**Integration:** Ties into social systems, reputation, economy.

---

## Item Properties

### Physical Properties

* **Weight** → affects carrying capacity
* **Size** → affects storage and visibility
* **Durability** → how long it lasts
* **Value** → economic worth

Influenced by:
* Material
* Crafting quality
* Item type

### Functional Properties

* **Usability** → what actions it enables
* **Effectiveness** → performance strength
* **Requirements** → skills or conditions needed
* **Side Effects** → unintended outcomes

*Example:* A cursed item (tag-based) may harm the user; a brittle weapon deals high damage but breaks quickly.

### Social Properties

* Faction markers
* Status symbols
* Cultural significance
* Trade value variation

Items exist in **society and narrative**, not just mechanics.

---

## 📊 Item Data Model (Canonical)

Every item is defined by this structure (Rust template):

```rust
pub struct Item {
    pub id: u64,                    // Unique identifier
    pub name: String,               // Display name
    pub category: ItemCategory,     // Type classification
    pub materials: Vec<(String, u32)>,  // List of (material_name, quantity)
    pub quality: f32,               // 0–100 (affects all properties)
    pub durability: f32,            // 0–100 (100 pristine, 0 broken)
    pub created_tick: u64,          // When created
    pub owner_id: Option<u64>,      // Current owner or None if dropped
    pub tags: HashSet<String>,      // Behavioral tags
    pub modifier: Option<ItemModifier>,  // Rare/special modifiers
}

pub enum ItemCategory {
    Tool,
    Weapon,
    Armor,
    Consumable,
    Material,
    Container,
    Decoration,
    Currency,
}

pub struct ItemProperties {
    pub weight: f32,                // kg
    pub size: f32,                  // volume
    pub value_base: f32,            // Base trade value
    pub effectiveness: f32,         // 0–100 (tool strength, damage scaling)
    pub durability_loss_rate: f32,  // % per use
    pub stat_modifier: Option<StatModifier>,  // If equipment
}
```

This structure is the single source of truth for items throughout the system.

---

## 📈 Material Influence on Item Properties

Materials fundamentally shape item properties. Every item inherits characteristics from its constituent materials.

### Material Properties

```rust
pub struct MaterialProperties {
    pub name: String,
    pub hardness: f32,              // 0–100 (affects tool longevity)
    pub density: f32,               // Affects weight
    pub value_multiplier: f32,      // Gold 1.5×, copper 0.8×, wood 0.5×
    pub durability_multiplier: f32, // Iron 1.2× durable, glass 0.4×
    pub rust_rate: f32,             // Corrosion in moisture (0 for non-metals)
    pub heat_conductivity: f32,     // How fast it transfers heat
}

// Material definitions
const MATERIALS: &[(&str, MaterialProperties)] = &[
    ("iron", MaterialProperties {
        hardness: 85.0, density: 7.87, value_multiplier: 1.0,
        durability_multiplier: 1.2, rust_rate: 0.3, heat_conductivity: 80.0
    }),
    ("gold", MaterialProperties {
        hardness: 45.0, density: 19.3, value_multiplier: 2.0,
        durability_multiplier: 0.8, rust_rate: 0.0, heat_conductivity: 300.0
    }),
    ("wood", MaterialProperties {
        hardness: 30.0, density: 0.6, value_multiplier: 0.5,
        durability_multiplier: 1.5, rust_rate: 1.0, heat_conductivity: 0.1
    }),
    // ... more materials
];
```

### Final Property Calculation

**Formula (from systems-math.md):**

```
FinalProperty = BaseItemValue × MaterialModifier × CraftingQuality
```

**For each item property:**

```
FinalEffectiveness = BaseEffectiveness × MaterialHardness/50 × (Quality/50)
FinalDurability = BaseDurability × MaterialDurabilityMult × QualityBonus
FinalWeight = BaseWeight × MaterialDensity
FinalValue = BaseValue × MaterialValueMult × QualityMult × RarityMult
```

**Example: Iron Sword**
```
Base Effectiveness: 10
Iron Hardness: 85 → Modifier = 85/50 = 1.7×
Quality: 80 → Quality Mult = 80/50 = 1.6×

Final Effectiveness = 10 × 1.7 × 1.6 = 27.2

---

Base Durability: 100
Iron Durability Mult: 1.2×
Quality: 80

Final Durability = 100 × 1.2 × (80/100) = 96
```

---

## 🛡️ Durability System

Durability represents item condition (0–100). As items are used, durability degrades. Lower durability reduces effectiveness.

### Durability States

```rust
pub enum DurabilityState {
    Pristine,       // 100–70: Full effectiveness (1.0×)
    Worn,           // 70–30: Reduced effectiveness (0.7×)
    Damaged,        // 30–1: Severely hampered (0.4×)
    Broken,         // 0: Unusable (0.0×)
}

impl DurabilityState {
    pub fn effectiveness_multiplier(durability: f32) -> f32 {
        match durability {
            71.0..=100.0 => 1.0,      // Pristine
            31.0..=70.0 => 0.7,       // Worn
            1.0..=30.0 => 0.4,        // Damaged
            _ => 0.0,                 // Broken
        }
    }
}
```

### Durability Loss Formula

**Formula (from systems-math.md):**

```
DurabilityLoss = BaseUseWear × MaterialFragility × EnvironmentalFactor × QualityInverse

Where:
  BaseUseWear: Item-category dependent (0.5% per use for weapons, 0.2% for tools)
  MaterialFragility: 0.5 (durable) to 2.0 (fragile)
  EnvironmentalFactor: 1.0 (normal) to 3.0 (corrosive)
  QualityInverse: 100 / Quality (higher quality = slower decay)
```

**Example: Iron Sword Used in Humid Weather**
```
Base Use Wear: 0.5% per swing
Iron Fragility: 1.2
Humidity Multiplier: 1.8 (rust factor)
Quality: 85

Loss = 0.5 × 1.2 × 1.8 × (100/85) = 1.27% per swing

After 10 swings: Durability 100 → 87.3

After 100 swings: Durability 100 → ~0 (breaks)
```

### Maintenance & Repair

Items can be restored to previous condition:

```rust
pub fn repair_item(item: &mut Item, quality: f32) {
    // Repair effectiveness depends on repair skill
    let restoration_amount = 20.0 * (quality / 100.0);
    item.durability = (item.durability + restoration_amount).min(100.0);
}
```

---

## 💼 Inventory & Weight System

Entities have limited carrying capacity. Encumbrance penalizes movement and fatigue recovery.

### Carrying Capacity Formula

**Formula (from systems-math.md):**

```
CarryingCapacity = (Might + Vigor) / 2 × 10 kg

Encumbrance% = CurrentWeight / CarryingCapacity × 100

Penalties:
  0–100%: No penalty
  101–150%: Speed ×0.7, fatigue drain ×1.5
  151%+: Cannot move
```

**Example:**
```
Entity: Might 50, Vigor 60
Capacity = (50 + 60) / 2 × 10 = 550 kg

Carrying 450 kg:
  Encumbrance = 450 / 550 × 100 = 81.8%
  Effects: No penalty

Carrying 700 kg:
  Encumbrance = 700 / 550 × 100 = 127.3%
  Effects: Movement speed ×0.7, fatigue drain ×1.5
```

### Inventory Slots

Equipment has specific slots:

```rust
pub struct EquipmentSlots {
    pub head: Option<u64>,          // Helmet, crown
    pub torso: Option<u64>,         // Chestplate, robe
    pub hands: Option<u64>,         // Gloves
    pub legs: Option<u64>,          // Leg armor
    pub feet: Option<u64>,          // Boots
    pub main_hand: Option<u64>,     // Primary weapon
    pub off_hand: Option<u64>,      // Shield, offhand weapon
    pub accessories: Vec<u64>,      // Multiple rings, amulets (cap at 4)
}
```

Only one item can occupy each slot. Stat modifiers from equipped items stack (see equipment system below).

---

## 🎲 Modifier System (Procedural Generation)

Items can have rare modifiers that enhance properties.

### Rarity Tiers & Modifier Slots

```rust
pub enum Rarity {
    Common,         // 0 modifiers
    Uncommon,       // 1 modifier (prefix OR suffix)
    Rare,           // 2 modifiers (prefix + suffix)
    Legendary,      // Fixed unique modifiers (3–4)
}

pub struct ItemModifier {
    pub name: String,
    pub rarity: Rarity,
    pub stat_bonus: StatModifier,
    pub effectiveness_multiplier: f32,  // Scales effectiveness
    pub durability_penalty: f32,        // Extra wear rate
    pub value_multiplier: f32,
}

// Example modifiers
const COMMON_MODIFIERS: &[ItemModifier] = &[
    ItemModifier {
        name: "Sharp".to_string(),
        rarity: Rarity::Uncommon,
        stat_bonus: StatModifier { might: 5.0, ..Default::default() },
        effectiveness_multiplier: 1.15,
        durability_penalty: 0.1,  // Dull faster
        value_multiplier: 1.2,
    },
    ItemModifier {
        name: "Durable".to_string(),
        rarity: Rarity::Uncommon,
        stat_bonus: StatModifier { vigor: 0.0, ..Default::default() },
        effectiveness_multiplier: 1.0,
        durability_penalty: -0.3,  // Lasts longer
        value_multiplier: 1.25,
    },
];
```

### Procedural Generation Example

```
Generated Item: Iron Sword

Roll Base: Iron (material) → Sword (weapon)
Roll Rarity: 1d100 = 65 → Rare (2 modifiers)
Roll Prefix: 1d20 → "Sharp" (+Might 5, ×1.15 effectiveness)
Roll Suffix: 1d20 → "of Durability" (–0.3 wear rate, ×1.05 durability)

Final Item:
  Name: "Sharp Iron Sword of Durability"
  Effectiveness: 10 × 1.7 (iron) × 1.6 (quality 80) × 1.15 (Sharp) = 31.3
  Durability Loss: 0.5 × per swing – 0.3 (suffix) = 0.2% (more durable!)
  Value: 100 × 1.0 (iron) × 1.6 (quality) × 3.0 (rare) × 1.2 × 1.05 = 576 currency
```

---

## ⚔️ Equipment System

Equipped items modify entity stats and effectiveness.

### Equipping Items

When an item is equipped, it grants stat bonuses and modifies effectiveness:

```rust
pub fn equip_item(entity: &mut Entity, item: &Item) {
    // Find appropriate slot
    let slot = determine_slot(&item.category);
    
    // Unequip old item if slot occupied
    if let Some(old_item_id) = entity.equipment_slots.get_mut(slot) {
        entity.unequip_item(*old_item_id);
        *old_item_id = None;
    }
    
    // Apply stat modifiers
    if let Some(mod) = &item.stat_modifier {
        entity.stats.might += mod.might;
        entity.stats.lithe += mod.lithe;
        entity.stats.vigor += mod.vigor;
        // ... etc
    }
    
    // Update effectiveness scaling
    if item.category == ItemCategory::Weapon {
        entity.combat_effectiveness *= item.effectiveness / 100.0;
    } else if item.category == ItemCategory::Armor {
        entity.defense_rating += item.effectiveness;
    }
}
```

### Stat Stacking Rules

Multiple equipped items can provide stat bonuses. Bonuses **stack additively** but have a cap:

```
TotalMightBonus = Sum of all equipped item might modifiers
FinalMight = BaseMight + TotalMightBonus
MaxBonus = 20  // Can't add more than 20 points from equipment

FinalMight = (BaseMight + TotalMightBonus).min(BaseMight + 20)
```

### Durability Effect on Stats

As items degrade, their bonus strength decreases:

```
AdjustedStatBonus = Item.StatBonus × DurabilityMultiplier
DurabilityMultiplier = DurabilityState.effectiveness_multiplier()

Example:
  Armor grants +10 Vigor
  Armor durability: Worn (70%) → Worn state = 0.7× effectiveness
  Adjusted Bonus = 10 × 0.7 = +7 Vigor (weakened)
```

---

## Item Lifecycle

### Creation

Items are created through:

1. **Gathering** → raw resources
2. **Processing** → intermediate materials
3. **Assembly** → crafting system
4. **Enhancement** → quality or enchantment

### Use & Degradation

* Items activated through actions
* Effects applied (damage, healing, etc.)
* Wear accumulates over time
* Maintenance restores condition

Ties into:
* Durability system
* Environmental effects
* Material traits

### Destruction & Recycling

Items transform, not vanish:

* Breakage → damaged or ruined state
* Consumption → item used up
* Loss → dropped, stolen, destroyed
* Recycling → yields materials

Reinforces **closed-loop crafting economy**.

---

## Inventory & Item Management

### Inventory System

* Personal inventory (carried items)
* Shared storage (settlements)
* Equipment slots (worn/active items)
* Quick access (hotbar-style usage)

### Trading System

* Barter-based exchange
* Dynamic market values
* Cultural preferences
* Illegal/black market systems

**Integration:** Item value and rarity directly affect trade.

---

## Crafting Integration

### Recipe Requirements

Items act as:

* Ingredients
* Tools
* Catalysts

### Tool Dependency

Certain crafting requires tools:

* Hammer for metalwork
* Loom for textiles
* Alchemy setup for potions

Tool quality affects output.

### Material Quality Influence

Input materials determine:

* Output durability
* Performance
* Special traits

### Batch Processing

Players can produce multiple items depending on:

* Resources available
* Skill level
* Station capability

---

## Advanced Item Features

### Procedural Generation

Items can gain:

* Random modifiers
* Unique traits
* Material-based variations

### Unique & Legendary Items

* Fixed identity
* Special abilities
* Lore & history

### Environmental Adaptation

Items may:

* Change in different climates
* React to weather
* Gain/lose properties

### Aging System

Items evolve over time:

* Improve (seasoned tools)
* Degrade (rust, decay)
* Gain historical value

---

## Social & Cultural Systems

### Heirloom System

Items can:

* Be passed down
* Gain value over time
* Carry history

### Reputation Items

Certain items affect:

* Status
* Trust
* Influence

### Gift Economy

Items can:

* Build relationships
* Repair alliances
* Trigger events

### Ritual & Cultural Use

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
