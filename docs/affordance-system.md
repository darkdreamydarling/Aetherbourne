# 🎯 Affordance System — Perception-to-Action Pipeline

The affordance system is how entities discover **what they can do** in any given moment. It bridges perception (what I sense) → decision (what should I do) → action (let me do it).

**Without affordances, entities have no way to generate valid actions.**

---

## Core Concept

**Affordance = Possibility for Action**

An affordance exists when:
1. The entity **perceives** a target (object, creature, location, opportunity)
2. The entity **has capability** to attempt the action (stats/skills/energy)
3. The **context permits** the action (e.g., can't fish without water)
4. The action **aligns with motivation** (need-driven or personality-driven)

---

## Affordance Detection Algorithm

### Step 1: Perception Scan (Every 60 Ticks)

Entities sense their environment and create a **perceived world** using their sensory ranges.

```rust
pub fn perception_scan(entity: &Entity, world: &World, tick: u64) -> PerceptualSnapshot {
    // Only runs every 60 ticks (1 minute)
    if tick % 60 != 0 { return PerceptualSnapshot::empty(); }
    
    // Retrieve sensory ranges (from systems-math.md)
    let vision_range = calculate_vision_range(entity.stats.vigil);
    let hearing_range = calculate_hearing_range(entity.stats.vigil);
    let smell_range = calculate_smell_range(entity.stats.vigil);
    
    let mut snapshot = PerceptualSnapshot::new();
    
    // 1. VISUAL PERCEPTION: What can I see?
    let visible_entities = query_spatial_range(entity.position, vision_range, SenseType::Visual);
    for target in visible_entities {
        snapshot.perceived_entities.push(PerceivedEntity {
            entity_id: target.id,
            name: target.name.clone(),
            distance: distance_to(entity.position, target.position),
            threat_level: estimate_threat(&target, &entity),
            sense: SenseType::Visual,
            confidence: 0.95,  // Visual is high confidence
            last_perceived_tick: tick,
        });
    }
    
    // 2. AUDITORY PERCEPTION: What sounds do I hear?
    let audible_entities = query_spatial_range(entity.position, hearing_range, SenseType::Auditory);
    for target in audible_entities {
        // Sound-emitting creatures: predators, prey fleeing, etc.
        if is_sound_emitter(&target) {
            snapshot.perceived_entities.push(PerceivedEntity {
                entity_id: target.id,
                name: format!("(sound) {}", target.name),
                distance: distance_to(entity.position, target.position),
                threat_level: 0.3,  // Unknown at distance
                sense: SenseType::Auditory,
                confidence: 0.6,   // Audio is lower confidence
                last_perceived_tick: tick,
            });
        }
    }
    
    // 3. OLFACTORY PERCEPTION: What scents do I smell?
    let scent_entities = query_spatial_range(entity.position, smell_range, SenseType::Olfactory);
    for target in scent_entities {
        // Some entities have distinctive scents
        if has_scent(&target) {
            let scent_age_ticks = tick - target.last_action_tick;
            let decay = (0.5_f32).powf(scent_age_ticks as f32 / 3600.0);  // 50% decay per hour
            
            snapshot.perceived_entities.push(PerceivedEntity {
                entity_id: target.id,
                name: format!("(scent) {}", target.name),
                distance: distance_to(entity.position, target.position),
                threat_level: 0.4,  // Scent is ambiguous
                sense: SenseType::Olfactory,
                confidence: decay.max(0.2),  // Scent fades
                last_perceived_tick: tick,
            });
        }
    }
    
    // 4. ENVIRONMENTAL PERCEPTION: Resources, hazards, landmarks
    let environment_features = query_spatial_range(entity.position, vision_range, SenseType::Environmental);
    for feature in environment_features {
        match feature.feature_type {
            FeatureType::WaterSource => {
                snapshot.perceived_resources.push(PerceivedResource {
                    name: "Water".to_string(),
                    resource_type: ResourceType::Water,
                    location: feature.position,
                    abundance: feature.abundance,  // 0–100
                    usefulness: estimate_usefulness_to_entity(&entity, ResourceType::Water),
                });
            }
            FeatureType::PlantResource => {
                snapshot.perceived_resources.push(PerceivedResource {
                    name: feature.name.clone(),
                    resource_type: ResourceType::Plant,
                    location: feature.position,
                    abundance: feature.abundance,
                    usefulness: estimate_usefulness_to_entity(&entity, ResourceType::Plant),
                });
            }
            FeatureType::MineralDeposit => {
                snapshot.perceived_resources.push(PerceivedResource {
                    name: feature.name.clone(),
                    resource_type: ResourceType::Mineral,
                    location: feature.position,
                    abundance: feature.abundance,
                    usefulness: estimate_usefulness_to_entity(&entity, ResourceType::Mineral),
                });
            }
            _ => {}
        }
    }
    
    snapshot
}

pub struct PerceptualSnapshot {
    pub perceived_entities: Vec<PerceivedEntity>,
    pub perceived_resources: Vec<PerceivedResource>,
    pub perceived_hazards: Vec<PerceivedHazard>,
}

pub struct PerceivedEntity {
    pub entity_id: u64,
    pub name: String,
    pub distance: f32,
    pub threat_level: f32,      // 0–1
    pub sense: SenseType,
    pub confidence: f32,        // 0–1 (how sure am I?)
    pub last_perceived_tick: u64,
}

pub enum SenseType {
    Visual,
    Auditory,
    Olfactory,
}
```

### Step 2: Generate Affordance Candidates

For each perceived entity/resource, generate potential affordances based on entity capability.

```rust
pub fn generate_affordances_for_snapshot(
    entity: &Entity,
    snapshot: &PerceptualSnapshot,
    beliefs: &BeliefSet,
) -> Vec<Affordance> {
    let mut affordances = Vec::new();
    
    // For each PERCEIVED ENTITY
    for perceived in &snapshot.perceived_entities {
        // Determine target type (prey, predator, ally, neutral)
        let target_type = classify_entity(perceived, beliefs);
        
        // Generate affordances based on type
        let entity_affordances = generate_entity_affordances(entity, perceived, target_type);
        affordances.extend(entity_affordances);
    }
    
    // For each PERCEIVED RESOURCE
    for resource in &snapshot.perceived_resources {
        let resource_affordances = generate_resource_affordances(entity, resource);
        affordances.extend(resource_affordances);
    }
    
    // For each PERCEIVED HAZARD
    for hazard in &snapshot.perceived_hazards {
        let hazard_affordances = generate_hazard_affordances(entity, hazard);
        affordances.extend(hazard_affordances);
    }
    
    affordances
}

// Affordance generation examples:

fn generate_entity_affordances(
    entity: &Entity,
    perceived: &PerceivedEntity,
    target_type: EntityType,
) -> Vec<Affordance> {
    let mut affordances = Vec::new();
    
    match target_type {
        EntityType::Prey => {
            // Entities capable of hunting can hunt prey
            if entity.stats.might > 30 {
                affordances.push(Affordance {
                    name: "Hunt".to_string(),
                    affordance_type: AffordanceType::Creature,
                    target_entity_id: Some(perceived.entity_id),
                    required_capability: AffordanceCapability {
                        required_stats: Some(StatRequirements {
                            might: Some(30),
                            ..Default::default()
                        }),
                        required_skills: Some(SkillRequirements {
                            hunting: Some(20),
                            ..Default::default()
                        }),
                        energy_cost: 15.0,
                        time_cost_ticks: 600,  // 10 minutes
                    },
                    priority_tier: if entity.needs.hunger > 50 {
                        PriorityTier::High
                    } else {
                        PriorityTier::Medium
                    },
                    salience: 0.0,  // Will be calculated next
                    success_chance: 0.0,  // Will be calculated next
                });
            }
            
            // Or flee if prey is actually a predator
            if perceived.threat_level > 0.7 {
                affordances.push(Affordance {
                    name: "Flee".to_string(),
                    affordance_type: AffordanceType::Environmental,
                    target_entity_id: Some(perceived.entity_id),
                    required_capability: AffordanceCapability {
                        required_stats: Some(StatRequirements {
                            lithe: Some(25),
                            ..Default::default()
                        }),
                        energy_cost: 10.0,
                        time_cost_ticks: 300,
                    },
                    priority_tier: PriorityTier::Immediate,
                    salience: 0.0,
                    success_chance: 0.0,
                });
            }
        }
        
        EntityType::Sentient => {
            // Can speak, trade, or attack
            affordances.push(Affordance {
                name: "Speak".to_string(),
                affordance_type: AffordanceType::Social,
                target_entity_id: Some(perceived.entity_id),
                required_capability: AffordanceCapability {
                    required_stats: None,
                    required_skills: None,
                    energy_cost: 1.0,
                    time_cost_ticks: 60,
                },
                priority_tier: PriorityTier::Medium,
                salience: 0.0,
                success_chance: 0.0,
            });
            
            if entity.inventory.has_tradeable_items() {
                affordances.push(Affordance {
                    name: "Trade".to_string(),
                    affordance_type: AffordanceType::Social,
                    target_entity_id: Some(perceived.entity_id),
                    required_capability: AffordanceCapability {
                        required_stats: None,
                        required_skills: Some(SkillRequirements {
                            bartering: Some(10),
                            ..Default::default()
                        }),
                        energy_cost: 5.0,
                        time_cost_ticks: 300,
                    },
                    priority_tier: PriorityTier::Medium,
                    salience: 0.0,
                    success_chance: 0.0,
                });
            }
        }
        _ => {}
    }
    
    affordances
}

fn generate_resource_affordances(
    entity: &Entity,
    resource: &PerceivedResource,
) -> Vec<Affordance> {
    let mut affordances = Vec::new();
    
    match resource.resource_type {
        ResourceType::Water => {
            if entity.needs.thirst > 20 {
                affordances.push(Affordance {
                    name: "Drink".to_string(),
                    affordance_type: AffordanceType::Environmental,
                    target_entity_id: None,
                    required_capability: AffordanceCapability {
                        required_stats: None,
                        energy_cost: 1.0,
                        time_cost_ticks: 60,
                    },
                    priority_tier: if entity.needs.thirst > 75 {
                        PriorityTier::High
                    } else {
                        PriorityTier::Medium
                    },
                    salience: 0.0,
                    success_chance: 0.95,  // Drinking success rate
                });
            }
        }
        ResourceType::Plant => {
            if entity.species_id.contains("herbivore") && entity.needs.hunger > 20 {
                affordances.push(Affordance {
                    name: "Forage".to_string(),
                    affordance_type: AffordanceType::Environmental,
                    target_entity_id: None,
                    required_capability: AffordanceCapability {
                        required_stats: Some(StatRequirements {
                            vigil: Some(20),
                            ..Default::default()
                        }),
                        required_skills: Some(SkillRequirements {
                            forage: Some(15),
                            ..Default::default()
                        }),
                        energy_cost: 5.0,
                        time_cost_ticks: 300,
                    },
                    priority_tier: if entity.needs.hunger > 75 {
                        PriorityTier::High
                    } else {
                        PriorityTier::Medium
                    },
                    salience: 0.0,
                    success_chance: 0.0,
                });
            }
        }
        ResourceType::Mineral => {
            if entity.inventory.has_item("pickaxe") {
                affordances.push(Affordance {
                    name: "Mine".to_string(),
                    affordance_type: AffordanceType::Environmental,
                    target_entity_id: None,
                    required_capability: AffordanceCapability {
                        required_stats: Some(StatRequirements {
                            might: Some(30),
                            ..Default::default()
                        }),
                        required_skills: Some(SkillRequirements { /* ... */ }),
                        energy_cost: 20.0,
                        time_cost_ticks: 600,
                    },
                    priority_tier: PriorityTier::Low,  // Non-essential
                    salience: 0.0,
                    success_chance: 0.0,
                });
            }
        }
    }
    
    affordances
}
```

### Step 3: Filter by Capability

Remove affordances the entity cannot perform.

```rust
pub fn filter_by_capability(
    entity: &Entity,
    affordances: Vec<Affordance>,
) -> Vec<Affordance> {
    affordances
        .into_iter()
        .filter(|aff| {
            let cap = &aff.required_capability;
            
            // Check energy
            if entity.energy.current < cap.energy_cost {
                return false;
            }
            
            // Check stats
            if let Some(reqs) = &cap.required_stats {
                if let Some(required_might) = reqs.might {
                    if entity.stats.might < required_might as f32 {
                        return false;
                    }
                }
                // ... check other stats
            }
            
            // Check skills
            if let Some(reqs) = &cap.required_skills {
                if let Some(required_hunting) = reqs.hunting {
                    if entity.skills.hunting < required_hunting as f32 {
                        return false;
                    }
                }
                // ... check other skills
            }
            
            true  // Passed all checks
        })
        .collect()
}
```

### Step 4: Calculate Salience

Score each affordance on relevance **right now**.

**Formula (from systems-math.md):**

```
Salience = base_importance × (1 + need_alignment) × (1 + threat_level) × (1 + novelty)

Where:
  base_importance ∈ [0, 1] — inherent priority (food=0.8, idle=0.1)
  need_alignment ∈ [0, 1] — does this satisfy current top need? (0 if unrelated, 1 if perfect match)
  threat_level ∈ [0, 2] — environmental danger scaling
  novelty ∈ [0, 0.5] — bonus for unexperienced actions
```

```rust
pub fn calculate_salience(
    entity: &Entity,
    affordance: &Affordance,
) -> f32 {
    // 1. BASE IMPORTANCE: Affordance type matters
    let base_importance = match affordance.affordance_type {
        AffordanceType::Environmental => {
            match affordance.name.as_str() {
                "Drink" => 0.95,    // Critical for survival
                "Eat" => 0.90,
                "Rest" => 0.80,
                "Flee" => 0.99,     // Immediate threat response
                "Find_Shelter" => 0.85,
                "Climb" => 0.20,    // Optional exploration
                "Inspect" => 0.10,  // Curious but low priority
                _ => 0.50,
            }
        }
        AffordanceType::Social => {
            if entity.needs.social > 50 {
                0.70  // Social needs high
            } else {
                0.30  // Social optional
            }
        }
        _ => 0.50,
    };
    
    // 2. NEED ALIGNMENT: Does this address my top need?
    let top_need = entity.dominant_need();  // Hunger, thirst, fear, etc.
    let need_alignment = match (affordance.name.as_str(), top_need) {
        ("Drink", Need::Thirst) => 1.0,
        ("Eat" | "Forage", Need::Hunger) => 1.0,
        ("Rest", Need::Fatigue) => 1.0,
        ("Flee", Need::Fear) => 1.0,
        ("Speak" | "Trade", Need::Social) => 0.8,
        _ => 0.0,  // Doesn't address top need
    };
    
    // 3. THREAT LEVEL: Emergency multiplier
    let threat_level = if entity.perceives_predator() {
        2.0  // Threat doubles importance of survival actions
    } else if entity.in_combat() {
        1.5
    } else {
        1.0  // Safe
    };
    
    // 4. NOVELTY: Bonus for new/untried actions
    let times_performed = entity.action_history.count(&affordance.name);
    let novelty = if times_performed == 0 {
        0.5  // 50% bonus for never-done actions
    } else if times_performed < 5 {
        0.2  // 20% bonus for rarely-done
    } else {
        0.0  // No bonus for routine
    };
    
    // COMBINE
    let salience = base_importance * (1.0 + need_alignment) * (1.0 + threat_level) * (1.0 + novelty);
    salience.min(100.0)  // Cap at 100
}
```

### Step 5: Tier by Priority & Sort by Salience

Group affordances into priority tiers, then sort by salience within tiers.

```rust
pub fn organize_affordances_by_salience(
    entity: &Entity,
    mut affordances: Vec<Affordance>,
) -> OrganizedAffordances {
    // Calculate salience for all
    for aff in &mut affordances {
        aff.salience = calculate_salience(entity, aff);
    }
    
    // Determine priority tier for each (hybrid model)
    for aff in &mut affordances {
        aff.priority_tier = determine_priority_tier(entity, aff);
    }
    
    // Group by tier
    let mut organized = OrganizedAffordances::new();
    
    for aff in affordances {
        match aff.priority_tier {
            PriorityTier::Immediate => organized.immediate.push(aff),
            PriorityTier::High => organized.high.push(aff),
            PriorityTier::Medium => organized.medium.push(aff),
            PriorityTier::Low => organized.low.push(aff),
        }
    }
    
    // Sort within each tier by salience (highest first)
    for tier in [
        &mut organized.immediate,
        &mut organized.high,
        &mut organized.medium,
        &mut organized.low,
    ] {
        tier.sort_by(|a, b| b.salience.partial_cmp(&a.salience).unwrap());
    }
    
    organized
}

pub struct OrganizedAffordances {
    pub immediate: Vec<Affordance>,  // Threats, critical needs
    pub high: Vec<Affordance>,       // Important objectives
    pub medium: Vec<Affordance>,     // Normal activity
    pub low: Vec<Affordance>,        // Idle/exploration
}

pub fn determine_priority_tier(entity: &Entity, aff: &Affordance) -> PriorityTier {
    // Immediate: Threat or critical need
    if aff.name == "Flee" && entity.perceives_predator() {
        return PriorityTier::Immediate;
    }
    if aff.name == "Drink" && entity.needs.thirst > 80 {
        return PriorityTier::Immediate;
    }
    
    // High: Top need above threshold
    if aff.salience > 60.0 {
        return PriorityTier::High;
    }
    
    // Medium: Normal activity
    if aff.salience > 30.0 {
        return PriorityTier::Medium;
    }
    
    // Low: Exploration/idle
    PriorityTier::Low
}
```

---

## Affordance Resolution

When an entity decides to execute an affordance, it converts to an **Interaction** that the interaction engine resolves.

```rust
pub fn resolve_affordance_to_interaction(
    entity: &Entity,
    affordance: &Affordance,
) -> Interaction {
    let interaction_type = match affordance.affordance_type {
        AffordanceType::Environmental => match affordance.name.as_str() {
            "Drink" => InteractionType::Consumption,
            "Eat" | "Forage" => InteractionType::Consumption,
            "Mine" => InteractionType::Extraction,
            "Flee" => InteractionType::Environmental,
            _ => InteractionType::Environmental,
        },
        AffordanceType::Creature => match affordance.name.as_str() {
            "Hunt" => InteractionType::Combat,
            "Herd" => InteractionType::Social,
            "Breed" => InteractionType::Reproduction,
            _ => InteractionType::Environmental,
        },
        AffordanceType::Social => match affordance.name.as_str() {
            "Speak" | "Trade" => InteractionType::Social,
            "Attack" => InteractionType::Combat,
            _ => InteractionType::Social,
        },
        _ => InteractionType::Environmental,
    };
    
    Interaction {
        id: generate_unique_id(),
        source_entity_id: entity.id,
        target_entity_id: affordance.target_entity_id,
        interaction_type,
        priority: match affordance.priority_tier {
            PriorityTier::Immediate => InteractionPriority::Critical,
            PriorityTier::High => InteractionPriority::High,
            PriorityTier::Medium => InteractionPriority::Normal,
            PriorityTier::Low => InteractionPriority::Low,
        },
        cost: InteractionCost {
            energy: affordance.required_capability.energy_cost,
            time_ticks: affordance.required_capability.time_cost_ticks,
            risk_factor: affordance.name == "Hunt" as i32 as f32 * 0.3,
        },
        context: InteractionContext {
            environment_type: entity.current_biome.clone(),
            weather_conditions: entity.world.weather.current_condition(),
            temperature: entity.world.get_temperature_at(entity.position),
            proximity_distance: affordance
                .target_entity_id
                .map(|id| distance_to_entity(entity.id, id))
                .unwrap_or(0.0),
        },
    }
}
```

---

## Full Affordance Pipeline (Integration)

```rust
pub fn affordance_perception_loop(
    entity: &mut Entity,
    world: &World,
    tick: u64,
) {
    // This runs every 60 ticks (1 minute)
    if tick % 60 != 0 { return; }
    
    // STEP 1: Perception scan
    let snapshot = perception_scan(entity, world, tick);
    
    // STEP 2: Generate candidates
    let candidates = generate_affordances_for_snapshot(
        entity,
        &snapshot,
        &entity.beliefs,
    );
    
    // STEP 3: Filter by capability
    let executable = filter_by_capability(entity, candidates);
    
    // STEP 4–5: Calculate salience & organize
    let organized = organize_affordances_by_salience(entity, executable);
    
    // STORE for decision maker
    entity.perceived_affordances = organized;
}

// In decision-making phase:
pub fn make_decision(entity: &Entity) -> Option<Affordance> {
    // Try immediate tier first (threats)
    if let Some(aff) = entity.perceived_affordances.immediate.first() {
        return Some(aff.clone());
    }
    
    // Then high tier (important goals)
    if let Some(aff) = entity.perceived_affordances.high.first() {
        return Some(aff.clone());
    }
    
    // Then medium tier (routine)
    if let Some(aff) = entity.perceived_affordances.medium.first() {
        // 30% chance to do medium-tier action
        if random() < 0.3 {
            return Some(aff.clone());
        }
    }
    
    // Fall back to first low-tier (if very bored)
    if let Some(aff) = entity.perceived_affordances.low.first() {
        if random() < 0.1 {
            return Some(aff.clone());
        }
    }
    
    None  // No action chosen
}
```

---

## Affordance Decay & Forgetting

Affordances based on memory fade over time. If an entity doesn't revisit a location or creature, its affordance salience for that target decreases.

```rust
pub fn affordance_decay_on_perception_loss(
    entity: &Entity,
    affordance: &mut Affordance,
    ticks_since_last_perceived: u64,
) {
    // Memory-based affordances fade
    let decay_rate = 0.1;  // Decay 10% per hour
    let hours_since = ticks_since_last_perceived as f32 / 3600.0;
    
    affordance.salience *= (1.0 - decay_rate) .powf(hours_since);
    
    if affordance.salience < 1.0 {
        // Affordance is "forgotten"—can be discarded
    }
}
```

---

## Summary: Affordance System

1. **Perception** (every 60 ticks): Scan environment, create perceived world
2. **Generation**: For each perception, generate possible affordances
3. **Filter**: Remove affordances entity lacks capability for
4. **Salience**: Score each on relevance (base importance × need alignment × threat × novelty)
5. **Organize**: Group by priority tier, sort by salience
6. **Offer**: Present top affordances to decision system
7. **Resolve**: Chosen affordance converts to interaction
8. **Execute**: Interaction engine processes the action
9. **Decay**: Forgotten affordances fade from memory

This creates an **emergent action space** where entities discover what they can do based on their perception, capability, and current state—not from hardcoded scripts.

