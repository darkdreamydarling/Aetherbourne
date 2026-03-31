# ⚙️ Systems Math — Canonical Formulas & Constants

This document defines every numerical calculation, constant, and mathematical rule in Aetherbourne. **This is the single source of truth for all numbers.** Every formula here is directly testable and implementable.

---

## 📊 Standard Scales & Normalization

All numerical systems are normalized to comparable ranges for consistency.

| System | Scale | Meaning | Examples |
|--------|-------|---------|----------|
| **Stats** | 0–100 | Capability dimension | Might 45, Lithe 78 |
| **Skills** | 0–100 | Practice/learning | Crafting 32, Combat 61 |
| **Needs** | 0–100 | Satisfaction deficit | Hunger 30, Thirst 65 |
| **Health** | 0–Max | Hit points | 85/100 |
| **Energy** | 0–Max | Stamina | 70/100 |
| **Durability** | 0–100 | Item condition | 75 (worn), 15 (damaged) |
| **Relationships** | –100 to +100 | Affinity | 65 (friend), –45 (enemy) |
| **Effectiveness** | 0–100 | Potency/quality | Tool quality 85, Weapon dmg 72 |
| **Probability** | 0.0–1.0 | Chance (0–100%) | Crit chance 0.15 = 15% |
| **Threat Level** | 0–1 | Danger scaling | Low 0.2, High 0.8 |

---

## 📐 Core Formulas

### Stat → Action Speed Modifier

**Purpose:** Higher stats make actions faster.

**Formula:**
```
SpeedMultiplier = 1.0 – (floor(Stat / 40) × 0.5)
ModifiedDuration = BaseDuration × SpeedMultiplier
```

**Breakdown:**
- Stat 0–39 → floor(X/40)=0 → multiplier = 1.0 (no speed bonus)
- Stat 40–79 → floor(X/40)=1 → multiplier = 0.5 (50% faster, half duration)
- Stat 80–100 → floor(X/40)=2 → multiplier = 0.0 (100% faster, instant)

**Examples:**
```
Walk (base 10 min) with Lithe 60:
  floor(60/40) = 1
  multiplier = 1.0 - (1 × 0.5) = 0.5
  modified duration = 10 × 0.5 = 5 minutes

Climb (base 15 min) with Vigor 85:
  floor(85/40) = 2
  multiplier = 1.0 - (2 × 0.5) = 0.0
  modified duration = 15 × 0.0 = 0 minutes (instant)
```

---

### Skill → Potency Multiplier

**Purpose:** Higher skills make effects stronger.

**Formula:**
```
PotencyMultiplier = 1.0 + (floor(Skill / 40) × 0.25)
ModifiedEffect = BaseEffect × PotencyMultiplier
```

**Breakdown:**
- Skill 0–39 → floor(X/40)=0 → multiplier = 1.0 (100% effect)
- Skill 40–79 → floor(X/40)=1 → multiplier = 1.25 (125% effect, 25% bonus)
- Skill 80–100 → floor(X/40)=2 → multiplier = 1.5 (150% effect, 50% bonus)

**Example:**
```
Cook with Cooking 65:
  floor(65/40) = 1
  potency multiplier = 1.0 + (1 × 0.25) = 1.25
  
  Base effect: +10% Hunger satisfaction
  Modified effect: +10% × 1.25 = +12.5% Hunger satisfaction
```

---

### Skill → Roll Bonus (d20 System)

**Purpose:** Higher skills improve success chance on checks.

**Formula:**
```
RollBonus = floor(Skill / 20)
Roll = d20 + RollBonus vs DC 12

Success Threshold:
  Roll ≥ DC → Success
  Roll < DC → Failure
```

**Breakdown:**
- Skill 0–19 → floor(X/20)=0 → +0 bonus
- Skill 20–39 → floor(X/20)=1 → +1 bonus
- Skill 40–59 → floor(X/20)=2 → +2 bonus
- Skill 80–100 → floor(X/20)=4 → +4 bonus

**Crit Range:**
- d20 roll = 20 (auto-success) → **Critical Success** (150% effect)
- d20 roll = 1 (auto-failure) → **Critical Failure** (50% effect)
- d20 roll ≥ DC → **Success** (100% effect)
- d20 roll < DC → **Failure** (75% effect)

**Example:**
```
Fishing with Fishing 55 vs DC 12 to catch fish:
  Roll Bonus = floor(55/20) = 2
  d20 roll: 15 (roll from random)
  Total: 15 + 2 = 17 ≥ 12 → Success!
  Effect: 100% fish catch
  
With Fishing 15 vs same DC:
  Roll Bonus = floor(15/20) = 0
  d20 roll: 9
  Total: 9 + 0 = 9 < 12 → Failure
  Effect: 75% fish catch (lower yield)
```

---

## 🧮 Needs System

### Need Decay Rules

Every need decreases on each simulation tick. Rates vary by need type and are modified by stats, environment, and activity.

**Base Decay Rates (% per tick):**

| Need | Base Rate | Modifiers | Notes |
|------|-----------|-----------|-------|
| **Hunger** | 0.5% | ×1.5 if running, ×0.6 if resting | Food decreases this |
| **Thirst** | 0.6% | ×2.0 in desert, ×0.3 in rain, ×0.8 cold | Water consumed to reduce |
| **Fatigue** | 0.4% | ×2.0 if active, ×–0.5 if rest (recovery!) | Rest recovers fatigue |
| **Warmth** | 0.3% | ×3.0 in snow, ×0.1 in hot biome | Fire/clothing reduces this |
| **Comfort** | 0.2% | ×2.0 in danger zones, ×0.1 in shelter | Safety/indoors reduces this |
| **Stress** | 0.25% | ×2.0 in combat, ×0.1 with allies | Companionship reduces stress |
| **Social** | 0.15% | ×2.0 if alone, ×0.0 with community | Socializing reduces this |
| **Fulfillment** | 0.1% | ×2.0 without purpose, ×0.0 if achieving goal | Achievement increases this |
| **Waste** | 0.4% | ×1.0 always | Hygiene/bathing reduces this |

**Stat Modifications:**
```
ActualDrainRate = BaseDrainRate
  × (1.0 – (Vigor / 100) × 0.5)  // Vigor reduces drain
  × (1.0 – (Grit / 100) × 0.3)   // Grit reduces stress drain (select needs)
  × EnvironmentMultiplier
  × ActivityMultiplier
```

**Example: Hunger in Running Action**
```
Base Hunger Drain: 0.5% per tick
Activity Multiplier: ×1.5 (running)
Entity Vigor: 70
Environmental Multiplier: 1.0 (neutral)

Actual Drain = 0.5 × 1.5 × (1 - (70/100) × 0.5) × 1.0
             = 0.75 × (1 - 0.35)
             = 0.75 × 0.65
             = 0.4875% per tick
```

---

### Satisfaction (Inverse of Need)

Need values represent **deficit**. When a need is satisfied, it **decreases** from 100 toward 0.

```
Satisfaction = 100 – Need
Satisfied when: Satisfaction > 80 (i.e., Need < 20)
Critical when: Satisfaction < 50 (i.e., Need > 50)
```

---

## ⚔️ Combat System

### Combat Damage Formula

**Formula:**
```
BaseDamage = (AttackerMight × SkillMultiplier) + WeaponBonus
DefenseReduction = DefenderDefense × ArmorMultiplier
FinalDamage = max(0, BaseDamage – DefenseReduction ± Variance)

Where:
  SkillMultiplier = Skill_Potency_Multiplier (from above)
  WeaponBonus = Weapon Effectiveness × 0.5
  ArmorMultiplier = (100 – ArmorRating) / 100  // Higher armor = more reduction
  Variance = Random(–10, +10)  // Per-hit randomness
```

**Breakdown:**
```
// Attacker stats & weapon
AttackerMight: 60
MeleeCombatSkill: 55

// Skill potency (from formula above)
SkillPotency = floor(55 / 40) = 1
PotencyMultiplier = 1 + (1 × 0.25) = 1.25

// Weapon
SwordEffectiveness: 75
WeaponBonus = 75 × 0.5 = 37.5

// Base damage
BaseDamage = (60 × 1.25) + 37.5 = 75 + 37.5 = 112.5

// Defender armor
ArmorRating: 30  // 30% reduction
ArmorMultiplier = (100 – 30) / 100 = 0.7
DefenderDefense: 40
DefenseReduction = 40 × 0.7 = 28

// Final
FinalDamage = 112.5 – 28 ± Random(–10, +10)
            = 84.5 ± [–10, +10]
            = Result: 74.5–94.5 damage
```

---

### Critical Hit System

```
CriticalChance = 0.05 + (Skill / 1000)  // Base 5% + 0.1% per skill point

If CriticalHit:
  FinalDamage = FinalDamage × 2.0  // Double damage on crit

If CriticalFail (d20 = 1):
  FinalDamage = FinalDamage × 0.5  // Half damage on crit fail
```

---

## 🌾 Growth & Decay

### Flora Growth Rate

**Formula:**
```
GrowthRate = BaseGrowth × SunlightFactor × WaterFactor × SoilQualityFactor × SeasonFactor

Where:
  BaseGrowth: 1% per tick (typical)
  SunlightFactor: 0.1 (shade) to 2.0 (full sun)
  WaterFactor: 0.5 (drought) to 1.5 (heavy rain)
  SoilQualityFactor: 0.7 (poor) to 1.3 (excellent)
  SeasonFactor: 0.1 (winter) to 2.0 (spring/summer)
```

**Example: Oak Tree in Spring**
```
Base Growth: 1% per day
Spring Factor: 1.5
Sunlight: Full sun = 1.5×
Water: Moderate rain = 1.0×
Soil: Good = 1.1×

Daily Growth = 1.0 × 1.5 × 1.5 × 1.0 × 1.1 = 2.475% per day
```

---

### Item Durability Loss (Decay)

**Formula:**
```
DurabilityLoss = BaseUseWear × MaterialFragility × EnvironmentalFactor × QualityInverse

Where:
  BaseUseWear: Item-category dependent (e.g., 0.5% per use for weapons)
  MaterialFragility: 0.5 (very durable, steel) to 2.0 (fragile, ceramic)
  EnvironmentalFactor: 1.0 (neutral) to 3.0 (extreme, humid→rust)
  QualityInverse: 100 / Quality (higher quality = slower decay)
```

**Example: Iron Sword in Humid Weather**
```
Base Use Wear: 0.5% per swing
Material (Iron): Fragility 1.2, but moderate rust rate
Environmental: Humid = ×1.8 rust multiplier
Quality: 85

Wear per Swing = 0.5 × 1.2 × 1.8 × (100/85)
               = 0.5 × 1.2 × 1.8 × 1.18
               ≈ 1.27% per swing

// Durability states trigger at:
// 100–70: Pristine
// 70–30: Worn (70% effectiveness)
// 30–1: Damaged (40% effectiveness)
// 0: Broken (unusable)
```

---

## 📦 Inventory & Weight System

### Carrying Capacity

**Formula:**
```
CarryingCapacity = BaseMight × 10  // kg

Where BaseMight = (Might + Vigor) / 2  // Physical strength

Encumbrance:
  0–100% → no penalty
  101–150% → Movement speed ×0.7, fatigue drain ×1.5
  151%+ → Cannot move
```

**Example:**
```
Entity with Might 50, Vigor 60:
  BaseMight = (50 + 60) / 2 = 55
  Capacity = 55 × 10 = 550 kg

Carrying 450 kg:
  Encumbrance % = (450 / 550) × 100 = 81.8%
  No penalty (under 100%)

Carrying 700 kg:
  Encumbrance % = (700 / 550) × 100 = 127.3%
  Penalties active: speed ×0.7, fatigue drain ×1.5
```

---

## 👁️ Perception Ranges

Perception improves with higher Vigil stat. All ranges calculated per modifier.

### Vision Range

**Formula:**
```
VisionRange = BaseRange × (1.0 + (Vigil / 100) × 0.1) × LightModifier × ObstacleModifier

Where:
  BaseRange: 30 tiles (default humanoid)
  Vigil: 0–100 stat
  LightModifier: 1.0 (daylight), 0.5 (twilight), 0.2 (moonlight), 0.05 (dark cave)
  ObstacleModifier: Reduced by cover (0.7 in forest, 0.4 in fog)
```

**Example: Human at Night with Vigil 70**
```
Base: 30 tiles
Vigil Bonus: 30 × (1 + (70/100) × 0.1) = 30 × 1.07 = 32.1 tiles
Light: Nighttime = ×0.2 = 6.42 tiles
Obstacles: Not in forest = ×1.0 = 6.42 tiles

Vision Range = 6.42 tiles (can see ~6 tiles at night)
```

### Hearing Range

**Formula:**
```
HearingRange = BaseRange × (1.0 + (Vigil / 100) × 0.08) × WindModifier × GeometryModifier

Where:
  BaseRange: 50 tiles (sound travels far)
  Vigil: 0–100 stat
  WindModifier: 0.5 (windy/masking), 1.5 (calm/echo)
  GeometryModifier: 1.5 (stone caves), 0.8 (open fields), 0.3 (dense forest)
```

---

### Smell Range

**Formula:**
```
SmellRange = BaseRange × (1.0 + (Vigil / 100) × 0.12) × SceneAgeDecay × WindDirectionFactor

Where:
  BaseRange: 40 tiles (depends on species)
  Vigil: 0–100 stat
  SceneAgeDecay: exponential decay (0.5 per hour = old scents fade)
  WindDirectionFactor: 2.0 (downwind), 0.2 (upwind), 0.5 (crosswind)
```

---

## 🎯 Action System

### Action Base Components

Every action has:

```
TimeCost (minutes) = ModifiedDuration  // Using stat → speed formula
EnergyCost = BaseCost × (1 – (Vigor / 200))  // Vigor reduces cost
SuccessChance = BaseChance + (Skill / 100)  // Skill improves success
EffectMagnitude = BaseEffect × PotencyMultiplier  // From skill formula
```

### Action Example: Cook

**Base Statistics:**
```
Base Duration: 10 minutes
Base Change: +10% Hunger satisfaction, –2% Fatigue
Primary Stat: Precision (Vigil + Lithe) / 2
Relevant Skill: Cooking
Base Success: DC 12
Energy Cost: 5
```

**Calculation with Entity (Precision 65, Cooking 45):**
```
// Speed modifier (using Precision as primary stat)
floor(65 / 40) = 1
SpeedMult = 1.0 – (1 × 0.5) = 0.5
ModifiedDuration = 10 × 0.5 = 5 minutes

// Potency modifier
floor(45 / 40) = 1
PotencyMult = 1.0 + (1 × 0.25) = 1.25
ModifiedChange = +10% × 1.25 = +12.5% Hunger, –2.5% Fatigue

// Success check
RollBonus = floor(45 / 20) = 2
d20 roll: 14
Total: 14 + 2 = 16 ≥ 12 → Success!
```

---

## 📊 Item Valuation

### Item Value Calculation

**Formula:**
```
FinalValue = BaseValue
  × QualityMultiplier
  × MaterialMultiplier
  × RarityMultiplier
  × UsabilityMultiplier
  × TimeSinceCreationMultiplier

Where:
  BaseValue: Item category base (10 for common tool, 100 for weapon)
  QualityMultiplier: (Quality / 50)  // 50 quality = 1x, 100 = 2x
  MaterialMultiplier: Material-dependent (gold: 2.0, copper: 0.6, iron: 1.0)
  RarityMultiplier: 1.0 (common), 1.5 (uncommon), 3.0 (rare), 10.0 (legendary)
  UsabilityMultiplier: 1.0 if usable, 0.3 if damaged
  TimeSinceCreationMultiplier: 1.0 + (YearsSinceCreation × 0.1)  // Heirlooms appreciate
```

**Example: Iron Sword**
```
Base Value: 100
Quality: 85 → QualityMult = 85/50 = 1.7
Material (Iron): 1.0×
Rarity: Rare → 3.0×
Usability: Pristine = 1.0×
Age: 5 years → 1.0 + 0.5 = 1.5×

Final Value = 100 × 1.7 × 1.0 × 3.0 × 1.0 × 1.5 = 765 currency units
```

---

## 👥 Relationship Affinity Changes

### Affinity Modification Events

When social interactions occur:

**Formula:**
```
AfinityDelta = BaseChange
  × (1.0 + Personality_Alignment_Bonus)
  × (1.0 + CulturalAlignmentBonus)
  × (1.0 + RecentHistoryModifier)

Where:
  BaseChange: +5 to +25 (depending on interaction type)
  PersonalityAlignment: [–0.5, +0.5] (opposites clash, similars bond)
  CulturalAlignment: [–0.3, +0.3] (shared values help)
  RecentHistoryModifier: [–0.5, +0.5] (past quarrels/cooperation linger)
```

**Example: Trade with Friend**
```
Base Affinity: +80 (friend level)
Interaction: Generous trade (base +10)
Personality Alignment: +0.2 (both pragmatic)
Cultural Alignment: +0.1 (same faction)
Recent History: –0.1 (minor argument 50 ticks ago)

Delta = 10 × (1 + 0.2) × (1 + 0.1) × (1 – 0.1)
      = 10 × 1.2 × 1.1 × 0.9
      = 11.88 ≈ +12

New Affinity = 80 + 12 = 92 (close friend)
```

---

## 📈 Skill Progression

### Skill Gain on Practice

**Formula:**
```
SkillGainPerAttempt = BaseGainRate × (1.0 + (BaseStat / 100)) × DifficultyFactor × SuccessFactor

Where:
  BaseGainRate: 0.1 (% skill per successful attempt)
  BaseStat: Relevant stat (e.g., Precision for Crafting)
  DifficultyFactor: 0.5 (easy) to 2.0 (very hard)
  SuccessFactor: 1.0 (success), 0.5 (failure), 0.0 (critical fail)

CapExponent: Skill does not cap linearly—diminishing returns apply
SkillGain reduces as you approach max: Multiplier = (100 / (100 + CurrentSkill)) × 0.1
```

**Example: Fishing Progression**
```
Current Skill: 35
Base Stat (Vigil): 65
Difficulty: Medium = 1.0×
Result: Success = 1.0×
BaseGain: 0.1

Raw Gain = 0.1 × (1 + 65/100) × 1.0 × 1.0 = 0.1 × 1.65 = 0.165%

Diminishing Returns: (100 / (100 + 35)) × 0.1 = 0.074 multiplier
Final Gain = 0.165 × 0.074 ≈ 0.012% per attempt

Over 1000 successful fishing actions: roughly +1.2% skill
```

---

## 🧬 Evolution & Adaptation

### Mutation Rate

**Formula:**
```
TraitValue_Next = TraitValue_Parent1 × 0.5 + TraitValue_Parent2 × 0.5

IF Random(0, 100) < MutationChance:
  MutationMagnitude = Random(–MutationVariance, +MutationVariance)
  TraitValue_Next = Clamp(TraitValue_Next + MutationMagnitude, 0, 100)
ELSE:
  TraitValue_Next = TraitValue_Next  // Straight inheritance
```

**Example: Predation Pressure on Speed Trait**
```
Parent 1 Speed: 70
Parent 2 Speed: 65
Base Inheritance: (70 + 65) / 2 = 67.5

Mutation Chance: 20%
Random: 15 (triggers mutation)
Variance: ±10
Mutation: +6

Child Speed: 67.5 + 6 = 73.5 (faster than parents!)
```

---

## Summary: Constant Reference Table

| Constant | Value | Usage |
|----------|-------|-------|
| **Stat Tier Step** | 40 | Every 40 stat pts = 1 tier |
| **Skill Multiplier** | 0.25 | Per tier for potency |
| **Roll DC (Standard)** | 12 | Default difficulty check |
| **Combat Variance** | ±10 | Damage randomness |
| **Crit Range** | Top 5% & bottom 5% | d20 = 20 or 1 |
| **Carrying Capacity** | Might × 10 | kg per unit strength |
| **Encumbrance Threshold** | 100% capacity | Movement penalty starts |
| **Vision Base** | 30 tiles | Default humanoid sight |
| **Hearing Base** | 50 tiles | Default humanoid hearing |
| **Smell Base** | 40 tiles | Default humanoid smell |
| **Critical Success** | 150% effect | Crit success outcome |
| **Success** | 100% effect | Normal success |
| **Failure** | 75% effect | Skill check failure |
| **Critical Failure** | 50% effect | Crit fail outcome |

---

## Special Multiplication Caps

Some systems have intentional caps to prevent runaway values:

```
SpeedMultiplier: Cannot exceed 1.0× (cap at instant)
EncumbrancePenalty: Cannot exceed 0.0× movement (cap at stop)
DamageMinimum: 0 (cannot deal healing damage combatively)
AfinityBounds: [–100, +100] (relationship caps)
NeedBounds: [0, 100] (needs always 0–100%)
QualityBounds: [0, 100] (items always 0–100%)
```

