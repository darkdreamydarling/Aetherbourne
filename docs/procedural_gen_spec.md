# Aetherbourne Procedural Generation Specification

This document outlines the detailed, codifiable process for procedurally generating all elements within the Aetherbourne world, from macro-level biomes to micro-level item attributes. A key focus is to include specific parameters and rules that directly inform the procedural generation of 2D pixel art for each element, ensuring visual consistency and emergent diversity.

## 1. World Generation Pipeline and Biome/Tile Systems

The world generation process is a multi-stage pipeline, starting with large-scale geographical features and progressively detailing down to individual tiles and their contents.

### 1.1. World Map Generation (Macro-Level)

This initial stage generates the global topography, climate zones, and major landmasses.

#### 1.1.1. Noise-Based Terrain Generation

*   **Algorithm:** Perlin Noise or Simplex Noise for elevation, moisture, and temperature maps.
*   **Parameters:**
    *   `NOISE_SCALE_ELEVATION: f32 = 200.0` (Controls continent size)
    *   `NOISE_OCTAVES_ELEVATION: u8 = 6` (Detail level)
    *   `NOISE_PERSISTENCE_ELEVATION: f32 = 0.5` (Influence of each octave)
    *   `NOISE_LACUNARITY_ELEVATION: f32 = 2.0` (Frequency increase per octave)
    *   Similar parameters for `MOISTURE` and `TEMPERATURE`.

#### 1.1.2. Biome Assignment

Biomes are assigned based on a combination of elevation, moisture, and temperature values.

**Biome Definition Table:**

| Biome Type | Elevation Range | Moisture Range | Temperature Range | Primary Color Palette (Hex) | Secondary Color Palette (Hex) | Dominant Texture Features (Pixel Art) |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| `Ocean` | `0.0 - 0.1` | `0.8 - 1.0` | `0.0 - 1.0` | `#000080` (Deep Blue), `#4682B4` (Steel Blue) | `#ADD8E6` (Light Blue), `#FFFFFF` (Foam) | **Pixel Art Style:** Smooth, gently rippling patterns with soft anti-aliasing. Occasional, small, foamy white caps. Use subtle gradients for depth. **Aesthetic:** Calm, inviting, vast. |
| `Beach` | `0.1 - 0.15` | `0.5 - 1.0` | `0.4 - 0.8` | `#F4A460` (Sandy Brown), `#D2B48C` (Tan) | `#8B4513` (Dark Brown, rocks), `#FFFFFF` (Shells) | **Pixel Art Style:** Fine, granular sand texture with soft edges. Scattered, rounded pebbles and small, stylized shells. Gentle wave lines at the water's edge. **Aesthetic:** Peaceful, warm, inviting for exploration. |
| `Desert` | `0.15 - 0.7` | `0.0 - 0.2` | `0.7 - 1.0` | `#F0E68C` (Khaki), `#D2B48C` (Tan) | `#8B4513` (Cactus Green), `#A0522D` (Sienna) | **Pixel Art Style:** Smooth, undulating dunes with clear highlights and shadows. Sparse, stylized cacti and rounded rock formations. Use warm, muted tones. **Aesthetic:** Arid, serene, but with a hint of harshness. |
| `Grassland` | `0.15 - 0.5` | `0.3 - 0.7` | `0.4 - 0.8` | `#7CFC00` (Lawn Green), `#ADFF2F` (Green Yellow) | `#8B4513` (Dirt), `#D2B48C` (Dry Grass) | **Pixel Art Style:** Short, dense grass sprites with subtle variations in height and color. Small, brightly colored wildflowers. Visible, earthy paths. **Aesthetic:** Lush, vibrant, open, and welcoming. |
| `Forest` | `0.15 - 0.6` | `0.6 - 1.0` | `0.3 - 0.7` | `#228B22` (Forest Green), `#3CB371` (Medium Sea Green) | `#8B4513` (Tree Trunks), `#A0522D` (Fallen Leaves) | **Pixel Art Style:** Varied, stylized tree sprites with rounded canopies. Dappled light effects on the forest floor. Rich, earthy tones for ground and trunks. **Aesthetic:** Enveloping, mysterious, yet comforting. |
| `Mountain` | `0.6 - 1.0` | `0.0 - 0.5` | `0.0 - 0.5` | `#808080` (Gray), `#A9A9A9` (Dark Gray) | `#FFFFFF` (Snow), `#696969` (Slate Gray) | **Pixel Art Style:** Sharp, angular rock formations with clear outlines. White, soft snow caps. Minimal, hardy vegetation sprites. **Aesthetic:** Majestic, challenging, stark beauty. |
| `Tundra` | `0.15 - 0.4` | `0.2 - 0.6` | `0.0 - 0.3` | `#B0C4DE` (Light Steel Blue), `#778899` (Light Slate Gray) | `#8B4513` (Frozen Earth), `#FFFFFF` (Snow Patches) | **Pixel Art Style:** Flat, muted ground textures with visible permafrost lines. Small, hardy shrub sprites. Occasional, sparkling icy patches. **Aesthetic:** Cold, resilient, subtle beauty. |

### 1.2. Tile-Level Generation (Micro-Level)

Each tile within a biome is further detailed with specific terrain features, resource nodes, and environmental modifiers.

#### 1.2.1. Tile Type Assignment

Within each biome, tiles can have variations (e.g., `Forest_Dense`, `Forest_Sparse`, `Forest_Riverbank`).

**Tile Type Definition Table (Example for Forest Biome):**

| Tile Type | Biome | Probability | Terrain Features | Pixel Art Visual Cues |
| :--- | :--- | :--- | :--- | :--- |
| `Forest_Dense` | `Forest` | `0.6` | High tree density, thick undergrowth | **Pixel Art Style:** Overlapping, slightly darker tree sprites with rounded canopies. Ground texture mostly obscured by dense foliage. **Aesthetic:** Secluded, rich, slightly mysterious. |
| `Forest_Sparse` | `Forest` | `0.3` | Lower tree density, clear paths | **Pixel Art Style:** Fewer, distinct tree sprites. Clear, earthy paths visible. Scattered, stylized leaf sprites on the ground. **Aesthetic:** Open, walkable, gentle. |
| `Forest_Riverbank` | `Forest` | `0.1` | Adjacent to `River` tiles, muddy ground | **Pixel Art Style:** Blended brownish-green ground texture. Clear, slightly wavy water edge. Small, upright reed sprites. **Aesthetic:** Tranquil, fertile, natural boundary. |

#### 1.2.2. Environmental Modifiers

Each tile can have dynamic modifiers that affect gameplay and visual representation.

*   **Temperature:** `current_temp_c: f32` (Influences visual effects like frost, heat haze).
*   **Humidity:** `current_humidity: f32` (Influences mist, dew, dryness).
*   **Wind Speed:** `current_wind_speed: f32` (Influences swaying trees, dust effects).
*   **Light Level:** `current_light_level: f32` (Day/night cycle, shadows, bioluminescence).

**Pixel Art Implications:**

*   **Frost:** A delicate overlay of light blue/white pixels, particularly on ground textures and the edges of plant sprites, applied when `current_temp_c < FROST_THRESHOLD`. **Aesthetic:** Crisp, delicate, cold.
*   **Heat Haze:** A subtle, shimmering distortion or blur effect applied to background tiles, increasing with `current_temp_c > HEAT_HAZE_THRESHOLD`. **Aesthetic:** Intense, dry, atmospheric.
*   **Mist/Fog:** A soft, semi-transparent gray/white overlay, with density varying based on `current_humidity` and `light_level`. It should gently obscure distant elements. **Aesthetic:** Ethereal, mysterious, damp.
*   **Shadows:** Dynamic darkening of tile pixels and entity sprites based on `light_level` and the sun/moon position. Shadows should have soft, feathered edges. **Aesthetic:** Realistic depth, time-of-day indication, dramatic. |

This foundational layer ensures a diverse and visually responsive world. The next phase will populate this world with flora and minerals.

## 2. Flora and Mineral Procedural Generation

This section details the procedural generation of plants and minerals, including their attributes, distribution logic, and specific instructions for their 2D pixel art representation to maintain the desired aesthetic.

### 2.1. Flora Generation

Plants are generated based on biome, tile type, and environmental modifiers. Each plant type has unique attributes and visual characteristics.

#### 2.1.1. Plant Attributes

```rust
pub struct Plant {
    pub plant_type: PlantType,
    pub growth_stage: f32,      // 0.0 - 1.0 (seedling to mature)
    pub health: f32,            // 0.0 - 1.0
    pub resource_yield: HashMap<ResourceType, f32>, // e.g., "food": 10.0, "wood": 5.0
    pub thermal_tolerance: (f32, f32), // Min/Max temperature
    pub moisture_tolerance: (f32, f32), // Min/Max moisture
    pub visual_properties: PlantVisualProperties,
}

pub struct PlantVisualProperties {
    pub base_sprite_id: String, // Reference to base sprite sheet
    pub color_palette_shift: (f32, f32, f32), // HSL shift for variation
    pub size_multiplier: f32,   // 0.5 - 2.0
    pub animation_frames: u8,   // Number of frames for idle animation (e.g., swaying)
    pub aesthetic_notes: String, // Specific art direction notes
}
```

#### 2.1.2. Plant Distribution Logic

Plants are distributed using a combination of weighted random selection and density-based placement within suitable tiles.

*   **Biome Suitability:** Each `PlantType` has a `BIOME_SUITABILITY_SCORE` for each biome.
*   **Tile Density:** `MAX_PLANTS_PER_TILE` varies by `TileType` (e.g., `Forest_Dense` has higher density).
*   **Environmental Check:** `current_temp_c` and `current_humidity` must be within `thermal_tolerance` and `moisture_tolerance`.

#### 2.1.3. Plant Types and Pixel Art Guidelines

| Plant Type | Biome (Primary) | Resource Yield (Example) | Thermal Tolerance (C) | Moisture Tolerance | Pixel Art Style | Aesthetic Notes |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| `BerryBush` | `Grassland`, `Forest` | `Food: 5.0` | `5 - 30` | `0.4 - 0.9` | Small, rounded bush sprite with distinct, brightly colored berries. Leaves are a vibrant green. | Cute, inviting, edible. Berries should pop visually. |
| `OakTree` | `Forest` | `Wood: 20.0`, `Acorns: 2.0` | `-10 - 25` | `0.6 - 1.0` | Sturdy, broad trunk with a large, rounded canopy. Leaves are deep green, turning orange/brown in autumn biomes. | Majestic, grounding, provides shelter. |
| `Cactus` | `Desert` | `Water: 3.0` | `20 - 45` | `0.0 - 0.2` | Tall, segmented, spiky sprite. Can have small, colorful flowers. | Resilient, unique, slightly dangerous. Spikes should be stylized, not overly sharp. |
| `Reeds` | `Riverbank`, `Swamp` | `Fiber: 2.0` | `0 - 25` | `0.8 - 1.0` | Thin, tall, green stalks with subtle swaying animation. | Graceful, natural, indicates water. |
| `Snowdrop` | `Tundra` | `Medicine: 1.0` | `-15 - 5` | `0.2 - 0.6` | Tiny, delicate white flower sprite emerging from snow/frozen ground. | Fragile, hopeful, symbol of resilience. |

**General Pixel Art Guidelines for Flora:**

*   **Outline:** Use a slightly darker shade of the main color for outlines, or a very thin dark gray/brown for contrast, avoiding harsh black lines.
*   **Color Palette:** Utilize a limited, harmonious palette per plant, with vibrant but not oversaturated colors. Employ subtle gradients for depth.
*   **Shape:** Lean towards rounded, organic shapes. Even spiky plants should have a stylized, less aggressive appearance.
*   **Detail:** Focus on key identifying features. For berries, make them clearly visible. For leaves, suggest texture rather than drawing every vein.
*   **Animation:** Simple idle animations (e.g., gentle swaying, subtle pulsing for glowing plants) add life.
*   **Growth Stages:** Implement distinct sprites or color shifts for `growth_stage` (e.g., seedling is smaller and lighter, mature is larger and darker/richer).

### 2.2. Mineral Generation

Minerals are static resources found within specific biomes and tile types, often associated with geological features.

#### 2.2.1. Mineral Attributes

```rust
pub struct Mineral {
    pub mineral_type: MineralType,
    pub rarity: f32,            // 0.0 - 1.0 (common to rare)
    pub hardness: f32,          // 0.0 - 1.0 (influences mining difficulty)
    pub resource_yield: HashMap<ResourceType, f32>, // e.g., "iron_ore": 10.0, "stone": 50.0
    pub visual_properties: MineralVisualProperties,
}

pub struct MineralVisualProperties {
    pub base_sprite_id: String, // Reference to base sprite sheet
    pub color_palette_shift: (f32, f32, f32), // HSL shift for variation
    pub sparkle_animation_chance: f32, // 0.0 - 1.0 (chance of subtle sparkle animation)
    pub aesthetic_notes: String, // Specific art direction notes
}
```

#### 2.2.2. Mineral Distribution Logic

Minerals are generated in clusters or veins, with rarity influencing their spawn probability.

*   **Biome Suitability:** Each `MineralType` has a `BIOME_SUITABILITY_SCORE` (e.g., `IronOre` in `Mountain`, `Coal` in `Forest` near ancient deposits).
*   **Geological Features:** Higher spawn chance near `Mountain` tiles, `Cave` entrances, or specific `RockFormation` tile features.
*   **Rarity:** `random() < mineral.rarity` check for initial spawn, then cluster generation.

#### 2.2.3. Mineral Types and Pixel Art Guidelines

| Mineral Type | Biome (Primary) | Resource Yield (Example) | Rarity | Pixel Art Style | Aesthetic Notes |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `Stone` | All | `Stone: 100.0` | `0.9` | Irregular, blocky gray/brown sprite with subtle texture. | Common, foundational, sturdy. |
| `IronOre` | `Mountain`, `Forest` | `Iron: 10.0` | `0.4` | Dark, metallic gray rock sprite with reddish-brown streaks. | Industrial, valuable, slightly rough. |
| `Coal` | `Forest`, `Mountain` | `Fuel: 50.0` | `0.3` | Black, jagged rock sprite with a dull sheen. | Energy source, dirty, essential. |
| `Crystal` | `Cave`, `Mountain` | `MagicShard: 5.0` | `0.1` | Translucent, faceted sprite with a soft, internal glow. Color varies (e.g., blue, purple, green). | Enchanting, rare, emits subtle light. Should feel precious. |

**General Pixel Art Guidelines for Minerals:**

*   **Outline:** Similar to flora, use slightly darker shades or thin, contrasting lines. Avoid overly sharp edges unless for specific crystal formations.
*   **Color Palette:** Use natural, earthy tones for common minerals. Rare minerals can have more vibrant, jewel-like colors, but keep them slightly muted to fit the realistic aspect.
*   **Shape:** Irregular, organic shapes for raw ores and stone. Crystals can be more geometric but should still feel naturally formed.
*   **Detail:** Suggest texture (e.g., rough for stone, metallic for ore) with pixel clusters. For crystals, focus on facets and internal light.
*   **Animation:** Subtle, infrequent sparkle animations for rare or magical minerals can enhance their appeal without being distracting.

This section ensures the world is rich with interactive resources, each with a distinct visual identity. Next, we will delve into the living inhabitants.

## 3. DNA-Driven Character and Creature Procedural Generation

Characters and creatures (referred to collectively as 'entities' for generation purposes) are the most complex procedurally generated elements, driven by their underlying DNA and influenced by environmental pressures. Their generation involves translating genetic information into physical appearance, behavioral tendencies, and statistical attributes.

### 3.1. Entity Attributes from DNA

As detailed in the `Aetherbourne Genetics & Evolution Specification`, an entity's DNA (`Genome`) determines its `Phenotype`. This phenotype directly translates into the attributes used for generation.

```rust
pub struct Entity {
    pub entity_id: u64,
    pub species_type: SpeciesType,
    pub genome: Genome,
    pub physiological_traits: PhysiologicalTraits, // Derived from Genome
    pub cognitive_traits: CognitiveTraits,         // Derived from Genome
    pub visual_properties: EntityVisualProperties,
}

pub struct PhysiologicalTraits {
    pub body_covering_type: BodyCoveringType,
    pub covering_thickness: f32, // 0.0 - 1.0
    pub covering_density: f32,   // 0.0 - 1.0
    pub mass_kg: f32,
    pub height_m: f32,
    pub limb_count: u8,
    pub coloration_hue: f32, // 0.0 - 1.0
    pub coloration_saturation: f32, // 0.0 - 1.0
    pub coloration_lightness: f32, // 0.0 - 1.0
    // ... other physiological traits from DNA
}

pub struct EntityVisualProperties {
    pub base_sprite_sheet_id: String, // e.g., "humanoid_base", "quadruped_base"
    pub body_shape_modifier: f32,     // 0.0 - 1.0 (e.g., lean to bulky)
    pub limb_length_modifier: f32,    // 0.0 - 1.0 (e.g., short to long)
    pub head_size_modifier: f32,      // 0.0 - 1.0 (e.g., small to large)
    pub eye_color_hsl: (f32, f32, f32),
    pub fur_pattern_id: Option<String>, // e.g., "stripes", "spots"
    pub scale_texture_id: Option<String>, // e.g., "smooth", "rough"
    pub aesthetic_notes: String, // Specific art direction notes
}
```

#### 3.1.1. Mapping DNA to Visual Properties

*   **`BodyCoveringType`:** Determined by `integument_type` gene (discrete allele). Each type has a base sprite sheet.
*   **`covering_thickness`, `covering_density`:** Directly mapped from `thermal_insulation` and `integument_density` genes (0-10, normalized to 0-1).
*   **`mass_kg`, `height_m`:** Derived from `body_mass_index` gene, scaled within species-specific min/max ranges.
*   **`coloration_hue`, `coloration_saturation`, `coloration_lightness`:** Derived from `coloration_gene` (e.g., 3 genes for HSL values, or a single gene mapped to a predefined palette).
*   **`fur_pattern_id`, `scale_texture_id`:** Can be determined by a `pattern_gene` (discrete allele for pattern type) or procedurally generated based on `coloration_gene` and `covering_density`.

### 3.2. Entity Generation Logic

Entities are generated at world creation and through reproduction, with initial populations influenced by biome suitability and evolutionary history.

#### 3.2.1. Initial Population Generation

*   **Species Selection:** For each biome, a weighted list of `SpeciesType` is available (e.g., `Forest` biome favors `Deer`, `Wolf`, `Bear`).
*   **DNA Initialization:** Initial `Genome` is generated with `DEFAULT_ALLELE_VALUE` (e.g., 5.0 for all genes), with slight random variations (`INITIAL_GENETIC_VARIANCE`).
*   **Placement:** Entities are placed in suitable `TileType`s within their biome, respecting `MAX_ENTITIES_PER_TILE` and `MIN_SPAWN_DISTANCE`.

#### 3.2.2. Reproduction-Based Generation

As detailed in the `Aetherbourne Genetics & Evolution Specification`, offspring inherit DNA from parents with mutation. The `Phenotype` is then calculated from this new `Genome`.

### 3.3. Character/Creature Pixel Art Guidelines

The goal is a "cute yet realistic" aesthetic. This means recognizable animal forms with slightly exaggerated, friendly features, and clear, readable silhouettes.

*   **Base Sprites:** Each `SpeciesType` (e.g., `Deer`, `Wolf`, `Humanoid`) has a set of base sprites for different poses and animations (idle, walk, run, attack, eat, sleep).
*   **Proportions:** Maintain generally realistic body proportions, but allow for slight exaggeration in head size or eye size to enhance cuteness, especially for younger entities. `head_size_modifier` can influence this.
*   **Coloration:**
    *   Use the `coloration_hue`, `saturation`, `lightness` from `PhysiologicalTraits` to dynamically tint base sprites.
    *   Ensure colors are vibrant but not garish. Use subtle gradients for shading and depth.
    *   **Aesthetic:** Natural, varied, but always pleasing to the eye. Avoid overly aggressive or dull palettes.
*   **Body Covering (Fur, Scales, Skin):**
    *   **Fur:** Render as soft, textured pixels. `covering_thickness` influences the 
visual density and fluffiness. `fur_pattern_id` dictates patterns like stripes or spots, rendered with soft, blended edges.
    *   **Scales:** Render as overlapping, distinct pixel shapes. `covering_thickness` influences the size and prominence of individual scales. `scale_texture_id` can add variations like smooth, rough, or iridescent.
    *   **Skin:** Render with subtle texture, possibly with pores or wrinkles depending on `covering_thickness` and `density`. Use soft shading.
*   **Facial Features:** Eyes should be expressive and generally large. Mouths can be simple lines or small, expressive shapes. Avoid overly sharp teeth unless for explicitly predatory species, and even then, stylize them.
*   **Animation:** Implement smooth, low-frame-count animations for movement (walk, run, idle). Emphasize keyframes to convey motion effectively. Subtle idle animations (e.g., breathing, head tilts) enhance realism and cuteness.
*   **Age Progression:** Implement distinct sprite sets or scaling/tinting for different age stages (e.g., `baby`, `juvenile`, `adult`, `elder`). Babies should have larger heads and eyes, softer features. Elders might have slightly faded colors or more pronounced features.
*   **Damage/Status Effects:** Visual cues for health status (e.g., slight desaturation for low health, small sweat droplets for overheating, shivering for cold). These should be overlays or subtle sprite alterations, not drastic changes.

This section ensures that every living entity in Aetherbourne is not only functionally unique due to its DNA but also visually distinct and appealing, contributing to the overall "cute yet realistic" aesthetic. Next, we will outline the generation of items and structures.

## 4. Item and Structure Procedural Generation

This section details the procedural generation of items and structures, outlining their attributes, distribution logic, and specific instructions for their 2D pixel art representation to align with the "cute yet realistic" aesthetic.

### 4.1. Item Generation

Items are diverse objects that entities can interact with, ranging from tools and weapons to food and crafting materials. Their generation is influenced by the environment and entity activities.

#### 4.1.1. Item Attributes

```rust
pub struct Item {
    pub item_id: u64,
    pub item_type: ItemType,
    pub base_material: MaterialType,
    pub quality: f32,           // 0.0 - 1.0 (poor to excellent)
    pub durability: f32,        // 0.0 - 1.0
    pub modifiers: Vec<ItemModifier>, // e.g., "Sharpness +2", "Insulated"
    pub visual_properties: ItemVisualProperties,
}

pub struct ItemVisualProperties {
    pub base_sprite_id: String, // Reference to base sprite sheet
    pub color_palette_shift: (f32, f32, f32), // HSL shift for variation
    pub size_multiplier: f32,   // 0.8 - 1.2
    pub glow_effect: Option<GlowEffect>, // For magical or special items
    pub aesthetic_notes: String, // Specific art direction notes
}

pub enum ItemModifier {
    Sharpness(f32),
    Insulated(f32),
    Healing(f32),
    // ... other modifiers
}
```

#### 4.1.2. Item Distribution Logic

Items can be found in the environment, dropped by entities, or crafted. Environmental generation follows specific rules.

*   **Biome/Tile Suitability:** Certain `ItemType`s are more likely to appear in specific biomes or tile types (e.g., `FishingRod` near `River` tiles, `Berries` in `Grassland`).
*   **Container Spawns:** Items can spawn within `Container` structures (e.g., `Chest`, `Crate`) with higher probabilities for specific `ItemType`s.
*   **Quality Generation:** `quality` is influenced by the `Biome` (e.g., higher quality tools in resource-rich areas) and `CraftingSkill` of the creator.
*   **Modifier Generation:** `ItemModifier`s are procedurally generated based on `quality` and `rarity` of the item, with rare items having a higher chance of powerful modifiers.

#### 4.1.3. Item Types and Pixel Art Guidelines

| Item Type | Base Material (Example) | Common Location | Quality Impact (Visual) | Pixel Art Style | Aesthetic Notes |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `StoneAxe` | `Stone`, `Wood` | `Forest`, `Mountain` | Higher quality = smoother edges, subtle metallic sheen on blade. | Simple, functional, slightly chunky. Stone head with a wooden handle. | Recognizable tool, feels sturdy. |
| `Berries` | `Plant` | `Grassland`, `Forest` | Freshness = vibrant color; decaying = duller, slightly shriveled. | Small, rounded, brightly colored clusters. | Appealing, edible, clearly distinguishable. |
| `LeatherTunic` | `Leather` | `Crafted`, `Dropped` | Higher quality = finer texture, more intricate stitching. | Soft, slightly textured fabric. Simple, practical design. | Comfortable, protective, not overly ornate. |
| `IronSword` | `Iron` | `Crafted`, `Ruins` | Higher quality = sharper blade, polished hilt, subtle gleam. | Stylized blade, distinct hilt. Can have a subtle sparkle. | Heroic, balanced, not overly aggressive. |
| `MagicOrb` | `Crystal` | `Rare`, `Dungeon` | Quality affects glow intensity and color vibrancy. | Translucent sphere with internal swirling light. | Mysterious, powerful, emits soft light. |

**General Pixel Art Guidelines for Items:**

*   **Outline:** Use a clear, slightly darker outline to make items pop against backgrounds. Avoid overly thick lines.
*   **Color Palette:** Use a base color palette appropriate for the material, then apply `color_palette_shift` for variations. Keep colors vibrant but grounded.
*   **Shape:** Maintain clear, recognizable silhouettes. Tools should look like tools, food like food. Exaggerate key features slightly for readability at small sizes.
*   **Detail:** Focus on essential details. For a sword, the blade and hilt are key. For food, texture and freshness cues are important.
*   **Perspective:** Use a consistent isometric or top-down perspective for all items to ensure they fit seamlessly into the game world.
*   **Condition/Durability:** Visually represent `durability` (e.g., cracks, rust, faded colors for worn items; sparkling for pristine).
*   **Modifiers:** Subtle visual cues for `ItemModifier`s (e.g., a faint blue aura for `Insulated`, a small glint for `Sharpness`).

### 4.2. Structure Generation

Structures are static, persistent features in the world, ranging from natural formations to player-built constructions and ancient ruins.

#### 4.2.1. Structure Attributes

```rust
pub struct Structure {
    pub structure_id: u64,
    pub structure_type: StructureType,
    pub base_material: MaterialType,
    pub condition: f32,         // 0.0 - 1.0 (ruined to pristine)
    pub purpose: StructurePurpose, // e.g., "Shelter", "Crafting", "Storage"
    pub visual_properties: StructureVisualProperties,
}

pub struct StructureVisualProperties {
    pub base_sprite_id: String, // Reference to base sprite sheet
    pub color_palette_shift: (f32, f32, f32), // HSL shift for variation
    pub size_multiplier: f32,   // 0.8 - 1.5
    pub damage_overlay_id: Option<String>, // e.g., "cracks", "moss"
    pub aesthetic_notes: String, // Specific art direction notes
}
```

#### 4.2.2. Structure Distribution Logic

Structures are generated based on biome, terrain, and proximity to other features or settlements.

*   **Biome Suitability:** `StructureType`s are tied to specific biomes (e.g., `WoodenHut` in `Forest`, `StoneRuins` in `Mountain`).
*   **Terrain Requirements:** Structures require specific `TileType`s (e.g., `FlatGround` for `Farm`, `WaterEdge` for `Dock`).
*   **Clustering:** Settlements generate clusters of related structures (e.g., `House`, `Workshop`, `Storage`).
*   **Random Placement:** Natural structures (e.g., `RockFormation`, `CaveEntrance`) are placed randomly within suitable areas, with density controls.

#### 4.2.3. Structure Types and Pixel Art Guidelines

| Structure Type | Base Material (Example) | Primary Biome | Condition Impact (Visual) | Pixel Art Style | Aesthetic Notes |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `WoodenHut` | `Wood` | `Forest`, `Grassland` | Deterioration = faded colors, missing planks, moss. | Simple, cozy, slightly asymmetrical. Visible wood grain. | Inviting, rustic, feels like home. |
| `StoneWall` | `Stone` | `Mountain`, `Plains` | Damage = visible cracks, crumbling sections. | Stacked, irregular stone blocks. Strong, defensive. | Sturdy, protective, ancient. |
| `FarmPlot` | `Soil` | `Grassland` | Condition reflects soil fertility and crop health. | Tilled earth with small, green crop sprouts. | Productive, organized, natural. |
| `AncientRuins` | `Stone` | `Desert`, `Mountain` | Heavy damage, overgrown with local flora, weathered textures. | Broken columns, crumbling walls, hints of past grandeur. | Mysterious, historical, melancholic. |

**General Pixel Art Guidelines for Structures:**

*   **Outline:** Clear, slightly thicker outlines for structural integrity. Use darker shades of the material color.
*   **Color Palette:** Earthy, natural tones for most structures. Ancient or magical structures can have more unique, faded, or glowing palettes.
*   **Shape:** Generally blocky and robust for functional structures. Ruins can be more fragmented and organic. Maintain a sense of scale.
*   **Detail:** Focus on material textures (wood grain, stone patterns). Add small details like windows, doors, or roof tiles. For ruins, emphasize decay and overgrowth.
*   **Condition:** Visually represent `condition` through `damage_overlay_id` (e.g., cracks, missing pieces, moss, rust). A pristine structure should look well-maintained, a ruined one clearly derelict.
*   **Interaction Points:** Clearly mark interactive elements like doors, windows, or crafting stations with distinct visual cues.

This section completes the procedural generation framework, ensuring that every element of the Aetherbourne world, from the smallest berry to the largest mountain, is generated with both functional depth and a consistent, appealing visual style.
