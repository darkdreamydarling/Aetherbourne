# Aetherbourne Physiological Trait Impact Specification

This document details the numerical impact of various physiological traits on core game mechanics within Aetherbourne. The goal is to provide clear, codifiable definitions for how an entity's physical characteristics influence its survival, movement, and interaction with the environment.

## 1. Key Physiological Traits and Variations

Entities in Aetherbourne possess a range of physiological traits that define their physical capabilities and vulnerabilities. These traits are categorized and assigned specific values to allow for quantitative analysis and simulation.

### 1.1. Body Covering

Body covering is a primary determinant of an entity's thermal regulation and environmental resistance. It is represented by a `BodyCoveringType` enum and associated numerical properties.

```rust
pub enum BodyCoveringType {
    Fur,        // Excellent insulation, moderate protection
    Feathers,   // Good insulation, light protection
    Skin,       // Poor insulation, minimal protection
    Scales,     // Moderate insulation, good protection
    Exoskeleton,// Minimal insulation, excellent protection
}

pub struct BodyCoveringComponent {
    pub covering_type: BodyCoveringType,
    pub thickness: f32, // 0.0 - 1.0, influences insulation and protection
    pub density: f32,   // 0.0 - 1.0, influences weight and insulation
}
```

**Variations:**

*   **Fur:** Ranges from thin (e.g., deer) to thick (e.g., bear). `thickness` and `density` would vary.
*   **Feathers:** Ranges from sparse (e.g., chicken) to dense (e.g., arctic bird).
*   **Skin:** Can be bare, smooth, or slightly textured. Generally low `thickness` and `density`.
*   **Scales:** Can be small and flexible (e.g., snake) to large and rigid (e.g., dragon). `thickness` and `density` influence protection.
*   **Exoskeleton:** Hard, rigid outer layer (e.g., insect, crab). High `thickness` and `density` for protection.

### 1.2. Body Mass

Body mass significantly impacts an entity's movement capabilities, energy consumption, and physical resilience. It is represented by a numerical `mass_kg` value and categorized for ease of reference.

```rust
pub struct BodyMassComponent {
    pub mass_kg: f32, // Mass in kilograms
}
```

**Categories (for descriptive purposes, actual calculations use `mass_kg`):**

*   **Small:** `mass_kg` < `SMALL_MASS_THRESHOLD` (e.g., mouse, small bird)
*   **Medium:** `SMALL_MASS_THRESHOLD` <= `mass_kg` < `LARGE_MASS_THRESHOLD` (e.g., human, wolf)
*   **Large:** `mass_kg` >= `LARGE_MASS_THRESHOLD` (e.g., bear, elephant)

### 1.3. Body Shape/Proportion

While not explicitly requested, body shape and limb proportions can also play a role in movement and energy efficiency. For now, we'll keep it simple, but this is an area for future expansion.

```rust
pub enum BodyShapeType {
    Bipedal,    // Two legs, upright stance
    Quadrupedal,// Four legs, horizontal stance
    Serpentine, // No limbs, elongated body
    Avian,      // Wings for flight
    Aquatic,    // Fins for swimming
}

pub struct BodyShapeComponent {
    pub shape_type: BodyShapeType,
    pub limb_count: u8, // e.g., 2 for bipedal, 4 for quadrupedal
    pub wing_span_m: Option<f32>, // For Avian types
    pub fin_area_sqm: Option<f32>, // For Aquatic types
}
```

### 1.4. Metabolic Rate

Metabolic rate dictates how quickly an entity consumes energy and generates heat. It is influenced by `BodyMassComponent` and `BodyCoveringComponent`.

```rust
pub struct MetabolismComponent {
    pub base_metabolism_rate: f32, // Base energy consumption per tick
    pub current_activity_multiplier: f32, // Multiplier based on current action
    pub thermal_efficiency: f32, // 0.0 - 1.0, how efficiently heat is retained/generated
}
```

**Variations:**

*   **Endothermic (Warm-blooded):** Higher `base_metabolism_rate`, higher `thermal_efficiency`.
*   **Ectothermic (Cold-blooded):** Lower `base_metabolism_rate`, lower `thermal_efficiency` (rely on external heat).

This section lays the groundwork for the traits. The next phase will quantify their impact.

## 2. Quantifying Trait Impact on Game Mechanics

This section defines the numerical formulas that translate physiological traits into tangible effects on entity mechanics, such as need decay rates, movement speed, and damage resistance.

### 2.1. Impact of Body Covering

Body covering primarily affects an entity's **thermal regulation** (Warmth need) and **physical protection**.

#### 2.1.1. Warmth Need Decay Modifier

This modifier adjusts the `Warmth` need decay rate based on the `BodyCoveringType` and its properties, relative to the ambient temperature.

```rust
Warmth_Decay_Modifier = 1.0 - (Base_Insulation_Factor[covering_type] * covering.thickness * covering.density * THERMAL_EFFICIENCY_MULTIPLIER)

// Where:
//   Base_Insulation_Factor: Lookup table for each BodyCoveringType
//   covering.thickness: 0.0 - 1.0
//   covering.density: 0.0 - 1.0
//   THERMAL_EFFICIENCY_MULTIPLIER: A global constant for how effective insulation is overall
```

**Example:** An entity with thick fur in a cold environment will have a lower `Warmth_Decay_Modifier`, meaning their `Warmth` need decreases slower.

#### 2.1.2. Physical Protection (Damage Reduction)

This modifier reduces incoming physical damage based on the protective qualities of the body covering.

```rust
Damage_Reduction_Factor = Base_Protection_Factor[covering_type] * covering.thickness * PROTECTION_EFFICIENCY_MULTIPLIER

// Where:
//   Base_Protection_Factor: Lookup table for each BodyCoveringType
//   covering.thickness: 0.0 - 1.0
//   PROTECTION_EFFICIENCY_MULTIPLIER: A global constant for how effective physical protection is overall
```

**Example:** An entity with an exoskeleton will have a higher `Damage_Reduction_Factor` than one with bare skin.

### 2.2. Impact of Body Mass

Body mass affects **movement speed**, **fatigue generation**, and **carrying capacity**.

#### 2.2.1. Movement Speed Modifier

Heavier entities generally move slower, but this is also influenced by `Lithe` stat and `BodyShapeType`.

```rust
Movement_Speed_Modifier = 1.0 - (body_mass.mass_kg / MAX_MASS_FOR_SPEED_PENALTY) * MASS_SPEED_PENALTY_FACTOR

// This modifier is then applied to the base movement speed, potentially with Lithe stat influence:
// Final_Speed = Base_Species_Speed * Movement_Speed_Modifier * (1.0 + (entity.stats.lithe / 100.0) * LITHE_SPEED_BONUS)
```

**Example:** A `mass_kg` of 50 will have less penalty than a `mass_kg` of 200.

#### 2.2.2. Fatigue Need Decay Modifier

Larger entities consume more energy to move, leading to faster fatigue.

```rust
Fatigue_Decay_Modifier = 1.0 + (body_mass.mass_kg / BASE_MASS_FOR_FATIGUE) * MASS_FATIGUE_MULTIPLIER

// This modifier is applied to the base fatigue drain rate.
```

**Example:** A creature with `mass_kg = 200` will have a higher `Fatigue_Decay_Modifier` than one with `mass_kg = 50`.

#### 2.2.3. Carrying Capacity Bonus

Body mass directly contributes to an entity's ability to carry items, alongside the `Might` and `Vigor` stats (as per original documentation, Line 1237).

```rust
Carrying_Capacity_Bonus = body_mass.mass_kg * MASS_CARRY_CAPACITY_FACTOR

// Total Carrying Capacity = (BaseMight * 10) + Carrying_Capacity_Bonus
```

**Example:** A larger entity can carry more weight.

### 2.3. Impact of Metabolic Rate

Metabolic rate primarily influences **energy consumption** and **internal heat generation**.

#### 2.3.1. Energy Need Decay Modifier

This directly scales the `Energy` (or `Fatigue`) need decay rate.

```rust
Energy_Decay_Modifier = metabolism.base_metabolism_rate * metabolism.current_activity_multiplier

// This modifier is applied to the base energy drain rate.
```

**Example:** An endothermic creature will have a higher `Energy_Decay_Modifier` than an ectothermic one, requiring more food.

#### 2.3.2. Internal Heat Generation

Metabolism generates internal heat, which counteracts cold environments.

```rust
Internal_Heat_Generation = metabolism.base_metabolism_rate * metabolism.thermal_efficiency * HEAT_GENERATION_FACTOR

// This value is subtracted from the environmental cold penalty when calculating Warmth need decay.
```

**Example:** A warm-blooded entity with high `thermal_efficiency` can generate significant internal heat, slowing `Warmth` need decay in cold climates.

## 3. Consolidated Numerical Parameters and Coefficients

This section provides a single, comprehensive reference for all numerical parameters, thresholds, and multipliers defined for physiological traits.

### 3.1. Body Covering Constants

| Constant Name | Value | Description |
| :--- | :--- | :--- |
| `THERMAL_EFFICIENCY_MULTIPLIER` | `0.8` | Global effectiveness of insulation. |
| `PROTECTION_EFFICIENCY_MULTIPLIER` | `1.0` | Global effectiveness of physical protection. |

| `BodyCoveringType` | `Base_Insulation_Factor` | `Base_Protection_Factor` |
| :--- | :--- | :--- |
| `Fur` | `0.9` | `0.3` |
| `Feathers` | `0.7` | `0.2` |
| `Skin` | `0.1` | `0.1` |
| `Scales` | `0.5` | `0.7` |
| `Exoskeleton` | `0.2` | `0.9` |

### 3.2. Body Mass Constants

| Constant Name | Value | Description |
| :--- | :--- | :--- |
| `SMALL_MASS_THRESHOLD` | `10.0` | Mass (kg) below which an entity is considered 'Small'. |
| `LARGE_MASS_THRESHOLD` | `100.0` | Mass (kg) above which an entity is considered 'Large'. |
| `MAX_MASS_FOR_SPEED_PENALTY` | `500.0` | Maximum mass (kg) at which speed penalty caps. |
| `MASS_SPEED_PENALTY_FACTOR` | `0.7` | How much mass influences movement speed penalty (0-1). |
| `LITHE_SPEED_BONUS` | `0.01` | Bonus to movement speed per point of Lithe stat. |
| `BASE_MASS_FOR_FATIGUE` | `50.0` | Reference mass (kg) for base fatigue calculation. |
| `MASS_FATIGUE_MULTIPLIER` | `0.005` | How much mass influences fatigue decay rate. |
| `MASS_CARRY_CAPACITY_FACTOR` | `0.5` | How much mass (kg) contributes to carrying capacity. |

### 3.3. Metabolic Rate Constants

| Constant Name | Value | Description |
| :--- | :--- | :--- |
| `HEAT_GENERATION_FACTOR` | `10.0` | How much metabolism contributes to internal heat generation. |

