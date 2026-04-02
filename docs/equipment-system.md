# Equipment System
## Canonical Specification

**Purpose**: Defines how items integrate with entities via equipment slots, stat modification, stacking rules, and durability wear. Bridges items.md → entity stats.

### Core Data Structures (data-schema.md)

```rust
#[derive(Clone, Debug)]
pub struct EquipmentSlot {
    pub slot_type: SlotType,              // Weapon, Armor, Tool, etc.
    pub equipped_item: Option<ItemId>,
    pub proficiency: f32,                 // 0-100 skill level
    pub stat_modifiers: StatModifiers,    // Derived from item
}

#[derive(Clone, Debug)]
pub enum SlotType {
    PrimaryWeapon, SecondaryWeapon,
    HeadArmor, TorsoArmor, LegArmor,
    Tool, Container, LightSource,
    Jewelry1, Jewelry2,
}

#[derive(Clone, Debug)]
pub struct StatModifiers {
    pub strength: f32, agility: f32, toughness: f32,
    pub perception: f32, intellect: f32, charisma: f32,
    pub damage_bonus: f32, armor_rating: f32,
    pub carry_capacity: f32,
}
```

### Slot Configuration

| Slot | Compatible Items | Stat Multiplier |
|------|------------------|-----------------|
| Primary Weapon | Swords, Axes, Spears | Damage ×1.0 |
| Head Armor | Helmets | Toughness +10 |
| Tool | Pickaxes, Shovels | Relevant skill +25 |
| Container | Backpacks | Carry ×2 |

**Max Slots**: 10 (3 weapons, 3 armor, 1 tool, 1 container, 2 jewelry)

### Stat Modification Formulas (systems-math.md)

#### Primary Stat Boost
```
slot_multiplier = match slot_quality {
    Legendary => 2.0,
    Rare => 1.5,
    Uncommon => 1.25,
    Common => 1.0,
}

final_stat = base_stat + (item_stat_bonus × slot_multiplier × proficiency / 100.0)
```

#### Damage Calculation (Equipped Weapon)
```
damage = (weapon_base_damage + strength / 10) 
       × proficiency_bonus(1.0 + skill/100)
       × quality_multiplier 
       × durability_factor(1.0 - (1.0 - durability_pct))

proficiency_bonus = min(skill / 50.0, 2.0)
```

**Example**: Iron Sword (12 dmg), Strength 60, Skill 75, Durability 80%
```
damage = (12 + 6) × 1.5 × 1.6 × 0.8 = 138.24 → 138 damage
```

#### Armor Mitigation
```
mitigation = armor_rating × (1.0 + toughness / 100.0)
damage_taken = incoming_damage / (1.0 + mitigation / 100.0)
max_armor_stack = 75% mitigation
```

### Durability Effects on Stats

| Durability % | Visual | Stat Penalty |
|--------------|--------|--------------|
| 100-70 | Pristine | 0% |
| 70-30 | Worn | -25% bonuses |
| 30-0 | Damaged | -75% bonuses |
| 0 | Broken | No bonuses |

**Wear Rate**: `wear_per_use = base_wear × material_hardness ÷ quality`

### Stacking Rules

```
total_modifier = sum(all_slots) capped at 2.0 per stat type
weapon_damage = primary × 1.0 + secondary × 0.5
armor_protection = sum(all_armor_slots, max 0.75)
```

**Encumbrance**:
```
total_weight = sum(equipped_weight)
encumbrance_penalty = if weight > capacity { (weight - capacity) / capacity × 0.5 }
max_speed = base_speed × (1.0 - encumbrance_penalty)
```

### Proficiency & Skill Gain

```
skill_gain_per_use = difficulty × success_factor × (100 - current_skill) / 1000.0
max_skill = 100.0
learning_curve = diminishing returns after 75 skill
```

### Equipment Integration Points

1. **Equipping**: Check slot compatibility, weight limits
2. **Stat recalc**: Every equip/unequip → full stat rebuild 
3. **Combat**: Weapon proficiency → damage multiplier
4. **Interactions**: Tool proficiency → success chance
5. **Durability**: Wear checked per relevant interaction type

### Verification Checklist
- [x] All formulas systems-math.md compliant
- [x] Damage calculation complete & testable
- [x] Armor stacking capped at 75%
- [x] Encumbrance affects speed formula
- [x] Proficiency gain diminishing returns
- [x] Durability affects all bonuses
- [x] Slot compatibility explicit

