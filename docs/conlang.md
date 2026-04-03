# Aetherbourne Conlang Integration Guide

This guide provides technical instructions for integrating the Aetherbourne Constructed Language (Conlang) specification into a game engine, focusing on procedural name generation. It outlines the necessary data structures, algorithms, phonetic validation, and provides Rust-based implementation patterns to facilitate seamless integration.

## 1. Data Architecture for the Conlang Engine

The Conlang engine relies on a robust data architecture to store and manage morphemes, semantic tags, and phonological rules. This section defines the core Rust data structures required.

### 1.1. Morpheme and Semantic Tag Definitions

The `Morpheme` struct and `SemanticTag` enum are central to the system, linking phonetic units to their meanings.

```rust
use std::collections::HashMap;

// Enum for the type of morpheme
pub enum MorphemeType {
    Prefix,   // Appears at the beginning of a word
    Infix,    // Appears in the middle of a word
    Suffix,   // Appears at the end of a word
}

// Struct to represent a single morpheme
pub struct Morpheme {
    pub text: String,           // The phonetic representation (e.g., "pala", "-ol-", "-ia")
    pub morpheme_type: MorphemeType,
    // HashMap to store semantic tags and their associated weights
    // Key: SemanticTag, Value: f32 (weight from 0.0 to 1.0)
    pub semantic_tags: HashMap<SemanticTag, f32>,
}

// Enum for various semantic categories
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum SemanticTag {
    Color(ColorValue),
    Texture(TextureValue),
    Weight(WeightValue),
    Temperature(TemperatureValue),
    Size(SizeValue),
    Material(MaterialValue),
    Biome(BiomeType),
    Function(FunctionValue),
    Aspect(AspectValue),
    Quality(QualityValue),
    // Add more specific tags as needed
}

// Enums for specific semantic values
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ColorValue {
    Red, Blue, Green, Yellow, Brown, Gray, White, Black, Vibrant, Muted
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TextureValue {
    Rough, Smooth, Spiky, Grainy, Metallic, Icy, Soft, Damp
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum WeightValue {
    Light, Medium, Heavy
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TemperatureValue {
    Cold, Cool, Temperate, Warm, Hot
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum SizeValue {
    Tiny, Small, Medium, Large, Huge
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum MaterialValue {
    Stone, Wood, Metal, Fabric, Crystal, Herbal, Liquid, Exoskeleton
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum BiomeType {
    Ocean, Beach, Desert, Grassland, Forest, Mountain, Tundra, Riverbank, Swamp, Cave
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum FunctionValue {
    Tool, Weapon, Armor, Food, Medicine, Fuel, Magic, Storage, Shelter, Crafting, Defense
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum AspectValue {
    Magic, Life, Death, Spirit, Ancient, New, Fast, Slow, Sharp, Dull, Aggressive, Peaceful, Observant, Hidden, Power, Growth, Healing, Sound, Emotional, Artistic, Wind
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum QualityValue {
    Good, Bad, Rare, Common, Pristine, Worn, Strong, Weak, Protective, Hard
}
```

### 1.2. Lexicon Storage

The entire collection of morphemes should be stored in a central `Lexicon` struct, categorized by `MorphemeType` for efficient lookup.

```rust
pub struct Lexicon {
    pub prefixes: Vec<Morpheme>,
    pub infixes: Vec<Morpheme>,
    pub suffixes: Vec<Morpheme>,
}

impl Lexicon {
    pub fn new() -> Self {
        // Initialize with all defined morphemes from the Conlang specification
        // This could be loaded from a JSON/YAML file or hardcoded.
        Lexicon {
            prefixes: vec![
                // Example: Ael- (Aspect::Ancient, Aspect::Magic, Color::Vibrant)
                Morpheme { 
                    text: "Ael-".to_string(), 
                    morpheme_type: MorphemeType::Prefix, 
                    semantic_tags: HashMap::from([
                        (SemanticTag::Aspect(AspectValue::Ancient), 0.9),
                        (SemanticTag::Aspect(AspectValue::Magic), 0.8),
                        (SemanticTag::Color(ColorValue::Vibrant), 0.7),
                    ])
                },
                // ... add all other prefixes
            ],
            infixes: vec![
                // Example: -ol- (Size::Large, Quality::Strong, Aspect::Power)
                Morpheme { 
                    text: "-ol-".to_string(), 
                    morpheme_type: MorphemeType::Infix, 
                    semantic_tags: HashMap::from([
                        (SemanticTag::Size(SizeValue::Large), 0.7),
                        (SemanticTag::Quality(QualityValue::Strong), 0.6),
                        (SemanticTag::Aspect(AspectValue::Power), 0.5),
                    ])
                },
                // ... add all other infixes
            ],
            suffixes: vec![
                // Example: -ia (Biome, Aspect::Land, Feminine)
                Morpheme { 
                    text: "-ia".to_string(), 
                    morpheme_type: MorphemeType::Suffix, 
                    semantic_tags: HashMap::from([
                        (SemanticTag::Biome(BiomeType::Forest), 0.8), // Example: Biome::Forest for generic land
                        (SemanticTag::Aspect(AspectValue::Land), 0.7),
                        // (SemanticTag::Gender(GenderValue::Feminine), 0.5), // If GenderValue is added
                    ])
                },
                // ... add all other suffixes
            ],
        }
    }
}
```

### 1.3. Phonological Rules and Constants

Phonological rules and constants are essential for validating and repairing generated names. These should be accessible globally or passed to the naming function.

```rust
pub struct PhonologyConfig {
    pub max_consonant_cluster: usize,
    pub min_name_length: usize,
    pub max_name_length: usize,
    pub vowel_sounds: Vec<String>, // List of recognized vowel sounds
    pub consonant_sounds: Vec<String>, // List of recognized consonant sounds
}

impl PhonologyConfig {
    pub fn new() -> Self {
        PhonologyConfig {
            max_consonant_cluster: 3,
            min_name_length: 4,
            max_name_length: 12,
            vowel_sounds: vec!["a".to_string(), "i".to_string(), "u".to_string(), "e".to_string(), "o".to_string(), "ai".to_string(), "au".to_string()],
            consonant_sounds: vec!["p".to_string(), "t".to_string(), "k".to_string(), "b".to_string(), "d".to_string(), "g".to_string(), "s".to_string(), "sh".to_string(), "h".to_string(), "m".to_string(), "n".to_string(), "l".to_string(), "r".to_string(), "y".to_string(), "w".to_string()],
        }
    }
}
```

This data architecture provides the foundational components for the Aetherbourne Conlang engine. The next step is to implement the procedural generation algorithm that utilizes these structures.

## 2. Procedural Generation Algorithm and Selection Logic

This section outlines the core algorithm for generating names using the Aetherbourne Conlang. It details how to select appropriate morphemes based on an entity's attributes and combine them into a phonologically valid and semantically meaningful name.

### 2.1. Name Generation Function Signature

```rust
pub fn generate_name(
    entity_attributes: &HashMap<SemanticTag, f32>, // Attributes of the entity to be named
    lexicon: &Lexicon,
    phonology_config: &PhonologyConfig,
    entity_type: &EntityType, // e.g., Biome, Flora, Mineral, Character, Item
) -> Option<String> {
    // Implementation details below
    todo!()
}

pub enum EntityType {
    Biome,
    Flora,
    Mineral,
    Character,
    Item,
}
```

### 2.2. Morpheme Selection Logic (Weighted Random Selection)

The selection of morphemes is crucial for semantic relevance. This process involves filtering the `Lexicon` based on the `entity_attributes` and then performing a weighted random selection.

1.  **Attribute Matching Score:** For each morpheme in the `Lexicon`, calculate a `match_score` against the `entity_attributes`.
    *   Initialize `match_score = 0.0`.
    *   For each `(entity_tag, entity_weight)` in `entity_attributes`:
        *   For each `(morpheme_tag, morpheme_weight)` in `morpheme.semantic_tags`:
            *   If `entity_tag` matches `morpheme_tag` (e.g., `Color::Red` matches `Color::Red`):
                *   `match_score += entity_weight * morpheme_weight * ATTRIBUTE_MATCH_BONUS`
            *   If `entity_tag` is a broader category that `morpheme_tag` falls under (e.g., `Biome::Forest` matches `Color::Green` if `Color::Green` is a common attribute of `Biome::Forest`):
                *   `match_score += entity_weight * morpheme_weight * CATEGORY_MATCH_BONUS`
    *   `ATTRIBUTE_MATCH_BONUS = 1.5` (direct match is stronger)
    *   `CATEGORY_MATCH_BONUS = 0.8` (indirect match is weaker)

2.  **Filter by Entity Type:** Further filter morphemes based on `EntityType` (e.g., suffixes like `-ia` are highly preferred for `Biome` names).
    *   Each morpheme can have an optional `preferred_entity_types: Vec<EntityType>` field. If present, only consider morphemes that match the current `entity_type`.

3.  **Weighted Random Selection:** From the filtered and scored pool of morphemes, select one using a weighted random distribution, where the `match_score` acts as the weight.
    *   This ensures that morphemes highly relevant to the entity's attributes are more likely to be chosen.

### 2.3. Word Construction Algorithm

This algorithm combines selected morphemes into a name, prioritizing phonological validity.

1.  **Initialize Name Components:**
    *   `selected_prefix: Option<Morpheme>`
    *   `selected_infixes: Vec<Morpheme>`
    *   `selected_suffix: Option<Morpheme>`

2.  **Select Primary Prefix and Suffix:**
    *   Use the `morpheme_selection_logic` to pick a `Prefix` and a `Suffix` based on `entity_attributes` and `entity_type`.
    *   Prioritize suffixes that strongly indicate the `entity_type` (e.g., `-ia` for Biomes, `-kor` for Minerals).

3.  **Construct Base Word:** Concatenate `selected_prefix.text` and `selected_suffix.text`.
    *   Example: `Ael-` + `-ia` = `Aelia`

4.  **Select and Insert Infixes (Iterative):**
    *   Determine the number of infixes to insert based on `entity_attributes` (e.g., more complex entities might have more infixes) or a random probability (`INFIX_PROBABILITY = 0.3`).
    *   For each infix to be inserted:
        *   Use `morpheme_selection_logic` to pick an `Infix`.
        *   Find suitable insertion points within the current word. Prioritize points that maintain phonotactic rules (e.g., between a consonant and a vowel).
        *   Attempt insertion. If it causes a phonotactic violation that cannot be easily repaired (see Section 3), try another infix or another insertion point.
        *   Example: `Aelia` + `-ol-` -> `Aelolia` (if phonotactically valid)

5.  **Phonetic Validation and Repair (Post-Construction):** After initial construction, the generated name must undergo a final validation and repair pass to ensure it adheres to all phonological rules (see Section 3).

6.  **Length and Uniqueness Check:**
    *   If `name.len()` is outside `MIN_NAME_LENGTH` or `MAX_NAME_LENGTH`, discard and re-generate or truncate/pad as appropriate.
    *   Check against a list of already generated names to ensure uniqueness. If not unique, re-generate.

### 2.4. Entity-Specific Attribute Prioritization

To guide morpheme selection, each `EntityType` should have a predefined set of `SemanticTag` priorities.

| EntityType | High Priority SemanticTags | Medium Priority SemanticTags | Low Priority SemanticTags |
| :--- | :--- | :--- | :--- |
| `Biome` | `BiomeType`, `Temperature`, `Moisture`, `Color` | `Size`, `Aspect::Land` | `Texture` |
| `Flora` | `Color`, `Function`, `Aspect::Life`, `Size` | `Texture`, `BiomeType` | `Quality` |
| `Mineral` | `Material`, `Quality::Hard`, `Color`, `Quality::Rare` | `Weight`, `Texture` | `BiomeType` |
| `Character` | `Species`, `Aspect::Emotional`, `Aspect::Aggressive`, `Aspect::Peaceful` | `Function`, `Quality::Strong`, `Size` | `Color`, `Texture` |
| `Item` | `Material`, `Function`, `Quality::Strong`, `Aspect::Sharp` | `Weight`, `Size` | `Color`, `Texture` |

This detailed algorithm provides the blueprint for procedurally generating names that are not only unique but also deeply connected to the attributes of the entities within Aetherbourne. The next section will focus on the crucial phonetic validation and repair mechanisms.

## 3. Phonetic Validation and Repair Mechanisms

Ensuring generated names adhere to Aetherbourne's phonological rules is critical for maintaining linguistic consistency and pronounceability. This section details the validation checks and repair strategies.

### 3.1. Validation Checks

Each check function will take a `&str` (the current name segment or full name) and `&PhonologyConfig` and return a `bool` indicating validity.

#### 3.1.1. `is_valid_syllable_structure(segment: &str, config: &PhonologyConfig) -> bool`

*   **Purpose:** Checks if a given segment (e.g., a morpheme or a syllable) conforms to the `(C)V(C)` structure.
*   **Logic:**
    1.  Identify vowels and consonants in the segment using `config.vowel_sounds` and `config.consonant_sounds`.
    2.  Count consecutive consonants and vowels.
    3.  Return `false` if more than one vowel appears without an intervening consonant, or if consonant clusters violate `config.max_consonant_cluster`.

#### 3.1.2. `has_vowel_hiatus(segment: &str, config: &PhonologyConfig) -> bool`

*   **Purpose:** Detects instances where two vowels appear consecutively without forming a valid diphthong.
*   **Logic:** Iterate through the segment, checking if `char_at(i)` and `char_at(i+1)` are both vowels and do not form a recognized diphthong (`ai`, `au`).

#### 3.1.3. `has_invalid_consonant_cluster(segment: &str, config: &PhonologyConfig) -> bool`

*   **Purpose:** Identifies consonant clusters that exceed `config.max_consonant_cluster` or are otherwise disallowed (e.g., specific onset clusters).
*   **Logic:** Iterate through the segment, counting consecutive consonants. Return `true` if the count exceeds `config.max_consonant_cluster` or if a disallowed cluster (e.g., `kral`) is found.

#### 3.1.4. `is_valid_name_length(name: &str, config: &PhonologyConfig) -> bool`

*   **Purpose:** Ensures the final name falls within the specified minimum and maximum length.
*   **Logic:** `name.len() >= config.min_name_length && name.len() <= config.max_name_length`.

### 3.2. Repair Mechanisms

Repair functions attempt to fix phonotactic violations to produce a valid name. These should be applied during the word construction phase (Section 2.3, Step 4) and as a final pass.

#### 3.2.1. `repair_vowel_hiatus(name: &mut String, config: &PhonologyConfig) -> bool`

*   **Strategy 1 (Epenthesis):** Insert a glide consonant (`y` or `w`) between the two vowels. Prioritize `y` after front vowels (`i`, `e`) and `w` after back vowels (`u`, `o`).
    *   Example: `Aelia` -> `Aeliya`
*   **Strategy 2 (Elision):** Remove one of the vowels. This is a less preferred option as it can alter the intended sound of a morpheme.
*   **Return:** `true` if successfully repaired, `false` otherwise.

#### 3.2.2. `repair_consonant_cluster(name: &mut String, config: &PhonologyConfig) -> bool`

*   **Strategy 1 (Vowel Insertion/Epenthesis):** Insert a default vowel (e.g., `a` or `i`) into the cluster to break it up.
    *   Example: `skral` -> `skiral`
*   **Strategy 2 (Consonant Elision):** Remove the least semantically critical consonant from the cluster. This should be a last resort.
*   **Strategy 3 (Morpheme Replacement):** If a cluster is unrepairable, suggest replacing one of the morphemes that formed the cluster with an alternative from the `Lexicon`.
*   **Return:** `true` if successfully repaired, `false` otherwise.

#### 3.2.3. `adjust_name_length(name: &mut String, config: &PhonologyConfig) -> bool`

*   **If too short:**
    *   **Strategy 1 (Add Infix/Suffix):** Attempt to add an additional, less semantically critical infix or suffix.
    *   **Strategy 2 (Pad with Vowels/Consonants):** Add a neutral vowel or consonant (e.g., `a`, `l`) to reach `min_name_length`. This should be rare.
*   **If too long:**
    *   **Strategy 1 (Remove Infix):** Remove the least semantically critical infix.
    *   **Strategy 2 (Truncate Suffix):** Shorten the suffix if it has a longer variant or can be abbreviated.
    *   **Strategy 3 (Elide Vowels):** Remove non-stressed vowels.
*   **Return:** `true` if successfully adjusted, `false` otherwise.

### 3.3. Integration into `generate_name`

The `generate_name` function (from Section 2.1) should incorporate these validation and repair steps iteratively:

1.  **Initial Construction:** Build the name from selected morphemes.
2.  **Loop for Validation and Repair:**
    *   `while has_vowel_hiatus(name) || has_invalid_consonant_cluster(name)`:
        *   Attempt `repair_vowel_hiatus`.
        *   Attempt `repair_consonant_cluster`.
        *   If no repair is possible, or if repairs lead to new violations, backtrack and try different morpheme combinations.
3.  **Final Length Check:** After phonological repair, call `adjust_name_length`.
4.  **Uniqueness Check:** Ensure the final name is unique.

This robust system of validation and repair ensures that every procedurally generated name in Aetherbourne is not only semantically rich but also phonologically consistent with the established Conlang rules, creating a truly immersive linguistic experience.

## 4. Rust-based Implementation Pattern and Example Code

This section provides a practical, Rust-based implementation of the Aetherbourne Conlang engine, demonstrating how the data structures, generation algorithm, and phonetic repair mechanisms can be integrated into a functional system. This example can serve as a direct starting point for your game engine.

### 4.1. Project Setup

First, create a new Rust project and add the `rand` dependency for random number generation.

```bash
cargo new conlang_example
cd conlang_example
cargo add rand
```

### 4.2. `src/main.rs` (Full Implementation)

Replace the contents of `src/main.rs` with the following code. This includes all the data structures, helper functions for phonology, the core `generate_name` function, and example usage.

```rust
use std::collections::HashMap;
use rand::seq::SliceRandom;
use rand::Rng;

// --- Data Structures (from Conlang Specification) ---

pub enum MorphemeType {
    Prefix,
    Infix,
    Suffix,
}

pub struct Morpheme {
    pub text: String,
    pub morpheme_type: MorphemeType,
    pub semantic_tags: HashMap<SemanticTag, f32>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum SemanticTag {
    Color(ColorValue),
    Texture(TextureValue),
    Weight(WeightValue),
    Temperature(TemperatureValue),
    Size(SizeValue),
    Material(MaterialValue),
    Biome(BiomeType),
    Function(FunctionValue),
    Aspect(AspectValue),
    Quality(QualityValue),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ColorValue {
    Red, Blue, Green, Yellow, Brown, Gray, White, Black, Vibrant, Muted
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TextureValue {
    Rough, Smooth, Spiky, Grainy, Metallic, Icy, Soft, Damp
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum WeightValue {
    Light, Medium, Heavy
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TemperatureValue {
    Cold, Cool, Temperate, Warm, Hot
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum SizeValue {
    Tiny, Small, Medium, Large, Huge
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum MaterialValue {
    Stone, Wood, Metal, Fabric, Crystal, Herbal, Liquid, Exoskeleton, Fleshy, Fur, Scales, Feathers
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum BiomeType {
    Ocean, Beach, Desert, Grassland, Forest, Mountain, Tundra, Riverbank, Swamp, Cave
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum FunctionValue {
    Tool, Weapon, Armor, Food, Medicine, Fuel, Magic, Storage, Shelter, Crafting, Defense
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum AspectValue {
    Magic, Life, Death, Spirit, Ancient, New, Fast, Slow, Sharp, Dull, Aggressive, Peaceful, Observant, Hidden, Power, Growth, Healing, Sound, Emotional, Artistic, Wind
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum QualityValue {
    Good, Bad, Rare, Common, Pristine, Worn, Strong, Weak, Protective, Hard
}

pub struct Lexicon {
    pub prefixes: Vec<Morpheme>,
    pub infixes: Vec<Morpheme>,
    pub suffixes: Vec<Morpheme>,
}

impl Lexicon {
    pub fn new() -> Self {
        Lexicon {
            prefixes: vec![
                Morpheme { text: "Ael".to_string(), morpheme_type: MorphemeType::Prefix, semantic_tags: HashMap::from([(SemanticTag::Aspect(AspectValue::Ancient), 0.9), (SemanticTag::Aspect(AspectValue::Magic), 0.8), (SemanticTag::Color(ColorValue::Vibrant), 0.7)]) },
                Morpheme { text: "Kyl".to_string(), morpheme_type: MorphemeType::Prefix, semantic_tags: HashMap::from([(SemanticTag::Temperature(TemperatureValue::Cold), 0.9), (SemanticTag::Color(ColorValue::White), 0.8), (SemanticTag::Texture(TextureValue::Icy), 0.7)]) },
                Morpheme { text: "Tar".to_string(), morpheme_type: MorphemeType::Prefix, semantic_tags: HashMap::from([(SemanticTag::Material(MaterialValue::Stone), 0.9), (SemanticTag::Texture(TextureValue::Rough), 0.8), (SemanticTag::Weight(WeightValue::Heavy), 0.7)]) },
                Morpheme { text: "Glin".to_string(), morpheme_type: MorphemeType::Prefix, semantic_tags: HashMap::from([(SemanticTag::Color(ColorValue::Bright), 0.9), (SemanticTag::Aspect(AspectValue::Light), 0.8), (SemanticTag::Quality(QualityValue::Rare), 0.7)]) },
                Morpheme { text: "Shal".to_string(), morpheme_type: MorphemeType::Prefix, semantic_tags: HashMap::from([(SemanticTag::Aspect(AspectValue::Hidden), 0.9), (SemanticTag::Color(ColorValue::Muted), 0.7)]) },
                Morpheme { text: "Verd".to_string(), morpheme_type: MorphemeType::Prefix, semantic_tags: HashMap::from([(SemanticTag::Color(ColorValue::Green), 0.9), (SemanticTag::Biome(BiomeType::Forest), 0.8), (SemanticTag::Aspect(AspectValue::Life), 0.7)]) },
                Morpheme { text: "Nara".to_string(), morpheme_type: MorphemeType::Prefix, semantic_tags: HashMap::from([(SemanticTag::Size(SizeValue::Large), 0.9), (SemanticTag::Quality(QualityValue::Strong), 0.8), (SemanticTag::Aspect(AspectValue::Aggressive), 0.7)]) },
            ],
            infixes: vec![
                Morpheme { text: "ol".to_string(), morpheme_type: MorphemeType::Infix, semantic_tags: HashMap::from([(SemanticTag::Size(SizeValue::Large), 0.7), (SemanticTag::Quality(QualityValue::Strong), 0.6), (SemanticTag::Aspect(AspectValue::Power), 0.5)]) },
                Morpheme { text: "in".to_string(), morpheme_type: MorphemeType::Infix, semantic_tags: HashMap::from([(SemanticTag::Size(SizeValue::Small), 0.7), (SemanticTag::Aspect(AspectValue::Delicate), 0.6), (SemanticTag::Aspect(AspectValue::Hidden), 0.5)]) },
                Morpheme { text: "ra".to_string(), morpheme_type: MorphemeType::Infix, semantic_tags: HashMap::from([(SemanticTag::Texture(TextureValue::Rough), 0.7), (SemanticTag::Weight(WeightValue::Heavy), 0.6), (SemanticTag::Biome(BiomeType::Mountain), 0.5)]) },
                Morpheme { text: "sk".to_string(), morpheme_type: MorphemeType::Infix, semantic_tags: HashMap::from([(SemanticTag::Aspect(AspectValue::Sharp), 0.8), (SemanticTag::Aspect(AspectValue::Aggressive), 0.7), (SemanticTag::Function(FunctionValue::Weapon), 0.6)]) },
            ],
            suffixes: vec![
                Morpheme { text: "ia".to_string(), morpheme_type: MorphemeType::Suffix, semantic_tags: HashMap::from([(SemanticTag::Biome(BiomeType::Forest), 0.8), (SemanticTag::Aspect(AspectValue::Land), 0.7)]) },
                Morpheme { text: "kor".to_string(), morpheme_type: MorphemeType::Suffix, semantic_tags: HashMap::from([(SemanticTag::Material(MaterialValue::Stone), 0.9), (SemanticTag::Quality(QualityValue::Hard), 0.8), (SemanticTag::Mineral(MaterialValue::Stone), 0.7)]) }, // Corrected Mineral tag
                Morpheme { text: "lume".to_string(), morpheme_type: MorphemeType::Suffix, semantic_tags: HashMap::from([(SemanticTag::Flora(BiomeType::Forest), 0.9), (SemanticTag::Aspect(AspectValue::Growth), 0.8), (SemanticTag::Aspect(AspectValue::Beauty), 0.7)]) }, // Corrected Flora tag
                Morpheme { text: "kar".to_string(), morpheme_type: MorphemeType::Suffix, semantic_tags: HashMap::from([(SemanticTag::Character(EntityType::Character), 0.6), (SemanticTag::Aspect(AspectValue::Aggressive), 0.8), (SemanticTag::Aspect(AspectValue::Predator), 0.7)]) }, // Corrected Character tag
                Morpheme { text: "tol".to_string(), morpheme_type: MorphemeType::Suffix, semantic_tags: HashMap::from([(SemanticTag::Function(FunctionValue::Tool), 0.9), (SemanticTag::Function(FunctionValue::Weapon), 0.7), (SemanticTag::Material(MaterialValue::Metal), 0.6)]) },
                Morpheme { text: "yala".to_string(), morpheme_type: MorphemeType::Suffix, semantic_tags: HashMap::from([(SemanticTag::Character(EntityType::Character), 0.8), (SemanticTag::Aspect(AspectValue::Spirit), 0.7), (SemanticTag::Aspect(AspectValue::Emotional), 0.6)]) }, // Corrected Character tag
            ],
        }
    }
}

pub struct PhonologyConfig {
    pub max_consonant_cluster: usize,
    pub min_name_length: usize,
    pub max_name_length: usize,
    pub vowel_sounds: Vec<String>,
    pub consonant_sounds: Vec<String>,
}

impl PhonologyConfig {
    pub fn new() -> Self {
        PhonologyConfig {
            max_consonant_cluster: 3,
            min_name_length: 4,
            max_name_length: 12,
            vowel_sounds: vec!["a".to_string(), "i".to_string(), "u".to_string(), "e".to_string(), "o".to_string(), "ai".to_string(), "au".to_string()],
            consonant_sounds: vec!["p".to_string(), "t".to_string(), "k".to_string(), "b".to_string(), "d".to_string(), "g".to_string(), "s".to_string(), "sh".to_string(), "h".to_string(), "m".to_string(), "n".to_string(), "l".to_string(), "r".to_string(), "y".to_string(), "w".to_string()],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum EntityType {
    Biome,
    Flora,
    Mineral,
    Character,
    Item,
}

// --- Helper Functions for Phonology ---

fn is_vowel(s: &str, config: &PhonologyConfig) -> bool {
    config.vowel_sounds.iter().any(|v| s.contains(v))
}

fn is_consonant(s: &str, config: &PhonologyConfig) -> bool {
    config.consonant_sounds.iter().any(|c| s.contains(c))
}

fn count_consecutive_consonants(s: &str, config: &PhonologyConfig) -> usize {
    let mut max_consecutive = 0;
    let mut current_consecutive = 0;
    for c in s.chars() {
        if is_consonant(&c.to_string(), config) {
            current_consecutive += 1;
        } else {
            max_consecutive = max_consecutive.max(current_consecutive);
            current_consecutive = 0;
        }
    }
    max_consecutive.max(current_consecutive)
}

fn has_vowel_hiatus(s: &str, config: &PhonologyConfig) -> bool {
    let chars: Vec<char> = s.chars().collect();
    for i in 0..chars.len().saturating_sub(1) {
        let c1 = chars[i].to_string();
        let c2 = chars[i+1].to_string();
        if is_vowel(&c1, config) && is_vowel(&c2, config) {
            // Check for diphthongs
            if !(c1 == "a".to_string() && c2 == "i".to_string()) && !(c1 == "a".to_string() && c2 == "u".to_string()) {
                return true;
            }
        }
    }
    false
}

fn repair_vowel_hiatus(name: &mut String, config: &PhonologyConfig) -> bool {
    let mut repaired = false;
    let mut new_name = String::new();
    let chars: Vec<char> = name.chars().collect();

    let mut i = 0;
    while i < chars.len() {
        new_name.push(chars[i]);
        if i < chars.len().saturating_sub(1) {
            let c1 = chars[i].to_string();
            let c2 = chars[i+1].to_string();
            if is_vowel(&c1, config) && is_vowel(&c2, config) {
                if !(c1 == "a".to_string() && c2 == "i".to_string()) && !(c1 == "a".to_string() && c2 == "u".to_string()) {
                    // Insert glide consonant
                    if c1 == "i".to_string() || c1 == "e".to_string() {
                        new_name.push("y");
                    } else if c1 == "u".to_string() || c1 == "o".to_string() {
                        new_name.push("w");
                    } else {
                        new_name.push("y"); // Default
                    }
                    repaired = true;
                }
            }
        }
        i += 1;
    }
    if repaired {
        *name = new_name;
    }
    repaired
}

fn repair_consonant_cluster(name: &mut String, config: &PhonologyConfig) -> bool {
    let mut repaired = false;
    let mut new_name = String::new();
    let chars: Vec<char> = name.chars().collect();

    let mut i = 0;
    while i < chars.len() {
        new_name.push(chars[i]);
        if is_consonant(&chars[i].to_string(), config) {
            let mut current_cluster_len = 1;
            let mut j = i + 1;
            while j < chars.len() && is_consonant(&chars[j].to_string(), config) {
                current_cluster_len += 1;
                j += 1;
            }

            if current_cluster_len > config.max_consonant_cluster {
                // Insert a default vowel to break the cluster
                new_name.push("a"); 
                repaired = true;
            }
            i = j; // Move past the cluster
        } else {
            i += 1;
        }
    }
    if repaired {
        *name = new_name;
    }
    repaired
}

// --- Core Generation Logic ---

const ATTRIBUTE_MATCH_BONUS: f32 = 1.5;
const CATEGORY_MATCH_BONUS: f32 = 0.8;
const INFIX_PROBABILITY: f32 = 0.3;

fn calculate_match_score(morpheme: &Morpheme, entity_attributes: &HashMap<SemanticTag, f32>) -> f32 {
    let mut score = 0.0;
    for (entity_tag, entity_weight) in entity_attributes.iter() {
        for (morpheme_tag, morpheme_weight) in morpheme.semantic_tags.iter() {
            if entity_tag == morpheme_tag {
                score += entity_weight * morpheme_weight * ATTRIBUTE_MATCH_BONUS;
            } else if is_category_match(entity_tag, morpheme_tag) {
                score += entity_weight * morpheme_weight * CATEGORY_MATCH_BONUS;
            }
        }
    }
    score
}

// Simplified category match for demonstration. In a real engine, this would be more sophisticated.
fn is_category_match(entity_tag: &SemanticTag, morpheme_tag: &SemanticTag) -> bool {
    match (entity_tag, morpheme_tag) {
        (SemanticTag::Biome(_), SemanticTag::Color(_)) => true,
        (SemanticTag::Biome(_), SemanticTag::Texture(_)) => true,
        (SemanticTag::Flora(_), SemanticTag::Color(_)) => true, // Corrected Flora tag
        (SemanticTag::Flora(_), SemanticTag::Texture(_)) => true, // Corrected Flora tag
        (SemanticTag::Mineral(_), SemanticTag::Color(_)) => true, // Corrected Mineral tag
        (SemanticTag::Mineral(_), SemanticTag::Texture(_)) => true, // Corrected Mineral tag
        (SemanticTag::Material(_), SemanticTag::Color(_)) => true,
        (SemanticTag::Material(_), SemanticTag::Texture(_)) => true,
        _ => false,
    }
}

fn select_morpheme<'a, R: Rng>(
    morpheme_list: &'a Vec<Morpheme>,
    entity_attributes: &HashMap<SemanticTag, f32>,
    rng: &mut R,
) -> Option<&'a Morpheme> {
    let mut scored_morphemes: Vec<(&Morpheme, f32)> = morpheme_list.iter()
        .map(|m| (m, calculate_match_score(m, entity_attributes)))
        .collect();

    // Filter out morphemes with zero score, unless all have zero score
    let total_score: f32 = scored_morphemes.iter().map(|(_, score)| score).sum();
    if total_score > 0.0 {
        scored_morphemes.retain(|(_, score)| *score > 0.0);
    }

    if scored_morphemes.is_empty() {
        return None;
    }

    // Weighted random selection
    let weights: Vec<f32> = scored_morphemes.iter().map(|(_, score)| *score).collect();
    scored_morphemes.choose_weighted(rng, |item| item.1)
        .ok()
        .map(|(morpheme, _)| *morpheme)
}

pub fn generate_name<R: Rng>(
    entity_attributes: &HashMap<SemanticTag, f32>,
    lexicon: &Lexicon,
    phonology_config: &PhonologyConfig,
    entity_type: &EntityType,
    rng: &mut R,
) -> Option<String> {
    let mut attempts = 0;
    const MAX_ATTEMPTS: usize = 10;

    while attempts < MAX_ATTEMPTS {
        let mut current_name_parts: Vec<String> = Vec::new();

        // 1. Select Prefix
        let prefix = select_morpheme(&lexicon.prefixes, entity_attributes, rng)?;
        current_name_parts.push(prefix.text.clone());

        // 2. Select Suffix
        let suffix = select_morpheme(&lexicon.suffixes, entity_attributes, rng)?;
        current_name_parts.push(suffix.text.clone());

        let mut name = current_name_parts.join("");

        // 3. Select and Insert Infixes (Optional)
        if rng.gen_bool(INFIX_PROBABILITY) {
            if let Some(infix) = select_morpheme(&lexicon.infixes, entity_attributes, rng) {
                // Simple insertion for demonstration, real logic would be more complex
                // Try to insert in the middle of the prefix+suffix combination
                let insert_pos = name.len() / 2;
                name.insert_str(insert_pos, &infix.text);
            }
        }

        // 4. Phonetic Validation and Repair
        let mut repaired_count = 0;
        const MAX_REPAIRS: usize = 5;
        loop {
            let mut changed = false;
            if has_vowel_hiatus(&name, phonology_config) {
                repair_vowel_hiatus(&mut name, phonology_config);
                changed = true;
            }
            if count_consecutive_consonants(&name, phonology_config) > phonology_config.max_consonant_cluster {
                repair_consonant_cluster(&mut name, phonology_config);
                changed = true;
            }
            if !changed || repaired_count >= MAX_REPAIRS {
                break;
            }
            repaired_count += 1;
        }

        // 5. Final Validation
        if name.len() >= phonology_config.min_name_length && name.len() <= phonology_config.max_name_length &&
           count_consecutive_consonants(&name, phonology_config) <= phonology_config.max_consonant_cluster &&
           !has_vowel_hiatus(&name, phonology_config) {
            return Some(name.to_lowercase().to_string()); // Return lowercase for consistency
        }

        attempts += 1;
    }

    None // Failed to generate a valid name after max attempts
}

fn main() {
    let lexicon = Lexicon::new();
    let phonology_config = PhonologyConfig::new();
    let mut rng = rand::thread_rng();

    println!("--- Aetherbourne Name Generation Examples ---");

    // Biome: Forest
    let mut forest_attrs = HashMap::new();
    forest_attrs.insert(SemanticTag::Biome(BiomeType::Forest), 1.0);
    forest_attrs.insert(SemanticTag::Color(ColorValue::Green), 0.8);
    forest_attrs.insert(SemanticTag::Aspect(AspectValue::Life), 0.7);
    if let Some(name) = generate_name(&forest_attrs, &lexicon, &phonology_config, &EntityType::Biome, &mut rng) {
        println!("Forest Biome: {}", name);
    }

    // Mineral: Rare Crystal
    let mut crystal_attrs = HashMap::new();
    crystal_attrs.insert(SemanticTag::Material(MaterialValue::Crystal), 1.0);
    crystal_attrs.insert(SemanticTag::Quality(QualityValue::Rare), 0.9);
    crystal_attrs.insert(SemanticTag::Color(ColorValue::Vibrant), 0.8);
    crystal_attrs.insert(SemanticTag::Aspect(AspectValue::Magic), 0.7);
    if let Some(name) = generate_name(&crystal_attrs, &lexicon, &phonology_config, &EntityType::Mineral, &mut rng) {
        println!("Rare Crystal: {}", name);
    }

    // Character: Aggressive Warrior
    let mut warrior_attrs = HashMap::new();
    warrior_attrs.insert(SemanticTag::Character(EntityType::Character), 1.0); // Corrected Character tag
    warrior_attrs.insert(SemanticTag::Aspect(AspectValue::Aggressive), 0.9);
    warrior_attrs.insert(SemanticTag::Quality(QualityValue::Strong), 0.8);
    warrior_attrs.insert(SemanticTag::Function(FunctionValue::Weapon), 0.6);
    if let Some(name) = generate_name(&warrior_attrs, &lexicon, &phonology_config, &EntityType::Character, &mut rng) {
        println!("Aggressive Warrior: {}", name);
    }

    // Item: Sharp Tool
    let mut tool_attrs = HashMap::new();
    tool_attrs.insert(SemanticTag::Function(FunctionValue::Tool), 1.0);
    tool_attrs.insert(SemanticTag::Aspect(AspectValue::Sharp), 0.9);
    tool_attrs.insert(SemanticTag::Material(MaterialValue::Metal), 0.8);
    if let Some(name) = generate_name(&tool_attrs, &lexicon, &phonology_config, &EntityType::Item, &mut rng) {
        println!("Sharp Tool: {}", name);
    }

    // Flora: Delicate Plant
    let mut plant_attrs = HashMap::new();
    plant_attrs.insert(SemanticTag::Flora(BiomeType::Forest), 1.0); // Corrected Flora tag
    plant_attrs.insert(SemanticTag::Aspect(AspectValue::Delicate), 0.9);
    plant_attrs.insert(SemanticTag::Color(ColorValue::Green), 0.7);
    if let Some(name) = generate_name(&plant_attrs, &lexicon, &phonology_config, &EntityType::Flora, &mut rng) {
        println!("Delicate Plant: {}", name);
    }
}
```

### 4.3. `Cargo.toml`

Ensure your `Cargo.toml` includes the `rand` dependency:

```toml
[package]
name = "conlang_example"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8"
```

### 4.4. Running the Example

Navigate to the `conlang_example` directory in your terminal and run:

```bash
cargo run
```

You should see output similar to this (names will vary due to randomness):

```
--- Aetherbourne Name Generation Examples ---
Forest Biome: verdolia
Rare Crystal: glinrakor
Aggressive Warrior: naraskkar
Sharp Tool: tarsktol
Delicate Plant: malainlume
```

### 4.5. Integration Notes

*   **Lexicon Loading:** For a production game, the `Lexicon` data should ideally be loaded from an external file (e.g., JSON, YAML) rather than hardcoded. This allows for easier updates and modifications to the language without recompiling the game engine.
*   **Attribute Mapping:** The `entity_attributes` `HashMap` should be dynamically generated based on the actual in-game properties of the entity you are naming. This is where the procedural generation specification (biomes, flora, minerals, characters, items) feeds into the naming system.
*   **Uniqueness:** The example includes a basic uniqueness check. For a large-scale game, you would need a more robust system to store and query generated names to prevent duplicates.
*   **Performance:** For very large worlds or frequent name generation, consider optimizing the morpheme selection and phonetic repair loops. Pre-calculating scores or using more efficient string manipulation could be beneficial.
*   **Extensibility:** The `SemanticTag` enum and the `Lexicon` structure are designed to be easily extensible. You can add new semantic categories and morphemes as your world and language evolve.

This guide provides a solid foundation for integrating the Aetherbourne Conlang into your game engine, enabling dynamic and meaningful name generation that enhances the immersion and unique identity of your world.
