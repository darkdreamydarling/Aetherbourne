# ⏰ Weather System

Dynamic weather affects resource availability, character comfort, travel, and world state. The weather system creates environmental pressure that influences survival difficulty, perception, and action effectiveness.

---

## Weather Conditions

### Clear

* **Light Level**: 1.0
* **Temperature Effect**: 0.0
* **Visibility**: 1.0
* **Movement Speed**: 1.0
* **Thirst Drain Multiplier**: 1.0
* **Vigil Bonus**: +0.1
* **Duration**: 6–24 hours
* **Effects**: Optimal conditions for most activities; enhanced perception

### Rainy

* **Light Level**: 0.5
* **Temperature Effect**: –3.0 (Warmth penalty)
* **Visibility**: 0.4
* **Movement Speed**: 0.9
* **Thirst Drain Multiplier**: 1.2
* **Vigil Penalty**: –0.15
* **Fire Extinguish Rate**: 0.8
* **Plant Watering Rate**: +0.3
* **Soil Erosion Rate**: +0.25
* **Duration**: 4–16 hours
* **Effects**: Reduced visibility hampers perception; fire risk increased; flora grows faster; terrain becomes treacherous

### Stormy (Severe)

* **Light Level**: 0.3
* **Temperature Effect**: –4.0 (Warmth penalty)
* **Visibility**: 0.2
* **Movement Speed**: 0.8
* **Thirst Drain Multiplier**: 1.5
* **Accuracy Penalty**: –0.25
* **Lightning Strike Chance**: 5% per cycle
* **Wind Effect Strength**: 1.5x
* **Duration**: 6–20 hours
* **Effects**: Severe visibility loss; extreme conditions force shelter-seeking; lightning danger

### Foggy

* **Light Level**: 0.6
* **Visibility**: 0.3
* **Movement Speed**: 0.85
* **Vigil Penalty**: –0.25
* **Confidence Penalty**: –0.2 (Entities doubt their perception)
* **Hearing Range Multiplier**: 1.2 (Sound travels differently)
* **Duration**: 2–12 hours
* **Effects**: Near-complete visual blindness; auditory perception enhanced; disorientation risk

### Windy

* **Light Level**: 0.9
* **Movement Speed**: 0.95 (harder to move against wind)
* **Projectile Accuracy**: –0.3
* **Scent Dispersal**: +0.5 (smell detection harder)
* **Sound Masking**: 0.7 (noise interference)
* **Dust/Debris**: Light damage to health if exposed (1–2% per cycle)
* **Duration**: 3–15 hours
* **Effects**: Travel hampered; ranged combat penalized; scent tracking becomes unreliable

### Snowy

* **Light Level**: 0.7
* **Temperature Effect**: –5.0 (Warmth penalty)
* **Visibility**: 0.5
* **Movement Speed**: 0.7 (deep snow)
* **Thirst Drain Multiplier**: 0.5 (water available from snow)
* **Health Drain**: +0.5% per cycle (cold exposure)
* **Thermal Tracking**: +0.4 (heat signature visible)
* **Duration**: 8–24 hours
* **Effects**: Extreme cold requires shelter; movement significantly hampered; heat very visible; tracks left in snow

### Hail

* **Light Level**: 0.4
* **Temperature Effect**: –6.0 (Warmth penalty)
* **Visibility**: 0.25
* **Movement Speed**: 0.6
* **Health Damage**: +1.0% per cycle (hail strikes)
* **Disorientation**: 30% chance per minute of exposure
* **Duration**: 1–4 hours
* **Effects**: Dangerous conditions; direct outdoor exposure extremely hazardous; entities seek shelter rapidly

### Drought

* **Light Level**: 1.1
* **Temperature Effect**: +3.0 (Heat penalty)
* **Thirst Drain Multiplier**: 3.0 (water extremely scarce)
* **Vegetation**: Flora dies rapidly; forage availability –75%
* **Water Sources**: Dry up or deplete faster
* **Fire Spread Rate**: +2.0
* **Duration**: 10–30 days (cumulative weather)
* **Effects**: Survival pressure increases dramatically; water becomes primary need; food scarcity

---

## Weather Mechanics

### Weather Transition

Weather changes based on:
* **Biome climate** (deserts rarely see rain, forests frequently)
* **Season** (winter favors cold/snow; summer favors clear/drought)
* **Random variation** (weather is not completely predictable)

Transition typically takes 1–3 hours.

### Effect Stacking

Multiple effects apply simultaneously:
* Storm + Snowy = extreme conditions
* Drought + Windy = dust storms (visibility +hazard)
* Rain + Cold = sleet (combination of rain + snow effects)

### Perception Modulation

Weather affects **all perception types**:
* Vision reduced by fog, darkness, heavy precipitation
* Hearing affected by wind, sound masking
* Scent dispersed or masked by wind, rain

### Resource Impact Cascade

```
Weather → Flora/Fauna/Minerals → Food/Water/Craft Resources → Population Density
```

**Example Drake Drought Impact:**
1. Drought hits for 15 days
2. Flora dies; water sources dry
3. Herbivore population crashes
4. Predator population starves
5. Survivors migrate or adapt
6. Settlement water stores become critical
7. Social conflict may rise (resource scarcity)

---

## Integration with Simulation Loop

Weather updates **before** entity actions each cycle:

1. Environment updates **(weather system)**
2. Biome states shift based on weather
3. Entities perceive weather effects
4. Needs adjust (thirst ↑ in heat, warm need ↑ in cold)
5. Action costs modified by weather
6. Perception ranges adjusted
7. Survival difficulty recalculated

---

## Summary Table

| Condition | Visibility | Movement | Cold/Heat | Duration | Key Effect |
|-----------|-----------|----------|-----------|----------|-----------|
| Clear | 1.0 | 1.0 | 0 | 6–24h | Optimal conditions |
| Rainy | 0.4 | 0.9 | –3 warmth | 4–16h | Reduced view, flora growth |
| Stormy | 0.2 | 0.8 | –4 warmth | 6–20h | Severe conditions, lightning |
| Foggy | 0.3 | 0.85 | 0 | 2–12h | Visual blindness |
| Windy | 0.9 | 0.95 | 0 | 3–15h | Scent/sound disrupted |
| Snowy | 0.5 | 0.7 | –5 warmth | 8–24h | Deep cold, heat visible |
| Hail | 0.25 | 0.6 | –6 warmth | 1–4h | Direct damage, dangerous |
| Drought | 1.1 | 1.0 | +3 heat | 10–30d | Severe thirst, crop failure |
