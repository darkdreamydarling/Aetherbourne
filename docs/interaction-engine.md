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

## ⚠️ Interaction Priority System (Conflict Resolution)

When multiple interactions are triggered simultaneously on the same entity, priority determines which executes.

### Priority Tiers

```rust
pub enum InteractionPriority {
    Critical,   // Life-threatening: flee, heal critical, immediate threat
    High,       // Important objectives: eat, hunt, reproduce
    Normal,     // Routine: move, craft, explore
    Low,        // Idle: rest, chat, experiment
}
```

### Conflict Resolution Algorithm

When **multiple interactions** queue for the same entity:

```
1. Group by priority tier
2. Within same tier, sort by salience (how urgent?)
3. Execute highest priority; queue others
4. Each interaction has InterruptionChance - can be interrupted by higher-priority

Example:
  Entity receives: [Attack(Critical), Rest(Low), Trade(Normal)]
  Order executed: Attack (Critical) → blocked Rest → Re-evaluate Trade vs next queued action
```

**Tie-breaker (same priority):**
```
If multiple interactions at same priority:
  → Higher salience wins
  → If salience equal → tie broken by entity personality (aggressive → combat, social → trade)
```

---

## 💰 Interaction Cost System

Every interaction has energy, time, and risk costs.

### Cost Definition

```rust
pub struct InteractionCost {
    pub energy: f32,           // Energy points consumed
    pub time_ticks: u32,       // Number of simulation ticks required
    pub risk_factor: f32,      // Chance of complication (0–1)
}

pub fn calculate_actual_cost(
    base_cost: InteractionCost,
    entity: &Entity,
    context: &InteractionContext,
) -> InteractionCost {
    // Energy cost modified by Vigor stat
    let actual_energy = base_cost.energy * (1.0 - (entity.stats.vigor / 200.0));
    
    // Time cost modified by stat → speed formula (from systems-math.md)
    let stat_multiplier = calculate_speed_multiplier(entity);
    let actual_time = (base_cost.time_ticks as f32 * stat_multiplier) as u32;
    
    // Risk modified by environment
    let actual_risk = base_cost.risk_factor * calculate_context_risk_multiplier(context);
    
    InteractionCost {
        energy: actual_energy,
        time_ticks: actual_time,
        risk_factor: actual_risk,
    }
}
```

### Cost Matrix: Interaction Type & Entity State

| Interaction | Base Energy | Base Time | Base Risk | Examples |
|---|---|---|---|
| **Consumption** | 1 | 60 | 0.0 | Eat, drink, fuel |
| **Combat** | 15 | 600 | 0.3 | Attack, defend, dodge |
| **Extraction** | 10 | 1200 | 0.1 | Mine, harvest, hunt |
| **Crafting** | 5 | 3600+ | 0.05 | Build, assemble, refine |
| **Transfer** | 2 | 300 | 0.0 | Give, trade, teach |
| **Growth** | 0 | varies | 0.0 | Plant growth (passive) |
| **Social** | 3 | 300–600 | 0.1 | Speak, trade, bond |
| **Environmental** | 5 | 300 | 0.2 | Move, climb, swim |

**Modifiers on Cost:**
- Fatigue: Energy cost ×1.5 if fatigue > 70
- Encumbrance: Time cost ×1.3 if encumbered
- Injury: Effectiveness reduced 10% per 10 health points missing
- Darkness: Perception-based interactions harder (×1.5 difficulty)

---

## ✅ Success/Failure Outcomes

Every interaction resolves to one of four outcomes, each with explicit effects.

### Four Outcome Types

```rust
pub enum InteractionOutcome {
    CriticalSuccess,    // 150% effect, possible bonus
    Success,            // 100% effect (normal)
    Failure,            // 75% effect, potential consequence
    CriticalFailure,    // 50% effect, negative consequence
}

pub fn roll_outcome(source: &Entity, interaction: &Interaction) -> InteractionOutcome {
    // d20 + skill modifiers
    let roll = roll_d20();
    let skill_bonus = calculate_skill_bonus(&source, &interaction);
    let total = roll + skill_bonus;
    
    // Critical ranges (always override)
    if roll == 20 { return InteractionOutcome::CriticalSuccess; }
    if roll == 1 { return InteractionOutcome::CriticalFailure; }
    
    // Normal vs Failure
    let dc = interaction.difficulty_class.unwrap_or(12);
    if total >= dc {
        InteractionOutcome::Success
    } else {
        InteractionOutcome::Failure
    }
}
```

### Outcome-Specific Effects

**For each interaction type, define effects per outcome:**

#### Consumption (Eat, Drink)
```
CriticalSuccess:  150% hunger reduction, +5 max health (savory meal!)
Success:          100% hunger reduction as designed
Failure:          75% hunger reduction (tasteless)
CriticalFailure:  50% hunger reduction, +10% poison damage (bad meal!)
```

#### Combat (Attack)
```
CriticalSuccess:  150% damage, target knocked back 1 tile
Success:          100% damage as calculated
Failure:          75% damage, attacker balance loss (–10% defense this round)
CriticalFailure:  50% damage, weapon durability loss ×2, attacker stumbles (−1 speed)
```

#### Extraction (Mining, Hunting)
```
CriticalSuccess:  250% resource yield (amazing haul!)
Success:          100% yield as planned
Failure:          50% yield (mediocre haul, time wasted)
CriticalFailure:  0% yield, tool durability –10, target hostile/flees (predator chance)
```

#### Social (Trade, Speak)
```
CriticalSuccess:  +20 affinity, trade favorable, information gained (+skill exp)
Success:          +10 affinity, fair trade, basic info
Failure:          –5 affinity, unfavorable trade, info withheld
CriticalFailure:  –15 affinity, hostile reaction, attacked if aggressive entity
```

#### Crafting
```
CriticalSuccess:  Item quality 150%, bonus mod granted (legendary roll)
Success:          Item quality 100% as designed
Failure:          Item quality 50%, but still usable
CriticalFailure:  Item creation fails, resources partially consumed (50% waste)
```

---

## 📏 Range & Distance Validation

Interactions are not instantaneous—distance matters.

### Range Types & Validation

```rust
pub enum InteractionRange {
    Adjacent,   // 1 tile (melee, touch)
    Short,      // 5 tiles (close conversation)
    Medium,     // Visual range (~ 30 tiles for humans)
    Long,       // Ranged combat (sight range)
}

pub fn validate_interaction_distance(
    source: &Entity,
    target: &Entity,
    range_type: InteractionRange,
) -> bool {
    let distance = distance_to(source.position, target.position);
    
    match range_type {
        InteractionRange::Adjacent => distance <= 1.0,
        InteractionRange::Short => distance <= 5.0,
        InteractionRange::Medium => distance <= 30.0,
        InteractionRange::Long => {
            // Use actual vision range (from systems-math.md)
            calculate_vision_range(source.stats.vigil) >= distance
        }
    }
}
```

### Range Penalties

If performing interaction near max range:

```
RangePenalty = (Distance / MaxRange) × 0.5

Example:
  Ranged combat at 28/30 tiles
  Penalty = (28 / 30) × 0.5 = 0.47
  Hit chance reduced by 47%, damage reduced by 20%
```

---

## ⏱️ Cooldown & Interrupt Systems

Prevent action spam and allow tactical interruption mid-action.

### Cooldown System

After executing an interaction, the source has a cooldown before repeating the same interaction type:

```rust
pub struct InteractionCooldown {
    pub interaction_type: InteractionType,
    pub cooldown_ticks: u32,
    pub can_use_at_tick: u64,
}

pub fn apply_cooldown(
    entity: &mut Entity,
    interaction: &Interaction,
) {
    let base_cooldown = match interaction.interaction_type {
        InteractionType::Combat => 300,      // 5 seconds between swings
        InteractionType::Extraction => 600,  // 10 seconds between mining strokes
        InteractionType::Consumption => 60,  // 1 second between bites
        InteractionType::Social => 600,      // 10 seconds between dialogue exchanges
        _ => 0,                              // No cooldown
    };
    
    // Skill reduces cooldown
    let stat_multiplier = calculate_speed_multiplier(entity);
    let final_cooldown = (base_cooldown as f32 * stat_multiplier) as u32;
    
    entity.cooldowns.push(InteractionCooldown {
        interaction_type: interaction.interaction_type.clone(),
        cooldown_ticks: final_cooldown,
        can_use_at_tick: current_tick + final_cooldown,
    });
}
```

### Interrupt System

If entity takes damage or triggered panic while mid-action:

```rust
pub enum InterruptTrigger {
    DamageTaken,
    HighThreat,
    CriticalNeed,
    Spell,
}

pub fn check_interruption(entity: &Entity, trigger: InterruptTrigger, current_action: &Interaction) -> bool {
    let interrupt_chance = match trigger {
        InterruptTrigger::DamageTaken => 0.5 + (damage_taken / entity.health.max) * 0.5,  // More damage = higher chance
        InterruptTrigger::HighThreat => 0.7,
        InterruptTrigger::CriticalNeed => 0.9,  // Immediate need always interrupts
        InterruptTrigger::Spell => 0.6,
    };
    
    // Focus stat resists interruption
    let focus_resistance = (entity.stats.vigil / 100.0) * 0.2;  // 0–20% reduction
    let actual_chance = (interrupt_chance - focus_resistance).max(0.0);
    
    random() < actual_chance
}

pub fn handle_interruption(
    entity: &mut Entity,
    current_action: &Interaction,
) {
    // Interrupted action is aborted
    entity.current_action = None;
    
    // Partial energy/time cost still applied
    entity.energy.current -= current_action.cost.energy * 0.3;
    
    // Generate new action (likely defensive)
    // or continue if focus was strong
}
```

---

## 🔗 Chaining Rules (Expanded)

Interactions can trigger secondary interactions automatically.

### Standard Chains

```
Death → Corpse Scavenging (predators/desperate entities)
Corpse → Decomposition → Fertilizer (environmental feedback)
Trade Success → Reputation Change → Market Price Shift
Reproduction → Offspring Birth → Population Growth
Combat Injury → Infection Risk → Health Crisis
```

### Chain Execution Rules

```rust
pub fn execute_interaction_chain(
    source: &Entity,
    interaction: &Interaction,
    outcome: &InteractionOutcome,
) -> Vec<Interaction> {
    let mut chain = Vec::new();
    
    match interaction.interaction_type {
        InteractionType::Combat => {
            if outcome == InteractionOutcome::Success || outcome == InteractionOutcome::CriticalSuccess {
                // Combat success → damage dealt → injury
                chain.push(create_injury_interaction(target, damage_amount));
                
                // Large damage → can chain to death
                if target.health <= 0 {
                    chain.push(create_death_interaction(target));
                    // Death → corpse scavenging available
                    chain.push(create_scavenging_affordance(target.position));
                }
            }
        }
        InteractionType::Social if interaction.name == "Trade" => {
            if outcome == InteractionOutcome::Success {
                // Trade success → relationship change
                chain.push(create_relationship_update(source, target, +10));
                
                // Trade → economy shift (price changes)
                chain.push(create_market_update(source.biome, interaction.traded_goods()));
            }
        }
        InteractionType::Reproduction => {
            if outcome >= InteractionOutcome::Success {
                // Reproduction → offspring birth
                chain.push(create_offspring_interaction(source, target));
                // Offspring → population increase
                chain.push(create_population_census_update(source.settlement_id));
            }
        }
        _ => {}
    }
    
    chain
}
```

### Chain Delay

Chained interactions don't execute immediately:

```
Immediate: Death → Corpse appears (same tick)
Delayed: Corpse → Scavenging opportunity (available next perception cycle)
Very Delayed: Trade → Market prices (update on next economy cycle)
```

---

## 👥 Social Deception & Trust System (Expanded)

Social interactions involve complex mental modeling.

### Trust Factor

```rust
pub struct TrustAssessment {
    pub base_trust: f32,        // Relationship affinity (-100 to +100)
    pub personality_bias: f32,  // How trusting is this entity by nature?
    pub recent_history: f32,    // Recent honesty/betrayal (-50 to +50)
    pub group_alignment: f32,   // Do we share values/faction?
}

pub fn calculate_trust(
    evaluator: &Entity,
    speaker: &Entity,
    context: &InteractionContext,
) -> f32 {
    let relationship = get_relationship_affinity(evaluator, speaker);
    let base_trust = (relationship / 100.0) * 0.5;  // Relationship is 50% of trust
    
    let personality_bonus = (evaluator.personality.empathy / 100.0) * 0.2;  // Empathy is 20%
    
    let history_bonus = (evaluator.recent_positive_trades / 10.0).min(0.2);  // Recent honesty is 20%
    
    let alignment_bonus = if shares_faction(evaluator, speaker) {
        0.1  // Shared faction = 10% bonus
    } else {
        -0.1  // Different faction = 10% penalty
    };
    
    (base_trust + personality_bonus + history_bonus + alignment_bonus).clamp(-1.0, 1.0)
}
```

### Deception Mechanics

When speaker attempts to **deceive**:

```rust
pub fn evaluate_deception(
    speaker: &Entity,
    listener: &Entity,
    claim: &str,
) -> DeceptionOutcome {
    // Speaker's deception skill vs listener's insight
    let deception_roll = roll_d20() + (speaker.skills.deception / 20) as i32;
    let insight_roll = roll_d20() + (listener.stats.insight / 20) as i32;
    
    // Trust modifier
    let trust = calculate_trust(listener, speaker, &context);
    let insight_bonus = (trust * 100.0) as i32;  // Higher trust = harder to deceive
    
    let total_deception = deception_roll - insight_bonus;
    
    if total_deception > insight_roll {
        DeceptionOutcome::Success  // Listener believes the lie
    } else {
        DeceptionOutcome::Detected  // Listener spots inconsistency
    }
}

pub enum DeceptionOutcome {
    Success,      // Deception believed
    Detected,     // Deception noticed → trust –20, possible conflict
    Backfire,     // Lie contradicts known fact → trust –50, reputation damage
}
```

### Emotional State Weighting

Entity emotional state affects interaction outcomes:

```
entity.emotion: Fear | Anger | Sadness | Joy | Neutral

During Fear → Persuasion DC +3, but Intimidation DC –2
During Anger → Persuasion DC +4, Intimidation DC –1, Combat DC +1
During Joy → Trade favorable, Persuasion DC –2
```

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
