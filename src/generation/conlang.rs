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
                Morpheme { text: "kor".to_string(), morpheme_type: MorphemeType::Suffix, semantic_tags: HashMap::from([(SemanticTag::Material(MaterialValue::Stone), 0.9), (SemanticTag::Quality(QualityValue::Hard), 0.8), (SemanticTag::Mineral), 0.7)]) },
                Morpheme { text: "lume".to_string(), morpheme_type: MorphemeType::Suffix, semantic_tags: HashMap::from([(SemanticTag::Flora), 0.9), (SemanticTag::Aspect(AspectValue::Growth), 0.8), (SemanticTag::Aspect(AspectValue::Beauty), 0.7)]) },
                Morpheme { text: "kar".to_string(), morpheme_type: MorphemeType::Suffix, semantic_tags: HashMap::from([(SemanticTag::Aspect(AspectValue::Aggressive), 0.8), (SemanticTag::Aspect(AspectValue::Predator), 0.7), (SemanticTag::Character), 0.6)]) },
                Morpheme { text: "tol".to_string(), morpheme_type: MorphemeType::Suffix, semantic_tags: HashMap::from([(SemanticTag::Function(FunctionValue::Tool), 0.9), (SemanticTag::Function(FunctionValue::Weapon), 0.7), (SemanticTag::Material(MaterialValue::Metal), 0.6)]) },
                Morpheme { text: "yala".to_string(), morpheme_type: MorphemeType::Suffix, semantic_tags: HashMap::from([(SemanticTag::Character), 0.8), (SemanticTag::Aspect(AspectValue::Spirit), 0.7), (SemanticTag::Aspect(AspectValue::Emotional), 0.6)]) },
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
        (SemanticTag::Flora, SemanticTag::Color(_)) => true,
        (SemanticTag::Flora, SemanticTag::Texture(_)) => true,
        (SemanticTag::Mineral, SemanticTag::Color(_)) => true,
        (SemanticTag::Mineral, SemanticTag::Texture(_)) => true,
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
    warrior_attrs.insert(SemanticTag::Character, 1.0);
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
    plant_attrs.insert(SemanticTag::Flora, 1.0);
    plant_attrs.insert(SemanticTag::Aspect(AspectValue::Delicate), 0.9);
    plant_attrs.insert(SemanticTag::Color(ColorValue::Green), 0.7);
    if let Some(name) = generate_name(&plant_attrs, &lexicon, &phonology_config, &EntityType::Flora, &mut rng) {
        println!("Delicate Plant: {}", name);
    }
}
