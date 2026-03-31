# 🧠 Behavior & Cognition Systems

## Information Flow

The entity's behavior emerges from a sequential pipeline:

**Reality** → **Perception** → **Memory** → **Belief** → **Intent** → **Attention** → **Decision** → **Action**

Each stage filters, stores, or transforms information based on:
* Stat modifiers (Vigil, Insight, Grit, others)
* Need states (urgency drives perception/intent)
* Personality (long-term bias in priorities)
* Experience (memory shapes belief)

---

## Perception System

### Sensory Input

Perception is **observer-centric and subjective** (entities don't perceive objective world).

**Visual Perception (Vigil-based)**
* Range: base_visual_range × (1 + Vigil × 0.1)
* Acuity: Ability to distinguish details, track movement
* Affected by: Terrain cover, weather (rain/snow), time of day (darkness)
* Examples: Nocturnal predators perceive better in darkness; diurnal prey see better in daylight

**Auditory Perception (Vigil-based)**
* Range: base_hearing_range × (1 + Vigil × 0.08)
* Acuity: Distinguish sounds, localize source
* Affected by: Distance, noise interference, entity's familiarity with sound

**Olfactory Perception (Vigil-based)**
* Range: base_smell_range × (1 + Vigil × 0.12)
* Trace Persistence: How long scent lingers (decay over time)
* Affected by: Wind, rainfall, terrain

**Tactile Perception (Lithe/Grit-based)**
* Triggered by: Contact with objects/entities
* Information: Texture, temperature, resistance

### Perception Filtering

Not all perceived information is equally memorable:

```
Salience = base_importance × 
    (1 + need_alignment) × 
    (1 + novelty_bonus) × 
    (1 + threat_level)
```

**High Salience:** Predator nearby (threat), Food when hungry (need), Novel creature (learning)
**Low Salience:** Familiar terrain, distant non-threatening entity, satisfied need

### Perception Memory

Entities remember key perceptions:
* **What**: Creature type, entity, object, location
* **Where**: Spatial coordinates relative to self + world map
* **When**: Recency decay (older memories fade faster)
* **Why**: Associated need state (was searching for food, avoiding threat)
* **How**: Sensory modality (saw vs. heard vs. smelled)

---

## Memory System

### Memory Tiers

**Immediate Memory (0–5 min)**
* Current perceptions
* Active threats/goals
* Working problem
* Capacity: High but short duration

**Short-Term Memory (5 min – 1 hour)**
* Recent events
* Completed goals
* Familiar locations revisited
* Capacity: Medium; fades unless reinforced

**Long-Term Memory (1 hour – lifetime)**
* Significant events
* Learned locations (dens, food sources, danger zones)
* Relationships with other entities
* Personality/beliefs about world
* Capacity: Limited but persistent

### Memory Consolidation

High-salience perceptions (threat, need fulfillment, novel discovery) convert to long-term memory faster.

Example:
* **Perception**: "Predator attacks" → High salience + threat → Immediate conversion to long-term
* **Memory**: "Predator location", "Predator danger level", "Escape route from location"

### Memory Decay

```
Memory_Strength(t) = original_strength × e^(−decay_rate × t)
```

Entities are more likely to forget old, low-salience memories (e.g., "saw grass flower 6 months ago").

Critical memories (predator lair, home den) have lower decay rates and resist forgetting.

---

## Belief System

### Belief Formation

Beliefs = persistent interpretations of memory shaped by experience + personality.

**Types of Beliefs:**

**Location Beliefs**
* "There is food at location X" (learned from foraging success)
* "There is danger at location X" (learned from predator encounters)
* "X is home" (reinforced repeat visits)
* Confidence: Increases with repeated positive/negative outcomes; decreases with conflicting evidence

**Entity Beliefs**
* "That entity is dangerous" (based on observed aggression)
* "That entity is friendly" (based on cooperative interactions)
* "That entity has desirable traits" (reproductive attractiveness)
* Confidence: Shaped by personality (aggressive entities weigh threats higher) + repeated interaction

**Capability Beliefs**
* "I can climb that cliff" (based on Lithe/Grit stats + success history)
* "I cannot outrun predator X" (based on speed comparison + past chases)
* Confidence: Realistic for intelligent entities; optimistic/pessimistic biases per personality

**Rule Beliefs**
* "Fire is hot" (inferred from memory)
* "Deep water drowns" (inferred from danger/observation)
* Example: Intelligent entities generalize across experiences; primitive entities treated each encounter as novel

### Belief Persistence

Beliefs remain until contradicted by experience:

```
Belief_Update = old_belief × persistence_factor + new_evidence × (1 - persistence_factor)
```

High persistence = entities trust old beliefs even with new contradiction (stubborn)
Low persistence = entities update beliefs quickly (adaptable)

Personality determines persistence bias (aggressive entities persist threat beliefs longer).

---

## Intent System

### Intent Formation

**Intent = Conscious motivation to achieve goal.**

Intents drive behavior. An entity may hold multiple intents at varying priorities:

```
Current_Intent = highest_priority_unsatisfied_need
```

**Primary Intents (Immediate Urgency):**
1. **Survival**: Hunger, Thirst, Warmth, Health
   - Drives food/water/shelter seeking
   - Overrides most other intents

2. **Safety**: Avoid predators, danger
   - Drives threat avoidance, defensive behavior
   - Can override hunger if threat is present

3. **Reproduction**: Mating intent when fertile + driven
   - Drives search for partner, courtship
   - Often overrides other intents in fertile season

**Secondary Intents (Learned/Cultural):**
4. **Social**: Interact with specific entities
   - Territorial dominance over rivals
   - Bonding with family/group
   - Cultural participation (ritual, trade)

5. **Exploration**: Curiosity, territoriality
   - Novel environment discovery
   - Territory patrol
   - Driven by Insight stat + personality (novelty-seeking)

6. **Fulfillment**: Entertainment, comfort seeking
   - Play behavior (juveniles)
   - Pleasure-seeking (when survival met)
   - Art/music/cultural activity (sophistic entities)

### Intent Calculation

```
Intent_Urgency(need) = 
    (1 - need_satisfaction_ratio) × 
    need_decay_rate ×
    (1 + personality_priority_bias[need])
```

**High Urgency Intents:**
* Thirst need (high decay) + low water satisfaction = immediate water-seeking
* Predator nearby + fear personality = immediate escape intent

**Low Urgency Intents:**
* Exploration when all needs satisfied + high Insight
* Social bonding (lower decay) when alone but not in danger

---

## Intent Scoring Formula

```
Intent_Priority_Score(intent) = 
    urgency × 
    (success_probability_estimate + 0.5) ×
    (1 + memory_of_past_success) ×
    personality_bias[intent_type]
```

**Example 1: Hungry deer near remembered food source**
* Urgency = high (hunger/thirst)
* Success probability = high (location known, low threat)
* Memory of past success = high (found food before)
* Personality bias = herbivore bias toward foraging = +0.2
* **Result**: Very high priority, execute immediately

**Example 2: Curious young predator, fed**
* Urgency = low (hunger satisfied)
* Success probability = medium (unknown territory)
* Memory of past success = low (new location)
* Personality bias = predator bias toward exploration = +0.3
* **Result**: Low-medium priority, explore if safe

---

## Attention System

### Attention Allocation

Entities cannot track all information equally. Attention is a limited resource.

```
Attention_Score(entity or object or goal) = 
    saliency ×
    (1 + relevance_to_current_intent) ×
    (1 + threat_level) ×
    (1 + emotional_attachment)
```

**Example:**
* **Predator nearby**: High threat level + immediate intent relevance = Very high attention
* **Mating season, potential mate visible**: High emotional attachment + reproductive intent = High attention
* **Familiar tree**: Low threat, not relevant to any immediate intent = Low attention

### Attention Shifting

Attention focuses on highest-priority target(s). Secondary targets receive reduced processing (slower reaction time, reduced accuracy).

Entity behavior reflects current attention:
* High attention on predator = tracking movement, preparing defense
* Low attention on background entity = barely noticed, not tracked

---

## Decision System

### Decision Making

An entity decides **which action to take** based on:

1. **Current Intent**: What is the entity trying to accomplish?
2. **Available Actions**: What can the entity do in current state/location?
3. **Estimated Outcomes**: What will each action achieve?
4. **Expected Reward**: Will the action satisfy intent?

### Decision Score Formula

```
Decision_Score(action) = 
    relevance_to_intent ×
    (1 - effort_cost / available_energy) ×
    expected_success_probability ×
    (1 + personality_action_preference[action])
```

**Example 1: Hungry predator, prey visible**
* Action: Hunt prey
* Relevance to intent = 1.0 (hunger intent)
* Effort cost = high (chase), but high energy available = 0.7 multiplier
* Success probability = medium (entity is skilled) = 0.8
* Personality preference = carnivore bias toward hunting = +0.4
* **Decision Score**: 0.84 (very likely to hunt)

**Example 2: Hungry predator, predator threat visible**
* Action: Hunt prey (same as above)
* Relevance to intent = 1.0 (hunger)
* Effort cost = high, energy available = 0.7
* Success probability = LOW (attention diverted, threat-focused) = 0.3
* Personality preference = +0.4
* **Decision Score**: 0.318 (unlikely; survival overrides hunger)

**Example 3: Sated entity, peaceful, novel location**
* Action: Explore
* Relevance to intent (low) = 0.4 (exploration not urgent)
* Effort cost = low = 0.95 multiplier
* Success probability = high (safe environment) = 0.9
* Personality preference = novelty-seeking trait = +0.5
* **Decision Score**: 0.57 (moderate likelihood; might explore if time available)

---

## Action System

See [Actions Reference](actions.md) for detailed action tables and mechanics.

### Action Resolution

Once an action is decided:

```
Action_Success_Probability = base_probability ×
    (1 + relevant_stat / 10) ×
    skill_efficiency ×
    environmental_modifier
```

**Action Execution:**
1. Entity commits to action
2. Calculate success via formula above
3. Apply effects (damage, resource consumption, status changes, memories)
4. Update memory of outcome
5. Feed back into belief system (success/failure updates future decision-making)

---

## Personality System (Waterfall Approach)

### Personality Dimensions

Each entity has personality traits that **bias** decisions, beliefs, and action preferences.

**Sociability** (Loner ↔ Gregarious)
* Low: Prefers solitude, avoids groups
* High: Seeks group interaction, comfort in crowds
* Effect: Intent priority (social intent higher for gregarious)

**Aggression** (Docile ↔ Hostile)
* Low: Conflict avoidance, preference for flight/cooperation
* High: Preference for confrontation, territorial dominance
* Effect: Action choice (aggressive entities choose combat over flee; docile entities flee)

**Curiosity** (Conservative ↔ Exploratory)
* Low: Prefers known locations/behaviors, avoids novelty
* High: Seeks new experiences, exploration intent high
* Effect: Behavior variety (curious entities try new actions/locations; conservative entities repeat proven behaviors)

**Fearfulness** (Brave ↔ Anxious)
* Low: Risk tolerance high, threats underestimated
* High: Risk-averse, threats overestimated
* Effect: Decision-making (brave entities take risky actions; anxious entities avoid)

**Intelligence Bias** (Practical ↔ Analytical)
* Practical: Immediate action preference, trial-and-error learning
* Analytical: Planning-first, memory-reliant, broader option evaluation
* Effect: Decision time (practical entities act faster, analytical entities deliberate)

### Personality Effects on Pipeline

**Perception**: Personality biases what information is salient (aggressive entity notices threats first; social entity notices group members first)

**Belief**: Personality biases belief persistence (aggressive entity holds threat beliefs longer; social entity softens negative beliefs with repetition)

**Intent**: Personality biases intent priority (gregarious entity prioritizes social intent; loner prioritizes survival)

**Decision**: Personality biases action preferences (aggressive entity prefers combat; fearful entity prefers stealth)

**Action**: Personality biases action style (aggressive entity attacks with force; cautious entity feints first)

---

## Examples: Decision Chains

### Example 1: Predator Hunting

1. **Reality**: Herbivore herd nearby; predator is hungry
2. **Perception**: Sees herd, sound of movement, scent of prey
3. **Memory**: "That species is prey; open grassland enables chase"
4. **Belief**: "I am skilled hunter; this prey is escapable; grassland is good hunting ground"
5. **Intent**: Hunger goal drives hunting intent
6. **Attention**: Focus on herd; predator narrowing attention to weakest member
7. **Decision**: "Hunt weakest prey; chase is likely successful"
8. **Action**: Stalk → Charge → Attack → Consume or retreat if failed

### Example 2: Young Social Entity in Novel Location

1. **Reality**: Young entity, not hungry, surrounded by unknown features; other entities present
2. **Perception**: Bright stone formation, sound of running water, other young entity nearby
3. **Memory**: "Novel location not yet explored; water means gathering place"
4. **Belief**: "This place is likely safe; water = good resource; that entity could be friend"
5. **Intent**: Exploration intent + social intent (juvenile gregariousness)
6. **Attention**: Split between novel feature and potential social friend
7. **Decision**: "Approach young entity; explore together" (social + exploration combining)
8. **Action**: Approach → Greet → Suggest exploration → Move to water feature

### Example 3: Entity Faced with Threat & Hunger

1. **Reality**: Predator visible; entity is hungry
2. **Perception**: Sees predator, smells potential food upwind
3. **Memory**: "Predator = danger; food = satisfaction; but predator in direct path to food"
4. **Belief**: "I cannot outrun this predator; food is more valuable than risk"
5. **Intent**: CONFLICT! Hunger vs. Safety. Safety wins (survival override).
6. **Attention**: Focus shifts to predator; food opportunity ignored
7. **Decision**: "Flee threat; food search later"
8. **Action**: Flee → Hide → Wait for threat to pass → Then seek food

---

## Personality-Behavior Synthesis

The full behavior emerges from:

**Immediate Pipeline** (Perception → Action)
* Drives moment-to-moment responsiveness

**Personality Overlay**
* Colors all decisions with consistent bias
* Creates recognizable character across interactions

**Learning Loop** (Memory → Belief → Future Decisions)
* Shapes long-term behavioral pattern
* Allows adaptation to local environment

**Emergent Personality** = Consistent bias + Learned specialization + Moment-by-moment responsiveness
