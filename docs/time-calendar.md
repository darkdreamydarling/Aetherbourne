# ⏳ Time & Calendar System

The Aetherbourne world operates on a consistent calendar that governs seasons, weather patterns, and cyclical events. Time is measured in standardized units and maps to seasonal and cultural significance.

---

## World Calendar

### Year Structure

* **Days per Year**: 300
* **Months per Year**: 10
* **Total Verification**: 28+32+30+26+31+29+33+27+34+30 = 300 days ✓

### Week Structure

* **Days per Week**: 8
* **Hours per Day**: 24
* **Minutes per Hour**: 60
* **Seconds per Minute**: 60

### Day Names

The 8 days of the week correspond to fundamental cosmic principles:

1. **Solis** — Sun day (light, clarity, revelation)
2. **Lunis** — Moon day (reflection, cycles, change)
3. **Terris** — Earth day (grounding, foundation, stability)
4. **Aquas** — Water day (flow, adaptation, cleansing)
5. **Ignis** — Fire day (transformation, energy, passion)
6. **Ventis** — Wind day (movement, communication, dispersal)
7. **Umbranis** — Shadow day (introspection, mystery, concealment)
8. **Luxis** — Light day (resurrection, renewal, hope)

---

## Months

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

## Seasonal Effects

### Winter (Brigara, Imbrelle, Duskael)

* **Temperature**: Low to very low
* **Daylight**: Shortened (fewer hours of sunlight)
* **Flora**: Dormant or dying; forage scarce
* **Fauna**: Hibernating or migrating; hunting harder
* **Mood**: Population mood tends toward melancholy; cultural focus on stories, ritual, community
* **Survival Pressure**: ↑↑ (Warmth and Food needs peak)

### Spring (Florayne, Lithara)

* **Temperature**: Moderate, rising
* **Daylight**: Increasing
* **Flora**: Rapid growth; new forage available
* **Fauna**: Migration peaks; reproduction season for some species
* **Mood**: Optimism rises; population energy increases
* **Survival Pressure**: ↓ (Resources becoming more available)

### Summer (Heliora, Aestara, Mavonel)

* **Temperature**: High
* **Daylight**: Maximum length (longer days)
* **Flora**: Abundant; forage plentiful
* **Fauna**: Active and abundant; hunting easy
* **Mood**: Population engaged, energetic; festivals common
* **Survival Pressure**: ↓↓ (Lowest—resources abundant, minimal environmental stress)

### Autumn (Ceresen, Yulith)

* **Temperature**: Moderate, falling
* **Daylight**: Decreasing
* **Flora**: Harvest-ready; final growth before dormancy
* **Fauna**: Fattening for winter; last migration window
* **Mood**: Urgency; preparation; cultural harvest celebrations
* **Survival Pressure**: ↑ (Preparation mindset; storing resources)

---

## Time Mechanics

### Day Cycle

**In-game day progression** (may be faster or slower than real time depending on simulation speed):

* **Dawn** (Hours 6–8): Light increases; nocturnal creatures retreat; diurnal creatures wake
* **Morning** (Hours 8–12): Full daylight; optimal activity time
* **Afternoon** (Hours 12–16): Peak heat/light; continued activity
* **Dusk** (Hours 16–18): Light decreases; diurnal creatures settle; nocturnal creatures wake
* **Night** (Hours 18–6): Darkness; limited vision; nocturnal activity peaks
* **Midnight** (Hour 0): Coldest time; rest period

### Time-Based Effects

**On Entities:**

* **Nocturnal creatures**: Enhanced perception at night, penalties during day
* **Diurnal creatures**: Enhanced perception during day, penalties at night
* **Crepuscular creatures**: Peak activity at dawn/dusk

**On Environment:**

* **Flora**: Photosynthesis reduced at night (less growth)
* **Temperature**: Coldest at pre-dawn; hottest mid-afternoon
* **Perception**: Vision-based detection impossible in full darkness (without light sources or night vision)

### Season-Triggered Events

Certain events happen at seasonal intervals:

* **Spring**: Migration peaks, settlement expansion begins
* **Summer**: Festivals, peak civilization growth, mating season for many species
* **Autumn**: Harvest festivals, final trade before winter, ceremonial hunts
* **Winter**: Hibernation, cultural ceremonies, population stress

---

## Integration with Weather

Weather and season interact:

* **Winter months** have higher chance of snow/storms
* **Summer months** prone to drought and heat waves
* **Spring/Autumn** more variable (transitional systems)

**Extreme weather** can override seasonal expectations (unseasonable cold in summer, warm winter blasts)

---

## Integration with Civilization

### Calendar-Based Activities

**Settlements track seasonal cycles**:

* **Spring**: Expansion phase (new settlements attempt foundation)
* **Summer**: Trade and growth phase (festivals facilitate trade)
* **Autumn**: Preparation phase (resource storage, preservation)
* **Winter**: Internal phase (cultural focus, introspection, population stress if resources fail)

### Festival & Ritual Calendar

Culture calendar often tied to seasonal markers:

* **Harvest Festivals** (Autumn, especially end of Yulith)
* **Winter Solstice Rituals** (Duskael)
* **Spring Renewal** (Florayne)
* **Summer Abundance** (mid-Mavonel)

---

## Technical Notes

### Simulation Time vs. Real Time

* Game ticks advance calendar
* Calendar day/month/season updates are tied to simulation cycles
* Entities age in parallel with calendar time
* Multi-generational play possible (entities born, grow, reproduce, die over calendar cycles)

### Calendar Reference

Given a timestamp (day #X), you can calculate:

```
Month = ⌊Day ÷ month_length⌋
Week Day = (Day mod 8) → day name
Season = determined by month
```

**Example**: Day 87 = Month 3 (Florayne, Spring), approximately Lunis (Moon Day)

---

## Summary Table

| Aspect | Value | Notes |
|--------|-------|-------|
| Days per Year | 300 | Balanced 10-month structure |
| Days per Week | 8 | 8 fundamental cosmic principles |
| Months per Year | 10 | 3 winter, 2 spring, 3 summer, 2 autumn |
| Shortest Month | 26 days (Lithara, Spring) | Building season |
| Longest Month | 34 days (Yulith, Autumn) | Prime harvest season |
| Seasons | 4 | Winter, Spring, Summer, Autumn |
| Daylight Cycle | 24 hours | Natural day/night rhythm |
