# 🌍 World Systems & Generation

## Biome System

### Biome Types

**Desert**
* Temperature: Hot
* Precipitation: Very Low
* Flora: Sparse, heat-resistant
* Fauna: Nocturnal, heat-adapted
* Resources: Sand, Minerals (rare)

**Forest**
* Temperature: Moderate
* Precipitation: High
* Flora: Dense, varied
* Fauna: Diverse, abundant
* Resources: Wood, Game, Forage

**Grassland**
* Temperature: Moderate
* Precipitation: Moderate
* Flora: Grass, scattered trees
* Fauna: Herds, predators
* Resources: Game, Forage

**Mountain**
* Temperature: Cold
* Precipitation: Moderate-High
* Flora: Alpine, sparse
* Fauna: Specialized climbers
* Resources: Ore, Stone, Rare Flora

**Waterfront**
* Temperature: Moderate
* Precipitation: High
* Flora: Aquatic, wetland
* Fauna: Water-dependent
* Resources: Fish, Water Plants, Shells

**Swamp**
* Temperature: Warm
* Precipitation: Very High
* Flora: Dense, rotting
* Fauna: Amphibious, wetland-adapted
* Resources: Rare Plants, Insects

**Tundra**
* Temperature: Very Cold
* Precipitation: Low
* Flora: Sparse, short
* Fauna: Few, specialized
* Resources: Bone, Fur, Ice

---

## Mineral System

### Ore Types

**Iron Ore** → Iron ingots → Tools, weapons, structures

**Copper Ore** → Copper ingots → Wiring, decoration, alloys

**Tin Ore** → Tin ingots → Alloys (Bronze)

**Silver Ore** → Silver ingots → Decoration, currency, religious items

**Gold Ore** → Gold ingots → Decoration, currency, prestige

**Gems** (Emerald, Ruby, Sapphire, Diamond) → Cut/Polished → Decoration, trade, ritual

### Mining

* Ore richness varies by location
* Depletion over time (if harvested)
* Regenin deep mineral zones
* Regional distribution by biome

---

## Flora System

### Plant Categories

**Trees**
* Size: Small → Massive
* Growth: Fast → Slow
* Fruit: Yes / No
* Wood Quality: Soft → Hard

**Shrubs**
* Size: 0.5–3m
* Berries: Yes / No
* Firewood: Good

**Grasses & Forbs**
* Size: Micro → 2m
* Forage Value: Low → High
* Edibility: Toxic → Nutritious

**Aquatic Plants**
* Floating / Rooted
* Edibility: Toxic → Nutritious

### Growth Mechanics

* Seeding: Random + wind + animal dispersal
* Growth Rate: Biome & season dependent
* Decay: Natural lifecycle + damage
* Interaction: Harvested, trampled, regrown

---

## Mineral Distribution

### Geological Zones

**Surface Layer**
* Shallow ore deposits
* Common stone
* Rapid depletion

**Mid Layer**
* Moderate ore concentration
* Slower depletion
* Easier access than deep

**Deep Layer**
* Rich deposits
* Very slow depletion
* Difficult access (mountain slopes, cliff walls, caves)

### Biome-Specific Resources

**Desert:** Salt, Rare gems, Copper, limited stone

**Forest:** Lumber, Firewood, Game, Forage

**Grassland:** Game, Forage, limited stone

**Mountain:** Iron, Silver, Gold, Stone, Gems

**Waterfront:** Stone, Sand, Shells, Fish

**Swamp:** Peat, Rare plants, Limited game

**Tundra:** Bone, Fur, Ice, Stone

---

## Weather System

See [Time & Seasons](time-and-seasons.md) for comprehensive weather mechanics.

**Impact on Entities:**

* Temperature affects Thirst Drain (cold reduces, hot increases)
* Rain/Snow affects Vigil (visibility penalty)
* Wind affects movement speed
* Extreme weather triggers Warmth need urgency

---

## Structures

### Settlement-Generated Structures

* Dwelling: Shelter, storage, family life
* Storehouse: Food, resource stockpile
* Market: Trade hub
* Workshop: Crafting, production
* Temple: Religious, cultural center
* Fortification: Defense

### Natural Structures

* Caves: Shelter, resources, predator lair
* Cliffs: Defense, view, climbing challenge
* Stone circles: Possible ritual purpose
* Groves: Shelter, forage, possible religious significance
* Ruins: Historical artifacts, danger

---

## Item Proliferation

See [Item System](items.md) for detailed crafting, inventory, and trading mechanics.

**World Item Spawning:**

* Loot from fauna corpses
* Resource nodes (ore, wood, plants)
* Crafted items in civilization zones
* Ritual items in spiritual locations
* Trade route items
* Heirloom accumulation in settled areas

---

## World Generation Pipeline

### 1. Terrain Generation

* Heightmap via Perlin noise
* Biome assignment via temperature/precipitation layers
* Feature placement (mountains, valleys, water)

### 2. Resource Distribution

* Mineral seeding based on geologic realism
* Flora distribution per biome
* Water source placement

### 3. Civilization Seeding

* Initial settlement placement
* Trade route establishment
* Territorial boundary formation

### 4. Fauna Population

* Species distribution per biome
* Population density calibration
* Predator/prey balance

### 5. Time Simulation

* World ages through initial cycles
* Fauna populations stabilize
* Civilizations develop
* Environmental pressure emerges
