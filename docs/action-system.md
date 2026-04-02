# Action System
## Canonical Specification

**Purpose**: Bridges cognition → interaction pipeline. Converts decisions into atomic actions with costs, validation, execution, and skill progression. Fully specified action space.

### Core Data Structures (data-schema.md)

```rust
#[derive(Clone, Debug)]
pub struct Action {
    pub action_id: ActionId,
    pub source: EntityId,
    pub target: Option<EntityId>,
    pub interaction_type: InteractionType,
    pub cost: ActionCost,
    pub duration_ticks: u32,
    pub skill_required: f32,      // 0-100
    pub success_chance: f32,     // Derived from skill
}

#[derive(Clone, Debug)]
pub struct ActionCost {
    pub energy: f32,
    pub time_ticks: u32,
    pub focus: f32,              // Mental stamina 0-100
    pub risk_factor: f32,        // Chance of interruption
}

#[derive(Clone, Debug)]
pub enum ActionState {
    Planning, Queued, Executing, Success, PartialSuccess,
    Failure, CriticalFailure, Interrupted,
}
```

### Action Generation Pipeline (10 Steps)

```
1. Decision selects goal → affordance_system.md provides candidates
2. Filter by: capability, range, cost affordability
3. Score by utility: immediate_reward + long_term × discount_factor
4. Select highest utility action
5. Validate preconditions (range, LOS, target valid)
6. Calculate costs (energy/time/focus)
7. Queue for execution
8. Execute per tick (interruptible)
9. Resolve outcome via interaction-engine.md
10. Update skills/memory/reputation
```

### Utility Scoring Formula (systems-math.md)

```
immediate_utility = reward × (1.0 - estimated_cost / max_capacity)
long_term_utility = future_reward × discount_factor^(delay_ticks)

discount_factor = 0.99 per tick (temporal discounting)

total_utility = immediate × 0.7 + long_term × 0.3
```

**Example**: Eat berry (reward 20 hunger, cost 5 energy, 2 ticks delay)
```
immediate = 20 × (1 - 5/100) = 19.0
long_term = 0 (instant)
total = 19.0
```

### Action Cost Matrix (Per Type)

| Action Type | Energy | Focus | Risk | Base Duration |
|-------------|--------|-------|------|---------------|
| Move 1 tile | 2 + terrain | 0 | 0.05 | 10 ticks |
| Attack | 15 | 25 | 0.4 | 20 ticks |
| Craft | 10 | 40 | 0.1 | 300 ticks |
| Trade | 5 | 30 | 0.15 | 60 ticks |
| Harvest | 8 | 15 | 0.2 | 40 ticks |

### Success Resolution (4 Outcomes)

```
success_roll = skill_level + random(-25, +25)
difficulty = base_dc + circumstance_modifiers

Outcome:
- Critical Success (success_roll >= dc + 10): 150% effect
- Success (success_roll >= dc): 100% effect  
- Failure (success_roll < dc): 50% effect or nothing
- Critical Failure (success_roll <= dc - 10): negative effect
```

### Interruption & Recovery

```
interrupt_chance = risk_factor × external_threat_level
recovery_cost = interrupt_duration × 2 energy + 10 focus

mid-action interruption → action_state = Interrupted
queue partial results or full rollback
```

### Skill Progression Per Action

```
skill_gain = difficulty × success_factor × (100 - current_skill) / 500.0
max_skill = 100.0

success_factor:
- Crit Success: 2.0
- Success: 1.0  
- Failure: 0.3
- Crit Fail: 0.0
```

**Cap**: Max 2 skill points per 100 actions once >80 skill

### Action Chaining & Queuing

```
max_queue = 5 actions
replan_frequency = every 30 ticks or goal change

chain_trigger: action_success → immediate_followup_actions
```

### Integration Points

```
Cognition (decision.rs) → queries ActionSystem::get_available_actions()
ActionSystem → queues actions → executes → triggers InteractionEngine
InteractionEngine → resolves → ActionSystem updates skills/memory
AffordanceSystem → provides action candidates to ActionSystem
TimeSystem → advances action progress per tick
```

### Memory Update on Action Outcome

```
memory_weight = importance × recency_decay × emotional_intensity
new_memory = outcome_effect × memory_weight
store in episodic memory with action_id as key
```

### Verification Checklist
- [x] Utility formula complete & testable
- [x] All 4 outcomes have explicit effects
- [x] Cost matrix covers all interaction types
- [x] Interruption mechanics specified
- [x] Skill gain formula diminishing returns
- [x] Queue limits & replanning frequency
- [x] Full pipeline from decision → interaction

