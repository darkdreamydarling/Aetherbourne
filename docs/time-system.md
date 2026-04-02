# ⏰ Time System — Tick-to-Season Integration

This document defines how real-world and simulation time interact, how ticks drive the calendar, and when each system updates. **This is the temporal heartbeat of Aetherbourne.**

---

## Core Time Model

### Tick: Primitive Time Unit

A **tick** is the smallest addressable time unit in the simulation. The simulation advances one tick per frame.

```rust
pub struct TimeState {
    pub current_tick: u64,      // Cumulative tick count since world creation
    pub tick_frequency: f32,    // Configurable: seconds per tick
    pub paused: bool,           // Simulation paused?
}

// Tick frequency is adjustable per session:
// 1.0 = 1 real second per tick (real-time feel)
// 60.0 = 60 real seconds per tick (30x acceleration)
// 0.1 = 0.1 real seconds per tick (10x acceleration)

pub const TICK_OPTIONS: &[(f32, &str)] = &[
    (1.0, "Real-time (1 tick/sec)"),
    (0.5, "Fast (2 ticks/sec)"),
    (0.2, "Very Fast (5 ticks/sec)"),
    (60.0, "30x Acceleration (30 sec/tick)"),
    (300.0, "150x Acceleration (5 min/tick)"),
];
```

---

## Calendar System

### Time Unit Hierarchy

```
1 Tick          = configurable (default: 1 second)
60 Ticks        = 1 Minute (default)
60 Minutes      = 1 Hour (default)
24 Hours        = 1 Day (default: ~24 hours)
8 Days          = 1 Week (8 days in Aetherbourne calendar)
300 Days        = 1 Year
1 Year          = 12 Months
```

### Detailed Calendar Structure

Aetherbourne uses a **10-month, 300-day calendar**:

```rust
pub struct CalendarDate {
    pub year: u32,
    pub month_index: u32,       // 0–9
    pub day: u32,               // 0–29 (varies per month)
    pub hour: u32,              // 0–23
    pub minute: u32,            // 0–59
}

pub fn ticks_to_calendar(ticks: u64) -> CalendarDate {
    // Constants (in ticks, assuming 1 tick = 1 second)
    const TICKS_PER_MINUTE: u64 = 60;
    const TICKS_PER_HOUR: u64 = 3_600;
    const TICKS_PER_DAY: u64 = 86_400;
    const DAYS_PER_YEAR: u64 = 300;
    const TICKS_PER_YEAR: u64 = 25_920_000;
    
    let year = (ticks / TICKS_PER_YEAR) as u32;
    let ticks_in_year = ticks % TICKS_PER_YEAR;
    let day_of_year = (ticks_in_year / TICKS_PER_DAY) as u32;
    let ticks_in_day = ticks_in_year % TICKS_PER_DAY;
    let hour = (ticks_in_day / TICKS_PER_HOUR) as u32;
    let minute = ((ticks_in_day % TICKS_PER_HOUR) / TICKS_PER_MINUTE) as u32;
    
    // Determine month from day of year
    let month_lengths = [28, 32, 30, 26, 31, 29, 33, 27, 34, 30];  // Sum = 300
    let mut month_index = 0;
    let mut day_in_month = day_of_year as i32;
    
    for (i, &month_len) in month_lengths.iter().enumerate() {
        if day_in_month < month_len as i32 {
            month_index = i;
            day_in_month += 1;  // 1-indexed days
            break;
        }
        day_in_month -= month_len as i32;
    }
    
    CalendarDate {
        year,
        month_index: month_index as u32,
        day: day_in_month as u32,
        hour,
        minute,
    }
}
```

### Month Descriptions (From time-calendar.md)

| Month        | Season | Length (days) | Cultural Significance                                                                   | Element | Element Qualities |
| ------------ | ------ | ------------- | --------------------------------------------------------------------------------------- | ------- | ----------------- |
| **Brigara**  | Winter | 28            | Season of Dominion; absolute stillness, structures endure, authority and order solidify | Solid   | Preeminent        |
| **Imbrelle** | Winter | 32            | Season of Flow Under Ice; hidden movement, resource management, quiet endurance         | Liquid  | Determinate       |
| **Florayne** | Spring | 30            | Season of Winds of Change; unpredictability, shifting paths, discovery and dispersal    | Gas     | Mercurial         |
| **Lithara**  | Spring | 26            | Season of Ignition; sparks of creation, innovation, rapid beginnings                    | Plasma  | Preeminent        |
| **Fulthane** | Spring | 30            | Season of Foundations; planting, building roots, deliberate growth                      | Solid   | Determinate       |
| **Heliora**  | Summer | 31            | Season of Overflow; flourishing life, emotional and ecological abundance                | Liquid  | Mercurial         |
| **Aestara**  | Summer | 29            | Season of Ascendant Currents; dominant winds, expansion, trade and movement peak        | Gas     | Preeminent        |
| **Mavonel**  | Summer | 33            | Season of Controlled Radiance; sustained energy, production, disciplined output         | Plasma  | Determinate       |
| **Ceresen**  | Autumn | 27            | Season of Fracture; breakdown of forms, release, decay begins                           | Solid   | Mercurial         |
| **Vivmora**  | Autumn | 30            | Season of Gathering Waters; consolidation, preservation, intentional storage            | Liquid  | Preeminent        |
| **Yulith**   | Autumn | 34            | Season of Settling Veils; slowing winds, stabilization, planning and negotiation        | Gas     | Determinate       |
| **Duskael**  | Winter | 30            | Season of Fading Embers; unstable energy, endings, reflection before renewal            | Plasma  | Mercurial         |
---

## Update Frequencies: When Each System Updates

Every system updates at a specific tick interval. This prevents performance explosion and creates intentional pacing.

```rust
pub struct UpdateSchedule {
    pub tick_interval: u64,     // Update every N ticks
    pub description: &'static str,
}

impl UpdateSchedule {
    pub fn should_update(&self, current_tick: u64) -> bool {
        current_tick % self.tick_interval == 0
    }
}

// Master update schedule
pub const UPDATE_INTERVALS: &[(&str, u64, &str)] = &[
    // Per Tick (every frame)
    ("Input Processing", 1, "Player actions"),
    ("Spatial Queries", 1, "Perception updates"),
    
    // Every 10 Ticks (~0.167 minutes)
    ("Needs Decay", 10, "Hunger, thirst, etc. decrease"),
    ("Animation", 10, "Visual state updates"),
    
    // Every 60 Ticks (~1 minute)
    ("Health Regeneration", 60, "Natural healing"),
    ("Memory Consolidation", 60, "Short-term → long-term"),
    ("Perception Attention", 60, "Filter top-N affordances"),
    
    // Every 600 Ticks (~10 minutes)
    ("Items Decay", 600, "Durability loss from use"),
    ("Environmental Effects", 600, "Fire spread, weather erosion"),
    ("Flora Growth", 600, "Plant growth rate updates"),
    
    // Every 3,600 Ticks (~1 hour)
    ("Weather System", 3_600, "Weather transitions, updates"),
    ("Skill Progression", 3_600, "Skill practice applies"),
    ("Population Pressure", 3_600, "Biome carrying capacity check"),
    ("Relationship Decay", 3_600, "Affinity slowly returns to neutral"),
    ("Fatigue Recovery", 3_600, "Rest accumulates slowly"),
    
    // Every 86,400 Ticks (~1 day)
    ("Calendar Advancement", 86_400, "Advance to next day"),
    ("Aging", 86_400, "Characters age 1 tick"),
    ("Lifecycle Stage Check", 86_400, "Check if entity changes stage"),
    ("Flora Reproduction", 86_400, "Plant seeding & dispersal"),
    ("Fauna Population", 86_400, "Birth/death cycles"),
    ("Settlement Shifts", 86_400, "Population moves, roles change"),
    ("Economy Updates", 86_400, "Trade prices adjust"),
    
    // Every 604,800 Ticks (~1 week)
    ("Civilization Events", 604_800, "Cultural shifts, conflicts"),
    ("Long-Term Planning", 604_800, "Entities revise long-term goals"),
    
    // Every 2,592,000 Ticks (~1 month)
    ("Seasonal Transition", 2_592_000, "Check month boundary"),
    ("Resource Availability", 2_592_000, "Biome resource nodes regenerate"),
    ("Evolution Pressure", 2_592_000, "Populations adapt to pressure"),
    
    // Every 25,920,000 Ticks (~1 year)
    ("Year Transition", 25_920_000, "Check year boundary"),
];
```

---

## System-Specific Update Details

### Needs Decay (Every 10 Ticks)

```rust
pub fn update_needs(entity: &mut EntityWithNeeds, tick: u64) {
    if tick % 10 != 0 { return; }  // Only every 10 ticks
    
    let multiplier = 10.0;  // 10 ticks elapsed
    
    entity.needs.hunger += entity.decay_rates.hunger_drain * multiplier;
    entity.needs.thirst += entity.decay_rates.thirst_drain * multiplier;
    entity.needs.fatigue += entity.decay_rates.fatigue_drain * multiplier;
    
    // Apply stat modifiers (from systems-math.md)
    entity.needs.hunger *= (1.0 - (entity.stats.vigor / 100.0) * 0.5);
    
    // Cap all needs at [0, 100]
    for need in [
        &mut entity.needs.hunger,
        &mut entity.needs.thirst,
        &mut entity.needs.fatigue,
        &mut entity.needs.warmth,
        &mut entity.needs.comfort,
        &mut entity.needs.stress,
        &mut entity.needs.social,
        &mut entity.needs.fulfillment,
        &mut entity.needs.waste,
    ] {
        *need = need.clamp(0.0, 100.0);
    }
}
```

### Weather System (Every 3,600 Ticks)

```rust
pub fn update_weather(world: &mut World, tick: u64) {
    if tick % 3_600 != 0 { return; }
    
    // Determine season from calendar date
    let date = ticks_to_calendar(tick);
    let season = determine_season(date.month_index);
    
    // Weather can transition to new state
    let current_weather = world.weather.current_condition();
    let next_weather = weather_transition(current_weather, season);
    
    if next_weather != current_weather {
        world.weather.set_condition(next_weather);
        // Trigger any weather-dependent systems
        apply_weather_effects(world, next_weather);
    }
}
```

### Flora Growth (Every 600 Ticks)

```rust
pub fn update_flora_growth(flora: &mut Flora, world: &World, tick: u64) {
    if tick % 600 != 0 { return; }
    
    let date = ticks_to_calendar(tick);
    let season_growth_factor = calculate_season_factor(date.month_index);
    let biome_factor = calculate_biome_growth(flora.biome_type);
    let weather_factor = calculate_weather_factor(world.weather.current_condition());
    
    let growth_rate = 1.0 * season_growth_factor * biome_factor * weather_factor;
    flora.size += flora.size * growth_rate;
    
    // Check if mature enough to seed
    if flora.size > flora.maturity_threshold && random() > 0.95 {
        disperse_seeds(flora, world);
    }
}
```

### Seasonal Transitions (Every 2,592,000 Ticks)

When the calendar ticks to a new month:

```rust
pub fn check_season_transition(world: &mut World, tick: u64) {
    let current_date = ticks_to_calendar(tick);
    let previous_date = ticks_to_calendar(tick - 1);
    
    // Month changed?
    if current_date.month_index != previous_date.month_index {
        on_month_transition(world, current_date);
    }
    
    // Season changed (every 3 months)?
    let current_season = determine_season(current_date.month_index);
    let previous_season = determine_season(previous_date.month_index);
    
    if current_season != previous_season {
        on_season_transition(world, current_season);
        
        // Apply all season-based changes
        adjust_biome_temperature(world, current_season);
        adjust_flora_growth_rates(world, current_season);
        adjust_fauna_populations(world, current_season);
        trigger_seasonal_events(world, current_season);
    }
}

pub enum Season {
    Winter(u32),   // Brigara, Imbrelle, Duskael
    Spring(u32),   // Florayne, Lithara, Fulthane
    Summer(u32),   // Heliora, Aestara, Mavonel
    Autumn(u32),   // Ceresen, Yulith, Vivmora
}

pub fn determine_season(month_index: u32) -> Season {
    match month_index {
        0 | 1 | 11 => Season::Winter(month_index),
        2 | 3 | 4 => Season::Spring(month_index),
        5 | 6 | 7 => Season::Summer(month_index),
        8 | 9 | 10 => Season::Autumn(month_index),
        _ => unreachable!(),
    }
}
```

### Aging & Lifecycle (Every 86,400 Ticks)

```rust
pub fn update_aging(entity: &mut Entity, tick: u64) {
    if tick % 86_400 != 0 { return; }
    
    entity.age_ticks += 1;  // Advance age by 1 day
    
    // Check if lifecycle stage should change
    let species = get_species_data(&entity.species_id);
    let lifespan_days = species.lifespan_days;
    let lifespan_ticks = lifespan_days * 86_400;
    
    let lifecycle_percent = (entity.age_ticks * 100) / lifespan_ticks;
    let new_stage = match lifecycle_percent {
        0..=10 => LifecycleStage::Infant,
        11..=25 => LifecycleStage::Child,
        26..=50 => LifecycleStage::Juvenile,
        51..=90 => LifecycleStage::Adult,
        91..=100 => LifecycleStage::Elder,
        101.. => LifecycleStage::Dead,
    };
    
    if new_stage != entity.lifecycle_stage {
        on_lifecycle_change(entity, entity.lifecycle_stage, new_stage);
        entity.lifecycle_stage = new_stage;
        
        // Apply stage-based stat adjustments
        apply_lifecycle_scaling(entity);
    }
}

pub fn apply_lifecycle_scaling(entity: &mut Entity) {
    // Stats scale based on lifecycle stage
    let scale_factor = match entity.lifecycle_stage {
        LifecycleStage::Infant => 0.3,
        LifecycleStage::Child => 0.6,
        LifecycleStage::Juvenile => 0.85,
        LifecycleStage::Adult => 1.0,
        LifecycleStage::Elder => 0.8,
        LifecycleStage::Dead => 0.0,
    };
    
    // Apply to all base stats
    for stat in [
        &mut entity.stats.might,
        &mut entity.stats.lithe,
        &mut entity.stats.vigil,
        &mut entity.stats.grit,
        &mut entity.stats.vigor,
    ] {
        *stat *= scale_factor;
    }
}
```

### Memory Consolidation (Every 60 Ticks)

Memories gradually move from immediate → short-term → long-term storage:

```rust
pub fn consolidate_memories(entity: &mut Entity, tick: u64) {
    if tick % 60 != 0 { return; }
    
    // Immediate memories older than 5 minutes → short-term
    let five_min_ticks = 5 * 60 * 60;  // 300 ticks
    entity.immediate_memories.retain(|mem| {
        if tick - mem.recorded_tick > five_min_ticks {
            entity.short_term_memories.push(mem.clone());
            false  // Remove from immediate
        } else {
            true
        }
    });
    
    // Short-term memories older than 1 hour → long-term
    let one_hour_ticks = 60 * 60 * 60;  // 3,600 ticks
    entity.short_term_memories.retain(|mem| {
        if tick - mem.recorded_tick > one_hour_ticks {
            entity.long_term_memories.push(mem.clone());
            false  // Remove from short-term
        } else {
            true
        }
    });
}
```

---

## Day/Night Cycle

The **hour** component of the calendar drives lighting.

```rust
pub fn calculate_light_level(hour: u32) -> f32 {
    // 24-hour cycle
    // 6:00–18:00 = daylight (gradually ramping)
    // 18:00–22:00 = twilight (gradual fade)
    // 22:00–6:00 = night (dark)
    
    match hour {
        0..=5 => 0.0,           // Dark night
        6 => 0.2,               // Pre-dawn
        7 => 0.5,               // Dawn
        8..=17 => 1.0,          // Full daylight
        18 => 0.5,              // Dusk
        19 => 0.2,              // Twilight
        20..=23 => 0.0,         // Night
    }
}
```

This affects:
- Vision range (see systems-math.md)
- Creature activity (nocturnal vs. diurnal)
- Plant photosynthesis
- Comfort/stress needs (dark increases stress for diurnal creatures)

---

## Special Calendar Events

Certain calendar milestones trigger events:

```rust
pub fn check_special_events(world: &mut World, tick: u64) {
    let date = ticks_to_calendar(tick);
    
    // First day of month → settlement market prices adjust
    if date.day == 1 {
        trigger_market_prices_update(world);
    }
    
    // Full moon (arbitrary day) → entity behavior shifts
    if date.day == 15 {
        trigger_full_moon_effects(world);
    }
    
    // New year → population census, cultural reflection
    if date.month_index == 0 && date.day == 1 {
        trigger_new_year_celebration(world);
    }
}
```

---

## Tick vs. Real Time: Acceleration Examples

With adjustable tick frequency, gameplay pacing can vary:

| Tick Frequency | Real Duration | Simulated Duration | Use Case |
|---|---|---|---|
| 1.0 sec/tick | 10 real min | 10 simulated min | Real-time storytelling |
| 0.5 sec/tick | 10 real min | 20 simulated min | Faster exploration |
| 0.1 sec/tick | 10 real min | 100 simulated min | Combat-paced |
| 60 sec/tick | 10 real min | 10 simulated hours | Day-scale gameplay |
| 300 sec/tick | 10 real min | 50 simulated hours | Week-scale progression |

---

## Implementation Notes

All time calculations should use **u64 ticks as the universal time reference**.

Never store absolute real-world time; always use tick counts.

```rust
// ✅ GOOD: Tick-based
pub struct Event {
    pub triggered_at_tick: u64,
}

// ❌ BAD: Real-world time
pub struct Event {
    pub triggered_at: SystemTime,
}
```

---

## Summary: Time Update Cycle

Each simulation tick:

1. Input processing
2. Physics/spatial updates
3. Every 10 ticks: Needs decay, animation updates
4. Every 60 ticks: Memory consolidation, perception filtering
5. Every 600 ticks: Item decay, flora growth
6. Every 3,600 ticks: Weather, skills, relationships
7. Every 86,400 ticks: Day transitions, aging, lifecycle checks
8. Every 604,800 ticks: Civilization events
9. Every 2,592,000 ticks: Month/season transitions
10. Every 25,920,000 ticks: Year transitions

---

## Next: Affordance System

Time-system.md is now complete. The affordance-system.md will use:
- Priority tiers and salience (from systems-math.md)
- Time decay on memory-based affordances (from this document)
- Calendar-based context (season, weather, time of day)

