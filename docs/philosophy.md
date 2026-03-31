# 🧭 Project Philosophy & Core Mental Model

## Project Philosophy

* **Observer-centric simulation**: The world is simulated from the perspective of observers, not omnisciently.
* **Emergence over scripting**: Behaviors and events arise from systems, not hardcoded scripts.
* **Cause → Effect → Feedback Loops**: Every action produces consequences that feed back into the system.

## Core Mental Model

* **Biology defines capability**
* **Perception defines reality**
* **Needs drive decisions**
* **Memory shapes behavior**
* **Beliefs bias interpretation**
* **Intent drives action**
* **Systems create emergence**

---

## Core Simulation Layers

### 1. Reality Layer
The reality layer encompasses seasons, biomes, and the flow of entropy. It includes minerals and flora, as well as fauna and man-made or natural structures. All of these exist within a defined spatial system.

### 2. Biology Layer
The biology layer defines species and their characteristics. It includes base stats and derived stats, tracks needs and energy, and details morphology and traits.

### 3. Mind Layer
The mind layer governs perception, which represents each entity's subjective reality. It stores memory, which decays over time, and maintains beliefs that create persistent biases. Personality is modeled using a waterfall approach, and an attention system regulates focus.

### 4. Intent Layer

```
Intent = f(Needs + Personality + Memory + Beliefs)
```

### 5. Action Layer

* Movement, Combat, Crafting, Interaction, Survival, Communication

---

## Core Simulation Loop (Unified)

1. Environment updates
2. Geology updates
3. Flora grows/decays
4. **Perception updates (subjective world)**
5. **Attention filtering (top N relevance)**
6. **Intent generation**
7. **Decision scoring (choose action)**
8. Actions execute
9. Interactions resolve
10. Memory updates
11. Beliefs update
12. Ecosystem balances
13. Civilization updates
14. Events trigger

---

# 📊 Stat System (Unified)

## Base Stats (0–10, decimals allowed)

* **Might** — force output
* **Lithe** — agility & efficiency
* **Vigil** — awareness & perception
* **Grit** — resilience & persistence
* **Vigor** — endurance & recovery

---

## Derived Stat Formula (Global Rule)

```
Derived = (A + B) / 2
Optional: + (A * B) * 0.05
```

---

## Derived Stats (Unified Set)

### 🔴 Physical

* **Prowess** = Might + Lithe
* **Discipline** = Might + Vigil
* **Tenacity** = Might + Grit
* **Fortitude** = Might + Vigor

### 🟡 Efficiency

* **Precision** = Lithe + Vigil
* **Finesse** = Lithe + Grit
* **Coordination** = Lithe + Vigor

### 🔵 Mental

* **Insight** = Vigil + Grit
* **Attunement** = Vigil + Vigor
* **Resolve** = Grit + Vigor

---

## Derived Stat Effects

### 🔴 Physical

**Prowess** (Might + Lithe)

```
Action Output = 1 + (Prowess × 0.08)
Energy Cost Multiplier = 1 + (Prowess × 0.05)
```

**Discipline** (Might + Vigil)

```
Action Precision = 1 + (Discipline × 0.06)
Waste Reduction = Discipline × 0.05
```

**Tenacity** (Might + Grit)

```
Comfort Drain Rate = 1 - (Tenacity × 0.06)
Health Protection = Tenacity × 0.05
```

**Fortitude** (Might + Vigor)

```
Health Recovery = Fortitude × 0.07
Warmth Retention = Fortitude × 0.05
```

### 🟡 Efficiency

**Precision** (Lithe + Vigil)

```
Action Success Modifier = 1 + (Precision × 0.07)
Error Reduction = Precision × 0.06
```

**Finesse** (Lithe + Grit)

```
Action Stability = 1 + (Finesse × 0.06)
Smooth Execution Bonus = Finesse × 0.05
```

**Coordination** (Lithe + Vigor)

```
Energy Efficiency = 1 - (Coordination × 0.06)
Stamina Consistency = Coordination × 0.05
```

### 🔵 Mental

**Insight** (Vigil + Grit)

```
Decision Quality = 1 + (Insight × 0.08)
Fulfillment from Actions = Insight × 0.05
```

**Attunement** (Vigil + Vigor)

```
Perception Range = 1 + (Attunement × 0.07)
Awareness Duration = Attunement × 0.05
```

**Resolve** (Grit + Vigor)

```
Health Recovery Rate = 1 + (Resolve × 0.07)
Social Influence = Resolve × 0.06
Thirst/Hunger Resistance = Resolve × 0.05
```

---

# ⚙️ Needs System

## Core Needs (0–100)

* Hunger
* Thirst
* Warmth
* Comfort / Safety
* Health
* Reproduction
* Social
* Fulfillment
* Waste

Each has:

* decay rate
* urgency curve
* competition logic

---

## Standard Drain Rates

Base need drain/decay per action cycle:

* **Hunger Drain Rate** = 1.0
* **Thirst Drain Rate** = 1.0
* **Warmth Loss Rate** = 1.0
* **Health Decline Rate** = 0.5
* **Comfort Decline Rate** = 0.75
* **Social Decline Rate** = 0.5
* **Fulfillment Decline Rate** = 0.5
* **Waste Accumulation Rate** = 1.0

Stats **multiply** these rates based on relevant derived stat.

---

# 🔋 Energy System

```
Energy = Intake − Action Cost
0 = death
```

Energy represents:
* Metabolic capacity to perform actions
* Recovery influenced by Vigor and Fortitude
* Action costs scaled by Prowess and Coordination
* Fundamental survival mechanism (zero energy = death)

---

# 🧠 Personality System (Waterfall)

The personality system operates as a 5-level waterfall, where each level modulates the lower levels:

## Level 1: Reflex (BIS/BAS)

**Behavioral Inhibition System (BIS) vs. Behavioral Activation System (BAS)**

Controls instinctive response to threat/reward:

* **High BIS**: Anxiety-prone, cautious, threat-sensitive
* **High BAS**: Approach-oriented, reward-seeking, opportunity-driven
* **Balanced**: Flexible response to environment

Influences: Initial perception salience, threat threshold, opportunity recognition

---

## Level 2: Behavioral (HEXACO)

**Honesty-Humility, Emotionality, eXtraversion, Agreeableness, Conscientiousness, Openness**

Shapes day-to-day behavioral tendencies:

* **Honesty-Humility**: Sincerity, fairness, greed aversion, modesty
* **Emotionality**: Fear, anxiety, dependency, sentimentality
* **Extraversion**: Social boldness, liveliness, sociability
* **Agreeableness**: Patience, forgiveness, gentleness, flexibility
* **Conscientiousness**: Organization, diligence, prudence, perfectionism
* **Openness**: Unconventional, inquisitive, creativity, variety-seeking

Influences: Social dynamics, risk tolerance, cooperation, stubbornness in beliefs

---

## Level 3: Ethical (Dark Triad)

**Narcissism, Machiavellianism, Psychopathy**

Represents self-interest vs. collectivism in moral frameworks:

* **Low Dark Triad**: Cooperative, empathetic, collective-oriented
* **High Dark Triad**: Self-interested, manipulative, low empathy
* **Balanced**: Capable of both cooperation and self-advocacy

Influences: Trade negotiations, alliance formation, loyalty, betrayal propensity, leadership style

---

## Level 4: Intentional (Enneagram)

**Core motivations and life direction**

Nine core types representing primary life drives:

1. **Type 1 (Reformer)**: Principled, purposeful, self-controlled
2. **Type 2 (Helper)**: Generous, interpersonally aware, people-pleasing
3. **Type 3 (Achiever)**: Success-oriented, efficient, image-conscious
4. **Type 4 (Individualist)**: Creative, introspective, emotionally aware
5. **Type 5 (Investigator)**: Perceptive, analytical, detached
6. **Type 6 (Skeptic)**: Responsible, anxious, loyal
7. **Type 7 (Enthusiast)**: Spontaneous, versatile, scattered
8. **Type 8 (Challenger)**: Powerful, dominating, confrontational
9. **Type 9 (Peacemaker)**: Receptive, trusting, complacent

Influences: Long-term goals, life purpose, core fears, growth paths, career/role selection

---

## Level 5: Communication (DISC)

**Dominance, Influence, Steadiness, Conscientiousness**

Controls communication style and conflict resolution:

* **Dominance**: Direct, decisive, results-focused communication
* **Influence**: Expressive, enthusiastic, relationship-focused
* **Steadiness**: Patient, supportive, process-focused
* **Conscientiousness**: Analytical, precise, quality-focused

Influences: How entities negotiate, persuade, lead, defer, argue

---

## Integration

The waterfall creates emergent personality:

```
Reflex (instinct) → Behavioral (tendencies) → Ethical (values) → Intentional (purpose) → Communication (expression)
```

Each level modulates lower levels:
* Ethical values may override behavioral tendency (e.g., Extravert who values integrity stays quiet about unfair practice)
* Intentional purpose drives sustained behavioral patterns (e.g., Type 1 Reformer pursues principled goals despite personal cost)
* Communication style adapts to context while staying true to DISC type

---

## Final Unified Principle

```
Reality → Perception → Memory → Belief → Intent → Action → Feedback
```
