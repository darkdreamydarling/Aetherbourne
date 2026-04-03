# Aetherbourne Linguistic & Naming Specification

This document outlines the procedural generation system for names of biomes, flora, minerals, items, and characters in Aetherbourne. The goal is to create a dynamic, in-game language that generates names which are not only unique but also semantically meaningful, reflecting the core attributes of the entities they describe. This system will use a morphological framework of prefixes, infixes, and suffixes, each imbued with specific semantic values.

## 1. Morphological Framework and Semantic Mapping

The naming system is built upon a lexicon of morphemes (prefixes, infixes, suffixes) that carry semantic weight. These morphemes are combined according to phonological and grammatical rules to form new words.

### 1.1. Morpheme Structure

Each morpheme is defined by its type, phonetic representation, and a set of semantic tags.

```rust
pub enum MorphemeType {
    Prefix,   // Appears at the beginning of a word
    Infix,    // Appears in the middle of a word
    Suffix,   // Appears at the end of a word
}

pub struct Morpheme {
    pub text: String,           // The phonetic representation (e.g., "Aeth", "-or-", "-ia")
    pub morpheme_type: MorphemeType,
    pub semantic_tags: Vec<SemanticTag>, // List of associated meanings
    pub phonological_rules: Vec<PhonologicalRule>, // Rules for combining with other morphemes
}

pub enum SemanticTag {
    Color(ColorValue),          // e.g., Color::Red, Color::Blue
    Texture(TextureValue),      // e.g., Texture::Rough, Texture::Smooth
    Weight(WeightValue),        // e.g., Weight::Light, Weight::Heavy
    Temperature(TemperatureValue), // e.g., Temperature::Cold, Temperature::Hot
    Size(SizeValue),            // e.g., Size::Small, Size::Large
    Material(MaterialValue),    // e.g., Material::Stone, Material::Wood
    Biome(BiomeType),           // e.g., Biome::Forest, Biome::Desert
    Function(FunctionValue),    // e.g., Function::Tool, Function::Food
    // ... other relevant attributes
}

pub enum ColorValue {
    Red, Blue, Green, Yellow, Brown, Gray, White, Black, Vibrant, Muted
}
// ... similar enums for TextureValue, WeightValue, etc.
```

### 1.2. Semantic Mapping Tables

These tables define the core morphemes and their associated semantic tags. The `weight` indicates how strongly a morpheme is associated with a tag.

#### 1.2.1. Prefixes

| Morpheme | Semantic Tags (Weight) | Phonological Notes |
| :--- | :--- | :--- |
| `Aeth-` | `Color::Vibrant (0.8)`, `Magic (0.7)`, `Ancient (0.6)` | Often starts with a vowel, soft 'th' sound. |
| `Glim-` | `Color::Bright (0.9)`, `Light (0.8)`, `Mineral (0.7)` | Suggests sparkle or reflection. |
| `Dusk-` | `Color::Muted (0.8)`, `Dark (0.7)`, `Shadow (0.6)` | Evokes evening or concealment. |
| `Ston-` | `Texture::Rough (0.9)`, `Weight::Heavy (0.8)`, `Material::Stone (1.0)` | Hard, solid sound. |
| `Leaf-` | `Color::Green (0.9)`, `Texture::Smooth (0.7)`, `Biome::Forest (0.8)` | Light, organic sound. |
| `Frost-` | `Temperature::Cold (1.0)`, `Color::White (0.8)`, `Texture::Icy (0.9)` | Sharp, cold sound. |
| `Iron-` | `Material::Metal (1.0)`, `Weight::Heavy (0.9)`, `Color::Gray (0.7)` | Strong, metallic sound. |
| `Aqua-` | `Biome::Ocean (0.9)`, `Material::Liquid (0.8)`, `Color::Blue (0.7)` | Watery, flowing sound. |
| `Pyro-` | `Temperature::Hot (0.9)`, `Color::Red (0.8)`, `Function::Fuel (0.7)` | Fiery, intense sound. |
| `Silv-` | `Material::Metal (0.9)`, `Color::White (0.8)`, `Rare (0.6)` | Suggests silver, precious. |
| `Clay-` | `Material::Stone (0.8)`, `Texture::Smooth (0.7)`, `Color::Brown (0.6)` | Earthy, malleable. |
| `Grim-` | `Color::Dark (0.9)`, `Danger (0.7)`, `Negative (0.6)` | Foreboding, ominous. |
| `Verd-` | `Color::Green (0.9)`, `Flora (0.8)`, `Lush (0.7)` | Verdant, growing. |
| `Scor-` | `Temperature::Hot (0.9)`, `Texture::Rough (0.7)`, `Biome::Desert (0.6)` | Burning, harsh. |
| `Whis-` | `Sound (0.8)`, `Subtle (0.7)`, `Magic (0.5)` | Quiet, ethereal. |

#### 1.2.2. Infixes

| Morpheme | Semantic Tags (Weight) | Phonological Notes |
| :--- | :--- | :--- |
| `-or-` | `Size::Large (0.6)`, `Power (0.7)`, `Aggressive (0.5)` | Often connects two strong sounds. |
| `-el-` | `Size::Small (0.7)`, `Delicate (0.6)`, `Flora (0.5)` | Light, airy sound. |
| `-und-` | `Texture::Rough (0.7)`, `Deep (0.6)`, `Biome::Mountain (0.5)` | Grounded, resonant sound. |
| `-ia-` | `Color::Vibrant (0.6)`, `Beauty (0.7)`, `Rare (0.5)` | Soft, flowing sound. |
| `-ven-` | `Function::Weapon (0.7)`, `Sharp (0.6)`, `Danger (0.5)` | Suggests danger or sharpness. |
| `-gard-` | `Function::Defense (0.7)`, `Protective (0.6)`, `Strong (0.5)` | Suggests protection or guarding. |
| `-kin-` | `Character (0.7)`, `Family (0.6)`, `Small (0.5)` | Suggests kinship or smallness. |
| `-shard-` | `Material::Crystal (0.8)`, `Sharp (0.7)`, `Fragment (0.6)` | Crystalline, broken. |
| `-gleam-` | `Color::Bright (0.8)`, `Light (0.7)`, `Material::Metal (0.6)` | Reflective, polished. |
| `-moss-` | `Texture::Soft (0.8)`, `Color::Green (0.7)`, `Biome::Forest (0.6)` | Organic, damp. |

#### 1.2.3. Suffixes

| Morpheme | Semantic Tags (Weight) | Phonological Notes |
| :--- | :--- | :--- |
| `-ia` | `Biome (0.8)`, `Land (0.7)`, `Feminine (0.5)` | Often for place names, soft ending. |
| `-ite` | `Mineral (0.9)`, `Hard (0.7)` | Common for mineral names, solid ending. |
| `-bloom` | `Flora (0.9)`, `Flower (0.8)`, `Beauty (0.7)` | Suggests growth and beauty. |
| `-claw` | `Character (0.7)`, `Aggressive (0.8)`, `Predator (0.9)` | Sharp, animalistic ending. |
| `-stone` | `Material::Stone (1.0)`, `Structure (0.8)` | Indicates a stone object or structure. |
| `-leaf` | `Flora (0.9)`, `Gentle (0.6)` | Soft, natural ending. |
| `-axe` | `Function::Tool (0.9)`, `Function::Weapon (0.7)` | Tool/weapon ending. |
| `-gem` | `Material::Crystal (0.9)`, `Rare (0.8)` | Precious stone ending. |
| `-ixir` | `Function::Medicine (0.9)`, `Material::Liquid (0.8)` | Potion/liquid ending. |
| `-guard` | `Function::Defense (0.9)`, `Protective (0.8)` | Defensive item ending. |
| `-wood` | `Material::Wood (1.0)`, `Flora (0.7)` | Tree/wood ending. |
| `-thorn` | `Texture::Spiky (0.9)`, `Flora (0.7)` | Spiky plant ending. |
| `-smith` | `Character (0.8)`, `Function::Crafting (0.9)`, `Material::Metal (0.7)` | Indicates a crafter, especially metal. |
| `-weaver` | `Character (0.8)`, `Function::Crafting (0.9)`, `Material::Fabric (0.7)` | Indicates a crafter, especially textiles. |
| `-kin` | `Character (0.8)`, `Small (0.7)`, `Affectionate (0.6)` | Diminutive, friendly character ending. |
| `-fang` | `Character (0.8)`, `Aggressive (0.7)`, `Predator (0.9)` | Sharp, predatory character ending. |
| `-root` | `Flora (0.9)`, `Deep (0.7)`, `Grounded (0.6)` | Suggests a root-based plant or foundation. |
| `-vale` | `Biome (0.8)`, `Valley (0.7)`, `Peaceful (0.6)` | Suggests a peaceful valley or low land. |
| `-glen` | `Biome (0.8)`, `SmallValley (0.7)`, `Secluded (0.6)` | Suggests a small, secluded valley. |
| `-mire` | `Biome (0.8)`, `Swamp (0.7)`, `Damp (0.6)` | Suggests a swampy, boggy area. |
| `-hide` | `Material::Fabric (0.9)`, `Protective (0.7)` | Suggests leather or animal skin. |
| `-scale` | `Material::Exoskeleton (0.9)`, `Protective (0.7)` | Suggests scales or armor. |
| `-shard` | `Material::Crystal (0.9)`, `Sharp (0.7)` | Suggests a sharp fragment of crystal. |
| `-glow` | `Magic (0.9)`, `Light (0.8)` | Suggests emitting light or magic. |
| `-heart` | `Character (0.8)`, `Emotional (0.7)`, `Core (0.6)` | Suggests a central, emotional character. |
| `-song` | `Character (0.8)`, `Artistic (0.7)`, `Sound (0.6)` | Suggests a musical or eloquent character. |

### 1.3. Phonological Rules

These rules ensure generated names are pronounceable and aesthetically pleasing.

*   **Vowel-Consonant Alternation:** Prefer alternating vowels and consonants (e.g., `Aeth-or-ia` is good, `Aeth-rr-ia` is bad).
*   **Max Consonant Clusters:** Limit consecutive consonants to `MAX_CONSONANT_CLUSTER = 3` (e.g., `str` is okay, `sthr` is not).
*   **Min/Max Length:** Names should be between `MIN_NAME_LENGTH = 4` and `MAX_NAME_LENGTH = 12` characters.
*   **Ending Sounds:** Prefer names ending in vowels or soft consonants for flora/biomes, and harder consonants for minerals/weapons.

### 1.4. Name Generation Algorithm (High-Level)

1.  **Identify Core Attributes:** Extract key attributes (e.g., `Color`, `Texture`, `BiomeType`) from the entity to be named.
2.  **Select Morphemes:** Based on the entity's attributes, select a pool of suitable prefixes, infixes, and suffixes. Prioritize morphemes with high `weight` for matching `SemanticTag`s.
3.  **Combine Morphemes:** Randomly combine selected morphemes, ensuring phonological rules are followed.
4.  **Validate and Refine:** Check for pronounceability and uniqueness. If a name is unpronounceable or already exists, re-generate or apply minor phonetic adjustments.

This framework provides the linguistic building blocks. The following sections will detail how these are applied to specific entity types.

## 2. Naming Rules and Grammatical Structures for World Elements

This section applies the morphological framework to generate names for static world elements: Biomes, Flora, and Minerals. The generation process prioritizes semantic relevance to the element's core attributes.

### 2.1. Biome Naming

Biome names often reflect their dominant geographical features, climate, and primary colors. They tend to be more descriptive and evocative.

#### 2.1.1. Biome Naming Algorithm

1.  **Identify Dominant Attributes:** Extract `BiomeType`, `TemperatureValue`, `MoistureValue`, and `DominantColor` from the biome's properties.
2.  **Select Core Morphemes:**
    *   **Prefix:** Choose a prefix strongly associated with `TemperatureValue` or `DominantColor`.
    *   **Infix (Optional):** If the biome has a secondary notable feature (e.g., `Mountain` in a `Forest`), select an infix reflecting that.
    *   **Suffix:** Always select a suffix strongly associated with `Biome` or `Land`.
3.  **Combine and Validate:** Assemble the morphemes, ensuring phonological rules are met. Prioritize names that evoke the biome's aesthetic (e.g., `Aeth-or-ia` for a vibrant, large, ancient land).

#### 2.1.2. Biome Naming Examples

| Biome Type | Key Attributes | Generated Name (Example) | Breakdown | Aesthetic |
| :--- | :--- | :--- | :--- | :--- |
| `Forest` | `Green`, `Moist`, `Dense` | `Leaf-und-ia` | `Leaf-` (Green, Forest) + `-und-` (Deep, Grounded) + `-ia` (Land) | Lush, ancient, earthy. |
| `Desert` | `Hot`, `Dry`, `Sandy` | `Dusk-or-ia` | `Dusk-` (Muted, Dry) + `-or-` (Large, Expansive) + `-ia` (Land) | Arid, vast, mysterious. |
| `Mountain` | `Cold`, `Rocky`, `High` | `Ston-und-ia` | `Ston-` (Rough, Heavy) + `-und-` (Deep, Grounded) + `-ia` (Land) | Rugged, imposing, ancient. |
| `Tundra` | `Cold`, `White`, `Sparse` | `Frost-el-ia` | `Frost-` (Cold, White) + `-el-` (Small, Delicate) + `-ia` (Land) | Stark, fragile, cold beauty. |

### 2.2. Flora Naming

Flora names often highlight their color, texture, size, or unique properties (e.g., medicinal, edible). They tend to be more descriptive of the plant itself.

#### 2.2.1. Flora Naming Algorithm

1.  **Identify Dominant Attributes:** Extract `DominantColor`, `TextureValue`, `SizeValue`, and `FunctionValue` (e.g., `Food`, `Medicine`) from the plant's properties.
2.  **Select Core Morphemes:**
    *   **Prefix:** Choose a prefix strongly associated with `DominantColor` or `TextureValue`.
    *   **Infix (Optional):** If the plant has a distinct size or unique property, select an infix reflecting that.
    *   **Suffix:** Always select a suffix strongly associated with `Flora` (e.g., `-bloom`, `-leaf`).
3.  **Combine and Validate:** Assemble the morphemes, ensuring phonological rules are met. Prioritize names that sound organic and reflect the plant's visual appeal.

#### 2.2.2. Flora Naming Examples

| Plant Type | Key Attributes | Generated Name (Example) | Breakdown | Aesthetic |
| :--- | :--- | :--- | :--- | :--- |
| `BerryBush` | `Red`, `Small`, `Edible` | `Glim-el-berry` | `Glim-` (Bright) + `-el-` (Small) + `-berry` (Suffix for fruit) | Sweet, inviting, small. |
| `OakTree` | `Green`, `Large`, `Wood` | `Leaf-or-wood` | `Leaf-` (Green) + `-or-` (Large) + `-wood` (Suffix for tree) | Sturdy, ancient, provides resources. |
| `Cactus` | `Green`, `Spiky`, `Water` | `Dusk-thorn` | `Dusk-` (Muted) + `-thorn` (Suffix for spiky plant) | Resilient, sharp, desert-dweller. |
| `Snowdrop` | `White`, `Small`, `Medicinal` | `Frost-el-bloom` | `Frost-` (Cold, White) + `-el-` (Small) + `-bloom` (Flower) | Delicate, rare, healing. |

### 2.3. Mineral Naming

Mineral names typically emphasize their material, color, hardness, or rarity. They often have a more solid, grounded sound.

#### 2.3.1. Mineral Naming Algorithm

1.  **Identify Dominant Attributes:** Extract `MaterialValue`, `DominantColor`, `HardnessValue`, and `RarityValue` from the mineral's properties.
2.  **Select Core Morphemes:**
    *   **Prefix:** Choose a prefix strongly associated with `MaterialValue` or `DominantColor`.
    *   **Infix (Optional):** If the mineral has a distinct hardness or rarity, select an infix reflecting that.
    *   **Suffix:** Always select a suffix strongly associated with `Mineral` (e.g., `-ite`, `-stone`).
3.  **Combine and Validate:** Assemble the morphemes, ensuring phonological rules are met. Prioritize names that sound solid and reflect the mineral's utility or preciousness.

#### 2.3.2. Mineral Naming Examples

| Mineral Type | Key Attributes | Generated Name (Example) | Breakdown | Aesthetic |
| :--- | :--- | :--- | :--- | :--- |
| `IronOre` | `Gray`, `Heavy`, `Hard` | `Iron-und-ite` | `Iron-` (Material) + `-und-` (Grounded) + `-ite` (Mineral) | Strong, essential, industrial. |
| `Crystal` | `Vibrant`, `Rare`, `Magic` | `Glim-ia-ite` | `Glim-` (Bright) + `-ia-` (Beauty, Rare) + `-ite` (Mineral) | Precious, magical, luminous. |
| `Coal` | `Black`, `Fuel`, `Common` | `Dusk-stone` | `Dusk-` (Dark) + `-stone` (Mineral) | Common, utilitarian, dark. |

This section provides the blueprint for generating semantically rich and aesthetically appropriate names for the static elements of Aetherbourne. The next phase will focus on dynamic elements like characters and items.

## 3. Character and Item Naming Logic

This section extends the morphological framework to generate names for dynamic elements: Characters (entities) and Items. These names will reflect their inherent attributes, roles, and the materials they are made from, while maintaining a consistent linguistic aesthetic.

### 3.1. Character Naming

Character names should evoke their species, dominant personality traits, and potentially their role or origin. They should feel personal yet procedurally generated.

#### 3.1.1. Character Naming Algorithm

1.  **Identify Core Attributes:** Extract `SpeciesType`, `DominantPersonalityTrait` (e.g., `Aggression`, `Curiosity`, `Sociability`), and potentially `PrimaryRole` (e.g., `Warrior`, `Crafter`, `Explorer`) from the character's properties.
2.  **Select Core Morphemes:**
    *   **Prefix:** Choose a prefix strongly associated with `SpeciesType` or `DominantPersonalityTrait`.
    *   **Infix (Optional):** If the character has a notable `PrimaryRole` or a secondary strong trait, select an infix reflecting that.
    *   **Suffix:** Select a suffix that provides a suitable ending for a character name, potentially reflecting `Gender` or `Species`.
3.  **Combine and Validate:** Assemble the morphemes, ensuring phonological rules are met. Prioritize names that sound fitting for a living being, avoiding overly harsh or clunky combinations.

#### 3.1.2. Character Naming Examples

| Character Type | Key Attributes | Generated Name (Example) | Breakdown | Aesthetic |
| :--- | :--- | :--- | :--- | :--- |
| `Aggressive Warrior` | `Humanoid`, `Aggression`, `Strength` | `Ston-or-claw` | `Ston-` (Strong, Hard) + `-or-` (Power) + `-claw` (Aggressive, Warrior) | Powerful, formidable, slightly intimidating. |
| `Curious Explorer` | `Humanoid`, `Curiosity`, `Agility` | `Glim-el-leaf` | `Glim-` (Bright, Observant) + `-el-` (Light, Agile) + `-leaf` (Nature, Explorer) | Nimble, inquisitive, gentle. |
| `Gentle Healer` | `Fae`, `Empathy`, `Wisdom` | `Aeth-ia-bloom` | `Aeth-` (Ancient, Magic) + `-ia-` (Beauty, Gentle) + `-bloom` (Growth, Healing) | Ethereal, nurturing, wise. |
| `Heavy Crafter` | `Dwarf`, `Grit`, `Material::Stone` | `Iron-und-stone` | `Iron-` (Material, Strong) + `-und-` (Grounded, Deep) + `-stone` (Crafter, Earthy) | Resilient, skilled, dependable. |

### 3.2. Item Naming

Item names should clearly communicate their primary material, function, and any significant modifiers. They should be concise and informative.

#### 3.2.1. Item Naming Algorithm

1.  **Identify Core Attributes:** Extract `BaseMaterial`, `FunctionValue` (e.g., `Tool`, `Food`, `Weapon`), `PrimaryModifier` (if any), and `Quality` from the item's properties.
2.  **Select Core Morphemes:**
    *   **Prefix:** Choose a prefix strongly associated with `BaseMaterial` or `PrimaryModifier`.
    *   **Infix (Optional):** If the item has a distinct `FunctionValue` or `Quality`, select an infix reflecting that.
    *   **Suffix:** Select a suffix that clearly indicates the `ItemType` (e.g., `-axe`, `-gem`, `-potion`).
3.  **Combine and Validate:** Assemble the morphemes, ensuring phonological rules are met. Prioritize names that are descriptive and easy to understand, while still sounding unique.

#### 3.2.2. Item Naming Examples

| Item Type | Key Attributes | Generated Name (Example) | Breakdown | Aesthetic |
| :--- | :--- | :--- | :--- | :--- |
| `Sturdy Axe` | `Iron`, `Tool`, `High Quality` | `Iron-or-axe` | `Iron-` (Material) + `-or-` (Strong, Large) + `-axe` (Tool) | Reliable, powerful, crafted. |
| `Healing Potion` | `Herbal`, `Consumable`, `Healing` | `Leaf-el-ixir` | `Leaf-` (Herbal) + `-el-` (Small, Delicate) + `-ixir` (Potion, Liquid) | Restorative, natural, gentle. |
| `Glowing Crystal` | `Crystal`, `Magic`, `Rare` | `Glim-ia-gem` | `Glim-` (Bright, Luminous) + `-ia-` (Beauty, Rare) + `-gem` (Crystal, Precious) | Enchanting, valuable, magical. |
| `Heavy Shield` | `Ston`, `Defense`, `Weight::Heavy` | `Ston-und-guard` | `Ston-` (Material, Hard) + `-und-` (Grounded, Heavy) + `-guard` (Defense, Shield) | Protective, robust, dependable. |

This section provides the blueprint for generating semantically rich and aesthetically appropriate names for the dynamic elements of Aetherbourne. The next phase will consolidate all linguistic tables and constants.

## 4. Consolidated Linguistic Tables and Constants

This section provides a single, comprehensive reference for all data structures, morphemes, semantic tags, and numerical constants used in the procedural naming system.

### 4.1. Core Data Structures

```rust
pub enum MorphemeType {
    Prefix,   // Appears at the beginning of a word
    Infix,    // Appears in the middle of a word
    Suffix,   // Appears at the end of a word
}

pub struct Morpheme {
    pub text: String,           // The phonetic representation (e.g., "Aeth", "-or-", "-ia")
    pub morpheme_type: MorphemeType,
    pub semantic_tags: Vec<SemanticTag>, // List of associated meanings
    pub phonological_rules: Vec<PhonologicalRule>, // Rules for combining with other morphemes
}

pub enum SemanticTag {
    Color(ColorValue),          // e.g., Color::Red, Color::Blue
    Texture(TextureValue),      // e.g., Texture::Rough, Texture::Smooth
    Weight(WeightValue),        // e.g., Weight::Light, Weight::Heavy
    Temperature(TemperatureValue), // e.g., Temperature::Cold, Temperature::Hot
    Size(SizeValue),            // e.g., Size::Small, Size::Large
    Material(MaterialValue),    // e.g., Material::Stone, Material::Wood
    Biome(BiomeType),           // e.g., Biome::Forest, Biome::Desert
    Function(FunctionValue),    // e.g., Function::Tool, Function::Food
    // ... other relevant attributes
}

pub enum ColorValue {
    Red, Blue, Green, Yellow, Brown, Gray, White, Black, Vibrant, Muted
}

pub enum TextureValue {
    Rough, Smooth, Spiky, Grainy, Metallic, Icy, Soft
}

pub enum WeightValue {
    Light, Medium, Heavy
}

pub enum TemperatureValue {
    Cold, Cool, Temperate, Warm, Hot
}

pub enum SizeValue {
    Tiny, Small, Medium, Large, Huge
}

pub enum MaterialValue {
    Stone, Wood, Metal, Fabric, Crystal, Herbal, Liquid
}

pub enum BiomeType {
    Ocean, Beach, Desert, Grassland, Forest, Mountain, Tundra, Riverbank, Swamp, Cave
}

pub enum FunctionValue {
    Tool, Weapon, Armor, Food, Medicine, Fuel, Magic, Storage, Shelter, Crafting
}

// Placeholder for PhonologicalRule enum/struct, to be defined if more complex rules are needed
pub struct PhonologicalRule { /* ... */ }
```

### 4.2. Morpheme Lexicon

#### 4.2.1. Prefixes

| Morpheme | Semantic Tags (Weight) | Phonological Notes |
| :--- | :--- | :--- |
| `Aeth-` | `Color::Vibrant (0.8)`, `Magic (0.7)`, `Ancient (0.6)` | Often starts with a vowel, soft \'th\' sound. |
| `Glim-` | `Color::Bright (0.9)`, `Light (0.8)`, `Material::Crystal (0.7)` | Suggests sparkle or reflection. |
| `Dusk-` | `Color::Muted (0.8)`, `Dark (0.7)`, `Temperature::Cool (0.6)` | Evokes evening or concealment. |
| `Ston-` | `Texture::Rough (0.9)`, `Weight::Heavy (0.8)`, `Material::Stone (1.0)` | Hard, solid sound. |
| `Leaf-` | `Color::Green (0.9)`, `Texture::Smooth (0.7)`, `Biome::Forest (0.8)` | Light, organic sound. |
| `Frost-` | `Temperature::Cold (1.0)`, `Color::White (0.8)`, `Texture::Icy (0.9)` | Sharp, cold sound. |
| `Iron-` | `Material::Metal (1.0)`, `Weight::Heavy (0.9)`, `Color::Gray (0.7)` | Strong, metallic sound. |
| `Aqua-` | `Biome::Ocean (0.9)`, `Material::Liquid (0.8)`, `Color::Blue (0.7)` | Watery, flowing sound. |
| `Silv-` | `Material::Metal (0.9)`, `Color::White (0.8)`, `Rare (0.6)` | Suggests silver, precious. |
| `Clay-` | `Material::Stone (0.8)`, `Texture::Smooth (0.7)`, `Color::Brown (0.6)` | Earthy, malleable. |
| `Grim-` | `Color::Dark (0.9)`, `Danger (0.7)`, `Negative (0.6)` | Foreboding, ominous. |
| `Verd-` | `Color::Green (0.9)`, `Flora (0.8)`, `Lush (0.7)` | Verdant, growing. |
| `Scor-` | `Temperature::Hot (0.9)`, `Texture::Rough (0.7)`, `Biome::Desert (0.6)` | Burning, harsh. |
| `Whis-` | `Sound (0.8)`, `Subtle (0.7)`, `Magic (0.5)` | Quiet, ethereal. |

#### 4.2.2. Infixes

| Morpheme | Semantic Tags (Weight) | Phonological Notes |
| :--- | :--- | :--- |
| `-or-` | `Size::Large (0.6)`, `Power (0.7)`, `Aggressive (0.5)` | Often connects two strong sounds. |
| `-el-` | `Size::Small (0.7)`, `Delicate (0.6)`, `Flora (0.5)` | Light, airy sound. |
| `-und-` | `Texture::Rough (0.7)`, `Deep (0.6)`, `Biome::Mountain (0.5)` | Grounded, resonant sound. |
| `-ia-` | `Color::Vibrant (0.6)`, `Beauty (0.7)`, `Rare (0.5)` | Soft, flowing sound. |
| `-ven-` | `Function::Weapon (0.7)`, `Sharp (0.6)`, `Danger (0.5)` | Suggests danger or sharpness. |
| `-gard-` | `Function::Defense (0.7)`, `Protective (0.6)`, `Strong (0.5)` | Suggests protection or guarding. |
| `-kin-` | `Character (0.7)`, `Family (0.6)`, `Small (0.5)` | Suggests kinship or smallness. |
| `-shard-` | `Material::Crystal (0.8)`, `Sharp (0.7)`, `Fragment (0.6)` | Crystalline, broken. |
| `-gleam-` | `Color::Bright (0.8)`, `Light (0.7)`, `Material::Metal (0.6)` | Reflective, polished. |
| `-moss-` | `Texture::Soft (0.8)`, `Color::Green (0.7)`, `Biome::Forest (0.6)` | Organic, damp. |
| `-ven-` | `Function::Weapon (0.7)`, `Sharp (0.6)`, `Dangerous (0.5)` | Suggests danger or sharpness. |
| `-gard-` | `Function::Defense (0.7)`, `Protective (0.6)`, `Strong (0.5)` | Suggests protection or guarding. |
| `-gleam-` | `Color::Bright (0.8)`, `Light (0.7)`, `Material::Metal (0.6)` | Reflective, polished. |
| `-moss-` | `Texture::Soft (0.8)`, `Color::Green (0.7)`, `Biome::Forest (0.6)` | Organic, damp. |

#### 4.2.3. Suffixes

| Morpheme | Semantic Tags (Weight) | Phonological Notes |
| :--- | :--- | :--- |
| `-ia` | `Biome (0.8)`, `Land (0.7)`, `Feminine (0.5)` | Often for place names, soft ending. |
| `-ite` | `Mineral (0.9)`, `Hard (0.7)` | Common for mineral names, solid ending. |
| `-bloom` | `Flora (0.9)`, `Flower (0.8)`, `Beauty (0.7)` | Suggests growth and beauty. |
| `-claw` | `Character (0.7)`, `Aggressive (0.8)`, `Predator (0.9)` | Sharp, animalistic ending. |
| `-stone` | `Material::Stone (1.0)`, `Structure (0.8)` | Indicates a stone object or structure. |
| `-leaf` | `Flora (0.9)`, `Gentle (0.6)` | Soft, natural ending. |
| `-axe` | `Function::Tool (0.9)`, `Function::Weapon (0.7)` | Tool/weapon ending. |
| `-gem` | `Material::Crystal (0.9)`, `Rare (0.8)` | Precious stone ending. |
| `-ixir` | `Function::Medicine (0.9)`, `Material::Liquid (0.8)` | Potion/liquid ending. |
| `-guard` | `Function::Defense (0.9)`, `Protective (0.8)` | Defensive item ending. |
| `-wood` | `Material::Wood (1.0)`, `Flora (0.7)` | Tree/wood ending. |
| `-thorn` | `Texture::Spiky (0.9)`, `Flora (0.7)` | Spiky plant ending. |
| `-smith` | `Character (0.8)`, `Function::Crafting (0.9)`, `Material::Metal (0.7)` | Indicates a crafter, especially metal. |
| `-weaver` | `Character (0.8)`, `Function::Crafting (0.9)`, `Material::Fabric (0.7)` | Indicates a crafter, especially textiles. |
| `-kin` | `Character (0.8)`, `Small (0.7)`, `Affectionate (0.6)` | Diminutive, friendly character ending. |
| `-fang` | `Character (0.8)`, `Aggressive (0.7)`, `Predator (0.9)` | Sharp, predatory character ending. |
| `-root` | `Flora (0.9)`, `Deep (0.7)`, `Grounded (0.6)` | Suggests a root-based plant or foundation. |
| `-vale` | `Biome (0.8)`, `Valley (0.7)`, `Peaceful (0.6)` | Suggests a peaceful valley or low land. |
| `-glen` | `Biome (0.8)`, `SmallValley (0.7)`, `Secluded (0.6)` | Suggests a small, secluded valley. |
| `-mire` | `Biome (0.8)`, `Swamp (0.7)`, `Damp (0.6)` | Suggests a swampy, boggy area. |
| `-hide` | `Material::Fabric (0.9)`, `Protective (0.7)` | Suggests leather or animal skin. |
| `-scale` | `Material::Exoskeleton (0.9)`, `Protective (0.7)` | Suggests scales or armor. |
| `-shard` | `Material::Crystal (0.9)`, `Sharp (0.7)` | Suggests a sharp fragment of crystal. |
| `-glow` | `Magic (0.9)`, `Light (0.8)` | Suggests emitting light or magic. |
| `-heart` | `Character (0.8)`, `Emotional (0.7)`, `Core (0.6)` | Suggests a central, emotional character. |
| `-song` | `Character (0.8)`, `Artistic (0.7)`, `Sound (0.6)` | Suggests a musical or eloquent character. |
| `-axe` | `Function::Tool (0.9)`, `Function::Weapon (0.7)` | Tool/weapon ending. |
| `-gem` | `Material::Crystal (0.9)`, `Rare (0.8)` | Precious stone ending. |
| `-ixir` | `Function::Medicine (0.9)`, `Material::Liquid (0.8)` | Potion/liquid ending. |
| `-guard` | `Function::Defense (0.9)`, `Protective (0.8)` | Defensive item ending. |
| `-wood` | `Material::Wood (1.0)`, `Flora (0.7)` | Tree/wood ending. |
| `-thorn` | `Texture::Spiky (0.9)`, `Flora (0.7)` | Spiky plant ending. |
| `-ia` | `Biome (0.8)`, `Land (0.7)`, `Feminine (0.5)` | Often for place names, soft ending. |
| `-ite` | `Mineral (0.9)`, `Hard (0.7)` | Common for mineral names, solid ending. |
| `-bloom` | `Flora (0.9)`, `Flower (0.8)`, `Beauty (0.7)` | Suggests growth and beauty. |
| `-claw` | `Character (0.7)`, `Aggressive (0.8)`, `Predator (0.9)` | Sharp, animalistic ending. |
| `-stone` | `Material::Stone (1.0)`, `Structure (0.8)` | Indicates a stone object or structure. |
| `-leaf` | `Flora (0.9)`, `Gentle (0.6)` | Soft, natural ending. |
| `-axe` | `Function::Tool (0.9)`, `Function::Weapon (0.7)` | Tool/weapon ending. |
| `-gem` | `Material::Crystal (0.9)`, `Rare (0.8)` | Precious stone ending. |
| `-ixir` | `Function::Medicine (0.9)`, `Material::Liquid (0.8)` | Potion/liquid ending. |
| `-guard` | `Function::Defense (0.9)`, `Protective (0.8)` | Defensive item ending. |
| `-wood` | `Material::Wood (1.0)`, `Flora (0.7)` | Tree/wood ending. |
| `-thorn` | `Texture::Spiky (0.9)`, `Flora (0.7)` | Spiky plant ending. |
| `-smith` | `Character (0.8)`, `Function::Crafting (0.9)`, `Material::Metal (0.7)` | Indicates a crafter, especially metal. |
| `-weaver` | `Character (0.8)`, `Function::Crafting (0.9)`, `Material::Fabric (0.7)` | Indicates a crafter, especially textiles. |
| `-kin` | `Character (0.8)`, `Small (0.7)`, `Affectionate (0.6)` | Diminutive, friendly character ending. |
| `-fang` | `Character (0.8)`, `Aggressive (0.7)`, `Predator (0.9)` | Sharp, predatory character ending. |
| `-root` | `Flora (0.9)`, `Deep (0.7)`, `Grounded (0.6)` | Suggests a root-based plant or foundation. |
| `-vale` | `Biome (0.8)`, `Valley (0.7)`, `Peaceful (0.6)` | Suggests a peaceful valley or low land. |
| `-glen` | `Biome (0.8)`, `SmallValley (0.7)`, `Secluded (0.6)` | Suggests a small, secluded valley. |
| `-mire` | `Biome (0.8)`, `Swamp (0.7)`, `Damp (0.6)` | Suggests a swampy, boggy area. |
| `-hide` | `Material::Fabric (0.9)`, `Protective (0.7)` | Suggests leather or animal skin. |
| `-scale` | `Material::Exoskeleton (0.9)`, `Protective (0.7)` | Suggests scales or armor. |
| `-shard` | `Material::Crystal (0.9)`, `Sharp (0.7)` | Suggests a sharp fragment of crystal. |
| `-glow` | `Magic (0.9)`, `Light (0.8)` | Suggests emitting light or magic. |
| `-heart` | `Character (0.8)`, `Emotional (0.7)`, `Core (0.6)` | Suggests a central, emotional character. |
| `-song` | `Character (0.8)`, `Artistic (0.7)`, `Sound (0.6)` | Suggests a musical or eloquent character. |


### 4.3. Phonological Constants

| Constant Name | Value | Description |
| :--- | :--- | :--- |
| `MAX_CONSONANT_CLUSTER` | `3` | Maximum number of consecutive consonants allowed in a generated name. |
| `MIN_NAME_LENGTH` | `4` | Minimum character length for a generated name. |
| `MAX_NAME_LENGTH` | `12` | Maximum character length for a generated name. |
| `VOWEL_SOUNDS` | `["a", "e", "i", "o", "u", "ae", "ea", "ou"]` | List of recognized vowel sounds for phonological checks. |
| `CONSONANT_SOUNDS` | `["b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v", "w", "x", "y", "z", "ch", "sh", "th"]` | List of recognized consonant sounds for phonological checks. |

This consolidated reference provides all the necessary components for implementing the procedural naming system, ensuring consistency and semantic depth across all generated names in Aetherbourne.
