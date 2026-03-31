# 🎯 Affordances System

The affordances system defines the **possibilities for action** that entities perceive in their environment. Affordances represent what an entity **can do** with an object or in a location, driven by the entity's capabilities, the object's properties, and the context.

---

## Core Concept

**Affordance = Possibility for Action**

An affordance is:
* Observable by the entity (perceived based on Vigil, knowledge, belief)
* Performable by the entity (requires capability: stats, skills, energy)
* Contextual (available based on location, time, weather, social state)
* Meaningful (aligned with entity needs, personality, or beliefs)

**Not an affordance**: An entity *can* fly off a cliff and die, but "death" is not an affordance—it's a consequence.

---

## Affordance Categories

### Environmental Affordances

**Flora**
* Harvest (fruit, berries, edible plants)
* Climb (trees, vines)
* Shelter (under canopy)
* Burn (wood fuel)

**Minerals**
* Mine (extract ore)
* Climb (rocky terrain)
* Break (for tools)

**Water**
* Drink
* Fish
* Cool (in heat)
* Cross (wade/swim)

**Weather/Climate**
* Seek shelter (in storm)
* Rest (clear weather optimal)
* Conserve warmth (cold)

### Creature/Entity Affordances

**Herbivores/Prey**
* Hunt (food resource)
* Chase (if fleeing predator)
* Observe (study behavior)
* Herd (domesticate/control)
* Breed (reproduction)

**Predators**
* Flee (threat avoidance)
* Fight (combat)
* Hide (from superior predator)
* Observe (study threat level)

**Sentient/Civilization**
* Speak (communicate)
* Trade (exchange items)
* Ally (form bonds)
* Follow (leadership)
* Revolt (resist leadership)
* Marry (reproduce + bond)

### Structural Affordances

**Dwellings**
* Enter/Exit
* Rest (restore stamina)
* Store (inventory management)
* Defend (fortified location)

**Workshops**
* Craft (produce items)
* Repair (restore durability)
* Learn (upgrade skills)

**Markers**
* Settlement signpost (route marking)
* Gather (if granary/storage)
* Meet (social gathering)

---

## Affordance Detection

### Perception

Affordances are **perceived**, not objective:

**High-Vigil entity** perceives more affordances:
* Notices the weak point in the wall (climb affordance)
* Sees the hidden cave entrance (explore affordance)
* Recognizes the track (hunt affordance)

**Low-Vigil entity** misses affordances:
* Doesn't see the climb path
* Walks past the cave
* Ignores the track

### Memory & Learning

Affordances strengthen through **experience**:

* **First encounter**: Low confidence ("Maybe I can climb this tree")
* **Repeated success**: High confidence ("I know how to climb this")
* **Failure**: Affordance weakens or disappears ("Can't climb that; too high")

### Belief Integration

Beliefs modulate affordance availability:

* **Belief**: "This forest is dangerous" → Reduce explore/hunt affordances, increase flee affordance
* **Belief**: "That creature is friendly" → Open social affordances, close fight affordance
* **Belief**: "I'm too weak to hunt" → Hide hunt affordances (even if stat-capable)

---

## Affordance Resolution

When an entity acts on an affordance:

1. **Check Availability**: Is affordance active in this context?
2. **Check Capability**: Does entity have required stats/skills/energy?
3. **Check Perception**: Does entity perceive the affordance?
4. **Initiate Action**: Execute the action
5. **Resolve Outcome**: Success/failure, need satisfaction, learning

---

## Affordance Examples

### Example 1: Apple Tree

**Environmental Context**: Fruit-bearing tree, spring season, ripe apples

**Possible Affordances**:

| Affordance | Required | Perceived By | Outcome |
|---|---|---|---|
| **Eat (fruit)** | Vigil ≥ 4 | Herbivores, omnivores | –20% Hunger, +0.5% Health |
| **Climb** | Lithe ≥ 5 | Climbers, young entities | Reach canopy height |
| **Harvest** | Crafting ≥ 20 | Farmers | +5 fruit items |
| **Shelter** | Any | Any | Basic shelter, –1% weather effect |
| **Burn** | Any | Survivors | Fuel source for fire |

**Modifiers**:
* Winter: No fruit (eat affordance disappears)
* Stormy weather: Climb penalized –0.3
* Low Vigil: Might not notice (perceive chance reduced)

---

### Example 2: River

**Environmental Context**: Water source, current varies

**Possible Affordances**:

| Affordance | Required | Perceived By | Outcome |
|---|---|---|---|
| **Drink** | Vigil ≥ 2 | Thirsty entities | –20% Thirst |
| **Fish** | Hunting ≥ 30 | Predators, omnivores | +2-5 food items |
| **Swim** | Lithe ≥ 4 | Any | Cross river / escape |
| **Cool** | Any | Hot entities | –15% heat effect |
| **Meditate** | Insightful personalities | Contemplative entities | +5% Fulfillment |

**Modifiers**:
* Current strength: High current penalizes swim
* Drought: Water level low, drink less effective
* Season: Spring floods make crossing harder

---

### Example 3: Sentient NPC (Elder)

**Social Context**: Respected community leader, tribe gathering

**Possible Affordances**:

| Affordance | Required | Perceived By | Outcome |
|---|---|---|---|
| **Speak** | Grit ≥ 3 | Any | Initiate dialogue, gain information |
| **Learn** | Insight ≥ 20 | Curious entities | +2% skill, +1% Fulfillment |
| **Follow** | Social ≥ 40 | Community-minded | +5% Social satisfaction, follow leadership |
| **Challenge** | Might ≥ 6 | Aggressive types | Dominate challenge, reputation risk |
| **Trade** | Grit ≥ 4 | Any | Exchange items, economic interaction |
| **Ally** | Honesty ≥ 40 | Trustworthy types | Form bond, +10% Social satisfaction |

**Modifiers**:
* Prejudice beliefs: "That elder is untrustworthy" → All affordances penalized
* Reputation high: More affordances available
* Reputation low: Some affordances close (learn, ally)

---

## Affordance Dynamics

### Affordance Fatigue

Repeated use of the same affordance reduces:
* Effectiveness (diminishing returns)
* Novelty satisfaction (fulfillment bonus drops)
* Motivation (personality shift toward different affordances)

**Example**: Hunting the same creature type 10 times → fulfillment bonus drops 50%; entity seeks new challenge

### Emergent Affordances

Complex affordances emerge from system interactions:

* **"Organize patrol"** → Emerges when multiple sentient entities + settlement structure
* **"Plan ambush"** → Emerges when intelligence + combat coordination
* **"Establish trade route"** → Emerges when multiple settlements + surplus resources

### Affordance Chains

Actions create new affordances:

```
Harvest apples → Craft apple pie → Trade pie → Gain reputation → Learn from respected entity
```

Each step opens new possibilities.

---

## Integration with Behavior Systems

### Intent → Affordance Selection

1. Entity has Intent (e.g., "Fulfill hunger")
2. Entity perceives available Affordances (e.g., hunt, harvest, trade)
3. Decision system scores Affordances based on:
   * Effectiveness (how well it satisfies intent)
   * Cost (energy, time, risk)
   * Personality fit (does this align with personality?)
   * Belief support (does this align with beliefs?)
4. Entity selects highest-scoring Affordance
5. Action executes

### Perception → Affordance Visibility

Entities perceive affordances filtered by:
* Vigil (awareness range, detail acuity)
* Memory (known affordances remembered)
* Belief (belief may hide or highlight affordances)
* Personality (what entity seeks out)

---

## Summary

Affordances are the **bridge between capability and choice**. They represent:

* What **can** be done (capability × context)
* What **is perceived** to be doable (perception × knowledge)
* What **will be chosen** (intent × personality × decision logic)

The affordance system creates emergent, dynamic behavior without scripting specific actions—entities discover and choose actions based on their capabilities, perceptions, and needs.
