# 🔗 Interaction Engine

The interaction engine is the unified system that resolves all interactions between entities and entities, entities and environment, or entities and objects. It operates on a simple but powerful model: Source + Target → Rule → Effect.

---

## Core Interaction Model

```
Source + Target → Rule → Effect
```

Where:

* **Source**: The acting entity (initiates interaction)
* **Target**: The affected entity/object (receives interaction)
* **Rule**: System logic determining outcome
* **Effect**: Consequence(s) applied to target (and sometimes source)

---

## Interaction Types

### Consumption

**Source** consumes **Target** (food, water, resources).

**Rule**:
* Source must have Hunger/Thirst/Need matching Target type
* Target must be edible/consumable
* Outcome: Target removed; Source need satisfied

**Effects**:
* Source: Hunger/Thirst reduced, Health +/– (based on nutrition), Fulfillment +/– (based on preference)
* Target: Destroyed/consumed

**Example**: Herbivore eats fruit
```
Herbivore (source) + Apple (target) → Consumption rule → Herbivore loses 20% hunger, Apple destroyed
```

---

### Combat

**Source** attacks **Target** (combat interaction).

**Rule**:
* Source must have Might/combat skill ≥ threshold
* Target must be hostile or perceived as hostile
* Outcome: Damage calculated; defense checked

**Combat Formula**:
```
Damage = (Attacker_Might × Skill_bonus) – (Defender_Defense × Armor_bonus) ± Random(–10, +10)
```

**Effects**:
* Target: Health decreased by Damage
* Source: Hunger/comfort cost + reputation effects (if witnessed)
* Target (if alive): Intent to flee or counter-attack

**Example**: Predator attacks Prey
```
Predator (source) + Prey (target) → Combat rule → Prey takes 15 damage, flees; Predator gains hunting satisfaction
```

---

### Growth

**Source** grows or is affected by **Target** (environmental growth, development).

**Rule**:
* Source must be flora or developmental stage entity
* Target must be environmental factor (sunlight, water, soil quality)
* Outcome: Growth rate affected

**Growth Formula**:
```
GrowthRate = BaseGrowth × Sunlight × Water × Soil_Quality
```

**Effects**:
* Source: Size/age increases; traits may develop
* Ecosystem: Resource availability shifts

**Example**: Fruit tree grows in spring
```
Apple_Tree (source) + Spring_Sunlight (target) + Rain (target) → Growth rule → Tree grows 5% per day, fruit production planned for summer
```

---

### Decay

**Source** ages and decays (entropic process).

**Rule**:
* All entities experience decay over time (aging, degradation)
* Environment accelerates decay (rust, rot, weathering)
* Outcome: Health or durability decreases

**Decay Formula**:
```
DecayRate = BaseDecay + Weather_Effect + Use_Damage + Age_Factor
```

**Effects**:
* Source: Health/durability decreased; may become unusable
* Ecosystem: Dead material becomes resource (scavenging, fertilizer)

**Example**: Metal sword rusts
```
Iron_Sword (source) + Humid_Weather (target) → Decay rule → Sword durability –0.1 per day (rust accumulation)
```

---

### Transfer

**Source** transfers **Target** (ownership, energy, property change).

**Rule**:
* Source must have item/energy to transfer
* Target must be capable recipient
* Outcome: Ownership or state changes

**Transfer Types**:

* **Item Transfer**: "Give apple to friend"
  ```
  Source (entity) + Item + Target (entity) → Transfer rule → Target gains item, Source loses item
  ```

* **Energy Transfer**: "Warm yourself by fire"
  ```
  Cold_Entity (source) + Fire (target) → Transfer rule → Entity gains warmth, Fire loses fuel
  ```

* **Knowledge Transfer**: "Teach a skill"
  ```
  Expert (source) + Apprentice (target) → Transfer rule → Apprentice gains skill exp, Expert satisfaction +
  ```

---

### Extraction

**Source** extracts resource from **Target** (mining, harvesting, hunting).

**Rule**:
* Source must have tool/skill appropriate to Target
* Target must have extractable resources
* Outcome: Resource removed from target, added to source

**Extraction Formula**:
```
ResourceYield = Base_Yield × Skill_Multiplier × Tool_Quality × Depletion_Factor
```

**Effects**:
* Target: Resource reduced; may regrow/respawn
* Source: Inventory increased; skill practiced
* Environment: Local resource concentration shifts

**Example**: Miner extracts iron ore
```
Miner (source, skill=50) + Iron_Ore_Deposit (target) + Pickaxe (tool) → Extraction rule → Miner gains 3 iron ore, deposit depletes slightly
```

---

### Social

**Source** interacts socially with **Target** (conversation, alliance, conflict).

**Rule**:
* Source and Target must be sentient
* Interaction type determined by personality + relationship history
* Outcome: Relationship score shifts; reputation affected

**Social Interaction Types**:

* **Dialogue**: Exchange information, build relationship
* **Trade**: Exchange goods, economic interaction
* **Alliance**: Form bond, coordinate actions
* **Conflict**: Dispute, compete, oppose
* **Courtship**: Reproductive/mating interaction
* **Domination**: Establish hierarchy

**Social Effects**:
* Both entities: Relationship score changes (–100 to +100 spectrum)
* Reputation: Modified based on context (generous trade increases reputation; unfair deal decreases)
* Memory: Interaction recorded; influences future decisions
* Belief: If interaction contradicts belief, belief updates (slowly, biased by personality)

**Formula (simplified)**:
```
RelationshipChange = Base_Interaction_Value × Personality_Modifiers × Expectation_Match
```

---

## Interaction Resolution Process

### Step 1: Check Preconditions

* Is Source capable? (Has stats/items/skills)
* Is Target valid? (Exists, accessible, appropriate)
* Is context appropriate? (Location, time, social state)

### Step 2: Determine Interaction Type

* Based on Source intention + Target properties

### Step 3: Calculate Outcome

* Apply relevant formula
* Generate success/failure roll (d20 + modifiers)
* Determine effectiveness (crit success, success, fail, crit fail)

### Step 4: Apply Effects

* Modify Source state (energy cost, skill gain, reputation)
* Modify Target state (health, resources, ownership)
* Update environment (resource depletion, state change)
* Trigger chained interactions (death → scavenging, trade → economy shift)

### Step 5: Record Memory & Learning

* Source records interaction in memory
* Both entities update beliefs based on outcome
* Reputation systems updated
* Skills practiced/improved

---

## Interaction Chaining

Interactions often trigger further interactions:

```
Combat Victory → Target Death → Scavenging → Source inventory ↑ → Trade possible → Economic shift → Settlement population change
```

---

## Context Modulation

**All interactions are context-sensitive**:

* **Time**: Night combat has different modifier than day
* **Weather**: Hunting harder in rain; cooking easier with fire in dry weather
* **Location**: Combat in settlement penalized (safety rules); more effective in wilderness
* **Relationship**: Trade with ally more favorable; trade with enemy more costly
* **Population**: Rare item more valuable when rare; common item worthless when abundant

---

## Integration with Behavior

Interactions are the **action layer** of the behavior pipeline:

```
Reality → Perception → Memory → Belief → Intent → (Attention) → Decision → Affordance Selection → Interaction Execution
```

The interaction engine is where intent becomes consequence.

---

## Summary

The interaction engine unifies all game mechanics through the simple **Source + Target → Rule → Effect** model. This creates:

* **Emergence**: Complex outcomes from simple rules
* **Consistency**: All interactions resolve through same system
* **Balance**: Context modulation prevents domination
* **Learning**: Memory integration creates learning
