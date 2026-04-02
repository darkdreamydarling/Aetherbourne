# Aetherbourne Cognitive Engine Specification

This document provides the explicit mathematical formulas, data flows, and numerical constants required to implement the core cognitive loop of Aetherbourne. It translates the conceptual design into concrete Rust-ready logic.

## 1. The Intent Function `f`

The Intent function `f(Needs + Personality + Memory + Beliefs)` is responsible for generating the entity's current primary goal. It operates by calculating a **Utility Score** for every possible intention and selecting the highest one.

### 1.1. Intention Generation Pipeline

Every tick (or every `N` ticks based on the cognitive update cycle), the entity evaluates potential intentions.

**Step 1: Need-Driven Intents**
For every `NeedType` where the current value is greater than `0`:

```rust
Base_Need_Utility = (Need_Value / 100.0) * NEED_WEIGHT_BASE
```

**Step 2: Personality Bias on Needs**
Personality traits act as multipliers on specific needs, reflecting how an entity prioritizes them.

*   **Survival Needs (Hunger, Thirst, Fatigue, Warmth):**
    `Utility = Base_Need_Utility * (1.0 + (Caution * CAUTION_SURVIVAL_MULTIPLIER))`
*   **Social Need:**
    `Utility = Base_Need_Utility * (1.0 + (Sociability * SOCIABILITY_SOCIAL_MULTIPLIER))`
*   **Comfort/Stress Needs:**
    `Utility = Base_Need_Utility * (1.0 - (Grit / 100.0) * GRIT_STRESS_REDUCTION)`

**Step 3: Memory and Belief Influence**
If the entity has a relevant memory or belief associated with satisfying a specific need (e.g., a memory of a rich food source when hungry), it boosts the utility of that intent.

*   **Memory Boost:** If a relevant memory exists:
    `Utility += Memory.emotional_weight * MEMORY_INTENT_IMPACT`
*   **Belief Boost:** If a relevant belief exists:
    `Utility += Belief.confidence * BELIEF_INTENT_IMPACT`

**Step 4: Personality-Driven Intents (Idle/Exploration)**
If needs are low, personality drives behavior.

*   **Exploration Intent:**
    If `Curiosity > CURIOSITY_EXPLORE_THRESHOLD`:
    `Explore_Utility = Curiosity * EXPLORE_BASE_UTILITY`
*   **Socialize Intent (Idle):**
    If `Sociability > SOCIABILITY_IDLE_THRESHOLD`:
    `Socialize_Utility = Sociability * SOCIALIZE_BASE_UTILITY`

**Step 5: Selection**
The intention with the highest final `Utility` score becomes the active `Intention`.

### 1.2. Intent Constants

| Constant Name | Value | Description |
| :--- | :--- | :--- |
| `NEED_WEIGHT_BASE` | `100.0` | The maximum base utility a need can generate (at 100/100 severity). |
| `CAUTION_SURVIVAL_MULTIPLIER` | `0.5` | How much Caution amplifies survival needs (e.g., Caution 1.0 = 50% boost). |
| `SOCIABILITY_SOCIAL_MULTIPLIER` | `0.8` | How much Sociability amplifies the Social need. |
| `GRIT_STRESS_REDUCTION` | `0.4` | How much Grit reduces the urgency of Comfort/Stress needs. |
| `MEMORY_INTENT_IMPACT` | `20.0` | Max utility bonus from a highly emotional relevant memory. |
| `BELIEF_INTENT_IMPACT` | `30.0` | Max utility bonus from a highly confident relevant belief. |
| `CURIOSITY_EXPLORE_THRESHOLD` | `0.4` | Minimum Curiosity required to generate an Explore intent. |
| `EXPLORE_BASE_UTILITY` | `40.0` | Base utility for exploration, scaled by Curiosity. |
| `SOCIABILITY_IDLE_THRESHOLD` | `0.5` | Minimum Sociability to seek idle socialization. |
| `SOCIALIZE_BASE_UTILITY` | `35.0` | Base utility for idle socialization, scaled by Sociability. |


## 2. Operationalizing Personality and Attention

### 2.1. Personality: The Waterfall Approach

The "waterfall approach" to personality models a hierarchy of influence, where stable core traits inform dynamic learned dispositions, which in turn shape transient emotional states. This provides a layered, realistic behavioral model.

**Data Structures:**

```rust
pub struct PersonalityComponent {
    pub core_traits: PersonalityTrait, // Immutable after creation, 0-1 range
    pub learned_dispositions: HashMap<String, f32>, // e.g., "generosity": 0.7, "biome_fear_forest": 0.9
    pub emotional_states: EmotionalStateBlock, // Transient, 0-100 range
}

pub struct EmotionalStateBlock {
    pub happiness: f32, // 0-100
    pub fear: f32,      // 0-100
    pub anger: f32,     // 0-100
    pub stress: f32,    // 0-100 (overlaps with NeedType::Stress, consider merging or clarifying)
}
```

**Flow and Influence:**

1.  **Core Traits:** These are foundational. They set initial values for `learned_dispositions` and `emotional_states` at entity creation. They also act as **multipliers or biases** in `Intent Generation` and `Decision Scoring` (as seen in Section 1 and upcoming Section 3).
2.  **Learned Dispositions:** These evolve over time based on `Memory` and `Beliefs`. For example, repeated positive interactions with a specific `EntityCategory` could lead to a `learned_disposition` of `"trust_species_X": 0.8`. These dispositions then modify how `emotional_states` react to future events.
    *   **Update Mechanism:** When a significant `RelationshipEvent` or `InteractionOutcome` occurs, update relevant `learned_dispositions`.
        `Disposition_Change = Event_Impact * Learning_Rate * (1.0 - Current_Disposition)`
3.  **Emotional States:** These are highly dynamic, changing with `Need` levels, `Perception` of threats, and `InteractionOutcome`s. They directly influence `Intent Generation` and `Decision Scoring` by adding temporary biases.
    *   **Update Mechanism:**
        *   `Happiness` increases with `NeedType::Fulfillment` satisfaction, positive `Social` interactions.
        *   `Fear` increases with `PerceivedEntity.threat_level`, negative `Combat` outcomes.
        *   `Stress` increases with high `NeedType` values, `PopulationPressure`.

### 2.2. Attention System: Regulating Focus

The attention system filters the vast amount of perceived information into a manageable, prioritized list of relevant `Affordances` and `PerceivedEntities`/`PerceivedItems` for the `Decision Scoring` process.

**Data Flow:**

1.  **Raw Perception:** The `PerceptionComponent` gathers all sensory input within range.
2.  **Initial Salience Calculation:** Each perceived object/affordance is assigned a `Salience` score (as per original documentation, Line 3534).
3.  **Need & Intent Filtering:** Objects/affordances directly relevant to the entity's current `Intention` or most critical `NeedType`s receive a significant `Salience` boost.
    *   `Salience_Boost = INTENT_ALIGNMENT_BONUS_FACTOR * (1.0 - (Current_Need_Value / 100.0))` (for relevant needs)
4.  **Threat Override:** Any `PerceivedEntity` with `threat_level > THREAT_OVERRIDE_THRESHOLD` is immediately prioritized with maximum `Salience`.
5.  **Personality Bias:**
    *   `Curiosity`: Adds `curiosity * NOVELTY_BONUS_FACTOR` to `Salience` for novel or unexplored objects.
    *   `Caution`: Multiplies `threat_level` by `(1.0 + caution * CAUTION_THREAT_MULTIPLIER)` before `Salience` calculation.
6.  **Focus-Limited Selection:** The system selects the top `N` items based on their final `Salience` score.
    *   `N = clamp(DEFAULT_ATTENTION_N + floor((Vigil / 20.0) - (Stress_Need / 50.0) + (Curiosity * 5.0)), N_MIN, N_MAX)`

**Output:** A `Vec<PrioritizedPerception>` containing the `N` most relevant perceived items/affordances, passed to the `Decision Scoring` system.

### 2.3. Attention System Constants

| Constant Name | Value | Description |
| :--- | :--- | :--- |
| `THREAT_OVERRIDE_THRESHOLD` | `0.7` | `threat_level` above which an entity is immediately prioritized. |
| `NOVELTY_BONUS_FACTOR` | `10.0` | How much Curiosity boosts salience for novel items. |
| `CAUTION_THREAT_MULTIPLIER` | `0.5` | How much Caution amplifies perceived threat levels. |
| `DEFAULT_ATTENTION_N` | `5` | Default number of items an entity can focus on. |
| `N_MIN` | `3` | Minimum number of items an entity can focus on. |
| `N_MAX` | `10` | Maximum number of items an entity can focus on. |


## 3. Decision Scoring: Utility Function and Action Selection

Decision scoring is the process by which an entity evaluates the available `Affordances` (filtered by the `Attention System`) and selects the optimal action to execute. This is achieved through a **Utility-Based Action Selection** algorithm, where each potential action is assigned a numerical utility score.

### 3.1. Action Evaluation Pipeline

For each `Affordance` provided by the `Attention System`:

**Step 1: Base Affordance Utility**
Start with the inherent `salience` of the affordance, which already incorporates basic importance, need alignment, novelty, and threat (as defined in the original documentation, Line 3534).

```rust
Action_Utility = Affordance.salience
```

**Step 2: Intent Alignment Bonus**
Actions that directly contribute to the entity's current `Intention` receive a significant utility bonus. This bonus is scaled by the `Intention`'s own `utility_score`.

```rust
if Affordance.goal_matches_intention(entity.current_intention) {
    Action_Utility += entity.current_intention.utility_score * INTENT_ALIGNMENT_BONUS_FACTOR;
}
```

**Step 3: Need Satisfaction Potential**
Calculate how effectively the action would satisfy the entity's most critical `NeedType`s. This requires a lookup of the action's potential effects.

```rust
for (need_type, satisfaction_amount) in Affordance.potential_need_satisfaction() {
    if entity.needs.get_value(need_type) > NEED_SATISFACTION_THRESHOLD {
        Action_Utility += satisfaction_amount * NEED_SATISFACTION_FACTOR;
    }
}
```

**Step 4: Cost Consideration (Penalties)**
Subtract penalties for the `energy_cost`, `time_cost_ticks`, and `risk_factor` associated with the action. Costs are normalized to be comparable with utility scores.

```rust
Cost_Penalty = (Affordance.energy_cost * ENERGY_PENALTY_FACTOR) 
             + (Affordance.time_cost_ticks * TIME_PENALTY_FACTOR) 
             + (Affordance.risk_factor * RISK_PENALTY_FACTOR);
Action_Utility -= Cost_Penalty;
```

**Step 5: Personality Bias**
Apply modifiers based on the entity's `PersonalityTrait`s. These biases can increase or decrease the utility of certain action types.

*   **Aggression:**
    `if Affordance.is_combat_action() { Action_Utility += entity.brain.core_traits.aggression * AGGRESSION_COMBAT_BONUS; }`
*   **Caution:**
    `if Affordance.risk_factor > CAUTION_RISK_THRESHOLD { Action_Utility -= entity.brain.core_traits.caution * CAUTION_RISK_PENALTY; }`
*   **Empathy:**
    `if Affordance.is_social_action() && Affordance.target_is_distressed() { Action_Utility += entity.brain.core_traits.empathy * EMPATHY_SOCIAL_BONUS; }`

**Step 6: Belief Influence**
If a `Belief` is relevant to the `Affordance` (e.g., "Location X is dangerous"), modify the utility. This can be a positive or negative influence based on the belief's confidence and nature.

```rust
if let Some(relevant_belief) = entity.brain.beliefs.get_relevant_to_affordance(Affordance) {
    Action_Utility += (relevant_belief.confidence - 0.5) * BELIEF_ACTION_MODIFIER_FACTOR;
}
```

**Step 7: Final Action Score**
The `Action_Utility` at this point represents the `Final_Action_Score` for that specific affordance.

### 3.2. Action Selection Logic

After calculating the `Final_Action_Score` for all available `Affordances`:

1.  **Sort:** Sort the `Affordances` in descending order based on their `Final_Action_Score`.
2.  **Select:** The `Affordance` with the highest `Final_Action_Score` is chosen as the entity's next action.
3.  **Tie-breaking:** In case of a tie, use secondary criteria such as `Affordance.priority_tier`, `Affordance.time_cost_ticks` (prefer faster actions), or a random selection.

### 3.3. Decision Scoring Constants

| Constant Name | Value | Description |
| :--- | :--- | :--- |
| `INTENT_ALIGNMENT_BONUS_FACTOR` | `0.8` | Multiplier for how much the current intention's utility boosts relevant actions. |
| `NEED_SATISFACTION_THRESHOLD` | `20.0` | Minimum need level for an action's need satisfaction potential to be considered. |
| `NEED_SATISFACTION_FACTOR` | `1.5` | Multiplier for how much each unit of need satisfaction contributes to action utility. |
| `ENERGY_PENALTY_FACTOR` | `0.2` | Utility penalty per unit of energy cost. |
| `TIME_PENALTY_FACTOR` | `0.005` | Utility penalty per tick of time cost. |
| `RISK_PENALTY_FACTOR` | `50.0` | Utility penalty per unit of risk (0-1). |
| `AGGRESSION_COMBAT_BONUS` | `30.0` | Utility bonus for combat actions, scaled by aggression. |
| `CAUTION_RISK_THRESHOLD` | `0.4` | Risk factor above which caution applies a penalty. |
| `CAUTION_RISK_PENALTY` | `40.0` | Utility penalty for risky actions, scaled by caution. |
| `EMPATHY_SOCIAL_BONUS` | `25.0` | Utility bonus for social actions targeting distressed entities, scaled by empathy. |
| `BELIEF_ACTION_MODIFIER_FACTOR` | `60.0` | Max utility modification from a belief (positive or negative). |



## 2. Operationalizing Personality and Attention

### 2.1. Personality: The Waterfall Approach

The "waterfall approach" to personality models a hierarchy of influence, where stable core traits inform dynamic learned dispositions, which in turn shape transient emotional states. This provides a layered, realistic behavioral model.

**Data Structures:**

```rust
pub struct PersonalityComponent {
    pub core_traits: PersonalityTrait, // Immutable after creation, 0-1 range
    pub learned_dispositions: HashMap<String, f32>, // e.g., "generosity": 0.7, "biome_fear_forest": 0.9
    pub emotional_states: EmotionalStateBlock, // Transient, 0-100 range
}

pub struct EmotionalStateBlock {
    pub happiness: f32, // 0-100
    pub fear: f32,      // 0-100
    pub anger: f32,     // 0-100
    pub stress: f32,    // 0-100 (overlaps with NeedType::Stress, consider merging or clarifying)
}
```

**Flow and Influence:**

1.  **Core Traits:** These are foundational. They set initial values for `learned_dispositions` and `emotional_states` at entity creation. They also act as **multipliers or biases** in `Intent Generation` and `Decision Scoring` (as seen in Section 1 and upcoming Section 3).
2.  **Learned Dispositions:** These evolve over time based on `Memory` and `Beliefs`. For example, repeated positive interactions with a specific `EntityCategory` could lead to a `learned_disposition` of `"trust_species_X": 0.8`. These dispositions then modify how `emotional_states` react to future events.
    *   **Update Mechanism:** When a significant `RelationshipEvent` or `InteractionOutcome` occurs, update relevant `learned_dispositions`.
        `Disposition_Change = Event_Impact * Learning_Rate * (1.0 - Current_Disposition)`
3.  **Emotional States:** These are highly dynamic, changing with `Need` levels, `Perception` of threats, and `InteractionOutcome`s. They directly influence `Intent Generation` and `Decision Scoring` by adding temporary biases.
    *   **Update Mechanism:**
        *   `Happiness` increases with `NeedType::Fulfillment` satisfaction, positive `Social` interactions.
        *   `Fear` increases with `PerceivedEntity.threat_level`, negative `Combat` outcomes.
        *   `Stress` increases with high `NeedType` values, `PopulationPressure`.

### 2.2. Attention System: Regulating Focus

The attention system filters the vast amount of perceived information into a manageable, prioritized list of relevant `Affordances` and `PerceivedEntities`/`PerceivedItems` for the `Decision Scoring` process.

**Data Flow:**

1.  **Raw Perception:** The `PerceptionComponent` gathers all sensory input within range.
2.  **Initial Salience Calculation:** Each perceived object/affordance is assigned a `Salience` score (as per original documentation, Line 3534).
3.  **Need & Intent Filtering:** Objects/affordances directly relevant to the entity's current `Intention` or most critical `NeedType`s receive a significant `Salience` boost.
    *   `Salience_Boost = INTENT_ALIGNMENT_BONUS_FACTOR * (1.0 - (Current_Need_Value / 100.0))` (for relevant needs)
4.  **Threat Override:** Any `PerceivedEntity` with `threat_level > THREAT_OVERRIDE_THRESHOLD` is immediately prioritized with maximum `Salience`.
5.  **Personality Bias:**
    *   `Curiosity`: Adds `curiosity * NOVELTY_BONUS_FACTOR` to `Salience` for novel or unexplored objects.
    *   `Caution`: Multiplies `threat_level` by `(1.0 + caution * CAUTION_THREAT_MULTIPLIER)` before `Salience` calculation.
6.  **Focus-Limited Selection:** The system selects the top `N` items based on their final `Salience` score.
    *   `N = clamp(DEFAULT_ATTENTION_N + floor((Vigil / 20.0) - (Stress_Need / 50.0) + (Curiosity * 5.0)), N_MIN, N_MAX)`

**Output:** A `Vec<PrioritizedPerception>` containing the `N` most relevant perceived items/affordances, passed to the `Decision Scoring` system.

### 2.3. Attention System Constants

| Constant Name | Value | Description |
| :--- | :--- | :--- |
| `THREAT_OVERRIDE_THRESHOLD` | `0.7` | `threat_level` above which an entity is immediately prioritized. |
| `NOVELTY_BONUS_FACTOR` | `10.0` | How much Curiosity boosts salience for novel items. |
| `CAUTION_THREAT_MULTIPLIER` | `0.5` | How much Caution amplifies perceived threat levels. |
| `DEFAULT_ATTENTION_N` | `5` | Default number of items an entity can focus on. |
| `N_MIN` | `3` | Minimum number of items an entity can focus on. |
| `N_MAX` | `10` | Maximum number of items an entity can focus on. |


## 3. Decision Scoring: Utility Function and Action Selection

Decision scoring is the process by which an entity evaluates the available `Affordances` (filtered by the `Attention System`) and selects the optimal action to execute. This is achieved through a **Utility-Based Action Selection** algorithm, where each potential action is assigned a numerical utility score.

### 3.1. Action Evaluation Pipeline

For each `Affordance` provided by the `Attention System`:

**Step 1: Base Affordance Utility**
Start with the inherent `salience` of the affordance, which already incorporates basic importance, need alignment, novelty, and threat (as defined in the original documentation, Line 3534).

```rust
Action_Utility = Affordance.salience
```

**Step 2: Intent Alignment Bonus**
Actions that directly contribute to the entity's current `Intention` receive a significant utility bonus. This bonus is scaled by the `Intention`'s own `utility_score`.

```rust
if Affordance.goal_matches_intention(entity.current_intention) {
    Action_Utility += entity.current_intention.utility_score * INTENT_ALIGNMENT_BONUS_FACTOR;
}
```

**Step 3: Need Satisfaction Potential**
Calculate how effectively the action would satisfy the entity's most critical `NeedType`s. This requires a lookup of the action's potential effects.

```rust
for (need_type, satisfaction_amount) in Affordance.potential_need_satisfaction() {
    if entity.needs.get_value(need_type) > NEED_SATISFACTION_THRESHOLD {
        Action_Utility += satisfaction_amount * NEED_SATISFACTION_FACTOR;
    }
}
```

**Step 4: Cost Consideration (Penalties)**
Subtract penalties for the `energy_cost`, `time_cost_ticks`, and `risk_factor` associated with the action. Costs are normalized to be comparable with utility scores.

```rust
Cost_Penalty = (Affordance.energy_cost * ENERGY_PENALTY_FACTOR) 
             + (Affordance.time_cost_ticks * TIME_PENALTY_FACTOR) 
             + (Affordance.risk_factor * RISK_PENALTY_FACTOR);
Action_Utility -= Cost_Penalty;
```

**Step 5: Personality Bias**
Apply modifiers based on the entity's `PersonalityTrait`s. These biases can increase or decrease the utility of certain action types.

*   **Aggression:**
    `if Affordance.is_combat_action() { Action_Utility += entity.brain.core_traits.aggression * AGGRESSION_COMBAT_BONUS; }`
*   **Caution:**
    `if Affordance.risk_factor > CAUTION_RISK_THRESHOLD { Action_Utility -= entity.brain.core_traits.caution * CAUTION_RISK_PENALTY; }`
*   **Empathy:**
    `if Affordance.is_social_action() && Affordance.target_is_distressed() { Action_Utility += entity.brain.core_traits.empathy * EMPATHY_SOCIAL_BONUS; }`

**Step 6: Belief Influence**
If a `Belief` is relevant to the `Affordance` (e.g., "Location X is dangerous"), modify the utility. This can be a positive or negative influence based on the belief's confidence and nature.

```rust
if let Some(relevant_belief) = entity.brain.beliefs.get_relevant_to_affordance(Affordance) {
    Action_Utility += (relevant_belief.confidence - 0.5) * BELIEF_ACTION_MODIFIER_FACTOR;
}
```

**Step 7: Final Action Score**
The `Action_Utility` at this point represents the `Final_Action_Score` for that specific affordance.

### 3.2. Action Selection Logic

After calculating the `Final_Action_Score` for all available `Affordances`:

1.  **Sort:** Sort the `Affordances` in descending order based on their `Final_Action_Score`.
2.  **Select:** The `Affordance` with the highest `Final_Action_Score` is chosen as the entity's next action.
3.  **Tie-breaking:** In case of a tie, use secondary criteria such as `Affordance.priority_tier`, `Affordance.time_cost_ticks` (prefer faster actions), or a random selection.

### 3.3. Decision Scoring Constants

| Constant Name | Value | Description |
| :--- | :--- | :--- |
| `INTENT_ALIGNMENT_BONUS_FACTOR` | `0.8` | Multiplier for how much the current intention's utility boosts relevant actions. |
| `NEED_SATISFACTION_THRESHOLD` | `20.0` | Minimum need level for an action's need satisfaction potential to be considered. |
| `NEED_SATISFACTION_FACTOR` | `1.5` | Multiplier for how much each unit of need satisfaction contributes to action utility. |
| `ENERGY_PENALTY_FACTOR` | `0.2` | Utility penalty per unit of energy cost. |
| `TIME_PENALTY_FACTOR` | `0.005` | Utility penalty per tick of time cost. |
| `RISK_PENALTY_FACTOR` | `50.0` | Utility penalty per unit of risk (0-1). |
| `AGGRESSION_COMBAT_BONUS` | `30.0` | Utility bonus for combat actions, scaled by aggression. |
| `CAUTION_RISK_THRESHOLD` | `0.4` | Risk factor above which caution applies a penalty. |
| `CAUTION_RISK_PENALTY` | `40.0` | Utility penalty for risky actions, scaled by caution. |
| `EMPATHY_SOCIAL_BONUS` | `25.0` | Utility bonus for social actions targeting distressed entities, scaled by empathy. |
| `BELIEF_ACTION_MODIFIER_FACTOR` | `60.0` | Max utility modification from a belief (positive or negative). |


## 4. Consolidated Numerical Parameters, Decay Rates, and Default Constants

This section provides a single, comprehensive reference for all numerical parameters, decay rates, thresholds, and default values discussed throughout this specification. These values are crucial for direct implementation and fine-tuning the simulation.

### 4.1. Global Simulation Constants

| Constant Name | Value | Description |
| :--- | :--- | :--- |
| `TICK_DURATION_SECONDS` | `0.1` | Real-world seconds per simulation tick. |
| `TICKS_PER_MINUTE` | `600` | Number of ticks in one simulation minute. |
| `TICKS_PER_HOUR` | `36000` | Number of ticks in one simulation hour. |
| `TICKS_PER_DAY` | `864000` | Number of ticks in one simulation day. |

### 4.2. Intent Function Constants

| Constant Name | Value | Description |
| :--- | :--- | :--- |
| `NEED_WEIGHT_BASE` | `100.0` | The maximum base utility a need can generate (at 100/100 severity). |
| `CAUTION_SURVIVAL_MULTIPLIER` | `0.5` | How much Caution amplifies survival needs (e.g., Caution 1.0 = 50% boost). |
| `SOCIABILITY_SOCIAL_MULTIPLIER` | `0.8` | How much Sociability amplifies the Social need. |
| `GRIT_STRESS_REDUCTION` | `0.4` | How much Grit reduces the urgency of Comfort/Stress needs. |
| `MEMORY_INTENT_IMPACT` | `20.0` | Max utility bonus from a highly emotional relevant memory. |
| `BELIEF_INTENT_IMPACT` | `30.0` | Max utility bonus from a highly confident relevant belief. |
| `CURIOSITY_EXPLORE_THRESHOLD` | `0.4` | Minimum Curiosity required to generate an Explore intent. |
| `EXPLORE_BASE_UTILITY` | `40.0` | Base utility for exploration, scaled by Curiosity. |
| `SOCIABILITY_IDLE_THRESHOLD` | `0.5` | Minimum Sociability to seek idle socialization. |
| `SOCIALIZE_BASE_UTILITY` | `35.0` | Base utility for idle socialization, scaled by Sociability. |

### 4.3. Personality & Emotional State Constants

| Constant Name | Value | Description |
| :--- | :--- | :--- |
| `LEARNING_RATE` | `0.1` | Rate at which learned dispositions change per event. |
| `DEFAULT_PERSONALITY_TRAIT` | `0.5` | Default starting value for all personality traits. |
| `AGGRESSION_COMBAT_BONUS` | `30.0` | Utility bonus for combat actions, scaled by aggression. |
| `CAUTION_RISK_THRESHOLD` | `0.4` | Risk factor above which caution applies a penalty. |
| `CAUTION_RISK_PENALTY` | `40.0` | Utility penalty for risky actions, scaled by caution. |
| `EMPATHY_SOCIAL_BONUS` | `25.0` | Utility bonus for social actions targeting distressed entities, scaled by empathy. |

### 4.4. Attention System Constants

| Constant Name | Value | Description |
| :--- | :--- | :--- |
| `THREAT_OVERRIDE_THRESHOLD` | `0.7` | `threat_level` above which an entity is immediately prioritized. |
| `NOVELTY_BONUS_FACTOR` | `10.0` | How much Curiosity boosts salience for novel items. |
| `CAUTION_THREAT_MULTIPLIER` | `0.5` | How much Caution amplifies perceived threat levels. |
| `DEFAULT_ATTENTION_N` | `5` | Default number of items an entity can focus on. |
| `N_MIN` | `3` | Minimum number of items an entity can focus on. |
| `N_MAX` | `10` | Maximum number of items an entity can focus on. |

### 4.5. Decision Scoring Constants

| Constant Name | Value | Description |
| :--- | :--- | :--- |
| `INTENT_ALIGNMENT_BONUS_FACTOR` | `0.8` | Multiplier for how much the current intention's utility boosts relevant actions. |
| `NEED_SATISFACTION_THRESHOLD` | `20.0` | Minimum need level for an action's need satisfaction potential to be considered. |
| `NEED_SATISFACTION_FACTOR` | `1.5` | Multiplier for how much each unit of need satisfaction contributes to action utility. |
| `ENERGY_PENALTY_FACTOR` | `0.2` | Utility penalty per unit of energy cost. |
| `TIME_PENALTY_FACTOR` | `0.005` | Utility penalty per tick of time cost. |
| `RISK_PENALTY_FACTOR` | `50.0` | Utility penalty per unit of risk (0-1). |
| `BELIEF_ACTION_MODIFIER_FACTOR` | `60.0` | Max utility modification from a belief (positive or negative). |

### 4.6. Need System Decay Rates and Modifiers

| Need Type | Default Drain (per 10 ticks) | Stat Modifier Formula (Example) |
| :--- | :--- | :--- |
| `Hunger` | `0.5` | `(1.0 - (entity.stats.vigor / 100.0) * 0.5)` |
| `Thirst` | `0.8` | `(1.0 - (entity.stats.grit / 100.0) * 0.3)` |
| `Fatigue` | `0.3` | `(1.0 - (entity.stats.vigor / 100.0) * 0.7)` |
| `Warmth` | `0.2` | `(1.0 - (entity.stats.grit / 100.0) * 0.4)` |
| `Comfort` | `0.1` | (No specific stat modifier defined yet) |
| `Stress` | `0.05` (gain) | `(1.0 - (entity.stats.tenacity / 100.0) * 0.6)` |
| `Social` | `0.08` | `(1.0 - (entity.brain.core_traits.sociability / 100.0) * 0.5)` |
| `Fulfillment` | `0.03` | (No specific stat modifier defined yet) |
| `Waste` | `0.02` (gain) | (No specific stat modifier defined yet) |

### 4.7. Belief System Constants

| Constant Name | Value | Description |
| :--- | :--- | :--- |
| `BELIEF_DECAY_RATE` | `0.00001` | Rate at which belief confidence decays per tick if not reinforced. |

### 4.8. Skill Progression Constants

| Constant Name | Value | Description |
| :--- | :--- | :--- |
| `SKILL_GAIN_MAX_POINTS_PER_100_ACTIONS` | `2.0` | Hard cap on skill points gained once skill > 80. |

### 4.9. Item Valuation Constants

| Constant Name | Value | Description |
| :--- | :--- | :--- |
| `USABILITY_MULTIPLIER_PRISTINE` | `1.0` | Multiplier for 100-70 durability. |
| `USABILITY_MULTIPLIER_WORN` | `0.7` | Multiplier for 70-30 durability. |
| `USABILITY_MULTIPLIER_DAMAGED` | `0.3` | Multiplier for 30-1 durability. |
| `USABILITY_MULTIPLIER_BROKEN` | `0.0` | Multiplier for 0 durability. |
| `TIME_SINCE_CREATION_MAX_MULTIPLIER` | `2.0` | Maximum appreciation multiplier for items. |
| `PERISHABILITY_MIN_SALVAGE_MULTIPLIER` | `0.1` | Minimum value multiplier for perishable items. |

### 4.10. Relationship System Constants

| Constant Name | Value | Description |
| :--- | :--- | :--- |
| `RECENT_HISTORY_WINDOW_TICKS` | `86400` | Time window (in ticks) for considering recent relationship events (1 day). |

### 4.11. Ecosystem & Civilization Constants

| Constant Name | Value | Description |
| :--- | :--- | :--- |
| `BASE_CARRYING_CAPACITY_FOREST` | `100.0` | Base carrying capacity for Forest biome. |
| `BASE_CARRYING_CAPACITY_DESERT` | `20.0` | Base carrying capacity for Desert biome. |
| `BASE_CARRYING_CAPACITY_PLAINS` | `150.0` | Base carrying capacity for Plains biome. |
| `BASE_CARRYING_CAPACITY_MOUNTAIN` | `50.0` | Base carrying capacity for Mountain biome. |
| `RESOURCE_AVAILABILITY_FACTOR_WEIGHTS` | `0.5` | How much resource availability influences carrying capacity. |
| `CLIMATE_FACTOR_WEIGHTS` | `0.3` | How much climate (temperature, rainfall) influences carrying capacity. |
| `POPULATION_PRESSURE_THRESHOLD` | `1.2` | If `CurrentPopulation / CarryingCapacity > 1.2`, population pressure begins. |
| `MORTALITY_RATE_INCREASE_PER_PRESSURE` | `0.01` | For every 0.1 increase in `PopulationPressure` above threshold, `MortalityRate` increases by 1%. |
| `CULTURE_FORMATION_MEMORY_THRESHOLD` | `100` | Number of entities sharing a high-salience memory for `SharedBelief` formation. |
| `TECHNOLOGY_ADVANCEMENT_RATE` | `0.001` | Base rate of `TechnologicalLevel` increase per successful `Crafting` or `Research` action. |
