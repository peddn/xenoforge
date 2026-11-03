# 04 — Other Hazards, Vehicle Combat, Gear

> Official errata and project decisions in [00-errata-und-festlegungen.md](00-errata-und-festlegungen.md) take precedence over this file.

Source: ALIEN RPG Core Rulebook (Free League, 1st ed. 2019). Scope: book pp. 106–147 (PDF 110–151).
All page references are **book pages**. Every stat table was checked against the rendered page image.

Notation used in this file:
- **success** = a six on any die (the book's crosshair icon, dropped from the text extraction).
- **facehugger** = a one on a Stress Die (the book's facehugger icon).
- **Base Dice** / **Stress Dice** as in the core rules. "Modification +N / –N" = add/remove N Base Dice (see cross-references).
- `AMBIGUOUS:` = my interpretation, not stated in the book. All are repeated in the "Ambiguities / Game Mother discretion" section.
- `ERRATA:` = corrected per official errata. `DECIDED:` = project decision where the book contradicts itself.
- "Derived" = a value computed from a rule in the book (not printed).

---

## 0. Cross-references outside this scope (summary only, for context)

These rules are referenced by the pages in scope. They belong to other chapters; the authoritative spec is that chapter's file. Summarized here so this file is self-contained.

- **Modification**: +1 = roll one extra Base Die; –1 = one fewer Base Die. Modifications add together. If not enough Base Dice to remove, remove Stress Dice; with no dice left the action cannot succeed. Sources: gear, difficulty, help from others. [p. 62]
- **Help from others**: +1 per helper, max +3; in combat helping counts as the same action type as the action supported. [p. 63]
- **Gear**: may add Base Dice (not always); some actions can't be attempted at all without a specific item. [p. 59]
- **Opposed roll**: you must roll successes and more successes than your adversary; each adversary success eliminates one of yours; only the attacker can push. [p. 64]
- **Time units**: Round (combat) = 5–10 s; Turn (stealth) = 5–10 min; Shift (recovery) = 5–10 h. [p. 83]
- **Encumbrance**: carry regular items up to 2 × STRENGTH; heavy = 2 regular (some 3 or 4, as given in gear lists); light = 1/2; some light = 1/4; tiny items don't count (but must be listed). Over-encumbered: up to STRENGTH × 4 items; then MOBILITY roll to run or crawl in a combat Round, fail = drop what you carry or stay put. [p. 34]
- **Consumables / Supply roll**: Air, Food, Water, Power each have a Supply rating. Supply roll = roll Stress Dice equal to current rating (max 6); each facehugger reduces rating by 1; at 0 you are out. Food and Water supplies encumber: rating 1–4 = 1 item, 5–8 = 2 items, +1 item per 4 more; at 0 no encumbrance. Air and Power normally don't encumber (in suit/gear); external tanks/batteries can. Group tracking allowed (GM). [p. 34–35]
- **Supply roll intervals**: Air = every Turn and after every strenuous activity (combat, MOBILITY roll); Water = once per day and after every strenuous activity; Food = once per day; Power = situational, per gear (this chapter). [p. 35]
- **Motion trackers**: usable once per Turn; Power Supply roll each use; senses large objects in motion out to LONG range (four zones max) indoors, EXTREME outdoors, and gives their zone; only moving objects; detection ≠ spotting (need same zone or line of sight; GM may require OBSERVATION for small/hidden). [p. 86]
- **Aiming**: fast action, +2 to the attack; lost if you do anything except shoot after aiming, or if hurt. [p. 94]
- **Full auto fire**: +2 modification; STRESS LEVEL immediately +1 and that Stress Die is added to the roll; extra successes beyond the first can go to secondary targets within SHORT range of the primary; the first success on a secondary target deals the weapon's Damage, further ones on it are stunts. [p. 96]
- **Ammo**: no bullet counting. If you fire and roll one or more facehuggers on Stress Dice, the magazine is empty after the attack (plus the Panic Roll). Reloading is a slow action. Track number of full reloads carried per weapon. Androids never run out this way. [p. 96]
- **Armor**: Armor Rating; only one suit of armor worn at a time. When you take damage from an attack, roll Base Dice = Armor Rating; each success reduces damage by 1; not an action. [p. 98]
- **Cover** combines with armor (add Armor Ratings). [p. 96]
- **Death Roll**: a STAMINA roll that cannot be pushed and uses no Stress Dice; fail = death. [p. 99]
- **Hypersleep**: hypersleep of any length makes you Dehydrated; Hydr8tion taken before stasis counteracts this. [p. 151]

---

## 1. OTHER HAZARDS

### 1.1 Conditions (general) [p. 106]

- There are four **conditions**: **Starving, Dehydrated, Exhausted, Freezing**. They can cause damage and block recovery. Marked in check boxes on the character sheet. [p. 106]
- Sickness (Disease) and Radiation are separate hazards with their own tracking (not among the four conditions). [p. 106, 109, 110]

### 1.2 Starving [p. 106]

| Aspect | Rule |
|---|---|
| Trigger | After **a day** without sufficient food. [p. 106] |
| Blocks | Cannot recover Health **or** relieve Stress. [p. 106] |
| Periodic roll | **Every day**: STAMINA roll. Fail → 1 point of damage **and** STRESS LEVEL +1. [p. 106] |
| If Broken | Death Roll **every day**. MEDICAL AID has no effect against these Death Rolls; only ingesting sustenance saves you. [p. 106] |
| Recovery | After eating, the effects above end **within one Shift**. [p. 106] |

- AMBIGUOUS: "a day without sufficient food" — the book does not tie this to Food Supply = 0 explicitly. Suggested: the day counter starts when the Food Supply rating reaches 0 (or the character has no food), GM can override.
- AMBIGUOUS: "wear off within one Shift" — suggested: effects (blocks, daily rolls) stop at the end of one Shift after eating; GM may end them earlier.

### 1.3 Dehydrated [p. 106]

| Aspect | Rule |
|---|---|
| Trigger | After **a day** without sufficient water. Also: hypersleep of any length makes you Dehydrated unless Hydr8tion was taken before stasis [p. 151, 137]. |
| Blocks | Cannot recover Health **or** relieve Stress. [p. 106] |
| Periodic effect | **Every Shift**: 1 point of damage **and** STRESS LEVEL +1 — **no roll**, automatic. [p. 106] |
| If Broken | Death Roll **after each Shift with no liquid**. MEDICAL AID can't stop these Death Rolls; only drinking helps. [p. 106] |
| Recovery | As soon as you drink, the effects wear off **within one Shift**. [p. 106] |

- AMBIGUOUS: same "day" trigger and "within one Shift" questions as Starving (Water Supply = 0 suggested as the start).

### 1.4 Exhausted [p. 106]

| Aspect | Rule |
|---|---|
| Requirement | You need to sleep at least **one Shift each day**. [p. 106] |
| Trigger | After **one day** without sufficient sleep. [p. 106] |
| Blocks | Cannot relieve Stress. (Unlike the other three conditions, blocking Health recovery is **not** listed.) [p. 106] |
| Periodic roll | STAMINA roll **each day** (GM decides when) with modification **–N**, N = number of days spent without decent sleep. Fail → you collapse and sleep for one Shift. [p. 106] |
| Recovery | As soon as you have slept at least one Shift, you are no longer Exhausted (immediate). [p. 106] |
| Related gear | Neversleep Pills remove the need for sleep for one day [p. 137]; Coffee postpones lack-of-sleep drawbacks for one Shift [p. 139]. |

- AMBIGUOUS: on the first day of being Exhausted N = 1 (one day without decent sleep) is suggested.
- AMBIGUOUS: collapsing and sleeping one Shift satisfies "slept for at least one Shift" and so ends Exhausted (suggested yes).

### 1.5 Freezing [p. 107]

| Aspect | Rule |
|---|---|
| Trigger | In an environment without enough clothes or shelter. [p. 107] |
| Blocks | Cannot recover Health **or** relieve Stress. [p. 107] |
| Periodic roll | STAMINA roll at intervals by temperature: **above freezing** = once per day; **sub-zero** = once per Shift; **deep cold of space** = every Turn. Fail → 1 point of damage **and** STRESS LEVEL +1. [p. 107] |
| If Broken | Death Roll **when your next cold roll would be due**. [p. 107] |
| Recovery | As soon as you get warm you stop rolling and can recover Health and relieve Stress normally (immediate, unlike Starving/Dehydrated). [p. 107] |

- Example: character with STAMINA 0 rolls only STRENGTH Base Dice; after the fail his STRESS LEVEL is 1, so next roll adds one Stress Die. [p. 108]
- AMBIGUOUS: when Broken, the Death Roll replaces the STAMINA roll at that interval, and continues at each later interval (suggested).
- Synthetics are immune to cold. [p. 111]

### 1.6 Vacuum [p. 107]

- Applies when without the protection of a space suit or a ship's hull (e.g. explosive decompression, thrown out of an airlock). [p. 107]
- **STAMINA roll every Round** without protection in a vacuum. Not an action, but must be rolled **before doing anything else in the Round**. [p. 107]
- Modification: Round 1 = unmodified; Round 2 = –1; Round 3 = –2; and so on (Round n → –(n–1)). [p. 107]
- **Failed roll** → drop directly to **zero Health** and make a **Death Roll every Round** until you die or enter a pressurized area. **No critical injury** is rolled. [p. 107]
- Putting on a space suit quickly = a single **slow action** and requires a successful **MOBILITY** roll. [p. 107]
- Example: a character must pass the vacuum STAMINA roll "in order to act at all" that Round. [p. 107]
- Synthetics are immune to vacuum. [p. 111]
- AMBIGUOUS: the book does not say what a *successful* roll protects against beyond being able to act that Round; suggested: success = no effect this Round. It does not say whether MEDICAL AID can stop the Death Rolls (suggested: no — only entering a pressurized area, per the text).

### 1.7 Explosive decompression [p. 107]

- **Missed shot** in a room adjoining an **outer bulkhead**: the bulkhead takes the weapon's **base Damage rating**; roll the bulkhead's Armor Rating (typical civilian ship: **Armor Rating 6**). [p. 107]
- If the armor roll fails to stop the shot, it **penetrates** the bulkhead. [p. 107]
- The room's air is lost to space **within a Turn**. In the strong draft, **every person** must roll **STAMINA in order to take any action**; that roll **is itself a fast action**. [p. 107]
- Most ships seal a breached compartment on their own. When its air is gone, whoever remains inside is exposed to **vacuum** (1.6). [p. 107]
- Related: on ships with safe-sensors, most military smart guns refuse to shoot at targets next to the outer hull; civilian weapons fire regardless ("No Weapons of Any Kind" sidebar). [p. 127]
- AMBIGUOUS: "fails to stop the shot" — suggested: penetration if armor successes < weapon base Damage (bulkhead armor roll reduces the base damage like normal armor; extra attack successes do not count since only the base Damage is applied). Whether Armor Piercing / Armor Doubled modify bulkhead armor is not stated (suggested: yes, apply weapon features).
- AMBIGUOUS: "STAMINA roll counts as a fast action" — suggested: each Round in the draft, a character spends the fast action on the STAMINA roll; on success they may perform their intended action (slow action) that Round.

### 1.8 Falling [p. 108]

- Falling on a hard surface automatically inflicts damage = **height in meters ÷ 2, rounded down**. [p. 108]
- In a **controlled jump**, roll **MOBILITY**: each success reduces the damage by 1. [p. 108]
- **Armor can protect** against falling damage (armor roll as normal). [p. 108]

### 1.9 Explosions [p. 108]

- Explosion force = **Blast Power**. [p. 108]
- For **each person within SHORT range** of the blast at detonation: roll **Base Dice = Blast Power**; each success = 1 point of damage. **Cannot be pushed.** [p. 108]
- Anyone at **ENGAGED** range of the detonation takes **1 extra point** of damage. [p. 108]
- **Effect radius**: charges with **Blast Power 7 or more** also harm people at **MEDIUM** range, with Blast Power **reduced by 6** for them. With many people at MEDIUM range the GM can roll once and apply the result to all of them. [p. 108]
- Weapons whose Damage reads "Blast N" inflict no direct damage but trigger an explosion with that Blast Power. [p. 118]

Algorithm:
```
for victim in range SHORT or ENGAGED: dice = BlastPower
for victim in range MEDIUM and BlastPower >= 7: dice = BlastPower - 6
damage = count(sixes on dice Base Dice); no push
if victim at ENGAGED: damage += 1
```
- AMBIGUOUS: the explosion section does not mention armor (unlike Fire and Falling, which say armor protects). Suggested: armor rolls apply as for any damage from an attack [p. 98]; GM decides.
- AMBIGUOUS: whether the ENGAGED +1 applies even when 0 successes are rolled. Suggested: yes (text says "suffer one extra point of damage").

### 1.10 Fire [p. 108]

- Fire has an **Intensity**; typical fire = **Intensity 8**. [p. 108]
- When exposed: roll **Base Dice = Intensity**; each success = 1 point of damage. **Armor can protect.** [p. 108]
- Taking damage means you **catch fire** and keep burning: **a new fire attack at the start of every Round**, **Intensity +1 per Round**. [p. 108]
- The first fire attack that deals **no damage** ends the fire on its own. [p. 108]
- Extinguish: **MOBILITY** roll (**slow action**) by the burning person or an ally at **ENGAGED** range; success puts the fire out. [p. 108]
- If **Broken by fire damage**, or you suffer fire damage while already Broken: **Death Roll every Round** until you die or are saved by a **MEDICAL AID** roll. [p. 108]
- Sources of Intensity 9 fire: M240 Incinerator (any target hit catches fire) [p. 125]; Multi-Directional Flame Unit [p. 147]; vehicle Fuel explosion [p. 115].
- AMBIGUOUS: order of "Intensity +1" — suggested: the burn attack at the start of Round 2 uses Intensity+1, Round 3 Intensity+2, etc.
- AMBIGUOUS: "inflicts no damage" — suggested: measured after the armor roll.
- AMBIGUOUS: for the M240, "any target hit also catches fire, Intensity 9" — suggested: target is on fire on any hit (even if armor negated the weapon damage) and suffers the first Intensity 9 fire attack at the start of the next Round (Intensity then rising per the rule above).

### 1.11 Disease [p. 109]

- On exposure to a dangerous contagion/infection: **Sickness Roll** = opposed roll **STAMINA vs. the disease's Virulence**. Typical **Virulence 3**; some much higher. Fail → you fall **sick**. [p. 109]
- While sick: [p. 109]
  - **One Shift after infection** the disease breaks out: you suffer **1 point of damage**.
  - You **can't recover Health** while sick.
  - New Sickness Roll **at the start of every Shift**; each failure = **1 more point of damage**.
  - If **Broken** while sick: another Sickness Roll **after every Shift** — **failure means death**.
  - One **successful** Sickness Roll cures you: no more rolls, Health recovers normally.
- **Medical Aid**: someone caring for you may roll your Sickness Rolls instead of you, rolling **MEDICAL AID vs. Virulence**. [p. 109]
- **Other diseases** can have unique effects (these rules describe a common but deadly disease). [p. 109]
- Synthetics are immune to disease and typically unaffected by alien pathogens. [p. 111]
- AMBIGUOUS: "opposed roll against Virulence" — suggested: the disease rolls Base Dice equal to its Virulence; the character must roll more successes (opposed roll rules [p. 64]).
- AMBIGUOUS: whether the first "start of Shift" Sickness Roll happens at the breakout Shift. Suggested timeline: T0 infection (failed roll) → T0+1 Shift: breakout damage 1, then at the start of each following Shift a Sickness Roll.

### 1.12 Radiation [p. 110]

- Exposure gives **Radiation Points (Rads)** that accumulate; check off Rad boxes on the character sheet. [p. 110]
- **Radiation Level** of the area sets the rate: [p. 110]

| Radiation Level | Rads gained |
|---|---|
| Weak | 1 Rad per Shift |
| Strong | 1 Rad per Turn |
| Extreme | 1 Rad per Round |

- **Effects**: every time you **gain** a Rad, roll a number of dice equal to your **total current accumulated Rads**; each success = **1 point of damage**. [p. 110]
- If **Broken by radiation damage**: **Death Roll each time you get another Rad**, until removed from the radiation hotspot. [p. 110]
- While **inside a hotspot** you **cannot recover Health in any way**. [p. 110]
- **Recovery**: after leaving the irradiated area, heal **1 Rad per Shift**. [p. 110]
- **Permanent radiation**: each time you are about to heal a Rad, roll **one Stress Die**; on a **facehugger** the Rad is **not healed** and becomes **permanent** (fill in the Rad box fully). Permanent Rads can never be healed. [p. 110]
- **Dirty bullets (U-238)**: anyone hit suffers **1 Rad** in addition to normal damage; carrying a weapon loaded with U-238 rounds gives **1 Rad per Shift**. [p. 123]

Algorithm:
```
on gainRad(): rads.temp += 1; dice = rads.temp + rads.permanent
             damage = count(sixes on dice)   // see AMBIGUOUS re dice type
             if character Broken by radiation: Death Roll
each Shift outside hotspot, if rads.temp > 0:
             roll 1 Stress Die; if 1 → rads.temp -= 1; rads.permanent += 1
                               else → rads.temp -= 1
```
- AMBIGUOUS: dice type is not stated ("a number of dice"); suggested Base Dice (Stress Dice would trigger panic).
- AMBIGUOUS: whether permanent Rads count toward "total current number of accumulated Rads" — suggested yes.
- AMBIGUOUS: armor does not apply (not mentioned) — suggested no armor roll.
- AMBIGUOUS: synthetics are not listed as immune to radiation (immunities: vacuum, cold, disease) — GM decides.

### 1.13 Drowning [p. 110]

- All PCs can swim; for movement, **swimming counts as crawling**. [p. 110]
- Underwater: **STAMINA roll every Round**; not an action; made during your turn before you perform actions. Fail → **1 point of damage**. [p. 110]
- If **Broken** when drowning: **Death Roll each Turn** until you die or a **MEDICAL AID** roll by someone saves you. [p. 110]
- AMBIGUOUS: the Death Roll interval is printed as "each Turn" while all other drowning timing is per Round; implement as printed (Turn), GM may use Round.

### 1.14 Suffocation [p. 110]

- When your Air Supply runs out (Air Supply 0, see p. 34): **STAMINA roll every Turn or after every strenuous activity** (e.g. a CLOSE COMBAT or MOBILITY roll). [p. 110]
- Modification: 1st roll unmodified, 2nd –1, 3rd –2, and so on (counter by roll, not by time). [p. 110]
- Fail → drop **directly to zero Health** and **Death Roll every Round** until you die or enter a pressurized area. [p. 110]
- Synthetics don't need air. [p. 111]

### 1.15 Synthetics [p. 111]

- In combat, androids act like humans and roll skill rolls normally, with these differences: [p. 111]
- **Attributes**: generally higher than humans. [p. 111]
- **Skills & Stress**: androids **can't push** skill rolls; **don't suffer stress**, have **no STRESS LEVEL**, **never make Panic Rolls**. [p. 111] (Hence they never run out of ammo via facehuggers [p. 96].)
- **Damage**: if a synthetic is **Broken**, roll a critical injury on the **Critical Injuries on Synthetics** table (D6). Until it suffers a **System Shutdown** it keeps operating, suffering only the crit effects. **Each further attack inflicting damage on a Broken android inflicts another critical injury.** If the same critical injury is rolled again, **shift it down one step** on the table (e.g. 4 → 5). [p. 111]
- **Repairs**: androids **don't heal**. **One Shift of work + a COMTECH roll** restores **all lost Health and removes all critical injuries**, but only if no System Shutdown has occurred. Androids **may repair themselves**. [p. 111]
- **Death**: androids **don't make Death Rolls** and cannot "die". After a **System Shutdown** they can be **reactivated for communication**, needing **a COMTECH roll, a power source and a Turn of work**. Once reactivated, it can be repaired (above) to working order but keeps a **permanent mental trauma**. [p. 111]
- **Resources**: synthetics don't need **air, food, water, or sleep**; immune to **vacuum, cold, and disease**; typically unaffected by **alien pathogens**. [p. 111]
- "Electric Sheep" sidebar (no dice mechanics): illegal to program androids for violence against humans; modern synthetics have behavioral inhibitors, earlier models do not; black-market bypass packages exist; lack of scheduled maintenance can cause mental degradation/psychosis rendering inhibitors moot. Useful as an NPC flag ("inhibitors: yes/no/bypassed"). [p. 111]

**Critical Injuries on Synthetics** [p. 111]

| D6 | Crit | Effect |
|---|---|---|
| 1 | Ruptured Fluid Pump | The android loses its next slow action. |
| 2 | Destroyed Leg Servos | Both legs are non-functional. The android must stay prone and can only crawl. |
| 3 | Destroyed Arm Servos | One arm is non-functional. The android can only use one-handed gear. |
| 4 | Head Dislocation | Neck severely fractured, head in unnatural position. All fast actions become slow actions. |
| 5 | Severe Chassis Breach | Internal parts spill out; android fully immobilized. |
| 6 | System Shutdown | Torn to shreds or crushed. Core systems severely damaged; a COMTECH roll is required to communicate with the android. |

- AMBIGUOUS: if the shifted-down result is also already held, suggested: keep shifting down; any result past 6 = 6 (System Shutdown). Rolling 6 when already in System Shutdown has no further effect.
- AMBIGUOUS: after System Shutdown the "Repairs" rule is blocked until reactivation (power source + Turn + COMTECH); suggested sequence: reactivate → then Shift + COMTECH to restore working order → add permanent mental trauma.
- AMBIGUOUS: self-repair while immobilized (crit 5) or with no functional arms — GM decides.

### 1.16 Xenomorphs (hazard notes on p. 112) [p. 112]

- Xenomorphs follow special combat rules (details Chapter 11 / GM section p. 290–291). [p. 112]
- **Multiple actions**: some act two or three times per Round; GM draws **initiative cards equal to their Speed** rating; at each initiative the Xenomorph performs **one fast and one slow action**. [p. 112]
- **Signature attacks**: most attack only in close combat at **ENGAGED** range; each attack the GM rolls **D6** on the creature's signature attack table. [p. 112]
- **Health**: at zero Health a Xenomorph is **not Broken**; the GM rolls **D6** on a special critical injury table (p. 291). [p. 112]
- **Armor Rating**: most have a natural Armor Rating. [p. 112]
- **Special abilities**: per creature description. [p. 112]

---

## 2. VEHICLE COMBAT

### 2.1 Basics [p. 113]

- Driving under normal circumstances requires **no dice rolls**; more advanced maneuvers require **PILOTING** rolls. [p. 113]
- **Entering, mounting or exiting** a vehicle = **fast action**. ERRATA (p. 88): the action table's enter/exit vehicle entry is a fast action, consistent with p. 113. [p. 88, 113]
- DECIDED: **Starting a motorized vehicle** (start engine) = **slow action**, per the p. 88 action table. p. 113 says **fast action**; supported as a documented alternative (GM option). [p. 88, 113]

### 2.2 Vehicle attributes [p. 114]

| Attribute | Meaning |
|---|---|
| Passengers | Number of people who can ride in the vehicle, **excluding the driver**. |
| Maneuverability | Modification to your **PILOTING** rolls when performing challenging maneuvers. |
| Speed | Number of **zones per driving action**. |
| Hull | How much damage the vehicle can take before becoming **Wrecked**. |
| Armor | The vehicle's **Armor Rating**. |

### 2.3 Vehicles in combat [p. 114]

- Operate a vehicle with the **drive** action: a **fast action**, moves you up to **Speed** zones. Like running, you can **drive twice in the same Round**. [p. 114]
- **Vehicle weapons**: mounted heavy weapons fired by the **crew** (in some cases the **driver**). List on p. 147. [p. 114]
- **Ramming enemies** (people): counts as a **close combat attack**, but roll **PILOTING** instead of CLOSE COMBAT, adding the vehicle's **Maneuverability**. **Base Damage = Hull ÷ 5, rounded up.** [p. 114]

### 2.4 Damage to vehicles [p. 114]

- Vehicles suffer damage like people; most have an Armor Rating (armor roll per p. 98). [p. 114]
- When total damage **≥ Hull**, the vehicle is **Wrecked**: inoperable. [p. 114]
- **Component damage**: vehicles **don't suffer critical injuries** when Wrecked. Instead, if a **single hit** deals damage **≥ half the Hull rating (round up)**, the vehicle suffers **component damage**: roll **D6** on the Component Damage table. A vehicle can suffer component damage **once** without being Wrecked; a **second component damage always Wrecks** it. [p. 114]
- **Ramming vehicles**: allowed only if **your Hull ≥ target's Hull**. Roll as for ramming a person. Damage goes to the enemy vehicle. The target's driver and passengers are hurt **only if it is Wrecked**; each then takes **damage equal to the vehicle's**. [p. 114]

**Component Damage** [p. 115]

| D6 | Damage | Effects |
|---|---|---|
| 1 | Driver hit | Driver takes damage = the damage dealt to the vehicle. |
| 2 | Passenger hit | One random passenger takes damage = the damage dealt to the vehicle. No passengers: re-roll. |
| 3 | Severe spin | Driver rolls PILOTING at once (not an action); on a failure the vehicle is Wrecked. |
| 4 | Weapon disabled | One random mounted weapon is disabled. No weapons: re-roll. |
| 5 | Engine disabled | Vehicle stops and can't go on. Aerial: controlled landing possible, descending 1 altitude zone per Round. |
| 6 | Fuel explosion | Vehicle blows up: everyone aboard is instantly exposed to **Intensity 9 fire**; vehicle **destroyed beyond repair**. |

- AMBIGUOUS: component-damage threshold compared to damage **after** the armor roll (suggested).
- AMBIGUOUS: whether the second component damage also rolls on the table (suggested: it Wrecks the vehicle; GM may also roll for flavor/effects).
- AMBIGUOUS: whether Driver/Passenger-hit damage gets a personal armor roll (suggested yes, GM).
- AMBIGUOUS: rammer takes no damage when ramming (not stated). "Same amount of damage as the vehicle did" = damage of the wrecking hit (suggested).
- AMBIGUOUS: whether the "component damage taken" counter resets after repairs (not stated; suggested: resets when all component damage is repaired).

### 2.5 Aerial vehicles [p. 115]

- Handled like ground vehicles, with exceptions: [p. 115]
- **Altitude**: always tracked, measured in **zones**. When moving, distribute the zones moved between vertical and horizontal movement. On the ground, an aerial vehicle's Speed is typically **1**. [p. 115]
- **Crashes**: if an aerial vehicle is **Wrecked**, it crashes; everyone inside suffers damage = **altitude (zones) × 3**. Brace with a **MOBILITY** roll: each success reduces the damage by 1. [p. 115]
- Engine disabled (component 5): controlled landing losing **one zone of altitude per Round**. [p. 115]
- AMBIGUOUS: which listed vehicles are "aerial" is not flagged in the vehicle table; from descriptions: Aerodyne Gyrocar (VTOL, ground or air, flight ceiling 3,000 m), UD-4L Cheyenne (VTOL dropship), WY-37B Cargo Lifter (suborbital, flies). Daihotai, NR-9 and M577 are ground vehicles.
- AMBIGUOUS: armor against crash damage not stated.

### 2.6 Repairs [p. 115]

- Repairing vehicle damage requires one or several **HEAVY MACHINERY** rolls; **one roll per Shift**. Only **one person** rolls to repair the hull; others can **help** (+1 each, max +3 [p. 63]). Each success removes **1 point of damage**. [p. 115]
- A Wrecked vehicle is no longer Wrecked once it has regained **at least one HP** (i.e. damage < Hull). [p. 115]
- **Component damage**: a disabled engine or mounted weapon typically requires a **HEAVY MACHINERY** roll **in addition** to any hull repair roll; the work takes **one Shift**. [p. 115]
- A vehicle destroyed by **Fuel explosion** is beyond repair. [p. 115]
- AMBIGUOUS: a vehicle Wrecked by a second component damage or Severe spin while damage < Hull — suggested: Wrecked flag clears when the component damage is repaired (one Shift, HEAVY MACHINERY).

---

## 3. GEAR — GENERAL RULES

### 3.1 Weapon stat columns [p. 118]

- **Bonus**: modification applied to **CLOSE COMBAT** / **RANGED COMBAT** rolls made with this weapon. [p. 118]
- **Damage**: base Damage rating — damage the target suffers if the attack succeeds; extra successes can deal additional damage. **"Blast N"** = no direct damage; triggers an explosion with Blast Power N (see 1.9). [p. 118]
- **Range**: **maximum** range category at which the weapon can be effectively used. [p. 118]
- **Weight**: number of regular items it takes up in the inventory (p. 34). **No weight listed = too heavy to carry.** [p. 118]
- **Cost**: typical price in **UA dollars**; actual price varies with supply and demand at the location. [p. 118]
- **Special** (Comment column): special effects. [p. 118]
  - **Armor Piercing**: Armor Ratings are **halved** vs. this weapon (**round up**). [p. 118]
  - **Armor Doubled**: any Armor Rating is **doubled** against this weapon. [p. 118]
  - **Full auto**: may fire full auto bursts (rules p. 96, see §0).
  - **Single-shot**: must be reloaded (**slow action**) after each shot (stated for the DV-303 Bolt Gun [p. 119]). AMBIGUOUS: applied to the ASSO-400 too, since it carries the same keyword.
  - **Power Supply N**: has a battery with Power Supply N when fully charged; make a Power Supply roll after every shot/attack/use (per item text).
  - **Fire Intensity N**: target hit catches fire at Intensity N (1.10).
  - **Stun effect**: see stun baton / G2 grenade.

### 3.2 Weight values (as used in the tables)

| Printed | Meaning | Source |
|---|---|---|
| 0 | Tiny item — does not count toward encumbrance (must still be listed). | [p. 34, 134 "Optical Scope … Tiny item"] |
| 1/4 | Light item counting as a quarter of a regular item. | [p. 34] |
| 1/2 | Light item (half a regular item). | [p. 34] |
| 1 | Regular item. | [p. 34] |
| 2 | Heavy item (two regular items). Text calls Mk.35 and Eco suit (weight 2) "Heavy item". | [p. 34, 128] |
| 3 | Heavy item counting as three (M56A2 Smart Gun). | [p. 34, 124] |
| – | Weapons: too heavy to be carried. | [p. 118] |

- AMBIGUOUS: "–" on non-weapon items (Computer Terminal, HoloTab, Modular Computing Device, Microview, Pauling MedPod, AutoDoc, P-5000) — suggested: installed/not carriable. On Unarmed Attack "–" simply means no item.

### 3.3 Other general gear points

- **Ammunition**: a firearm's full reload usually costs roughly **5% of the weapon's price**. [p. 124]
- **Dirty bullets (U-238)**: any ammunition type can be treated; costs **4 × normal ammo**. U-238 rounds **reduce the Armor Rating of any target by 2**, applied **after** Armor Piercing / Armor Doubled. Anyone hit suffers **1 Rad** in addition to normal damage. Carrying a weapon loaded with U-238 rounds gives **1 Rad per Shift**. [p. 123]
  - AMBIGUOUS: Armor Rating can't drop below 0 (suggested). Rad from a hit triggers the normal radiation damage roll (1.12) (suggested).
- **Armor**: Armor Rating explained p. 98; Air Supply p. 34. [p. 128]
- **Availability**: no formal availability rule in the gear chapter. Text notes only that you can't expect your choice of firearms and that civilians will have trouble getting a permit for an M41A. [p. 118, 121]
- **Gear damage / breaking**: no rule in scope (gear does not degrade).
- **Battery recharge / air refill**: no rule in scope. AMBIGUOUS: GM decides when a Power/Air supply is restored to its maximum.

---

## 4. WEAPONS

Values are from the **summary tables**. Where the item's **stat card** (picture box) on a following page prints different values, both are given. ERRATA (pp. 119–124): for the M4A3 Service Pistol, Armat M41A Pulse Rifle, M42A Scope Rifle and Armat M41AE2 Heavy Pulse Rifle the **table values are correct**; the stat-card values were misprints and are kept below only as notes (no Game Mother override).

### 4.1 Pistols — table [p. 119]

| Weapon | Bonus | Damage | Range | Weight | Cost | Comment (printed) | Stat card differs | Additional rules |
|---|---|---|---|---|---|---|---|---|
| M4A3 Service Pistol | +2 | 1 | Medium | 1/2 | $200 | – | ERRATA: table correct; card Bonus +1 was a misprint [p. 119] | — |
| .357 Magnum Revolver | +1 | 2 | Medium | 1 | $300 | – | no [p. 119] | — |
| Rexim RXF-M5 EVA Pistol | +1 | 1 | Medium | 1/2 | $400 | Armor piercing | no [p. 120] | Weaponized laser welder. |
| Watatsumi DV-303 Bolt Gun | – | 3 | Short | 1 | $400 | Armor piercing, single-shot | no [p. 120] | Must be **reloaded (slow action) after each shot** [p. 119]. As a tool: **HEAVY MACHINERY +2** in relevant situations [p. 135]. |

### 4.2 Rifles — table [p. 120]

| Weapon | Bonus | Damage | Range | Weight | Cost | Comment (printed) | Stat card differs | Supply | Additional rules |
|---|---|---|---|---|---|---|---|---|---|
| Armat M41A Pulse Rifle | +1 | 2 | Long | 1 | $800 | Armor piercing, full auto | ERRATA: table correct; card Cost $1,200 was a misprint (card comment also lists the grenade launcher, matching the text) [p. 122] | – | Has an underslung **U1 grenade launcher** ("see the U1 below") [p. 121]; use U1 stats/rules for grenades (4.3). |
| AK-4047 Pulse Assault Rifle | – | 2 | Long | 1 | $500 | Full auto | (no card) | – | Sturdy (fluff). [p. 121] |
| M42A Scope Rifle | +2 | 2 | Extreme | 1 | $1,000 | Armor piercing | ERRATA: table correct; card Bonus +1, Damage 3 was a misprint [p. 122] | – | Semi-automatic sniper rifle. [p. 121] |
| Armat Model 37A2 12 Gauge Pump Action | +2 | 3 | Short | 1 | $500 | Armor doubled | no [p. 122] | – | — |
| SpaceSub ASSO-400 Harpoon Grappling Gun | – | 1 | Medium | 1 | $300 | Armor doubled, single-shot | no [p. 123] | – | Hit = hook **attaches** to the target. Target **heavier than you**: rappel toward it (**slow action**, moves you from **MEDIUM to ENGAGED**). **You heavier**: pull the target toward you (**STRENGTH vs. STRENGTH** opposed roll if the target resists). [p. 121] |
| Armat XM99A Phased Plasma Pulse Rifle | – | 4 | Extreme | 2 | $20,000 | Armor piercing, Power Supply 5 | Card omits "Power Supply 5" (text confirms it) [p. 123] | **Power 5** | **Must be aimed carefully (fast action) before it is fired.** **Power Supply roll after every shot**; battery Power Supply **5** when fully charged. [p. 121] |

- AMBIGUOUS: XM99A mandatory aim — suggested: this is the normal Aim action [p. 94] and therefore also grants its +2.

### 4.3 Heavy weapons — table [p. 124]

| Weapon | Bonus | Damage | Range | Weight | Cost | Comment (printed) | Stat card differs | Additional rules |
|---|---|---|---|---|---|---|---|---|
| Armat U1 Grenade Launcher | +1 | Blast 9 | Long | 1/2 | $600 | Fires other grenade types too | (no card) | See grenade rules below. [p. 125] |
| Armat M41AE2 Heavy Pulse Rifle | +1 | 3 | Extreme | 2 | $1,500 | Armor piercing, full auto | ERRATA: table correct; card Bonus +2, Damage 2, Cost $1,200 was a misprint (Range, Weight matched) [p. 124] | Squad automatic weapon; longer barrel instead of U1 (no grenade launcher). [p. 125] |
| M56A2 Smart Gun | +3 | 3 | Long | 3 | $6,000 | Armor piercing, full auto | no [p. 126] | Harness-mounted; IR tracking sends to a **Head-Mounted Sight**. [p. 125] |
| M240 Incinerator Unit | – | 2 | Medium | 1 | $500 | Fire Intensity 9 | no [p. 126] | **Any target hit also catches fire, Intensity 9** (Fire rules p. 108). [p. 125] |
| UA 571-C Sentry Gun | +2 | 4 | Extreme | – | $12,000 | Armor piercing, full auto, RANGED COMBAT 8 | (no card) | Weight "–" = too heavy to carry. Once **activated**: **permanently in overwatch**; attacks use **RANGED COMBAT skill level 8 (no AGILITY)**. **Remote operation** via a Head-Mounted Sight. Its ARS (Advanced Recognition Software) can be set to recognize friendlies, with mixed reliability; set to motion/heat only it fires at anything hot that moves. IFF Transponder prevents its friendly fire. [p. 125, 131] |
| G2 Electroshock Grenade | – | Stun | Medium* | 1/2 | $400 | Stun effect (–2) | (no card) | *Range Medium **when thrown by hand**. **Each human in the target zone** rolls **STAMINA, hard (–2, not an action)**; failure = **stunned for one Round**. Can be fired from the U1. [p. 124, 125] |

**U1 grenade launcher ammunition** [p. 125]

| Grenade | Effect |
|---|---|
| Frag (M40 HE fragmentation, standard payload) | Target **hit**: **Blast Power 9** explosion damage **+1 point**. **Everyone else in that zone**: explosion damage (no +1). |
| Smoke | No damage; **blocks line of sight** within the target zone and across its edge in both directions. |
| Flash | Each target in the zone rolls **STAMINA** at once (**no action**); failure = **next slow action lost**. |
| G2 Electroshock | As G2 above. |
| Any, on a **miss** | The grenade lands in a **random adjacent zone**. |

- AMBIGUOUS: U1 frag — whether the Effect Radius rule (Blast Power ≥ 7 also hits MEDIUM range at Blast Power 3) also applies; the U1 text only names the same zone. Suggested: apply the general Explosion rules (same zone = SHORT; adjacent = MEDIUM at Blast 3) — GM decides.
- AMBIGUOUS: Sentry Gun — whether its +2 Bonus is added to the 8 skill dice (suggested: yes, 10 Base Dice total; GM may treat 8 as all-inclusive).
- AMBIGUOUS: "stunned for one Round" is not defined anywhere in the core book. Suggested: the stunned character loses all actions (fast and slow) for their next Round.
- AMBIGUOUS: "All humans" (G2) — suggested: synthetics/Xenomorphs unaffected unless GM rules otherwise.
- AMBIGUOUS: the price of individual grenades/smoke/flash rounds is not given (G2 = $400 listed); use the 5% reload rule or GM.

### 4.4 Close combat weapons — table [p. 126]

| Weapon | Bonus | Damage | Range | Weight | Cost | Comment (printed) | Supply | Additional rules |
|---|---|---|---|---|---|---|---|---|
| Unarmed Attack | – | 1 | Engaged | – | – | Armor doubled | – | — |
| Blunt Instrument | +1 | 1 | Engaged | 1 | – | – | – | — |
| Knife | – | 2 | Engaged | 1/2 | $50 | – | – | — |
| Stun Baton | +1 | 1 | Engaged | 1/2 | $80 | Stun effect, Power Supply 5 | **Power 5** | Anyone hit **who takes 1+ points of damage** must make an immediate **STAMINA roll (not an action)** or be **stunned for one Round**. **Power Supply roll after each attack**; battery Power Supply **5** when fully charged. [p. 127] |
| Cutting Torch (Mechanical Cutting Torch) | – | 3 | Engaged | 1 | $300 | Armor piercing, Power Supply 5 | **Power 5** | **Power Supply roll after every use**; battery Power Supply **5**. As a tool: **HEAVY MACHINERY +2** [p. 127, 135]. |

Other items usable as close combat weapons (not in the table):

| Item | Damage | Other | Page |
|---|---|---|---|
| Surgical Kit | Base Damage 2 when used as a weapon | AMBIGUOUS: bonus/range not given — suggested Bonus –, Range Engaged | [p. 136] |
| P-5000 Power Loader | Base Damage 3 in close combat attacks | CLOSE COMBAT +3; requires HEAVY MACHINERY skill level 2+ | [p. 128] |

- AMBIGUOUS: stun baton "after each attack" — suggested: the Power Supply roll is made after every attack, hit or miss.

### 4.5 Vehicle weapons — table [p. 147]

No Weight column (vehicle-mounted; cannot be carried).

| Weapon | Bonus | Damage | Range | Cost | Comment (printed) | Mounted on (standard armament) |
|---|---|---|---|---|---|---|
| 20mm Gatling Gun | +3 | 3 | Long | $7,500 | Armor piercing. Full auto. | M577 APC (turret) [p. 142] |
| 25mm Gatling Gun | +3 | 3 | Extreme | $9,000 | Armor piercing. Full auto. | UD-4L Cheyenne [p. 145] |
| Phased Plasma Pulse Cannon Turret | +1 | 6 | Extreme | $54,000 | Armor piercing. | M577 APC [p. 142] |
| Multi-Directional Flame Unit | – | 4 | Medium | $4,200 | Fire Intensity 9. | (none standard) |
| Mk.16 150mm Banshee 70 Unguided Rocket Launcher | – | 3 | Long | $48,000 | Direct damage plus a **Blast Power 12** explosion. Full auto. | UD-4L Cheyenne [p. 145] |
| AGM-220C Hellhound II Tactical Missile Launcher | +2 | 7 | Extreme | $76,000 | **Targets: ground vehicles and buildings only.** Armor piercing. **Per missile: $12,000.** | UD-4L Cheyenne: **7 × missiles** [p. 145] |
| AIM-90E Headlock Air-to-Air Missile Launcher | +3 | 5 | Extreme | $64,000 | **Targets: air vehicles only.** Armor piercing. **Per missile: $9,000.** | UD-4L Cheyenne: **3 × missiles** [p. 145] |

- Banshee Blast Power 12 ≥ 7 → also hits MEDIUM range at Blast Power 6 (general Explosion rule, 1.9). Derived.
- AMBIGUOUS: missile counts (7 Hellhound, 3 Headlock on the Cheyenne) suggest each missile fired consumes one; the book gives no explicit per-shot rule. Suggested: track missiles as finite ammunition.
- AMBIGUOUS: who fires which mounted weapon ("crew, and in some cases the driver") — GM assigns; suggested: one operator per weapon per Round, using that operator's RANGED COMBAT.

---

## 5. SUITS AND ARMOR — table [p. 127], text [p. 128]

The "Air Supply" column is the suit's **Maximum Air Supply** (text wording). "Armor Rating" works per p. 98 (only one suit of armor at a time).

| Suit | Armor Rating | Air Supply (max) | Weight | Cost | Comment (printed) | Skill mod | Vacuum protection | Additional rules (text) |
|---|---|---|---|---|---|---|---|---|
| M3 Personnel Armor | 6 | – | 1 | $1,200 | Built-in comm unit | – | No | Built-in comm unit **and Personal Data Transmitter**; contact points transmit wearer's vitals to a tactical monitoring station; combat webbing. [p. 128] |
| IRC Mk.50 Compression Suit | 2 | 5 | 1 | $15,000 | AGILITY –1, Air Supply 5 | **AGILITY –1** (table only) | Yes (maintains internal pressure in vacuum) | Helmet with comm unit, HUD, head light, wireless helmet cam. [p. 128] |
| IRC Mk.35 Pressure Suit | 5 | 4 | 2 | $2,000 | Air Supply 4 | – | Yes (full protection) | "Heavy item." Must spend time in a decompression chamber after spacewalking in it. Hard joints may seize in extreme motion (fluff). [p. 128] |
| Eco All-World Survival Suit | 4 | 6 | 2 | $30,000 | (blank) | – | Yes (maximum protection from space hazards) | "Heavy item." HUD; own **thrusters for tether-free zero-G maneuvers**; comm array **unshielded** (sensitive to rogue transmissions). [p. 128] |
| Weyland-Yutani APEsuit | 3 | 4 | 1 | $5,000 | SURVIVAL +3 | **SURVIVAL +3** | **No** (cannot protect from vacuum) | Filtered air supply (Air Supply 4); resistance to temperature extremes; **impervious to caustic substances**; protective eyewear and face mask. [p. 128] |
| P-5000 Power Loader (Caterpillar P-5000 Powered Work Loader) | 3 | – | – | $50,000 | HEAVY MACHINERY and CLOSE COMBAT +3 | **HEAVY MACHINERY +3, CLOSE COMBAT +3** | No | **Base Damage 3** for close combat attacks. Usable only with **HEAVY MACHINERY skill level 2+**. Weight "–" (not carried). [p. 128] |

- AMBIGUOUS: Mk.50 "AGILITY –1" appears only in the table comment, not in the text; it is unclear whether it is –1 to the AGILITY attribute or to AGILITY-based skill rolls (suggested: –1 modification to all AGILITY-based skill rolls while worn). The Mk.35 text ("bulky… hard joints tend to seize up") fits a penalty better; GM may move it.
- AMBIGUOUS: APEsuit "resistance to temperature extremes" — suggested: GM may waive Freezing while worn; no explicit rule.
- AMBIGUOUS: Mk.35 decompression chamber time is not quantified.
- AMBIGUOUS: P-5000 listed as armor (AR 3) — whether it stacks with worn armor is not stated; "only one suit of armor at a time" [p. 98] suggests no stacking.

---

## 6. OTHER EQUIPMENT

### 6.1 Computer mainframes — table [p. 130]

| Mainframe | Skills | Cost |
|---|---|---|
| A.P.O.L.L.O. | Comtech 5, Piloting 4, Observation 5 | $2,000,000 |
| MU/TH/UR 6000 | Comtech 5, Piloting 5, Observation 5 | $2,500,000 |
| MU/TH/UR 6500 | Comtech 6, Piloting 5, Observation 6 | $3,500,000 |
| MU/TH/UR 7000 | Comtech 7, Piloting 6, Observation 7 | $5,000,000 |
| MU/TH/UR 9000 | Comtech 10, Piloting 8, Observation 10, Ranged Combat 9 | $50,000,000 |

- MU/TH/UR talks over the intercom, but most units allow **direct access only from the computer core**; only the **commanding officer** gets core entry (card + punch code) and can give alternate commands and overrides. [p. 130]
- Standard access: both systems issue reports/updates to the crew and analysis on request through a standard terminal; CO and science/medical officers often have better interface privileges. [p. 130]
- AMBIGUOUS: skill values are presumably rolled as the mainframe's dice pool (cf. opposed roll example vs. "Mother's 8 Base Dice", p. 64); the book does not say how to roll them.

### 6.2 Data storage — table [p. 131]

| Device | Weight | Cost | Effect |
|---|---|---|---|
| Long-Data Disc | 0 | $30 | Stores max. 10 zettabytes (ZB) |
| Magnetic Tape | 0 | $5 | Stores max. 120 terabytes (TB) (60, 90 or 120 TB cassettes; a strong magnetic pulse can easily corrupt it; gives off no readily detectable wave signals) [p. 131] |

### 6.3 Diagnostics and display — table [p. 131], text [p. 132]

| Device | Weight | Cost | Effect (printed) | Skill bonus | Notes |
|---|---|---|---|---|---|
| Computer Terminal | – | Varies | Access and process data (COMTECH roll) | – (requires COMTECH roll) | With the right codes and COMTECH rolls can give access to what MU/TH/UR or A.P.O.L.L.O. knows. [p. 132] |
| PR-PUT Uplink Terminal | 1 | $9,000 | Remote-pilot a spacecraft (COMTECH roll) | – (requires COMTECH roll) | Given access codes, links through a colony uplink tower to fly a ship in orbit remotely. [p. 132] |
| Seegson C-Series Magnetic Tape Recorder | 1/2 | $50–$100 | Music recording/playback (MANIPULATION +1) | **MANIPULATION +1** | [p. 132] |
| Samani E-Series Watch | 0 | $50 | Shows time, O2 and pressure readings (SURVIVAL +1) | **SURVIVAL +1** | E-550 model has O2/pressure sensors (hull breach warning). [p. 132] |
| Personal Data Transmitter | 0 | $100 | Monitors location and vitals | – | Transmits location; some models vitals; can be implanted, built into a suit, or worn. [p. 132] |
| IFF Transponder | 0 | $250 | Prevents Sentry Gun friendly fire | – | Usually surgically implanted. [p. 132] |
| Data Transmitter Cards | 0 | $50 | Transfer of audiovisual data | – | [p. 132] |
| Seegson P-DAT | 1/2 | $500 | Coordinate info for field teams | – | Syncs with mapping devices, PDTs, helmet cams. [p. 132] |
| Seegson System Diagnostic Device | 1 | $300 | Troubleshoot computer systems (COMTECH +2) | **COMTECH +2** | Can be used to hack doors and terminals. [p. 132] |
| HoloTab | – | $100,000 | Strategic analysis platform (COMMAND +2) | **COMMAND +2** (text: **COMMAND and OBSERVATION +2**) | [p. 131, 132] |
| Modular Computing Device | – | $8,000 | Full audiovisual holographic projector | – | Holograms fill a 6 × 6 m area. [p. 132] |

- AMBIGUOUS: HoloTab — table gives COMMAND +2 only; text gives COMMAND and OBSERVATION +2. Suggested: both.

### 6.4 Vision devices — table [p. 133], text [p. 134]

| Device | Weight | Cost | Effect (printed) | Skill bonus | Supply | Notes |
|---|---|---|---|---|---|---|
| Optical Scope | 0 | $60 | Range increased one category | – | – | Pistol or rifle; e.g. MEDIUM→LONG, LONG→EXTREME; **only for aimed shots**; Tiny item. [p. 134] |
| Hi-beam Flashlight | 1/2 | $45 | Cancels darkness in one zone | – | – | [p. 133] |
| Binoculars | 1/2 | $100 | Bonus +2 to OBSERVATION | **OBSERVATION +2** | – | Only at **LONG range or more** when **scouting actively** (not passive awareness). [p. 134] |
| M314 Motion Tracker | 1 | $1,200 | Rules p. 86; indoors: LONG range. | – | **Power 5** | EXTREME range in open terrain, LONG in close quarters; **Power Supply roll after every use**; usable once per Turn [p. 86]; can't fire a weapon and operate it at the same time. [p. 134] |
| M316 Motion Tracker | 0 | $3,000 | Rules p. 86; indoors: MEDIUM range. | – | **Power 3** | Pulse-rifle mounted; LONG outdoors, MEDIUM indoors; operates about the same as the M314. [p. 134] |
| Head-Mounted Sight | 1/2 | $200 | Remote control of Sentry Gun | – | – | Syncs with smart gun or sentry tech to help focus/choose targets. [p. 134] |
| Neuro Visor | 1 | $10,000 | Monitor hypersleep patient | – | – | Communicating with a hypersleeping subject requires a **COMTECH** roll. [p. 134] |
| "Pups" Mapping Device (W-Y Parameter Uplink Spectrograph) | 1 | $50,000 each | Scans one zone per Round | – | – | Detects any enemies in zones they pass; also lifeforms, atmosphere, toxins; **Range EXTREME**; sends data to a synced device (HoloTab etc.). [p. 134] |
| Seegson Microview-2000SE | – | $25,000 per deck | Location tracking | – | – | Station navigation "You are Here" map. [p. 134] |

- AMBIGUOUS: M316 Power Supply roll after each use is implied ("operates about the same as the M314" + motion tracker rule p. 86), not restated.
- AMBIGUOUS: optical scope on an Extreme-range weapon has no higher category.

### 6.5 Tools — table and text [p. 135]

| Device | Weight | Cost | Effect (printed) | Skill bonus | Supply |
|---|---|---|---|---|---|
| Watatsumi DV-303 Bolt Gun | 1 | $400 | HEAVY MACHINERY +2 | **HEAVY MACHINERY +2** (relevant situations) | – |
| Cutting Torch | 1 | $300 | HEAVY MACHINERY +2, Power Supply 5 | **HEAVY MACHINERY +2** (relevant situations) | **Power 5**, roll after every use |
| Maintenance Jack | 1 | $150 | HEAVY MACHINERY +1 | **HEAVY MACHINERY +1** (opening unpowered airlocks, diverting power at junction boxes) | – |
| Electronic Tools | 1/2 | $250 | COMTECH +1 | **COMTECH +1** | – |

- The DV-303 and Cutting Torch are the same items as the weapons in 4.1/4.4 (one item, two uses). [p. 135]

### 6.6 Medical supplies — table [p. 135], text [p. 136]

| Device | Weight | Cost | Effect (printed) | Skill bonus / fixed skill | Notes |
|---|---|---|---|---|---|
| Personal Medkit | 1/4 | $50 | MEDICAL AID +2 | **MEDICAL AID +2** | **Can only be used once.** [p. 136] |
| Surgical Kit | 1/2 | $25–$200 | MEDICAL AID +1 (death prevention) | **MEDICAL AID +1**, only on rolls to **prevent death from a fatal critical injury** (p. 99) | Base Damage 2 as a weapon. [p. 136] |
| Pauling MedPod | – | $2,000,000 | MEDICAL AID 10 | Fixed **MEDICAL AID skill level 10** (roll for the skill level only); **can't push** | Programmed with a **simple (+2) COMTECH** roll. Diagnoses/treats infections, wound repair, pre-programmed surgery. [p. 136] |
| AutoDoc | – | $500,000 | MEDICAL AID 6 | Fixed **MEDICAL AID skill level 6**; **can't push** | Programmed with an **unmodified COMTECH** roll. Treats lesser wounds and infections; can reset broken bones; **not capable of complicated surgery**. [p. 136] |

- AMBIGUOUS: AutoDoc "skill level 6" — "roll for the skill level only" is stated only for the MedPod; suggested same for AutoDoc (6 Base Dice).
- AMBIGUOUS: what "complicated surgery" excludes for the AutoDoc (e.g. which fatal criticals) — GM decides.

### 6.7 Pharmaceuticals — table [p. 136], text [p. 137]

| Substance | Weight | Cost | Effect (printed) | Full rule | Duration | Side effects |
|---|---|---|---|---|---|---|
| Neversleep Pills | 0 | $2 | STRESS LEVEL +1 per dose | Each dose: **STRESS LEVEL +1** and **no sleep needed for one day**; for that day you **can't relieve stress** (p. 104). | One day | Excessive use can cause stroke or heart attack (no mechanic; GM). |
| Hydr8tion | 0 | $5 | Removes Dehydration from hypersleep | Taken **before stasis**, counteracts the Dehydrated condition caused by hypersleep (p. 151). | The hypersleep | **No side effects.** |
| Naproleve | 0 | $20 | Reduces STRESS LEVEL to zero | Injectable; **immediately reduces STRESS LEVEL to zero**. Overdose: **every extra dose (after the first) in one Shift** = **–1 on all AGILITY-based skill rolls for the rest of that Shift**. | Instant (penalty to end of Shift) | Intoxication (the AGILITY penalty). |
| Recreational Drugs | 0 | Varies | Varies | Legal recreational drugs; price from $5 (pack of cigarettes) to $60,000 (kilo of benzoylecgonine). | Varies | Varies (GM) |
| X-drugs | 0 | Varies | Varies | Increase strength, endurance, senses; effects and prices vary. | Varies | Prolonged use: hallucinations, seizures, psychosis, stroke (GM). |

- AMBIGUOUS: Naproleve penalty stacks (2nd dose –1, 3rd dose –2 total, …) — suggested yes ("each dose beyond the first").
- AMBIGUOUS: X-drug and recreational drug mechanics are entirely GM-defined.

### 6.8 Food and drink — table [p. 137], text [p. 139]

| Sustenance | Weight | Cost | Effect (printed) | Full rule (text) | Duration |
|---|---|---|---|---|---|
| Prefab Meal | 1/4 | $10 | Food Supply +1 | Increases your **Food Supply** one step. [p. 139] | – |
| Water Bottle | 1/4 | $2–$100 | Water Supply +1 | Increases your **Water Supply** one step. Shipboard company water $2; bottles of purified water $10, up to $100/bottle on a barren rock. [p. 139] | – |
| "Bug Juice" Protein Drink | 1/4 | $5 | Food Supply +1 and Water Supply +1 | Increases **both** Food and Water Supply one step. [p. 139] | – |
| Carbonated Beverage | 1/4 | $2 | Water Supply +1 | Text ("Soda and candy bars"): increases your **Food Supply** one step. [p. 139] | – |
| Candy Bar | 1/4 | $2–$5 | Food Supply +1 | Increases your Food Supply one step. [p. 139] | – |
| Coffee | 0 | Free–$1.50/cup | STRESS LEVEL +1 | **STRESS LEVEL +1**; lack-of-sleep drawbacks are **put off for one Shift**. Free on most company ships/stations. [p. 139] | One Shift |
| Beer | 1/4 | $4 | STRESS LEVEL –1; WITS-based skills –1 | Each drink: **STRESS LEVEL –1** and **–1 to all WITS-based skills for the next Shift**. [p. 139] | Next Shift |
| Hard Liquor | 1 | $10–$500/bottle | Per glass: STRESS LEVEL –1; WITS-based skills –1 | As beer, **per glass**. [p. 137, 139] | Next Shift |
| Colony Specialty Meals | 1/4 | $20–$300 | Food Supply +1, STRESS LEVEL –1 | Increases Food Supply **and** reduces STRESS LEVEL one step. [p. 139] | – |

- Beer and booze are also "covered under entertainment in Chapter 7". [p. 139]
- AMBIGUOUS: Carbonated Beverage — table says Water Supply +1, text says Food Supply +1. Suggested: table (Water) as default.
- AMBIGUOUS: the –1 WITS penalties from multiple drinks stack (suggested yes, "for each glass"); each drink's penalty lasts for the Shift following it.
- AMBIGUOUS: whether food items are carried as items (weight 1/4) until consumed and then add +1 to the Supply rating, or are added to the Supply rating on purchase. Suggested: carried as items; "consume" action converts to Supply +1 (Supply then follows the p. 34 encumbrance table).
- AMBIGUOUS: Coffee "postpones drawbacks" — suggested: Exhausted effects (STAMINA roll, collapse, maybe the no-relieve-stress block) are suspended for one Shift.

---

## 7. VEHICLES

### 7.1 Vehicle table [p. 140]

| Vehicle | Passengers | Maneuverability | Speed | Hull | Armor | Cost | Ram base Damage (derived: ⌈Hull/5⌉) | Component-damage threshold (derived: ⌈Hull/2⌉) | Aerial (AMBIGUOUS, from text) |
|---|---|---|---|---|---|---|---|---|---|
| Daihotai Tractor | 4 | +0 | 2 | 6 | 4 | $17,000 | 2 | 3 | No |
| Weyland NR-9 ATV | 1 | +2 | 2 | 2 | 2 | $3,000 | 1 | 1 | No |
| M577 APC | 13 | +1 | 3 | 8 | 8 | $500,000 | 2 | 4 | No |
| Aerodyne Gyrocar | 5 | +3 | 3 | 4 | 3 | $40,000 | 1 | 2 | Yes |
| UD-4L Cheyenne VTOL Dropship | 15 | +2 | 4 | 10 | 7 | $3,100,000 | 2 | 5 | Yes |
| WY-37B Cargo Lifter Transport Sled | 3 | –1 | 3 | 12 | 4 | $800,000 | 3 | 6 | Yes |

Passengers exclude the driver. [p. 114]

### 7.2 Vehicle details (spec boxes and rule-relevant notes)

| Vehicle | Encountered | Top speed | Length | Standard armament | Rule-relevant notes | Page |
|---|---|---|---|---|---|---|
| Daihotai Series 8x8 (All Terrain Transport) Tractor | Frontier Colonies | 148 km/h | 6.6 m | None | Cabin enclosed and **pressurized**, self-contained environment: functions in **poisonous atmosphere or on an airless moon**. Modular attachments; can tow trailers; some colonists mount weapons. | [p. 140, 141] |
| Weyland NR-9 Series ATV | Frontier Colonies | 140 km/h | 3.2 m | None | Seats two people back-to-back (driver + 1) under a roll cage; tri-bogie treads. | [p. 141] |
| USCMC M577 Colonial Marines APC (M570 series) | USCMC battlefields and military bases across the Frontier and Outer Rim | 150 km/h | 9.2 m | Phased Plasma Pulse Cannon Turret; 20mm Gatling Gun Turret | **Amphibious and vacuum tight**; tactical command center linked to soldiers' helmet cams and biosigns; chaff launchers; laser-absorbent paint; carried in a Cheyenne's vehicle bay; variants with other weapon configurations exist; reckless driving on rocks risks the transaxle (fluff). | [p. 142] |
| Aerodyne Light VTOL Automotive Gyrocar | Densely populated Core and Outer Veil Colonies | 250 km/h | 5.2 m | None | Wheeled ground transport **or** air cushion flight, **maximum flight ceiling 3,000 m**; 360° cameras, proximity alert; high upkeep, rare on the Frontier. | [p. 142, 143] |
| USCMC UD-4L Cheyenne Dropship (UD-4 series VTOL tactical dropship) | USCMC battlefields/military bases, Frontier and Outer Rim; some held by mercenaries | – (not given) | 25.2 m | 25mm Gatling Gun; Mk.16 150mm Banshee 70 Unguided Rocket Launcher; **7 × AGM-220C Hellhound II Tactical Missiles**; **3 × AIM-90E Headlock Air-to-Air Missiles** | Carries an M577 APC and its squad; weapon pods retracted in supersonic descent, deployed for subsonic combat; can be **flown remotely from a portable terminal** with satellite uplink (cf. PR-PUT). | [p. 144, 145] |
| WY-37B Cargo Lifter Transport Sled (W-Y Suborbital Transport Sled & Crane) | Frontier colonies, star-freighters, large space stations | – (not given) | 34.2 m | None | Pressurized cab **rated for vacuum**; capable of **low-orbit cargo transfers** (not deep space); magnetizable flatbed plates; crew can exit on harness/winch in flight; remote-operated crane — **deploying the crane in flight can destabilize the Lifter and crash it** (GM: suggested PILOTING roll). | [p. 146] |

---

## 8. Gear bonus and supply quick index

**Skill modifications from gear** (positive = extra Base Dice):

| Item | Skill | Mod | Condition | Page |
|---|---|---|---|---|
| Weapons (Bonus column) | CLOSE COMBAT / RANGED COMBAT | per table | when attacking with it | [p. 118] |
| Seegson C-Series Tape Recorder | MANIPULATION | +1 | — | [p. 131, 132] |
| Samani E-Series Watch | SURVIVAL | +1 | — | [p. 131, 132] |
| Seegson System Diagnostic Device | COMTECH | +2 | troubleshooting/hacking systems | [p. 131, 132] |
| HoloTab | COMMAND (+ OBSERVATION per text) | +2 | — | [p. 131, 132] |
| Binoculars | OBSERVATION | +2 | LONG range or more, active scouting | [p. 133, 134] |
| Watatsumi DV-303 Bolt Gun (tool) | HEAVY MACHINERY | +2 | relevant situations | [p. 135] |
| Cutting Torch (tool) | HEAVY MACHINERY | +2 | relevant situations | [p. 135] |
| Maintenance Jack | HEAVY MACHINERY | +1 | relevant situations | [p. 135] |
| Electronic Tools | COMTECH | +1 | — | [p. 135] |
| Personal Medkit | MEDICAL AID | +2 | single use | [p. 135, 136] |
| Surgical Kit | MEDICAL AID | +1 | only to prevent death from a fatal critical injury | [p. 135, 136] |
| Weyland-Yutani APEsuit | SURVIVAL | +3 | while worn | [p. 127, 128] |
| P-5000 Power Loader | HEAVY MACHINERY, CLOSE COMBAT | +3 each | requires HEAVY MACHINERY ≥ 2 | [p. 127, 128] |
| IRC Mk.50 Compression Suit | AGILITY | –1 | while worn (AMBIGUOUS) | [p. 127] |
| Naproleve (overdose) | AGILITY-based skills | –1 per extra dose | until end of Shift | [p. 137] |
| Beer / Hard Liquor | WITS-based skills | –1 per drink/glass | next Shift | [p. 137, 139] |
| G2 grenade (target's roll) | STAMINA | –2 | to resist stun | [p. 125] |

**Fixed-skill devices** (roll their own skill level): Pauling MedPod MEDICAL AID 10; AutoDoc MEDICAL AID 6; UA 571-C Sentry Gun RANGED COMBAT 8 (no AGILITY); mainframes per 6.1. [p. 125, 130, 136]

**Items requiring a roll to use**: Computer Terminal (COMTECH), PR-PUT (COMTECH), Neuro Visor communication (COMTECH), Pauling MedPod programming (COMTECH +2), AutoDoc programming (COMTECH, unmodified). [p. 131, 132, 134, 136]

**Supply ratings on items**:

| Item | Supply | Max | When to roll | Page |
|---|---|---|---|---|
| Armat XM99A Phased Plasma Pulse Rifle | Power | 5 | after every shot | [p. 121] |
| Stun Baton | Power | 5 | after each attack | [p. 127] |
| Cutting Torch | Power | 5 | after every use | [p. 127, 135] |
| M314 Motion Tracker | Power | 5 | after every use (max once per Turn) | [p. 86, 134] |
| M316 Motion Tracker | Power | 3 | after every use (implied) | [p. 134] |
| IRC Mk.50 Compression Suit | Air | 5 | Air Supply roll every Turn + after strenuous activity [p. 35] | [p. 127, 128] |
| IRC Mk.35 Pressure Suit | Air | 4 | as above | [p. 127, 128] |
| Eco All-World Survival Suit | Air | 6 | as above | [p. 127, 128] |
| Weyland-Yutani APEsuit | Air | 4 | as above | [p. 127, 128] |
| Food & drink items | Food / Water | +1 per item | Food daily; Water daily + strenuous [p. 35] | [p. 137, 139] |

**Consumable item counts**: Personal Medkit (1 use), pharmaceutical doses, U-238 loaded weapons, missiles (Cheyenne 7 + 3), reloads per weapon [p. 96], grenades for U1.

---

## 9. Time-related rules

| Rule | Interval / timing | Page |
|---|---|---|
| Time units | Round 5–10 s; Turn 5–10 min; Shift 5–10 h | [p. 83] |
| Starving onset | after 1 day without sufficient food | [p. 106] |
| Starving STAMINA roll (fail: 1 dmg + Stress +1) | every day | [p. 106] |
| Starving Death Roll (if Broken) | every day | [p. 106] |
| Starving ends | within one Shift after eating | [p. 106] |
| Dehydrated onset | after 1 day without sufficient water; or any hypersleep without Hydr8tion | [p. 106, 151] |
| Dehydrated damage 1 + Stress +1 (automatic) | every Shift | [p. 106] |
| Dehydrated Death Roll (if Broken) | after every Shift without liquid | [p. 106] |
| Dehydrated ends | within one Shift after drinking | [p. 106] |
| Sleep requirement | ≥ 1 Shift of sleep each day | [p. 106] |
| Exhausted onset | after 1 day without sufficient sleep | [p. 106] |
| Exhausted STAMINA roll (–days without decent sleep) | each day, GM picks when; fail = sleep one Shift | [p. 106] |
| Exhausted ends | after ≥ 1 Shift of sleep | [p. 106] |
| Freezing STAMINA roll | above freezing: 1/day; sub-zero: 1/Shift; deep space: 1/Turn | [p. 107] |
| Freezing Death Roll (if Broken) | next time a cold roll is due | [p. 107] |
| Freezing ends | immediately when warm | [p. 107] |
| Vacuum STAMINA roll | every Round, before any action; mod –(Round–1) | [p. 107] |
| Vacuum Death Roll (after failing) | every Round | [p. 107] |
| Donning a space suit quickly | one slow action + MOBILITY roll | [p. 107] |
| Explosive decompression venting | air vents out in a Turn | [p. 107] |
| Decompression draft STAMINA roll | per action; counts as a fast action | [p. 107] |
| Fire burn attack | start of each new Round; Intensity +1 per Round | [p. 108] |
| Put out fire | slow action, MOBILITY | [p. 108] |
| Fire Death Roll (if Broken) | every Round | [p. 108] |
| Disease breakout (1 dmg) | one Shift after infection | [p. 109] |
| Sickness Roll | start of each Shift (Broken: after every Shift, fail = death) | [p. 109] |
| Radiation gain | Weak 1 Rad/Shift; Strong 1 Rad/Turn; Extreme 1 Rad/Round | [p. 110] |
| Radiation Death Roll (if Broken by radiation) | each additional Rad gained | [p. 110] |
| Rad healing (+ permanent check, 1 Stress Die) | 1 Rad per Shift after leaving the area | [p. 110] |
| U-238 loaded weapon carried | 1 Rad per Shift | [p. 123] |
| Drowning STAMINA roll | every Round (not an action, before your actions) | [p. 110] |
| Drowning Death Roll (if Broken) | each Turn (as printed) | [p. 110] |
| Suffocation STAMINA roll | every Turn or after every strenuous activity; cumulative –1 per roll | [p. 110] |
| Suffocation Death Roll (after failing) | every Round | [p. 110] |
| Air Supply roll | every Turn + after strenuous activity | [p. 35] |
| Water Supply roll | once per day + after strenuous activity | [p. 35] |
| Food Supply roll | once per day | [p. 35] |
| Synthetic repair | one Shift of work + COMTECH | [p. 111] |
| Synthetic reactivation after System Shutdown | a Turn of work + power source + COMTECH | [p. 111] |
| Synthetic crit 1 | loses its next slow action | [p. 111] |
| Enter/exit/mount vehicle | fast action (ERRATA p. 88) | [p. 88, 113] |
| Start motorized vehicle | DECIDED: slow action (p. 88 table); alternative: fast action (p. 113) | [p. 88, 113] |
| Drive | fast action; up to Speed zones; twice per Round | [p. 114] |
| Severe spin PILOTING roll | immediate, not an action | [p. 115] |
| Engine disabled (aerial) | controlled landing, –1 zone altitude per Round | [p. 115] |
| Vehicle hull repair | one HEAVY MACHINERY roll per Shift | [p. 115] |
| Vehicle component repair | one Shift + HEAVY MACHINERY roll | [p. 115] |
| Reload | slow action | [p. 96] |
| DV-303 Bolt Gun reload | slow action after each shot | [p. 119] |
| XM99A aim before firing | fast action | [p. 121] |
| ASSO-400 rappel to target | slow action (MEDIUM → ENGAGED) | [p. 121] |
| Flash grenade | fail STAMINA → lose next slow action | [p. 125] |
| G2 grenade / Stun baton | fail STAMINA → stunned for one Round | [p. 125, 127] |
| Motion tracker use | once per Turn | [p. 86] |
| Pups mapping device | scans one zone per Round | [p. 134] |
| Neversleep | no need for sleep for one day; no stress relief that day | [p. 137] |
| Naproleve overdose penalty | each extra dose in same Shift; until end of Shift | [p. 137] |
| Coffee | postpones lack-of-sleep drawbacks for one Shift | [p. 139] |
| Beer / liquor WITS penalty | the next Shift (per drink) | [p. 139] |
| Hydr8tion | taken before stasis | [p. 137, 151] |

---

## 10. State the app must track

### 10.1 Per character

- Health, Broken flag, STRESS LEVEL (owned by other spec files; referenced here) and **cause of Broken** (fire, radiation, dehydration, starvation, cold, vacuum, suffocation, drowning, sickness) because each cause sets its own Death Roll schedule. [p. 106–110]
- **Death Roll schedule**: active causes + interval (per Round / Turn / Shift / day / per Rad / next cold roll). [p. 106–110]
- **Conditions** (checkboxes): Starving, Dehydrated, Exhausted, Freezing. [p. 106]
  - Days without sufficient food; days without sufficient water; **days without decent sleep** (Exhausted modifier); hours/Shifts slept today.
  - Pending recovery timer ("wears off within one Shift") for Starving and Dehydrated.
  - Freezing environment band: none / above freezing / sub-zero / deep space; time of last cold roll.
- **Sickness**: sick flag, disease Virulence, time of infection, breakout applied, caretaker (MEDICAL AID roller), special disease effects. [p. 109]
- **Radiation**: temporary Rads, **permanent Rads**, current area Radiation Level (none/weak/strong/extreme), in-hotspot flag, Broken-by-radiation flag. [p. 110]
- **Vacuum exposure Round counter**; **suffocation roll counter**; underwater flag; on-fire flag with **current fire Intensity**. [p. 107, 108, 110]
- Decompression draft flag (needs STAMINA per action). [p. 107]
- **Stunned** (until end of next Round), **loses next slow action** flags (flash grenade, synthetic crit 1). [p. 111, 125, 127]
- **Aim state** (for XM99A mandatory aim, optical scope). [p. 94, 121, 134]
- Grappled/tethered by ASSO-400 (target link). [p. 121]
- **Drug/food effects**: Naproleve doses this Shift (and resulting AGILITY penalty), Neversleep active-until, Coffee postponement-until, alcohol WITS penalties with expiry, X-drug custom effects. [p. 137, 139]
- **Supplies**: Air, Food, Water, Power ratings (per character or per group; Air/Power normally per suit/item). [p. 34]
- **Encumbrance**: sum of item weights (0, 1/4, 1/2, 1, 2, 3), food/water supply weight, limit 2 × STRENGTH, over-encumbered up to 4 × STRENGTH. [p. 34]
- **Worn armor** (max one suit). [p. 98]
- **Synthetic-only**: is_synthetic flag (no stress, no push, no panic, no Death Rolls, immune to vacuum/cold/disease, no air/food/water/sleep); list of synthetic crits held (D6 results), System Shutdown flag, reactivated flag, permanent mental trauma flag; behavioral inhibitor status (NPC). [p. 111]

### 10.2 Per item (equipment manager)

- Item type/category (pistol, rifle, heavy, close combat, vehicle weapon, suit/armor, mainframe, data storage, diagnostics, vision, tool, medical, pharmaceutical, food/drink), name, weight (printed + numeric), cost (min/max or "varies"), page.
- Weapon stats: Bonus, Damage (number / Blast N / Stun), Range, features: armor_piercing, armor_doubled, full_auto, single_shot, fire_intensity, blast_power, stun_effect (+ modifier), power_supply_max, special targeting restrictions (ground-only / air-only). ERRATA (pp. 119–124): no alternate stat-card values; table values are official.
- **Loaded state** (single-shot needs reload; magazine emptied after a facehugger), **full reloads carried**, **ammo type** (normal / U-238 dirty bullets), U1 grenade inventory by type (frag, smoke, flash, G2). [p. 96, 119, 123, 125]
- **Current Power Supply** (XM99A, stun baton, cutting torch, M314, M316) and **current Air Supply** (suits) with max values. [p. 121, 127, 128, 134]
- Skill modification(s) granted and their condition (see §8); fixed skill levels for devices.
- Uses remaining (Personal Medkit = 1), dose counts (pharmaceuticals), quantity (food/drink, grenades, missiles).
- Equipped/worn flag (armor: one at a time); mounted accessories (optical scope on a weapon, M316 on a pulse rifle).
- Sentry gun: activated flag, mode (ARS friend/foe vs. motion/heat only), remote-control link (Head-Mounted Sight), location/zone. [p. 125]
- Device sync links (Head-Mounted Sight ↔ smart gun/sentry, P-DAT ↔ PDTs/helmet cams, Pups ↔ HoloTab). [p. 132, 134]
- Power loader: operator skill check (HEAVY MACHINERY ≥ 2). [p. 128]

### 10.3 Per vehicle

- Type, Passengers capacity, Maneuverability, Speed, Hull, Armor, Cost, aerial flag. [p. 140]
- Driver, passengers (≤ capacity), crew assigned to each mounted weapon. [p. 114]
- **Damage taken** (HP = Hull – damage), **Wrecked** flag, **component damage count** (0/1; second → Wrecked), **engine disabled**, **disabled weapons** list, **destroyed beyond repair** (fuel explosion). [p. 114, 115]
- **Altitude** in zones (aerial), landing-in-progress (engine disabled). [p. 115]
- Started/running flag (DECIDED: starting = slow action; fast action as a documented alternative setting). [p. 88, 113]
- Mounted weapons with ammo/missile counts (Cheyenne: 7 Hellhound, 3 Headlock). [p. 145, 147]
- Repair progress: last repair roll Shift, component repairs outstanding. [p. 115]
- Environmental sealing (pressurized/vacuum-tight: Daihotai, M577, WY-37B cab). [p. 140, 142, 146]

### 10.4 Per scene / location (needed to drive the hazard rules)

- Zone pressurization / vacuum, breached compartments and seal status; bulkhead Armor Rating (default 6 civilian). [p. 107]
- Temperature band (for Freezing), radiation level per zone/hotspot, underwater zones, active fires (Intensity), darkness (flashlight), smoke-blocked zones. [p. 107, 108, 110, 125, 133]
- Active disease definitions (Virulence, unique effects). [p. 109]

---

## 11. Ambiguities / Game Mother discretion

Book-internal contradictions (table vs. stat card / text):
1. ERRATA (resolved): **M4A3 Service Pistol** Bonus = table **+2**. Card +1 was a misprint. [p. 119]
2. ERRATA (resolved): **Armat M41A Pulse Rifle** Cost = table **$800**. Card $1,200 was a misprint. [p. 120, 122]
3. ERRATA (resolved): **M42A Scope Rifle** = table **Bonus +2 / Damage 2**. Card +1 / 3 was a misprint. [p. 120, 122]
4. ERRATA (resolved): **Armat M41AE2 Heavy Pulse Rifle** = table **Bonus +1 / Damage 3 / Cost $1,500**. Card +2 / 2 / $1,200 was a misprint. [p. 124]
5. **XM99A** card omits "Power Supply 5" (text and table include it). [p. 120, 121, 123]
6. **IRC Mk.50** "AGILITY –1" appears only in the table; unclear if attribute or AGILITY-based rolls; the Mk.35 text describes the bulky suit. [p. 127, 128]
7. **HoloTab**: table COMMAND +2; text COMMAND and OBSERVATION +2. [p. 131, 132]
8. **Carbonated Beverage**: table Water Supply +1; text (soda) Food Supply +1. [p. 137, 139]
- ERRATA (resolved): entering/exiting a vehicle = fast action (p. 88 table corrected to match p. 113). [p. 88, 113]
- DECIDED: starting a vehicle engine: p. 88 action table = slow, p. 113 = fast → **slow action**; fast action kept as a documented alternative. [p. 88, 113]

Interpretations (not stated in the book):
9. Starving/Dehydrated "after a day" — start the day count when Food/Water Supply hits 0 (suggested). [p. 106]
10. "Effects wear off within one Shift" — effects end at the end of one Shift after eating/drinking (suggested). [p. 106]
11. Exhausted first-day modifier = –1; collapsing for a Shift ends Exhausted (suggested). [p. 106]
12. Freezing while Broken: the Death Roll replaces the cold STAMINA roll at each later interval (suggested). [p. 107]
13. Vacuum: success = no effect that Round; MEDICAL AID does not stop vacuum Death Rolls (suggested). [p. 107]
14. Explosive decompression: penetration if armor successes < base Damage; weapon armor features apply to bulkheads; STAMINA roll uses the fast action, success allows the slow action (suggested). [p. 107]
15. Explosions: armor roll applies (not stated); ENGAGED +1 applies even with 0 successes (suggested). [p. 108]
16. Fire: Intensity +1 applies from the Round-2 burn attack; "no damage" measured after armor; M240 target always ignites on a hit, first burn at start of next Round (suggested). [p. 108, 125]
17. Disease: Virulence = Base Dice rolled for the disease in the opposed roll; timeline of first Sickness Roll after breakout (suggested). [p. 109]
18. Radiation: dice are Base Dice; permanent Rads count toward the dice pool; no armor roll (suggested). [p. 110]
19. Synthetics and radiation: not listed among synthetic immunities (GM). [p. 111]
20. Drowning Death Roll printed "each Turn" (others per Round) — implement as printed, GM may switch to Round. [p. 110]
21. Synthetic crits: keep shifting down if the shifted result is also held; cap at 6; reactivation required before repair after System Shutdown; self-repair while immobilized is GM call. [p. 111]
22. Vehicles: component-damage threshold vs. damage after armor; second component damage = Wrecked (table roll optional); personal armor vs. Driver/Passenger hit; rammer takes no damage; "same damage as the vehicle did" = damage of the wrecking hit; component counter resets on repair; Wrecked flag from component damage clears on component repair (all suggested). [p. 114, 115]
23. Which vehicles are aerial (Gyrocar, Cheyenne, WY-37B suggested); armor vs. crash damage (not stated). [p. 115, 140–146]
24. Vehicle weapon operators: one operator per weapon per Round, using their RANGED COMBAT; missiles are finite ammo (suggested). [p. 114, 145, 147]
25. "Single-shot" on the ASSO-400 means reload (slow action) after each shot, like the DV-303 (suggested). [p. 119, 120]
26. XM99A mandatory aim also grants the normal +2 aim bonus (suggested). [p. 121]
27. U1 frag: whether the Effect Radius (MEDIUM at Blast 3) applies beyond the target zone (GM). [p. 108, 125]
28. Sentry Gun: +2 Bonus added to the 8 skill dice (suggested) or 8 all-inclusive (GM). [p. 124, 125]
29. "Stunned for one Round" undefined — suggested: lose all actions in the next Round. [p. 125, 127]
30. G2 affects only humans (as written); synthetics/Xenomorphs unaffected unless GM rules otherwise. [p. 125]
31. Stun baton Power Supply roll after every attack, hit or miss (suggested). [p. 127]
32. Surgical Kit as weapon: Bonus –, Range Engaged (suggested; only Damage 2 given). [p. 136]
33. Weight "–" on non-weapons = installed/not carriable (suggested). [p. 118]
34. U-238: Armor cannot drop below 0; the Rad from a hit triggers the normal radiation roll (suggested). [p. 123]
35. No battery recharge / air refill / gear breakage / availability rules in scope — GM decides. [p. 118–147]
36. APEsuit "resistance to temperature extremes" may waive Freezing (GM); Mk.35 decompression time unquantified (GM). [p. 128]
37. P-5000 armor does not stack with worn armor (one suit at a time) (suggested). [p. 98, 128]
38. Mainframe skill values rolled as dice pools (suggested). [p. 130]
39. M316 needs a Power Supply roll after every use like the M314 (suggested). [p. 134]
40. Optical scope cannot raise a weapon beyond EXTREME. [p. 134]
41. AutoDoc rolls its skill level only (6 dice), like the MedPod (suggested); "complicated surgery" scope is GM call. [p. 136]
42. Naproleve AGILITY penalty stacks per extra dose (suggested). [p. 137]
43. X-drugs, recreational drugs, "other diseases": GM-defined effects. [p. 109, 137]
44. Alcohol WITS penalties stack per drink/glass (suggested). [p. 139]
45. Food items carried as items until consumed, then add +1 to the Supply rating (suggested). [p. 137, 139]
46. Coffee suspends Exhausted drawbacks for one Shift (suggested). [p. 139]
47. Deploying the WY-37B crane in flight: risk of destabilizing/crashing — suggested PILOTING roll, GM. [p. 146]
48. Price of individual U1 grenades (other than G2 $400) not given — use 5% reload rule or GM. [p. 124]
