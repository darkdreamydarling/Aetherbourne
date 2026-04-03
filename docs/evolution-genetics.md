# 🧬 Evolution & Genetics System

## Core Principle

**Evolution emerges from survival pressure** —entities adapt to environment through:

1. Trait inheritance via DNA
2. Mutation introduction
3. Differential reproduction (survivors breed more)
4. Population-level adaptation over generations

---

## Environmental Pressure Model

### Pressure Categories

**Predation Pressure**
* Driven by: Predator population & aggression
* Organism Response: Increase speed, defense, camouflage, group size
* Signature Adaptations: Horns, thick hide, sharp claws, pack mentality

**Starvation Pressure**
* Driven by: Food scarcity vs. population
* Organism Response: Increase metabolism efficiency, diversify diet, reduce body size
* Signature Adaptations: Faster digestion, omnivory, smaller frames

**Climate Pressure**
* Driven by: Temperature extremes, precipitation variance
* Organism Response: Thermoregulation, water conservation, migration, hibernation
* Signature Adaptations: Thicker fur, smaller ears (heat retention), reduced water needs

**Reproductive Pressure**
* Driven by: Mating competition, parental care demands
* Organism Response: Sexual display, larger bodies (dominance), faster breeding
* Signature Adaptations: Bright coloration, size dimorphism, elaborate courtship

**Disease/Parasite Pressure**
* Driven by: Disease prevalence in population
* Organism Response: Immune system strengthening, disease resistance
* Signature Adaptations: Behavioral avoidance, symbiotic relationships, immune markers

**Social Pressure** (for sentient societies)
* Driven by: Cooperative vs. competitive social structures
* Organism Response: Intelligence increase, moral capacity, group cohesion
* Signature Adaptations: Larger brains, verbal communication, cultural transfer

---

## DNA & Trait System

### Trait Categories

**Physical Traits**
* Body size
* Coloration
* Limb count
* Sensory acuity (vision, smell, hearing)
* Fur/scale/skin type
* Thermoregulation
* Movement speed
* Strength

**Behavioral Traits**
* Temperament (docile/aggressive/fearful)
* Social structure preference
* Diet specialization
* Activity cycle (nocturnal/diurnal)
* Territorial range
* Parental investment
* Reproduction rate

**Immunity Traits**
* Disease resistance
* Poison tolerance
* Parasite resistance

**Metabolic Traits**
* Hunger rate
* Water requirements
* Digestion speed
* Energy conversion efficiency

### Trait Encoding

Each trait has:

* Base Value (0–10 scale)
* Dominance (whether it overrides recessive traits)
* Mutation Chance (0–10%)
* Mutation Magnitude (0–5% shift)
* Pressure Sensitivity (how much relevant pressure affects heritability)

---

## Reproduction & Inheritance

### Sexual Reproduction

1. **Mating Decision**: Two entities form pair bond (personality + reproductive need compatibility)
2. **Offspring Generation**:
   - Inherit 50% of traits from each parent
   - Traits blend or express dominant alleles
   - Mutations introduce novelty
3. **Offspring Placement**: Born at parent location or nearest safe biome

### Asexual Reproduction

1. **Decision**: Single entity reaches reproduction threshold
2. **Cloning**: Offspring is genetic copy with small mutations
3. **Placement**: Near parent

### Inheritance Rules

**Dominant Traits:**
* Expressed even if inherited from one parent
* Example: Sharp claws (dominates over blunt)

**Recessive Traits:**
* Only expressed if inherited from both parents
* Example: Red coloration (requires both parents carry recessive)

**Blended Traits:**
* Average of both parents + mutation
* Example: Body size = (Parent1_size + Parent2_size) / 2 ± mutation

### Mutation

On each trait inheritance:

```
IF random(0–100) < MutationChance:
    newValue = inheritedValue ± random(0–MutationMagnitude)
    newValue = clamp(newValue, 0, 10)
ELSE:
    newValue = inheritedValue
```

---

## Population Feedback Loop

### Cycle

1. **Population Period (T₁)**:
   - Entities reproduce based on needs satisfaction
   - High reproduction in abundant conditions
   - Low/no reproduction in scarce conditions

2. **Pressure Emergence (T₁→T₂)**:
   - Population grows → competition increases
   - Resources deplete → starvation pressure rises
   - Breeding success drops

3. **Selection Effect (T₂)**:
   - Entities with high reproduction trait values breed more
   - Entities with high hunger efficiency reproduce despite scarcity
   - Entities with speed/defense survive predation better

4. **Population Adaptation (T₃)**:
   - Next generation inherits "survival" traits more frequently
   - Population-level averages shift toward pressure-relevant traits
   - Trait diversity may decrease (or stabilize around new norm)

5. **Pressure Feedback (T₃→T₁)**:
   - If adaptation successful → pressure reduces → population stabilizes
   - If adaptation insufficient → pressure continues/intensifies → population crashes
   - If conditions change → old adaptations become maladaptive

---

## Environmental Pressure Modifiers

### Pressure Calculation

```
Starvation Pressure = (population / available_food)
Predation Pressure = (predator_count * predator_efficiency) / (prey_count)
Climate Pressure = extremeness_of_temperature_variance
Disease Pressure = (infection_rate * virulence) / (population_immunity)
```

### Adaptation Rate Modifier

```
Adaptation_Rate = base_mutation_chance * (Pressure_Level / 10)
```

Higher pressure = faster mutation rate (more variation for selection to act upon)

---

## Niche Formation

### Ecological Niche

A species' **niche** = its role + resource use + environmental preference.

Examples:
* Apex predator niche: Large, fast, aggressive, nocturnal hunter
* Grazer niche: Medium herbivore, herd-living, fast reproduction
* Scavenger niche: Carrion diet, low-energy expenditure, disease-resistant
* Specialist niche: One specific food source + high efficiency (fragile if food vanishes)

### Niche Divergence

When pressure is extreme:
1. Individuals with divergent traits survive
2. Traits diverge across subpopulations
3. Reproductive isolation may emerge
4. Multiple niches within former single-species population

Example: Original population forced uphill by predators → High-altitude subpopulation evolves cold resistance, smaller offspring, slower reproduction → Eventually becomes reproductively isolated → New species.

---

## Environmental Instability Model

### Stability Levels

**Stable (Low Pressure)**
* Resources abundant
* Climate consistent
* Predators rare
* Result: Slow evolution, large population, trait diversity stable

**Unstable (Moderate-High Pressure)**
* Seasonal scarcity
* Climate variance
* Predators common
* Result: Faster evolution, population oscillates, dominant trait emerges

**Catastrophic (Extreme Pressure)**
* Famine / mass predation / climate crash
* Population collapse
* Result: Rapid evolution or extinction; survivors carry extreme trait values

### Evolutionary Response Time

* **Short-lived species** (rabbits): Adapt within 2–5 generations
* **Medium-lived species** (humans): Adapt within 10–50 generations
* **Long-lived species** (elephants): Adapt within 50–200+ generations

Generational time is determined by growth rate + maturity age.

---

## DNA Implementation Details

### Gene Structure

```
Gene {
    id: unique identifier
    name: trait name (e.g., "bite_force")
    base_value: 5.0
    dominance: 0.8 (0=recessive, 1=dominant)
    mutation_chance: 5 (%)
    mutation_magnitude: 2 (%)
    pressure_sensitivity: 0.6 (0=immune to selection, 1=highly sensitive)
    expression: current_value (0–10)
}
```

### Organism DNA

```
Organism.DNA = {
    alleles: [paternal_genes, maternal_genes],
    expressed_traits: [phenotype calculations],
    mutation_history: [recorded changes],
    lineage: [ancestor IDs]
}
```

### Phenotype Calculation

```
FOR each_trait:
    IF trait.dominance > random(0–1):
        expressed_value = max(allele1, allele2)  // Dominant
    ELSE:
        expressed_value = (allele1 + allele2) / 2  // Recessive/blended
    
    IF random(0–100) < mutation_chance:
        expressed_value ± random(0–mutation_magnitude)
    
    phenotype[trait] = clamp(expressed_value, 0, 10)
```

---

## Survival Formula (Evolution Context)

The probability an entity reproduces = function of:

```
Reproduction_Likelihood = 
    base_fertility *
    physiological_condition *
    (1 + environmental_resource_multiplier) *
    (1 - disease_resistance_impact) *
    (1 - predation_mortality_rate) *
    behavioral_trait_modifier
```

Where:

* **physiological_condition**: Health/Comfort/Warmth/Hunger/Thirst satisfaction [0–100%]
* **environmental_resource_multiplier**: +modifier if abundant, -modifier if scarce
* **disease_resistance_impact**: Reduces fertility if high Disease Pressure
* **predation_mortality_rate**: Reduces fertility if high Predation Pressure
* **behavioral_trait_modifier**: Temperament/intelligence/social structure impacts mating success

Entities with higher relevant trait values survive/breed more  
→ Trait frequency increases in next generation  
→ Population evolves

---

## Evolution Timescale Examples

### Fast Evolution (Starvation Pressure, Abundant Mutations)
* **Generation**: 5 years
* **Trait Lineage**: Ancestor (normal size) → Gen 2 (10% smaller) → Gen 5 (30% smaller) → Gen 10 (50% smaller efficient mini-species)
* **Timeframe**: ~50 years to significant adaptation

### Slow Evolution (Stable Environment, Low Mutation)
* **Generation**: 30 years
* **Trait Lineage**: Ancestor (baseline) → Gen 5 (5% variation) → Gen 20 (subtle refinement) → Gen 50+ (recognizable shift)
* **Timeframe**: 500+ years to significant adaptation

### Speciation (Niche Divergence + Reproductive Isolation)
* **Generation**: 10 years
* **Event**: Environmental split (mountain barrier) → Subpopulation 1 evolves cold-adapted → Subpop 2 evolves heat-adapted → After 100 years & reproductive isolation → Attempt interbreeding fails → New species
* **Timeframe**: 100–200 years (depending on pressure intensity)

## 1. The DNA Genome Structure

Every entity possesses a genome consisting of multiple **Genes**. Each Gene controls a specific trait or a set of related physiological properties.

### 1.1. Gene Definition

A Gene is the atomic unit of inheritance.

```rust
pub struct Gene {
    pub trait_id: String,          // e.g., "thermal_insulation", "body_mass", "metabolic_rate"
    pub allele_value: f32,         // 0.0 - 10.0 scale (normalized)
    pub dominance: f32,            // 0.0 (recessive) to 1.0 (fully dominant)
    pub mutation_chance: f32,      // 0.0 - 1.0 (e.g., 0.05 for 5%)
    pub mutation_magnitude: f32,   // 0.0 - 1.0 (max shift in allele_value)
    pub pressure_sensitivity: f32, // 0.0 - 1.0 (how much selection pressure affects this gene)
}
```

### 1.2. The Organism Genome

An organism's DNA consists of pairs of alleles (paternal and maternal).

```rust
pub struct Genome {
    pub alleles: Vec<(Gene, Gene)>, // Paternal and Maternal pairs
    pub mutation_history: Vec<MutationRecord>,
}
```

## 2. Physiological Expression (Phenotype)

The **Phenotype** is the expressed physical state of the entity, calculated from its DNA.

### 2.1. Trait Expression Formula

For each trait, the expressed value is determined by the dominance of the alleles and a blending factor.

```rust
pub fn calculate_phenotype_value(allele1: &Gene, allele2: &Gene) -> f32 {
    let expressed_value = if allele1.dominance > allele2.dominance {
        allele1.allele_value // Dominant allele 1
    } else if allele2.dominance > allele1.dominance {
        allele2.allele_value // Dominant allele 2
    } else {
        (allele1.allele_value + allele2.allele_value) / 2.0 // Blended/Recessive
    };
    
    expressed_value.clamp(0.0, 10.0)
}
```

### 2.2. Mapping DNA to Physiological Constants

The normalized `allele_value` (0-10) is mapped to the actual physiological constants used in the `Aetherbourne Physiological Trait Impact Specification`.

| Trait ID | Mapping to Physiological Property |
| :--- | :--- |
| `thermal_insulation` | Maps to `BodyCoveringComponent.thickness` (0.0 - 1.0) |
| `integument_density` | Maps to `BodyCoveringComponent.density` (0.0 - 1.0) |
| `body_mass_index` | Maps to `BodyMassComponent.mass_kg` (scaled to species range) |
| `metabolic_efficiency`| Maps to `MetabolismComponent.base_metabolism_rate` |
| `sensory_acuity` | Maps to `Vigil` stat and perception ranges |

**Example Mapping (Body Mass):**
`expressed_mass_kg = species_min_mass + (expressed_value / 10.0) * (species_max_mass - species_min_mass)`

## 3. Inheritance and Mutation

When two entities reproduce, the offspring's DNA is generated through recombination and mutation.

### 3.1. Recombination (Crossover)

The offspring inherits one allele from each parent for every gene.

```rust
pub fn inherit_dna(parent1: &Genome, parent2: &Genome) -> Genome {
    let mut offspring_alleles = Vec::new();
    for i in 0..parent1.alleles.len() {
        let p1_choice = if random() > 0.5 { &parent1.alleles[i].0 } else { &parent1.alleles[i].1 };
        let p2_choice = if random() > 0.5 { &parent2.alleles[i].0 } else { &parent2.alleles[i].1 };
        
        let mut child_allele1 = p1_choice.clone();
        let mut child_allele2 = p2_choice.clone();
        
        // Apply Mutation
        apply_mutation(&mut child_allele1);
        apply_mutation(&mut child_allele2);
        
        offspring_alleles.push((child_allele1, child_allele2));
    }
    Genome { alleles: offspring_alleles, mutation_history: Vec::new() }
}
```

### 3.2. Mutation Logic

Mutations introduce random variation into the gene pool.

```rust
pub fn apply_mutation(gene: &mut Gene) {
    if random() < gene.mutation_chance {
        let shift = (random() * 2.0 - 1.0) * gene.mutation_magnitude * 10.0;
        gene.allele_value = (gene.allele_value + shift).clamp(0.0, 10.0);
    }
}
```

## 4. Evolutionary Selection (Survival Pressure)

Survival is not guaranteed. Entities with traits poorly suited to their environment are less likely to reproduce.

### 4.1. The Survival Probability Formula

The probability of an entity surviving to reproductive age is a function of its physiological satisfaction and environmental pressure.

```rust
Survival_Prob = (1.0 - Hunger_Level/100) * (1.0 - Thirst_Level/100) * (1.0 - Warmth_Deficit/100) * (1.0 - Predation_Risk)
```

*   **Fur vs. Scales in Cold:** An entity with `covering_type: Scales` in a cold biome will have a high `Warmth_Deficit`, significantly lowering its `Survival_Prob`.
*   **Mass vs. Speed:** A high `body_mass_index` reduces `Movement_Speed_Modifier`, which increases `Predation_Risk` if predators are present.

### 4.2. Selection Constants

| Constant Name | Value | Description |
| :--- | :--- | :--- |
| `BASE_MUTATION_CHANCE` | `0.05` | 5% chance per gene per generation. |
| `BASE_MUTATION_MAGNITUDE` | `0.1` | 10% shift in allele value per mutation. |
| `PRESSURE_ADAPTATION_RATE` | `1.5` | Multiplier for mutation chance under extreme environmental pressure. |

## 5. Summary of DNA Options (Phenotype Mappings)

| Trait | Genetic Marker | Impact on Physiology |
| :--- | :--- | :--- |
| **Covering Type** | `integument_type` | Determines `Fur`, `Scales`, `Skin`, etc. (Discrete Alleles) |
| **Insulation** | `thermal_insulation`| Determines `thickness` of covering. |
| **Density** | `integument_density` | Determines `density` of covering. |
| **Mass** | `body_mass_index` | Determines total `mass_kg`. |
| **Metabolism** | `metabolic_rate` | Determines `base_metabolism_rate`. |
| **Efficiency** | `energy_efficiency` | Determines `thermal_efficiency` and energy drain. |

This specification ensures that the biological "hardware" of your entities is as dynamic and systemic as their cognitive "software." Every physical trait is traceable to a gene, and every gene is subject to the cold, hard logic of survival.

---

## Applications in Civilization

Evolution impacts civilization through:

1. **Gene Flow**: Cultural exchange → genetic exchange
2. **Environmental Pressure**: Civilizations modify environment → selective pressure on local fauna/flora/themselves
3. **Domestication**: Selective breeding of food animals → rapid evolution toward human needs
4. **Cultural Pressure**: Social structures reward/punish traits → genetic selection on intelligence, cooperation, aggression
5. **Sexual Selection**: Mate choice driven by culture → genetic change in aesthetically-favored traits (coloration, size, display)

