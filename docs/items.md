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
