# ⚡ Action System & Reference Tables

## Stat → Speed Modifier

```
ModifiedDuration = BaseDuration × (1 – floor(Stat ÷ 40) × 0.5)
```

* Stat 0–39 → no speed boost
* Stat 40–79 → 50% faster
* Stat 80+ → 100% faster

---

## Skill → Potency & Success

```
ModifiedChange = BaseChange × (1 + floor(Skill ÷ 40) × 0.25)
Roll = d20 + floor(Skill ÷ 20) vs DC 12
```

**Outcomes:**
* Success → 100% effect
* Fail → 75% effect
* Crit Success → 150% effect
* Crit Fail → 50% effect

---

## Example Walkthrough

Character (Focus 60, Cooking 80) uses **Cook**:

1. **Find Cook's Stats**
   - Base Change: +10% Hunger, –2% Fatigue
   - Base Duration: 10 min

2. **Apply Stat → Speed**
   ```
   ModifiedDuration = 10 × (1 – floor(60÷40)×0.5)
                    = 10 × (1 – 1×0.5)
                    = 5 minutes
   ```

3. **Apply Skill → Potency**
   ```
   ModifiedChange = 10% × (1 + floor(80÷40)×0.25)
                  = 10% × (1 + 2×0.25)
                  = 15% Hunger boost
   ```

4. **Resolve Success Roll**
   ```
   Roll = d20 + floor(80÷20) = d20 + 4 vs DC 12
   • Success → +15% in 5 min
   • Fail    → 75% of 15% = +11.25%
   • Crit    → 150% = +22.5%
   ```

---

# 📋 Action Reference Tables

## Table Legend

| Column | Meaning |
|--------|---------|
| **Action** | In-game activity name |
| **Primary Stat(s)** | Governs action speed |
| **Relevant Skill(s)** | Determines potency & success |
| **Description** | What the action does |
| **Needs Affected** | Which needs change |
| **Base Change** | Raw % before modifiers |
| **Base Duration** | Minutes to full effect |

---

## Movement

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Walk | Lithe | — | Move to nearby tile | Thirst –1.5%, Warmth –1% | –1.5%, –1% | 10 min |
| Run | Vigor, Lithe | — | Move quickly | Thirst –3%, Hunger –1.5%, Warmth –2% | –3%, –1.5%, –2% | 10 min |
| Jump | Lithe, Vigor | — | Traverse gaps | Health –1% | –1% | 5 min |
| Climb | Vigor, Might | — | Ascend/descend | Warmth –2%, Health –2% | –2%, –2% | 10 min |
| Crouch | Lithe | Stealth | Move stealthily | Comfort –1%, Warmth –1% | –1%, –1% | 5 min |
| Carry | Might | — | Transport heavy objects | Hunger –2%, Warmth –2% | –2%, –2% | 10 min |

---

## Interaction

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Inspect | Vigil | Foraging, Observation | Examine details | Fulfillment +2%, Hunger –1% | +2%, –1% | 10 min |
| Pick Up | Lithe | — | Collect items | Health –1% | –1% | 2 min |
| Use | Lithe | — | Interact with tools/doors | Contextual | Varies | Varies |
| Speak | Grit | Conversation, Charisma | Initiate dialogue | Social +5%, Fulfillment +2% | +5%, +2% | 5 min |
| Trade | Vigil, Grit | Bartering, Negotiation | Exchange goods | Social +3%, Fulfillment +2% | +3%, +2% | 10 min |

---

## Combat

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Attack | Might, Lithe, Prowess | Melee Combat, Archery | Offensive move | Hunger –2%, Health +2%, Comfort –3% | –2%, +2%, –3% | 10 min |
| Defend | Might, Tenacity | Defense | Block/reduce damage | Warmth +1%, Health +1% | +1%, +1% | 5 min |
| Dodge | Lithe, Vigil | Reflex, Martial Arts | Evade attack | Health +1%, Warmth –1% | +1%, –1% | 5 min |
| Special Ability | Varies | Spellcraft, Tactics | Use power/spell | Health –3%, Comfort –2% | –3%, –2% | 5 min |
| Equip/Swap | Lithe | — | Change weapons | Health –1% | –1% | 2 min |

---

## Utility

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Craft | Precision, Lithe | Crafting, Smithing, Tailoring, Alchemy | Create items | Fulfillment +5%, Health –1% | +5%, –1% | 15 min |
| Heal | Vigor, Grit | First Aid, Spiritual Healing, Alchemy | Restore health | Health +10%, Hunger –1% | +10%, –1% | 10 min |
| Rest | Vigor | — | Recover stamina | Warmth +5%, Health +3%, Hunger –3% | +5%, +3%, –3% | 30 min |
| Signal | Grit, Vigil | Tactics, Communication | Call allies | Social +3%, Fulfillment +2% | +3%, +2% | 5 min |

---

## Social

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Befriend | Grit, Grit | Diplomacy, Empathy | Build rapport | Social +10%, Fulfillment +5% | +10%, +5% | 20 min |
| Intimidate | Might, Grit | Leadership | Influence via fear | Social –5%, Comfort –5% | –5%, –5% | 10 min |
| Persuade | Grit, Grit | Negotiation, Bartering | Convince | Fulfillment +5%, Social +3% | +5%, +3% | 10 min |
| Lie | Grit, Vigil | Deception | Deceive | Fulfillment –2%, Comfort +2% | –2%, +2% | 5 min |
| Form Bond | Grit, Grit | Empathy, Charisma | Close relationship | Social +15%, Fulfillment +10% | +15%, +10% | 30 min |

---

## Tactical

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Wait | Grit | — | Delay action | Comfort –2%, Warmth –1% | –2%, –1% | 5 min |
| Prepare | Precision, Insight | Tactics | Ready weapon/stance | Fulfillment +5%, Comfort +2% | +5%, +2% | 10 min |
| Distract | Lithe, Vigil | Stealth, Performance | Divert attention | Comfort –3%, Fulfillment +2% | –3%, +2% | 5 min |
| Camouflage | Lithe, Insight | Stealth | Blend in | Comfort –4%, Fulfillment +2% | –4%, +2% | 10 min |

---

## Construction

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Build | Might, Vigor | Masonry, Carpentry, Engineering | Construct structures | Hunger –3%, Warmth –2% | –3%, –2% | 10 min |
| Repair | Lithe, Might | Engineering, Masonry | Fix structures | Fulfillment –2%, Comfort +1% | –2%, +1% | 10 min |
| Survey | Insight, Vigil | Design, Architecture | Plan layouts | Fulfillment –3%, Comfort +2% | –3%, +2% | 10 min |
| Excavate | Might, Vigor | Earthwork | Dig tunnels | Hunger –2%, Warmth –2% | –2%, –2% | 10 min |

---

## Resource Gathering

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Plant | Lithe, Precision | Farming | Grow crops | Fulfillment +3%, Hunger –2% | +3%, –2% | 10 min |
| Harvest | Lithe, Vigil | Farming, Foraging | Collect crops | Hunger +5%, Health –1% | +5%, –1% | 10 min |
| Tame | Grit, Lithe | Taming | Train animals | Fulfillment +5%, Social +5% | +5%, +5% | 15 min |
| Hunt | Vigil, Lithe | Tracking, Archery, Combat | Kill wild animals | Hunger +10%, Health –3%, Comfort –2% | +10%, –3%, –2% | 15 min |
| Fish | Lithe, Vigil | Fishing | Catch aquatic resources | Hunger +8%, Health –1% | +8%, –1% | 15 min |
| Mine | Might, Vigor | Mining | Extract minerals | Hunger –3%, Warmth –2% | –3%, –2% | 10 min |
| Chop | Might, Lithe | Logging | Cut trees | Hunger –2%, Warmth –1% | –2%, –1% | 10 min |
| Gather | Lithe, Vigil | Foraging | Collect herbs | Hunger +5%, Health –1% | +5%, –1% | 10 min |

---

## Crafting

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Forge | Might, Precision | Smithing | Create weapons/armor | Fulfillment +5%, Hunger –1% | +5%, –1% | 15 min |
| Carve | Lithe, Precision | Carpentry, Crafting | Shape wood | Fulfillment +5%, Health +1% | +5%, +1% | 15 min |
| Weave | Lithe, Precision | Tailoring, Weaving | Create textiles | Fulfillment +4%, Comfort +2% | +4%, +2% | 15 min |
| Tinker | Lithe, Insight | Crafting, Tinkering | Improve items | Fulfillment +3%, Comfort +1% | +3%, +1% | 10 min |
| Refine | Precision, Lithe | Tanning, Glasswork, Metallurgy | Process materials | Hunger –2%, Warmth –1% | –2%, –1% | 15 min |
| Assemble | Lithe, Precision | Engineering, Crafting | Fit components | Fulfillment +2%, Health –1% | +2%, –1% | 10 min |

---

## Consumables

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Cook | Precision, Lithe | Cooking | Prepare meals | Hunger +10%, Warmth +1% | +10%, +1% | 10 min |
| Bake | Precision, Lithe | Baking | Make baked goods | Hunger +10%, Warmth +1% | +10%, +1% | 10 min |
| Brew | Precision, Grit | Brewing, Alchemy | Create drinks | Fulfillment +5%, Hunger +3% | +5%, +3% | 15 min |

---

## Daily Life

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Clean | Lithe, Grit | — | Maintain hygiene | Comfort +5%, Health +5% | +5%, +5% | 15 min |
| Organize | Insight, Lithe | Logistics | Manage inventory | Fulfillment +3%, Comfort +1% | +3%, +1% | 10 min |
| Care | Vigor, Grit | Healing, Nurturing | Tend to others | Health +10%, Social +5% | +10%, +5% | 15 min |
| Socialize | Grit, Grit | Charisma, Conversation | Casual interaction | Social +10%, Fulfillment +5% | +10%, +5% | 20 min |
| Teach | Insight, Grit | Mentorship, Instruction | Instruct others | Fulfillment +5%, Social +2% | +5%, +2% | 15 min |
| Work | Varies | Varies | Job-related tasks | Fulfillment +4%, Hunger –2% | +4%, –2% | 15 min |

---

## Warfare

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Strategize | Insight, Grit | Tactics, Strategy | Plan battles | Comfort –5%, Fulfillment –3% | –5%, –3% | 20 min |
| Scout | Vigil, Lithe | Navigation, Tracking | Survey areas | Health –2%, Comfort –2% | –2%, –2% | 10 min |

---

## Trade

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Haggle | Grit, Vigil | Bartering, Negotiation | Negotiate deals | Social +5%, Fulfillment +2% | +5%, +2% | 10 min |
| Deliver | Vigor, Lithe | Logistics | Transport items | Hunger –2%, Warmth –1% | –2%, –1% | 10 min |
| Manage | Insight, Grit | Accounting, Leadership | Oversee resources | Fulfillment +5%, Comfort –2% | +5%, –2% | 15 min |

---

## Magic

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Cast Spell | Grit, Precision | Spellcraft | Use magic | Health –5%, Comfort –2% | –5%, –2% | 5 min |
| Mix | Precision, Lithe | Alchemy, Herbalism | Brew potions | Fulfillment +5%, Hunger +2% | +5%, +2% | 15 min |
| Enchant | Precision, Grit | Spellcraft, Runesmithing | Imbue objects | Fulfillment +3%, Health –2% | +3%, –2% | 15 min |
| Divine | Insight, Grit | Divination | Seek insight | Comfort –5%, Fulfillment +5% | –5%, +5% | 15 min |

---

## Arts & Culture

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Perform | Lithe, Grit | Performance | Sing, dance, entertain | Fulfillment +15%, Social +5% | +15%, +5% | 30 min |
| Write | Insight, Precision | Writing, Curation | Record knowledge | Fulfillment +5%, Comfort +1% | +5%, +1% | 15 min |
| Study | Insight, Grit | Scholarship | Gain knowledge | Fulfillment +5%, Comfort +1% | +5%, +1% | 15 min |

---

## Exploration

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Track | Vigil, Insight | Tracking | Follow signs | Health –2%, Fulfillment –1% | –2%, –1% | 10 min |
| Navigate | Insight, Lithe | Navigation | Guide travel | Warmth –1%, Comfort –1% | –1%, –1% | 10 min |
| Watch | Vigil, Grit | Vigilance | Observe surroundings | Comfort –3%, Fulfillment +1% | –3%, +1% | 10 min |

---

## Cognitive & Behavioral (Internal)

| Action | Primary Stat(s) | Relevant Skill(s) | Description |
|--------|-----------------|-------------------|-------------|
| **Desire** | Grit | Motivation, Ambition | Generates needs and goals |
| **Remember** | Insight | Introspection, Scholarship | Logs experiences; influences decisions |
| **Decide** | Insight | Strategy, Tactics | Chooses action from available options |
| **Plan** | Insight | Strategy, Organization | Creates action sequence for goals |
| **Forget** | Grit | Focus, Introspection | Purges low-value memories |
| **Learn** | Insight, Grit | Scholarship, Socialization | Acquires knowledge/skills |

---

## Social (Advanced)

| Action | Primary Stat(s) | Relevant Skill(s) | Description | Need(s) Affected | Base Change | Base Duration |
|--------|-----------------|-------------------|-------------|------------------|--------------|----------------|
| Reproduce | Vigor, Grit | Nurturing, Empathy | Create offspring (passes DNA) | Reproduction +100% | — | — |
| Flee | Lithe, Vigor | Reflex, Survival | Escape danger | Health –2%, Warmth –2% | –2%, –2% | 10 min |
| Steal | Lithe, Vigil | Stealth, Deception | Take without permission | Comfort +2%, Health –1% | +2%, –1% | 5 min |
| Give Item | Grit, Lithe | Empathy, Bartering | Hand over object | Social +5%, Fulfillment +2% | +5%, +2% | 2 min |
| Observe | Vigil, Insight | Observation, Vigilance | Watch/listen for info | Fulfillment –1%, Comfort +1% | –1%, +1% | 10 min |
| Hide Item | Lithe, Insight | Stealth, Deception | Conceal objects | Comfort –2%, Fulfillment +1% | –2%, +1% | 10 min |
