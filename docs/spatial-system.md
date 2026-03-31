# 🗺️ Spatial System

The spatial system manages entity positioning, movement, perception ranges, and pathfinding within the world. It provides the foundation for all location-based mechanics.

---

## Coordinate System

### 2D World Grid

* **World coordinates**: (X, Y) integer positions
* **Tile-based movement**: Entities occupy discrete tiles (1 meter ≈ 1 tile)
* **Boundary**: World wraps or has hard boundaries (configurable)

### Altitude/Depth

* **Z-axis**: For vertical positioning (caves, cliffs, flying)
* **Layering**: Multiple vertical levels can occupy same X,Y (cave level, surface, sky)

---

## Spatial Partitioning

### Purpose

Efficient storage and query of entities for combat, perception, and resource gathering (avoid O(n²) checks every frame).

### Grid-Based Partitioning

* **Chunk size**: Typically 32×32 or 64×64 tiles per chunk
* **Chunk grid**: World divided into regular chunks
* **Entity registration**: Each entity belongs to a chunk
* **Update on movement**: When entity moves to new chunk, registered in new chunk grid

### Spatial Queries

Optimize common operations:

**"What entities within 50 tiles of position (100, 200)?"**
* Calculate chunk(s) covering circular range
* Query only those chunks
* Return entities within distance threshold

**"What resources in region X?"**
* Query chunk(s) covering region
* Filter for resource type
* Return list

---

## Movement

### Movement Types

**Cardinal Movement** (North, South, East, West)
* Cost: Distance × Terrain_multiplier
* Time: Distance / Speed_stat

**Diagonal Movement** (NE, SE, SW, NW)
* Cost: Distance × √2 × Terrain_multiplier (slightly costlier than cardinal)

**Vertical Movement** (Climb, Descend)
* Cost: Distance × Climb_difficulty × Skill_factor
* Time: Longer than horizontal; requires Lithe/Vigor

**Special Movement** (Swim, Fly, Teleport)
* Restricted by capability

### Terrain Effects

Each tile has terrain type modifying movement:

| Terrain | Speed Multiplier | Climb Cost | Notes |
|---------|-----------------|-----------|-------|
| Grass | 1.0 | 0.5 | Normal movement |
| Forest | 0.7 | 1.0 | Dense; slower |
| Mountain | 0.5 | 3.0 | Steep; climb required |
| Water (shallow) | 0.6 | N/A | Swimable |
| Water (deep) | 0.3 | N/A | Slow; strong swimmers only |
| Desert | 0.8 | 0.5 | Some resistance |
| Marsh | 0.4 | 1.5 | Bog; very slow |
| Road | 1.2 | 0.2 | Faster travel |
| Urban | 0.9 | 1.0 | Settlement terrain |

### Cost Calculation

```
MovementCost = Distance × Terrain_Multiplier × Weather_Multiplier × Encumbrance_Multiplier
MovementTime = MovementCost / (Speed_Derived_Stat × Skill_Bonus)
```

**Speed Derived Stat**: Coordination (Lithe + Vigor)

---

## Perception Ranges

Entities perceive based on **sensory modality** and **stat bonuses**:

### Vision

```
Visual_Range = Base_Range × (1 + Vigil × 0.1) × Light_Modifier × Obstacle_Modifier
```

* **Base Range**: 30 tiles (typical humanoid)
* **Light Modifier**: 1.0 (daylight), 0.3 (night), 0.1 (dark cave)
* **Obstacle Modifier**: Reduced by terrain cover, fog, etc.

### Hearing

```
Hearing_Range = Base_Range × (1 + Vigil × 0.08) × Sound_Carrying_Factor × Wind_Modifier
```

* **Base Range**: 50 tiles (sound travels farther)
* **Sound_Carrying_Factor**: 1.0 (normal), 0.5 (rain/wind masking), 1.5 (stone caves echo)

### Smell

```
Smell_Range = Base_Range × (1 + Vigil × 0.12) × Scent_Age_Decay × Wind_Direction_Factor
```

* **Base Range**: 40 tiles (varies by species)
* **Scent_Age_Decay**: Older scents fainter; exponential decay
* **Wind_Direction_Factor**: Downwind smell enhanced; upwind blocked

---

## Pathfinding

### Algorithm

**A\* Pathfinding** for optimal paths:

* Heuristic: Manhattan or Euclidean distance to goal
* Node cost: Terrain difficulty + obstacle avoidance
* Updates: Real-time recalculation if path blocked

### Intelligent Movement

**Entities don't just pathfind to goal; they:**

* **Avoid threats**: Detour around perceived predators
* **Follow personality**: Aggressive entity charges; cautious entity takes longer, safer route
* **Remember terrain**: Use learned efficient paths
* **Respond to obstacles**: Update path if new obstacle appears

---

## Distance Calculations

### Euclidean Distance

```
Distance = √((X₂ - X₁)² + (Y₂ - Y₁)²)
```

Used for precise range calculations.

### Manhattan Distance

```
Distance = |X₂ - X₁| + |Y₂ - Y₁|
```

Used for quick approximations (cheaper computationally).

### Chebyshev Distance

```
Distance = max(|X₂ - X₁|, |Y₂ - Y₁|)
```

Used for square-based ranges (e.g., visual field in tactical mode).

---

## Collision & Blocking

### Solids

Entities cannot pass through:
* Mountains, cliffs (unless climbing)
* Buildings (unless inside)
* Deep water (unless swimming capable)
* Other solid entities (unless pass-through flagged)

### Interaction Range

Entities can interact at range (without moving through):
* **Touch/Adjacent**: 1 tile range (melee combat, harvesting)
* **Near**: 5 tile range (conversation, trade offer)
* **Sight**: Visual range (perception, ranged combat)

---

## Location Semantics

### Named Locations

Biome regions, settlements, landmarks tracked:

* **Location entry**: Entities remember location coordinates
* **Cultural significance**: Locations tied to beliefs, events
* **Resource density**: Locations have resource markers

**Example**: "Apple grove at (450, 320)" remembered by fruitarian entities

### Territory & Movement Rules

**Social territories**:
* Settlement boundary: Entities inside enjoy settlement benefits
* Tribal territory: Outsiders have restrictions
* Dangerous zones: High-threat areas trigger flee response

---

## Integration with Simulation

### Per-Cycle Spatial Updates

1. **Entities declare movement**: Intent → target location
2. **Pathfinding**: Calculate path from current → target
3. **Collision checking**: Ensure path valid
4. **Movement execution**: Update entity position
5. **Chunk reassignment**: Update spatial grid
6. **Perception recalculation**: Determine visible/audible entities at new location
7. **Interaction triggers**: Check proximity for social/resource interactions

---

## Optimization Techniques

### Spatial Hashing

Quick spatial queries without full chunk scanning

### Perception Caching

Cache perception results (update only when entity moves > threshold)

### Lazy Pathfinding

Only compute path when destination changes, not continuously

### Spatial Coherence

Entities near each other likely do similar actions; batch updates

---

## Summary Table

| Aspect | Mechanism | Notes |
|--------|-----------|-------|
| Coordinates | (X, Y, Z) integer positions | Tile-based grid |
| Partitioning | Chunk-based grid | Efficient spatial queries |
| Movement | Terrain-modified cost | Speed = Coordination stat |
| Vision Range | Base 30 tiles × Vigil × modifiers | Light/obstacle affected |
| Hearing Range | Base 50 tiles × Vigil × modifiers | Weather/echo affected |
| Smell Range | Base 40 tiles × Vigil × modifiers | Wind/decay affected |
| Pathfinding | A* algorithm | Threat/personality aware |
| Collision | Solid blocking | Some exceptions (climbing, swimming) |

---

## Example: Deer in Forest

**Scenario**: Deer at (100, 100) smells predator at (130, 95)

1. **Perception**: Smell range = 40 × (1 + 5×0.12) × decay × wind = ~50 tiles
2. **Detection**: Predator scent detected at distance 35 tiles (within range)
3. **Intent**: Generate fear intent; activate flight response
4. **Pathfinding**: A* to escape; avoid open clearings (exposed)
5. **Movement**: Find path through forest (0.7× multiplier) away from predator
6. **Outcome**: Deer moves away; terrain helps hide

If predator follows:
* Continued perception checks
* Chase ensues with terrain advantage
* Possible combat if caught
* Possible escape if reaches dense cover or distance > perception range
