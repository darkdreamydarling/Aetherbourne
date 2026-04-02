# Resource Flow System
## Canonical Specification

**Purpose**: Defines resource supply, demand, pricing, scarcity, and flow tracking across the entire simulation. Single source of truth for economy mechanics.

### Core Data Structures (from data-schema.md)

```rust
#[derive(Clone, Debug)]
pub struct ResourceFlow {
    pub resource_id: ResourceId,
    pub global_supply: f32,        // Total available (0-1 normalized)
    pub global_demand: f32,        // Total desired (0-1 normalized)  
    pub price_index: f32,          // 0.5 = equilibrium, >1 = scarcity
    pub flow_rate: f32,            // Per tick resource movement
    pub regional_supply: Vec<f32>, // Normalized supply per region
}

#[derive(Clone, Debug)]
pub struct Price {
    pub base_value: f32,           // Material-intrinsic value (0-100)
    pub scarcity_modifier: f32,    // Price adjustment from supply/demand
    pub quality_modifier: f32,     // Item quality multiplier (0.5-2.0)
    pub reputation_modifier: f32,  // Trader relationship effect (±20%)
    pub final_price: f32,
}
```

### Fundamental Formulas (all from systems-math.md)

#### 1. Scarcity Index
```
scarcity_index = demand / supply
equilibrium = 1.0
prices above 1.2 = scarcity pricing (+20%)
prices below 0.8 = abundance (-20%)
```

#### 2. Dynamic Price Calculation
```
final_price = base_value 
            × scarcity_modifier(1.0 + (scarcity_index - 1.0) × 0.5)
            × quality_modifier
            × reputation_modifier(1.0 + relationship_tier / 100.0)
```

**Example**:
```
Iron (base=15) in scarcity (1.5), quality 1.6, good rep (+10%)
final = 15 × 1.25 × 1.6 × 1.10 = 33.0
```

#### 3. Regional Supply Flow
```
net_flow = (regional_supply_avg - global_supply) × flow_rate × terrain_factor
```
- Mountains: ×0.3 (hard transport)
- Plains: ×1.0  
- Rivers: ×2.0 (easy transport)

### Supply/Demand Tracking (Per Tick)

| System | Supply Change | Demand Change |
|--------|---------------|---------------|
| Mining | +0.01 iron/lead | -0.005 tools |
| Farming | +0.02 food | -0.015 population |
| Crafting| -0.01 materials | +0.008 tools |
| Trade | ±0.005 | ±0.005 |
| Decay | -0.002 | 0 |

**Update Frequency**: Every 100 ticks (10 minutes @ 1s/tick)

### Trading Mechanics

#### Haggling System (5 rounds max)
```
offer = final_price × (0.8 + random(0.4))
counter = min(player_offer × 1.1, npc_max_price)
```

#### Bulk Trade Penalties
```
bulk_discount = 1.0 - (quantity / 100) × 0.3
max_bulk = 200 units
```

#### Caravan Logistics
```
travel_time = distance / (wagon_speed × road_quality)
wagon_capacity = 500 × oxen_count
loss_rate = 0.01 per hex mountains, 0.001 plains
```

### Scarcity Pressure Events

| Scarcity Index | Event | Effect |
|----------------|--------|---------|
| >2.0 | Rationing | Demand ×0.8 |
| >3.0 | Hoarding | Supply ×0.5 |
| >4.0 | Riots | Trade halted |
| <0.3 | Oversupply | Price ×0.5 |

### Cultural Price Modifiers

```
cultural_modifier = 1.0 + culture_affinity × 0.2
- Enemy: ×0.8
- Neutral: 1.0  
- Ally: ×1.2
- Worship: ×1.5
```

### Storage & Spoilage

```
storage_efficiency = container_quality × location_protection
spoilage_rate = 0.01/day preserved, 0.1/day unprotected food
capacity_loss = 0.001 per day overcrowding
```

### Verification Checklist
- [x] All formulas reference systems-math.md
- [x] Price computable from base → final
- [x] Supply/demand normalized 0-1
- [x] Regional flow testable
- [x] Trading simulation complete
- [x] Scarcity events threshold-based

