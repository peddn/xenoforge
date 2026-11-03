# 05 — Travel, Ships, Ship Design & Space Combat

> Official errata and project decisions in [00-errata-und-festlegungen.md](00-errata-und-festlegungen.md) take precedence over this file.

Source: ALIEN RPG Core Rulebook (Free League, 1st ed. 2019). All page references are **BOOK pages** (PDF page − 4).
Scope: FTL travel & hypersleep (p. 150–151), mechanical rules from "Living in space" (p. 152–165), Spacecraft chapter (p. 166–179), Spaceships of 2183 AD (p. 180–187), Space Combat (p. 188–203), plus cross-referenced rules from elsewhere in the book that the ship / space-combat modules need (AI mainframes p. 130, talents p. 76–79, campaign travel encounters p. 327–352, ship sheet p. 388).

All stat tables in this file were checked against rendered page images.

Conventions in this file:
- **success** = a six on any die (Base Die or Stress Die). **facehugger** = a one on a Stress Die. (The book prints these as icons; they're dropped from the text extraction.)
- "AMBIGUOUS:" marks my interpretation or a gap or contradiction in the book. Every one of these is listed again in §8 with an ID (A-nn).
- A modifier (e.g. Thrusters +1, Bonus +2) is a **modification**: +n adds n Base Dice and −n removes n Base Dice. If you run out of Base Dice, remove Stress Dice. With 0 dice left, the action can't succeed. [p. 62]

---

## 0. Core mechanics this module relies on (cross-reference only)

| Rule | Summary | Page |
|---|---|---|
| Skill roll | Attribute + skill Base Dice + Stress Dice (= STRESS LEVEL). Needs ≥1 success. | p. 58–61 |
| Modification | ±n Base Dice. Negative modifiers remove Stress Dice once Base Dice run out. At 0 dice the roll can't succeed. | p. 62 |
| Help from others | Declare before rolling. +1 per helper, max 3 helpers (max +3). In combat, helping counts as the same type of action. | p. 63 |
| Opposed roll | To win you need ≥1 success **and** more successes than the opponent. Each opponent success cancels one of yours. Only the active party (attacker) can push. | p. 64 |
| Pushing | Re-roll the dice that didn't show success. STRESS LEVEL +1. A facehugger on a Stress Die triggers a Panic Roll and means you can't push. NPCs never push. | p. 60–62, 104 |
| Panic Roll | D6 + current STRESS LEVEL. In space combat use the **Space Combat Panic Roll** table (§4.19). | p. 104, 193, 201 |
| Initiative cards | Cards numbered 1–10, drawn once at the start of the conflict. Lowest acts first. The order is kept for the whole conflict. | p. 87 |
| Time units | Round = 5–10 seconds (combat). **Turn = 5–10 minutes**. **Shift = 5–10 hours**. Typically 4 Shifts per day. | p. 83 |
| Weapon "Range" | The maximum range category at which the weapon can be used. Ship armaments use the personal-weapon rules on a different scale. | p. 94, 118, 176 |

---

## 1. TRAVEL

### 1.1 FTL travel time [p. 150, 167, 170]
- Every FTL-capable craft has an **FTL rating** = the number of **days to traverse one parsec** on the star map. Lower is faster. [p. 167, 170]
- "Presently, no existing ship has an FTL rating lower than 1." → **minimum FTL rating 1**. [p. 167]
- **Travel time (days) = FTL rating × parsecs.** Example: rating 6, 9 parsecs → 54 days. [p. 167]
- Reaching FTL speed isn't instantaneous: ion thrusters build speed until the displacement drive takes over. This is flavour only, with no numbers given. [p. 150]
- **Tractor Hitch** towing external cargo: the FTL rating is **doubled** (6 → 12). [p. 171]
- Ranges stated for small craft: the Starcub has a maximum range of **13 parsecs** before refuelling [p. 183]. The Class C EEV Type 337 has an FTL range of **1.4 parsecs** [p. 173]. Most other EEVs can't go FTL [p. 173].
- Class C ships have FTL "–" in the class table, i.e. no FTL capability by default [p. 166]. AMBIGUOUS (A-01): the Starcub (a C class) nevertheless has FTL rating 15 and no upgrade that explains it [p. 183].
- Speed claims (50 to over 700 × FTL for modern ships) are flavour. The only mechanical speed measure is days/parsec. [p. 150]

### 1.2 Overcharging engines [p. 167]
1. The engineer makes a **HEAVY MACHINERY** roll, which takes **one Shift** of work.
2. **Success** → the ship's FTL rating is **decreased by one step (−1) for one week** (6 → 5).
3. **Failure** → the ship suffers a **reactor leak** (minor component damage result "Reactor leak", D66 53–54, p. 199). The engineer must make **another immediate HEAVY MACHINERY roll**.
4. If that also fails → **reactor breach** (major component damage, 2D6 result 10, p. 199), and the engineer must make a **third HEAVY MACHINERY roll**.
5. Third roll fails too → **reactor explodes: ship destroyed, everyone aboard dies**.
- **Limit: once per week** per engineer (the book ties the limit to the engineer).
- AMBIGUOUS (A-02): if the 2nd or 3rd roll succeeds, the escalation stops. The book doesn't say that a later success grants the −1 FTL bonus, so assume it doesn't. It also says "an engineer may only attempt…once per week", which reads as a per-engineer limit, not a per-ship one. Assume per ship unless the GM decides otherwise.

### 1.3 Planetfall [p. 167, 178]
Only ships with the **Planetfall Capacity** upgrade can do this.
1. Planetfall takes **about one Shift** and needs a **PILOTING** roll.
2. Roll fails → **minor component damage** (D66, p. 198). The pilot may now **abort** and try again **next Shift**, or **continue**.
3. Continue → **second PILOTING** roll. Fails → **major component damage** (2D6, p. 199). If the result is **Engine failure** or **Reactor breach**, **the ship crashes** ("hurry to those EEVs!").
4. If the ship is still operable, the pilot again chooses abort or continue. Continue → **third PILOTING** roll. Fails → **the ship crashes hard, killing all on board**.
- AMBIGUOUS (A-03): the box doesn't say whether Thrusters modify planetfall rolls. p. 170 makes Thrusters a general PILOTING bonus, and p. 167 applies the Thrusters modification to ship maneuvering. Recommendation: apply Thrusters.
- AMBIGUOUS (A-04): it's unclear whether success on the 1st (or 2nd) roll ends the landing successfully. The structure implies the first successful roll lands the ship. The damage/crash consequences of a "crash" (other than the third-failure death) aren't given.

### 1.4 Hypersleep and Neurological Distortion Disorder (NDD) [p. 151]
- **Stasis protects travellers from NDDs** and arrests aging. [p. 151]
- **NDD roll trigger:** after a number of **days of FTL travel equal to the ship's FTL rating**, roll **D6** on the NDD table. Each time the same number of days passes again, roll again with a cumulative **+1** per roll: roll *n* = D6 + (n − 1). [p. 151]
  - AMBIGUOUS (A-05): the book implies that only characters **awake** (not in stasis) during FTL travel roll. It doesn't say whether a new result replaces an older effect, or how long Epilepsy lasts (assume until the FTL travel ends, like Paranoia).
- **Rude awakening** (abruptly woken from hypersleep): counts as an **attack with six Base Dice** against you (each success = 1 point of damage, as for attacks). If this damage leaves you **Broken**, make a **Death Roll every Turn**. [p. 151]
- **Any length of hypersleep makes you Dehydrated** (Dehydrated rules p. 106). **Hydr8tion** taken **before** stasis counteracts this. Hydr8tion costs $5 [p. 136] and has no side effects [p. 137]. [p. 151]

**NEUROLOGICAL DISTORTION DISORDER** [p. 151]

| Roll | Effect |
|---|---|
| ≤5 (printed "–5") | **No effect.** Yet. |
| 6–7 | **Paranoia.** Convinced someone or something on board is out to get you. Lasts until the FTL travel ends. Roleplayed. |
| 8–9 | **Epilepsy.** Each **Shift** roll a **Stress Die**; on a **facehugger** you have a seizure and are out of action for one whole **Turn**. |
| 10–11 | **Dementia.** Can't recall who you or the other characters are. Lasts **D6 days after the FTL travel ends**. Roleplayed. |
| 12+ | **Psychosis.** You at once attack the closest person or creature, friend or foe, until one of you is **Broken**. Each friendly character who sees it makes an immediate **Panic Roll**. |

### 1.5 Travel encounters (campaign chapter, cross-reference) [p. 351–352]
- Roll on **Star System Encounters** (D66) **before going into hyperspace** and **again after coming out** (before landing or docking). Ships lingering in a system roll **once per day** near a habitable planet, **once per week** further out, and **once per month** in unexplored systems. [p. 351]
- Modifiers to the **tens die**: **Outer Rim or Frontier system −3**. **Uncharted Space −5**. A result of **10 or less = no encounter**. [p. 351]

**STAR SYSTEM ENCOUNTERS** [p. 352]

| D66 | Encounter | D66 | Encounter |
|---|---|---|---|
| 00–10 | None | 41 | Military Missile Cruiser |
| 11 | Research Vessel | 42 | Tugship Not Currently Towing |
| 12 | Asteroid Drill Ship | 43 | Heavy Shuttle |
| 13 | Small Survey Ship | 44 | Fast Courier Vessel |
| 14 | Colony Supply Vessel | 45 | Modular Cargo Transport |
| 15 | Military Recon Craft | 46 | Large Space-Station |
| 16 | Private Executive Transport | 51 | Light Tugship |
| 21 | Hypersleep Transport | 52 | In-System Shuttle |
| 22 | Light Shuttle | 53 | Container Ship |
| 23 | Bulk Freighter | 54 | Communications Relay Station |
| 24 | Liquefied Gas Transport | 55 | Military Dropship In Orbit |
| 25 | Tugship Towing A Platform | 56 | In-System Shuttle |
| 26 | Bulk Freighter | 61 | Customs Cutter |
| 31 | Container Ship | 62 | In-System Shuttle |
| 32 | Colony Supply Vessel | 63 | Military Assault Ship |
| 33 | Petroleum Carrier | 64 | Corporate Space-Station |
| 34 | Salvage Ship | 65 | Private Security Cutter |
| 35 | Military Patrol Craft | 66 | Mobile Construction Rig |
| 36 | Emergency Response Vessel | | |

**SHIP REACTIONS** (2D6) [p. 352]

| 2D6 | Reaction |
|---|---|
| 2 | **Unusual.** Unusual trajectory, registration of a missing ship, or refuses to respond to the PCs' signals. |
| 3–4 | **Dismissive.** Declines requests for information or assistance. |
| 5–8 | **Radio silence.** Pays the PCs' ship no attention; courteous when hailed. |
| 9–10 | **Friendly.** May pass on information or advice, if relevant. |
| 11 | **Assistance.** May ask for info, help, a spare part or similar. |
| 12 | **Familiar vessel.** You know this ship, maybe its crew too. |

None of the encounter ships have stat blocks (see §5).

### 1.6 "Living in space": the only mechanical content [p. 152–165]
Everything else in p. 152–165 is setting text without rules. That includes Gravity Drives (p. 152, no numbers), charted-space regions (p. 152, which feed the encounter modifiers in §1.5), stations, communications, media, religion and law enforcement.

**LIVING EXPENSES** (weekly) [p. 160]

| Weekly cost | Living standard |
|---|---|
| $25 | Minimum |
| $100 | Basic |
| $300 | Normal |
| $2,000 | Luxurious |

**WORK FOR HIRE** (weekly salary range) [p. 162]

| Occupation | Min | Max | Includes |
|---|---|---|---|
| Colonial Labor | $500 | $640 | Miners, prospectors, farmers, drivers, mechanics, service personnel |
| Commercial Employees | $400 | $960 | Space truckers, officers, technicians, cargo handlers, pilots, navigators, medical and science officers |
| Corporate and Government Officials | $540 | $1,200 | Administration, execs, secretaries, sales persons, auditors, hitmen, agents |
| Law Enforcement | $400 | $700 | Colonial Marshals, federal investigators, colonial guard and security |
| Military Officers and Enlisted Personnel | $500 | $760 | Army, Marines, Navy (any role) |
| Elite Occupations | $1,300 | $20,000 | CEOs, chairpersons, entertainment professionals, dreamers, scientists, doctors, elite assassins, governors, high-ranking military |

- Currencies: UA dollars, 3WE yen, UPP yuen. Bills come in 5, 10, 20, 100, 1,000 and 10,000 (no singles). Corporate notes: W-Y dollars, Seeg bills, Binat coins. **No exchange rates are given** ("always in flux"; W-Y dollars hold their value). [p. 161]
- Luxury accommodations hold supplies for up to **two years**. This is flavour. [p. 159]

---

## 2. SHIPS

### 2.1 Spaceship attributes [p. 167, 170]

| Attribute | Meaning | Observed range (class table + prebuilt ships) | Page |
|---|---|---|---|
| **FTL Rating** | Days to travel one parsec. Lower = faster. Min 1. "–" = no FTL drive. | 2 … 20, or "–" (C class) | p. 166–167 |
| **Signature** | How hard enemies find it to lock their sensors on to the ship. Used as a modification to enemy **Target Lock** rolls ("Target Ship Signature: Varies", p. 192), so higher = easier to lock. | −1 … +2 (a custom ship can go to −1 − 2 = −3 via Stealth) | p. 167, 190–192 |
| **Thrusters** | Modification to your **PILOTING** rolls when maneuvering the ship. "–" in the class table = +0 (confirmed: M-class Bison +0; Star Clipper = M "–" + 1 Overpowered = +1). | −1 … +2 (custom up to base + 2) | p. 166–167, 170, 193 |
| **Hull** | How much damage the ship can take before it becomes **Disabled**. It also sets the component damage thresholds. | 2 … 12 (custom up to base + 3) | p. 167, 170, 197 |
| **Armor (Armor Rating)** | Number of dice rolled to reduce damage, as for body armor. | 4 … 10 (custom up to base + 3) | p. 167, 170, 197 |
| **Armaments** | Number of armament hardpoints of each size. | see class table | p. 167, 176 |
| **Modules** | Number of internal modules of each size. | see class table | p. 167, 170 |
| Crew / Length / Cost | Typical values per class. "Individual ships can vary greatly." | | p. 166 |
| Other sheet fields | Name, Manufacturer, AI, Passengers, Lease cost, Damage, Upgrades, component damage, Ship's Log | | p. 388 |

### 2.2 Ship classes [p. 166]
- Commercial craft are tracked by model and **weight class**. **A–G classes** are light and relatively cheap. **Class M** are the workhorses of the UA and 3WE. [p. 166]
- Every spaceship has **core components**, **internal modules** (optional, removable/replaceable), **armaments** (offensive weapons and defensive countermeasures) and **upgrades**. [p. 166]

**SHIP CLASSES** [p. 166] (verified against the image)

| Class | Length | Crew | FTL | Signature | Thrusters | Hull | Armor | Armaments (hardpoints) | Modules (slots) | Cost |
|---|---|---|---|---|---|---|---|---|---|---|
| C | 15m | 1–2 | – | −1 | +2 | 2 | 4 | – (none) | 2 × size II, 4 × size I | $2,000,000 |
| G | 50m | 3–6 | 20 | +0 | +1 | 5 | 5 | 1 × size II, 2 × size I | 3 × size III, 5 × size II, 7 × size I | $9,000,000 |
| M | 300m | 7–10 | 12 | +1 | – (=+0) | 9 | 6 | 1 × size III, 2 × size II | 4 × size IV, 6 × size III, 8 × size II | $28,000,000 |
| R | 700m | 12–18 | 8 | +2 | −1 | 12 | 7 | 2 × size III, 4 × size II | 5 × size V, 7 × size IV, 9 × size III | $400,000,000 |

- Only C, G, M and R have stats. Classes A, B, D, E, F, H–L appear only in the Hangar capacity table (p. 172). AMBIGUOUS (A-06): the designer can only build C/G/M/R. Other classes have no stats.
- Slot counts are **per size** and separate (e.g. a G-class has 15 module slots: 3 × III + 5 × II + 7 × I). This is confirmed by the prebuilt ships fitting (see §3.8).

### 2.3 Core components [p. 170]
Core components "cannot easily be replaced, but they can be upgraded". They're present on almost every ship and aren't bought separately (no prices).

| Component | Rule content | Related stat / upgrade |
|---|---|---|
| Hull | Structural strength = **Hull rating** → damage before Disabled (p. 197). | Hull. Reinforced Frame. |
| Armor | Exterior bulkheads. **Armor Rating** works like armor on vehicles or personnel. Railguns slice through any armor. | Armor. Armored Bulkheads. |
| Bridge | Controls sensors, communications and engines. The commander and pilot are stationed here. | Major damage "Bridge exposed to vacuum". |
| Sensors | Passive (optical, IR, radio) and active (radar). Used for Target Lock (p. 190–192). | Enhanced Sensors. Sensor damage results. |
| Communications array | Interstellar FTL and intrasystem comms. | Minor damage "Comms malfunction". |
| Reactor | Fusion. Military: lithium hydride. Civilian: deuterium/tritium. | Reactor leak / breach / detonation. Reactor Overload. Overcharging. |
| Sublight thrusters | Rocket motors. **The Thrusters rating functions as a bonus to PILOTING.** | Thrusters. Overpowered Thrusters. Thruster damage. |
| Displacement drives | FTL (tachyon shunt hyperdrive). FTL rating = days per parsec. | FTL. Boosted Displacement Drives. Drive malfunction / failure. |

### 2.4 Internal modules [p. 170–172]
- Modules come in **size categories I–V**. Size I ≈ a personal vehicle. Size V ≈ a large building. [p. 170]
- **Module slots per size are capped by class** (class table). **A module fits a slot of its own size or larger, never a smaller one.** [p. 170]
- Modules can be removed and replaced (p. 166). Fitting needs a dry dock (§2.8).
- "More ship modules will be described in future supplements." [p. 170]

Module effects [p. 171]:
- **Artificial Intelligence**: computer mainframe that can run the ship while the crew is in cryosleep. Price "varies" = the mainframe price (MU/TH/UR / A.P.O.L.L.O. table, §2.6). It can fill any crew position in combat (p. 188). It can do the COMTECH service but not the HEAVY MACHINERY service (p. 179).
- **Air Scrubbers**: breathable air. Capacity (people supplied) depends on size.
- **Cargo Bay**: internal hold with external loading doors. Larger bays have H-frame cranes. Max load depends on size.
- **Corporate Suite**: luxury area (wet bar, wall safe, some with a personal escape pod). No mechanical effect.
- **Cryo Deck**: cryo chambers. Capacity depends on size (hypersleep p. 151).
- **Docking Umbilical**: airlock plus extendable tunnel for docking with other ships. **Without one, crew can only board or disembark via a hangar: the ship lands in another's hangar, or smaller ships land in its own.** Also called "Docking Station" (p. 193, p. 198).
- **Emergency Escape Vehicle (EEV)**: ICC-sanctioned craft are **required by law** to carry EEVs for the **entire crew complement**. This is "not always enforced" (§2.5).
- **Galley**: kitchen, refrigeration, coffee maker. Capacity depends on size.
- **Hangar**: lets smaller ships land inside, with airlocks. The size sets the class of ship accommodated.
- **Medlab**: infirmary/lab with surgical kits, medkits and drugs. Install an AutoDoc ($500,000, MEDICAL AID 6) or Pauling MedPod ($2,000,000, MEDICAL AID 10) here [p. 135].
- **Salvage Crane**: salvages wrecks and space junk into the cargo hold or onto the tractor hitch. No numbers.
- **Science Lab**: **+2 modification to OBSERVATION rolls when using the Analysis talent.**
- **Tractor Hitch**: connect huge external cargo modules (over a million tons, e.g. gas tanks, refineries, atmospheric processors). **While towing, the FTL rating is doubled.**
- **Vehicle Bay**: garage with loading ramp. Capacity depends on size.

**INTERNAL MODULES** [p. 172] (verified against the image)

| Module | Size | Capacity / Type | Price |
|---|---|---|---|
| Artificial Intelligence | I | — | varies (see §2.6) |
| Air Scrubbers | I | Up to 10 people | $50,000 |
| Air Scrubbers | II | Up to 50 people | $200,000 |
| Air Scrubbers | III | Up to 500 people | $1,200,000 |
| Air Scrubbers | IV | Up to 2,500 people | $3,000,000 |
| Cargo Bay | I | 500 kg | $10,000 |
| Cargo Bay | II | 10 tons | $25,000 |
| Cargo Bay | III | 250 tons | $100,000 |
| Cargo Bay | IV | 5,000 tons | $250,000 |
| Cargo Bay | V | 100,000 tons | $4,000,000 |
| Corporate Suite | II | — | $1,000,000 |
| Cryo Deck | I | 1 person | $50,000 |
| Cryo Deck | II | Up to 10 people | $200,000 |
| Cryo Deck | III | Up to 50 people | $2,000,000 |
| Cryo Deck | IV | Up to 500 people | $15,000,000 |
| Cryo Deck | V | Up to 2,500 people | $50,000,000 |
| Docking Umbilical | II | — | $300,000 |
| Emergency Escape Vehicle | I | Class A Ejection Pod | $100,000 |
| Emergency Escape Vehicle | II | Class B EEV Type 20 Series | $250,000 |
| Emergency Escape Vehicle | II | Class C EEV Type 337 FTL | $500,000 |
| Emergency Escape Vehicle | III | Class D Lifeboat Module | $1,500,000 |
| Galley | I | 1 person | $20,000 |
| Galley | II | Up to 10 people | $50,000 |
| Galley | III | Up to 50 people | $125,000 |
| Galley | IV | Up to 500 people | $500,000 |
| Galley | V | Up to 2,500 people | $3,000,000 |
| Hangar | I | Class A Ship | $100,000 |
| Hangar | II | Class B Ship | $250,000 |
| Hangar | III | Class C–D Ship | $400,000 |
| Hangar | IV | Class E–G Ship | $800,000 |
| Hangar | V | Class H–M Ship | $1,200,000 |
| Medlab | II | — | $250,000 |
| Salvage Crane | III | — | $100,000 |
| Science Lab | III | — | $750,000 |
| Tractor Hitch | III | — | $600,000 |
| Vehicle Bay | I | ATV | $75,000 |
| Vehicle Bay | II | VTOL Gyrocar | $150,000 |
| Vehicle Bay | III | APC, Daihotai Tractor | $250,000 |
| Vehicle Bay | IV | 10 × APC | $800,000 |
| Vehicle Bay | V | 100 × APC | $5,000,000 |

Notes:
- There's no Air Scrubbers V. Single-size modules: AI (I), Corporate Suite (II), Docking Umbilical (II), Medlab (II), Salvage Crane (III), Science Lab (III), Tractor Hitch (III).
- AMBIGUOUS (A-07): Hangar capacity: the text ties the number of ships held to hangar size, but the table gives only a ship class. Assume 1 ship of up to that class (and presumably smaller classes).
- AMBIGUOUS (A-08): "EEV II" in stat blocks doesn't say Class B ($250,000) or Class C ($500,000). Bison H lists "EEV II × 2 (Starcub Shuttles)". The Starcub is a C-class ship, which per the Hangar table would need a size III hangar.
- Capacity checks (crew/passengers vs Air Scrubbers, Cryo Deck, Galley, EEVs) aren't stated as hard rules. AMBIGUOUS (A-09): the app should **warn**, not block.

### 2.5 EEVs [p. 173]
In an emergency MU/TH/UR dumps flight-recorder data to the EEV, locks on to the nearest homing beacon and auto-navigates away. Navigation can be overridden. Apart from the 337 and shuttles with a tachyon shunt, EEVs can't go FTL. EEVs ejected inside an atmosphere attempt a soft landing "with limited success".

| EEV | Module size | Capacity | Length | Rules |
|---|---|---|---|---|
| Class A Ejection Pod | I | 1 | 2m | Ejection **in space only**. Limited thrust. Semi-stasis **up to a week**. |
| Class B EEV Type 20 Series | II | 2–4 | 6–8m | Minimal thrusters. No cryosleep. Sustains occupants **ten days**. **Cannot land. Burns up on re-entry.** |
| Class C EEV Type 337 FTL | II | 5 | 14m | MU/TH/UR can transfer cryo capsules and auto-eject. Sustains a crew in stasis **50+ years**. **FTL range 1.4 parsecs.** Controlled landing possible. |
| Class D Lifeboat Module | III | 2–20 | 23m | 7 configurable rooms. **3 active occupants for 2 years, or up to 20 in cryosleep for 50 years.** Can make planetfall and do atmospheric flight. |

- The Corvus has a rear dorsal lifeboat that "can operate independently of its vessel for up to three months" (flavour, p. 180). Its stat block says "EEV II".
- ICC protocols prohibit leaving port without a full complement of auxiliary vehicles or EEVs (flavour, p. 183).

### 2.6 AI mainframes (price of the AI module) [p. 130]
| Mainframe | Skills | Cost |
|---|---|---|
| A.P.O.L.L.O. | Comtech 5, Piloting 4, Observation 5 | $2,000,000 |
| MU/TH/UR 6000 | Comtech 5, Piloting 5, Observation 5 | $2,500,000 |
| MU/TH/UR 6500 | Comtech 6, Piloting 5, Observation 6 | $3,500,000 |
| MU/TH/UR 7000 | Comtech 7, Piloting 6, Observation 7 | $5,000,000 |
| MU/TH/UR 9000 | Comtech 10, Piloting 8, Observation 10, Ranged Combat 9 | $50,000,000 |

- A MU/TH/UR (or comparable mainframe) can man some or all crew positions when needed [p. 188].
- AMBIGUOUS (A-10): the skill numbers are presumably the full dice pool (the p. 64 example gives Mother "8 Base Dice"). Only the 9000 has RANGED COMBAT. None has HEAVY MACHINERY: an AI can't do HEAVY MACHINERY maintenance (p. 179) and so presumably can't do HM repairs or engineer HM actions. Whether a non-9000 AI can act as gunner is GM discretion. AIs are NPCs, so they never push and have no stress.

### 2.7 Armaments [p. 176–177]
- Two types: **offensive weapon systems** and **defensive countermeasures**. They can be removed and replaced and use the same size categories as modules. [p. 176]
- They're mounted on **external hardpoints**. Each ship has a specific number of hardpoints per size. **A smaller armament fits a bigger hardpoint, not the reverse.** [p. 176]
- Damage works like personal weapons but at a different scale. **Ship armaments can't target individuals.** [p. 176]
- "Bonus" = modification to the gunner's RANGED COMBAT roll. "–" = +0. "Range" = maximum range category (see §0). [p. 176–177, 196]

Weapon special rules [p. 176]:
- **ASAT missiles** (Short Lance / Long Lance): long range, heavy punch. Flavour: slow to arrive, so the target can shoot them down or use countermeasures. There's no separate mechanical delay; the counter is the Launch Countermeasures reaction (ERRATA p. 192: Evade is not an action). **A missile battery typically fits 8 missiles. Missiles are bought separately** (Short Lance $100,000 each, Long Lance $400,000 each).
- **Railguns** (Light/Medium/Heavy): inaccurate, lethal at short range. **Armor is halved (round up) against railguns** ("Armor piercing").
- **Particle Beam Weapons** (400MeV / 800MeV): (1) **countermeasures have no effect against them**; (2) **base Damage is reduced one step for each range category beyond CONTACT**; (3) **they inflict no hull damage, only component damage**. The table says "See page 196", but the rule is actually on p. 176.
  - Effective damage by range: 400MeV (Dmg 4, max Long): Contact 4 / Short 3 / Medium 2 / Long 1. 800MeV (Dmg 5, max Extreme): Contact 5 / Short 4 / Medium 3 / Long 2 / Extreme 1.
- **Orbital Mines**: cheap, short-range (Contact), "very effective against unwanted boarding parties". **Individual mines must be bought separately**. AMBIGUOUS (A-11): no price is given for individual mines. The $10,000 is the armament/launcher price.
- **Tactical Nukes**: deployed from orbit against **planetary targets** (Range "Surface"). A 50-megaton nuke destroys all buildings and kills all living beings within **10 km** and damages buildings and people up to **100 km**. A high-altitude detonation makes an **EMP that destroys unshielded electronics**. **A nuclear missile battery typically fits 5 missiles**, bought separately at $1,000,000 each. There's no ship-to-ship use (Bonus/Damage N/A).
- **CIWS Laser Array**: countermeasure that shoots down incoming missiles "and even railgun rounds".
- **Sensor Drones**: false signatures that lure away missiles and railgun targeting.
- **Sensor Decoys**: false radar signatures to confuse weapons.
- AMBIGUOUS (A-12): whether countermeasures (decoys/drones) are consumed when used isn't stated. Assume not. Whether each Fire Weapon with a missile weapon consumes exactly 1 missile isn't stated either. Assume yes, and track ammo.

**OFFENSIVE ARMAMENTS** [p. 177] (verified against the image)

| Armament | Bonus | Damage | Range | Hardpoint | Cost | Comment |
|---|---|---|---|---|---|---|
| Short Lance ASAT Missile | +1 | 3 | Long | Size II | $12,000,000 | Missile cost: $100,000 |
| Long Lance ASAT Missile | +1 | 4 | Extreme | Size III | $25,000,000 | Missile cost: $400,000 |
| Light Railgun Turret | – (+0) | 3 | Short | Size I | $1,500,000 | Armor piercing |
| Medium Railgun Turret | – (+0) | 4 | Short | Size II | $8,500,000 | Armor piercing |
| Heavy Railgun Turret | – (+0) | 5 | Short | Size III | $20,000,000 | Armor piercing |
| 400MeV Particle Beam Weapon | +2 | 4 | Long | Size II | $12,000,000 | Damage reduced over range |
| 800MeV Particle Beam Weapon | +2 | 5 | Extreme | Size III | $30,000,000 | Damage reduced over range |
| Orbital Mines | +2 | 2 | Contact | Size II | $10,000 | — |
| Tactical Nuke | N/A | N/A | Surface | Size III | $50,000,000 | Missile cost: $1,000,000 |

**DEFENSIVE COUNTERMEASURES** [p. 177] (verified against the image)

| Armament | Bonus | Hardpoint | Cost |
|---|---|---|---|
| Sensor Decoys | – (+0) | Size I | $1,200,000 |
| Sensor Drones | +1 | Size II | $3,600,000 |
| CIWS Laser Array | +2 | Size III | $45,000,000 |

### 2.8 Reworking your ship, leasing, prices [p. 168, 177]
- To mount new modules or armaments, or to make upgrades, you need a **dry dock on a major space station or planetside colony**. [p. 177]
- **Module and armament prices normally include installation.** [p. 177]
- The work takes **a week or more**, depending on module size and on whether you pay extra to speed it up. AMBIGUOUS (A-13): there's no formula for duration or rush cost.
- Listed prices are for common models on the Frontier. **Older versions are cheaper and less reliable**, with no numbers given. [p. 168]
- **Lease: 2% of the ship's total price per year.** Corporations often lease ships to crews with an option to buy. [p. 168]

### 2.9 Upgrades [p. 178]
"Step" = 1 point. All upgrades are bought at a dry dock (p. 177). The cost is per purchase.

| Upgrade | Effect | Max purchases | Cost | Page |
|---|---|---|---|---|
| Added Hardpoint, size I | +1 hardpoint, **same size as your current biggest hardpoint or smaller** | **2 in total** (all sizes combined) | $1,000,000 | p. 178 |
| Added Hardpoint, size II | (same) | (shared limit) | $4,000,000 | p. 178 |
| Added Hardpoint, size III | (same) | (shared limit) | $10,000,000 | p. 178 |
| Armored Bulkheads | Armor Rating +1 | 3 | $2,000,000 | p. 178 |
| Boosted Displacement Drives | FTL rating −1 step | 6 | $1,400,000 | p. 178 |
| Enhanced Sensors | **+1 modification to the Target Lock action** | 2 | $2,200,000 | p. 178 |
| External Module, size I | Fittings for 1 extra module, **same size as your current biggest module or smaller**, mounted externally. **The module itself is bought separately.** | **2 in total** | $100,000 | p. 178 |
| External Module, size II | (same) | (shared limit) | $500,000 | p. 178 |
| External Module, size III | (same) | (shared limit) | $1,000,000 | p. 178 |
| External Module, size IV | (same) | (shared limit) | $4,000,000 | p. 178 |
| External Module, size V | (same) | (shared limit) | $10,000,000 | p. 178 |
| Overpowered Thrusters | Thrusters +1 step | 2 | $1,500,000 | p. 178 |
| Planetfall Capacity | Allows atmospheric entry and landing (§1.3) | 1 (on/off) | $1,800,000 | p. 178 |
| Reinforced Frame | Hull +1 step | 3 | $1,200,000 | p. 178 |
| Stealth Technology | Signature −1 step | 2 | $50,000,000 | p. 178 |

- AMBIGUOUS (A-14): "current biggest hardpoint/module" could mean the biggest **slot** or the biggest **installed** item. Recommendation: biggest slot. A C-class ship has no hardpoints, so Added Hardpoint is presumably impossible for it. Boosted Displacement Drives on a ship without an FTL drive (class C "–") is presumably impossible.
- FTL can't go below 1 (p. 167). With 6 Boosted Drives: G → 14, M → 6, R → 2.

### 2.10 Regular maintenance [p. 179]
- **Every week**, a crew member must make **one HEAVY MACHINERY roll and one COMTECH roll** to service the ship. **Each roll takes one Shift.**
- **Only one person can roll** (per service roll). **Others can help** (+1 each, max +3, p. 63).
- If everyone is in cryosleep, a **synthetic** can do the work. A **ship AI** (e.g. MU/TH/UR) **can handle the COMTECH service but not the HEAVY MACHINERY service.**
- **If a service roll fails, or isn't made during a given week, the ship suffers minor component damage** (D66, p. 198).
- AMBIGUOUS (A-15): if both rolls fail (or both are skipped), apply one or two minor damage rolls? Recommendation: one D66 roll per failed or missing service roll.

### 2.11 SHIP DESIGN PROCEDURE (reconstructed)
**The book gives no explicit step-by-step construction procedure.** What it gives: the class table (typical characteristics, slots, base cost), module/armament/upgrade price lists, slot-fitting rules, upgrade limits and the dry-dock rule. The procedure below is the minimal consistent reading. Each inferred step is marked.

**Step 1: Choose class** (C, G, M or R) [p. 166]. Load base values: Length, Crew range, FTL, Signature, Thrusters, Hull, Armor, hardpoint slots, module slots and base Cost.
- AMBIGUOUS (A-16): the class table is described as "typical characteristics" and "individual ships can vary greatly". The book doesn't say that a custom ship *must* use exactly these numbers or that the class cost covers the hull and core components. Treat the class row as the base hull including all core components (bridge, sensors, comms, reactor, thrusters, displacement drive, hull, armor).

**Step 2: Choose the AI** (optional, size I module, price from §2.6) [p. 130, 171–172].

**Step 3: Choose internal modules** [p. 170, 172]. Constraints:
- Each module has one size. A module may occupy a slot of its own size or larger.
- **Fit check (exact):** for every size s ∈ {I…V}: `count(modules with size ≥ s) ≤ count(module slots with size ≥ s)`, where slots include External Module fittings. Because the sizes are nested, this condition is necessary and sufficient. Greedy assignment works too: largest module first, into the smallest free slot ≥ its size.
- **Mandatory modules: none are stated as rules.** AMBIGUOUS (A-17):
  - EEVs for the whole crew are **legally required for ICC-sanctioned craft** but "not always enforced" (p. 171). Suggest a warning.
  - Air Scrubbers are "needed to supply the crew with breathable air" (p. 171). Every prebuilt ship has them. Suggest a warning if missing.
  - A Docking Umbilical (or a hangar arrangement) is needed for crew to enter/exit (p. 171) and for the Dock action (p. 193).
  - Cryo Deck, Galley and capacities vs crew aren't enforced.

**Step 4: Choose armaments** [p. 176–177].
- **Fit check:** same nested condition over hardpoint sizes I–III (including Added Hardpoints).
- Buy missiles, nukes and mines separately (ammo).

**Step 5: Choose upgrades** [p. 178]: respect the max counts, the shared limits for Added Hardpoint and External Module, and "≤ current biggest" sizing (A-14).

**Step 6: Derive stats:**
```
FTL        = classFTL − BoostedDisplacementDrives         (min 1; "–" stays "no FTL")
Signature  = classSig − StealthTechnology
Thrusters  = classThr + OverpoweredThrusters              (class "–" = 0)
Hull       = classHull + ReinforcedFrame
Armor      = classArmor + ArmoredBulkheads
TargetLockBonus = EnhancedSensors (each +1)
HardpointSlots  = classHardpoints + AddedHardpoints
ModuleSlots     = classModuleSlots + ExternalModules
CanPlanetfall   = PlanetfallCapacity purchased
```
Runtime modifiers on top of these (combat/travel): towing ×2 FTL, overcharge −1 FTL (1 week), sensors/engines powered down (−1/−2 Signature), component damage (e.g. Thruster damage −2 Thrusters; Displacement drive malfunction FTL ×2, DECIDED A-35).

**Step 7: Total cost.** AMBIGUOUS (A-18): not stated. Recommended formula:
```
Cost = classCost + Σ modulePrice (AI = mainframe price) + Σ armamentPrice
     + Σ upgradePrice (per purchase) + Σ ammo (missiles/nukes/mines)
Lease per year = 2% × Cost   [p. 168]
```
**Warning:** the book's prebuilt ship prices do **not** equal this sum (§3.8). For prebuilt ships, use the listed price.

**Step 8: Fitting time and location.** Dry dock only. A week or more (A-13).

**What's missing from the book (summary):**
- Whether class stats are fixed or ranges.
- Whether crew/length are free choices.
- A cost formula.
- Mandatory modules.
- How to build classes other than C/G/M/R.
- How a C class gets FTL.
- The cost of individual orbital mines.
- Rush-job pricing and duration.
- Resale value.
- Hangar count.

---

## 3. SPACESHIPS OF 2183 AD [p. 180–187]

### 3.1 Overview table [p. 180] (verified against the image)

| Class | Model | Number | Role | Cost |
|---|---|---|---|---|
| C Class | Starcub | — | Light Intrasystem Shuttle | $2,300,000 |
| G Class | Corvus | CM-90S | Commercial Deep Space Salvage Vessel | $17,000,000 |
| M Class | Bison | CM-88G | Commercial Star Freighter | $36,000,000 |
| M Class | Bison | CM-88H | Commercial Towing Vehicle | $42,000,000 |
| M Class | Star Clipper | CYG-NS3 | Commercial Transport Ship | $37,000,000 |
| Conestoga | Frigate | RRTT-3 | Rapid Response Troop Transport | $1,040,000,000 |

The Conestoga's class column says "Conestoga", not a letter. By length and crew it resembles class R, but it doesn't follow the R template (§3.8).

### 3.2 Model CM-90S Corvus: G-class commercial deep space salvage vessel [p. 175, 180–181]
| Field | Value |
|---|---|
| Manufacturer | Lockmart |
| Crew | 6 |
| AI | MU/TH/UR 6000 |
| Length | 54m |
| FTL Rating | 20 |
| Signature | +0 |
| Thrusters | +1 |
| Hull | 6 |
| Armor Rating | 5 |
| Internal modules | Artificial Intelligence I · Air Scrubbers II · Cargo Bay III · Cryo Deck II · Docking Umbilical II · EEV II · Galley II · Hangar III · Salvage Crane III |
| Armaments | None |
| Upgrades | Reinforced Frame II (printed thus; see note) |
| Cost | $17,000,000 |
- Notes [p. 180]: overpowered multivector engines, tow crane rig, rear dorsal lifeboat (crew lifeboat, independent for up to three months). Suggested ship for **Explorers** campaigns [p. 329].
- AMBIGUOUS (A-19): "Reinforced Frame II". Hull 6 = G 5 + 1, so it equals **one** purchase. "II" is probably a typo (upgrades have no sizes).

### 3.3 Model CM-88G Bison: M-class commercial star freighter [p. 181]
| Field | Value |
|---|---|
| Manufacturer | Lockmart |
| Crew | 7 |
| AI | MU/TH/UR 6500 |
| Length | 334m |
| FTL Rating | 12 |
| Signature | +1 |
| Thrusters | +0 |
| Hull | 9 |
| Armor Rating | 6 |
| Internal modules | Artificial Intelligence I · Air Scrubbers II · Cargo Bay IV × 4 · Cryo Deck II · Docking Umbilical II · EEV II × 2 · Galley II · Medlab II |
| Armaments | None |
| Upgrades | Planetfall Capacity |
| Cost | $36,000,000 |
- Suggested ship for **Space Truckers** campaigns [p. 327].

### 3.4 Model CM-88H Bison: M-class commercial towing vehicle [p. 182, 185]
| Field | Value |
|---|---|
| Manufacturer | Lockmart |
| Crew | 7 |
| AI | MU/TH/UR 7000 |
| Length | 334m |
| FTL Rating | 8 |
| Signature | +1 |
| Thrusters | +0 |
| Hull | 9 |
| Armor Rating | 6 |
| Internal modules | Artificial Intelligence I · Air Scrubbers II · Cargo Bay IV × 2 · Cryo Deck II · Docking Umbilical II · EEV II × 2 (Starcub Shuttles) · Galley II · Medlab II · Tractor Hitch III |
| Armaments | None |
| Upgrades | Boosted Displacement Drives × 4 · Planetfall Capacity |
| Cost | $42,000,000 |
- Engines can be swapped with "only a week in drydock" (flavour) [p. 182]. While towing, FTL 8 → 16 (Tractor Hitch rule, p. 171).

### 3.5 Model CYG-NS3 Star Clipper: M-class commercial transport ship [p. 182–183]
| Field | Value |
|---|---|
| Manufacturer | Lockmart |
| Crew | 6 |
| Passengers | 200 (in cryosleep) |
| AI | MU/TH/UR 6500 |
| Length | 300m |
| FTL Rating | 6 |
| Signature | +1 |
| Thrusters | +1 |
| Hull | 9 |
| Armor Rating | 6 |
| Internal modules | Artificial Intelligence I · Air Scrubbers III · Cargo Bay III × 4 · Cryo Deck IV · Docking Umbilical II · EEV II × 4 · Galley III · Medlab II |
| Armaments | None |
| Upgrades | Boosted Displacement Drives × 6 · Planetfall Capacity · Overpowered Thrusters |
| Cost | $37,000,000 |
- The cargo bay ceiling has stasis-pod coupling racks for extra pods/passengers (flavour) [p. 182].

### 3.6 Starcub-class shuttle: C-class intrasystem vessel [p. 183]
| Field | Value |
|---|---|
| Manufacturer | Lockmart |
| Crew | 1 |
| Passengers | 3 (in cryosleep) |
| AI | NONE |
| Length | 16m |
| FTL Rating | 15 |
| Signature | −1 |
| Thrusters | +2 |
| Hull | 2 |
| Armor Rating | 4 |
| Internal modules | Air Scrubbers I · Cargo Bay I · Cryo Deck II |
| Upgrades | None |
| Armaments | None |
| Cost | $2,300,000 |
- **Max range 13 parsecs** before refuelling. Short-distance FTL [p. 183]. Older models had 2 cryo chambers, later ones 3. Used as auxiliary craft and EEVs (Bison H) [p. 182–183].
- See A-01 (FTL 15 on a class with FTL "–").

### 3.7 Conestoga-class frigate [p. 186 (stat block), p. 194–195 (art page repeat)]
| Field | Value |
|---|---|
| Manufacturer | Weyland-Yutani |
| Crew | 12, or a MU/TH/UR 9000 + one Hyperdyne 341-B android |
| Troop capacity | 90 |
| Length | 731m |
| FTL Rating | 2 |
| Signature | +0 |
| Thrusters | +1 |
| Hull | 12 |
| Armor Rating | 10 |
| Internal modules | Artificial Intelligence I · Air Scrubbers IV · Cargo Bay III · Cryo Deck IV · Docking Umbilical II · EEV I × 20 · Galley IV · Hangar IV · Medlab II |
| Armaments | Long Lance ASAT Missiles (8 missiles) · Heavy Railgun Turret · 800MeV Particle Beam Weapon · Orbital Mines (60 mines) · Tactical Nukes (20 missiles) · CIWS Laser Array · Sensor Drones · Sensor Decoys |
| Upgrades | Boosted Displacement Drives × 6 · Enhanced Sensors · Overpowered Thrusters · Stealth Technology |
| Cost | $1,040,000,000 |
- The MU/TH/UR 9000 can perform missions without crew "so long as it does not need to make planetfall" (no Planetfall Capacity is listed anyway). Dropships deploy from a ventral hangar. An external docking rack for EVAC fighters is possible (flavour). [p. 186]
- Suggested ship for **Colonial Marines** campaigns [p. 328].
- The p. 195 repeat lists "Boosted Displacement Drives" **without "× 6"**. p. 186 has × 6, and FTL 2 = 8 − 6 is consistent with × 6.
- Flavour only (no rule): "Smuggler's Run" claims the Conestoga's big guns can't fire straight ahead or behind [p. 187]. AMBIGUOUS (A-20): not a rule. The space combat rules have no facing.

### 3.8 Validation of prebuilt ships against the design rules (app guidance)
Slot fits (nested-size check from §2.11):
| Ship | Module fit | Hardpoint fit | Stat derivation from class + upgrades |
|---|---|---|---|
| Starcub (C) | OK (≥II: 1/2; ≥I: 3/6) | n/a | Sig, Thr, Hull, Armor match C. **FTL 15 vs C "–"**: no upgrade explains it (A-01). |
| Corvus (G) | OK, exactly full (≥III: 3/3; ≥II: 8/8; ≥I: 9/15) | n/a | All match G + 1 Reinforced Frame. |
| Bison G (M) | OK (≥IV 4/4; ≥II 11/18; ≥I 12/18) | n/a | All match M. |
| Bison H (M) | OK (≥IV 2/4; ≥III 3/10; ≥II 10/18; ≥I 11/18) | n/a | FTL 12 − 4 = 8. OK. |
| Star Clipper (M) | OK (≥IV 1/4; ≥III 7/10; ≥II 13/18; ≥I 14/18) | n/a | FTL 12 − 6 = 6. Thr 0 + 1 = +1. OK. |
| Conestoga (R?) | **FAILS** if each EEV I is a module: 28 modules vs 21 R slots (+2 external max = 23) | **FAILS**: 5 × size III armaments vs 2 III slots (+2 added max) | FTL 8 − 6 = 2 OK. Hull 12 OK. **Sig** expected +2 − 1 = +1, printed +0. **Thrusters** expected −1 + 1 = 0, printed +1. **Armor** expected 7, printed 10 (no Armored Bulkheads listed). |

Price check (class cost + modules + AI + armaments/ammo + upgrades; EEV II priced as Class B or Class C):
| Ship | Computed | Listed |
|---|---|---|
| Starcub | $2,260,000 | $2,300,000 |
| Corvus | $14,300,000 – $14,550,000 (Reinforced Frame × 1) | $17,000,000 |
| Bison G | $35,800,000 – $36,300,000 | $36,000,000 |
| Bison H | $43,000,000 – $43,500,000 (EEV priced as Class B/C, not as a $2.3M Starcub) | $42,000,000 |
| Star Clipper | $61,475,000 – $62,475,000 (Cryo Deck IV alone is $15M) | $37,000,000 |
| Conestoga (as R) | ≈ $732,060,000 (incl. 8 Long Lance missiles, 20 nukes; mines unpriced) | $1,040,000,000 |

→ **App rule:** load prebuilt ships as fixed records with their printed stats and price. Don't recompute them, and mark the Conestoga as "exceeds class template". AMBIGUOUS (A-21).

---

## 4. SPACE COMBAT [p. 188–203]

### 4.1 Overview [p. 188]
- Designed for **a few ships per side**. The typical encounter is a **one-on-one duel**.
- Played in **Turns** (5–10 minutes, p. 83), **not Rounds**.

### 4.2 Crew positions [p. 188]
| Position | Function | Skill | Notes |
|---|---|---|---|
| **Captain** | Overall command. **Gives orders and draws initiative.** Declares each phase's action for the ship (p. 191). | — (no roll defined) | Talent **Spaceship Commander**: draw two initiative cards, choose one (p. 79). |
| **Sensor Operator** | Operates the ship sensors. | **COMTECH** | Acts in the Sensor Phase. |
| **Pilot** | At the flight controls. | **PILOTING**, modified by the ship's **Thrusters** | Acts in the Pilot Phase. Talents: Full Throttle, Like the Back of Your Hand, Reckless. |
| **Gunners** (plural) | Fire the ship's armaments. | **RANGED COMBAT** (+ weapon/countermeasure Bonus) | "Each weapon or countermeasure generally requires at least one gunner." **Only one gunner action per ship per Turn** (p. 196). Talent Spaceship Gunner +2. |
| **Engineers** (plural) | Emergency repairs etc. | **COMTECH** or **HEAVY MACHINERY** | **Only one engineer action per ship per Turn, however large the crew. Others can help, up to +3** (p. 196). Talent Spaceship Mechanic +2 on repairs. |

- **One person can fill two or more positions on very small ships.** [p. 188]
- **A MU/TH/UR or similar mainframe can fill any or all positions** (skills §2.6, A-10). [p. 188]
- **Taking orders:** crew normally follow the captain's orders but may disobey and perform another action. The consequences are roleplayed. [p. 191]
- AMBIGUOUS (A-22): "Only one action per Turn can be performed" (p. 191) is read as one action **per position (phase)** per Turn. A single person filling several positions acts once in each of their phases. Multiple gunners or engineers give no extra actions; the extra engineers can only help.

### 4.3 Distance, zones and range [p. 188–189]
- Abstract **zones** (each hundreds or thousands of km wide). **All movement is one-dimensional along a range track.** [p. 188]
- **Space combat map** (back of book, p. 386–387): 9 zones labelled **−4, −3, −2, −1, 0, +1, +2, +3, +4**. [p. 188, 386–387]

**RANGE CATEGORIES** [p. 189]
| Range | Description | Zone distance (app) |
|---|---|---|
| Contact | In the same zone. **Ramming and boarding possible.** | 0 |
| Short | Adjacent zone. **Visual contact limit.** | 1 |
| Medium | Two zones away. | 2 |
| Long | Up to four zones distant. | 3–4 |
| Extreme | Up to eight zones distant. **Sensor limit.** | 5–8 |
| (out of range) | — | > 8 |

### 4.4 Approach velocity and movement [p. 189, 193, 202–203]
- When two ships approach, **the GM sets the initial approach velocity**, typically **2 zones per Turn**, possibly higher or lower. **Ships that have synched vectors have approach velocity 0.** [p. 189]
- With more than two ships: split them into **two sides**. **Each ship has an initial approach velocity relative to the opposing side's ships.** [p. 189]
- Velocity is **tracked per ship**. In the example, both ships start at 2. One accelerates to 3, the other decelerates to 1, and they move 3 and 1 zones respectively. [p. 202]
- **Movement:** in the Pilot Phase, **once every pilot action is resolved**, **each ship moves zones = its current approach velocity: toward the enemy ship(s), or away if they have already passed each other.** Movement happens **before the Gunner Phase**. [p. 193]
  - App model: each ship has a fixed direction along the track (towards the other side's starting edge) and moves `position += direction × velocity`. In the example the Lyssa moved from +1 by 5 to −4, passing the Porphyrion, which moved −3 → −2. They then separated. [p. 203]
- **Off-map:** if a ship moves off the map but the distance between the two ships is still **≤ 8 zones**, shift both ships back onto the map by the same number of zones. Only relative distance matters. [p. 193]
- AMBIGUOUS (A-23): velocity can't go below 0 (reversing isn't described). It's unclear what happens beyond 8 zones (sensor limit): presumably contact is lost and the encounter ends, at the GM's discretion. With 3+ ships on one track, positions and "passed each other" are per pair.

### 4.5 Detection, transponders, running silent [p. 190–191]
- **Starship sensors have EXTREME range (8 zones).** When two ships enter sensor range of each other **they automatically detect each other**. On the map, place each ship in the **outermost zones (+4 and −4)**, which are 8 zones apart. The marker shows the "ping" side (unidentified). [p. 190]
- **Transponders:** all civilian craft must travel with their transponder on (it broadcasts position, name, class, home port and destination). **If the approaching ship's transponder is active, you immediately learn this information and you are able to target it with your armaments.** [p. 190]
- **Running silent** (transponder off) is unlawful and suspicious. Only smugglers and military do it. **If the approaching ship runs silent, your sensor operator needs a successful Target Lock before you can target it with weapons.** [p. 190]
  - DECIDED (A-24): an active transponder replaces the Target Lock: a ship with its transponder on can be fired upon without a lock. A ship running silent can only be fired upon after a successful Target Lock. (Book conflict: the p. 192 table and the p. 196 Fire Weapon text demand a lock in every case; p. 190 implies the transponder suffices.) Still open: whether a ship can switch its transponder off mid-combat (presumably yes, as a free choice; it isn't part of Go Dark).
- **Reduce Signature (power down):** can be done **even before the encounter starts**. [p. 190]
  - **Sensors off → Signature −1.** While sensors are down **you can't Target Lock**.
  - **Engines off → Signature −2.** While engines are offline **you can't perform any pilot actions at all**.

**RUNNING SILENT** [p. 190]
| Power down | Signature | Power up |
|---|---|---|
| Sensors | −1 | One Turn (Sensor Operator) |
| Engine | −2 | Two Turns (Engineer) |
- The p. 190 text says "Powering a ship back up takes one Turn". The table and p. 196 say engines take **two Turns: two successive engineer actions**. → Use the table: sensors 1 Turn, engines 2 Turns.
- AMBIGUOUS (A-25): the Target Lock table (p. 192) **also** lists "Target Ship Sensors Offline −1" and "Target Ship Engines Offline −2", the same numbers as the Signature reduction. **Apply once only.** Recommendation: use the target's *current* Signature, including power-down reductions, and don't add the table rows separately.

### 4.6 Initiative and Turns [p. 190, 87, 79, 193]
- On encountering a ship, **draw initiative as in personal combat** (p. 87), but **only the captain of each ship draws a card**. Cards 1–10. **Lowest acts first.** Drawn once for the whole encounter. [p. 190, 87]
- **Spaceship Commander** talent: draw two cards, choose one (p. 79). AMBIGUOUS (A-26): it's unclear whether the general talent **Fast Reflexes** (two cards, p. 78) also applies to a captain's draw. The book doesn't say. The Spaceship Commander talent text omits the "shuffle the other back" clause of Fast Reflexes.
- **Maneuver** pilot action: winning lets you **trade initiative cards with the enemy ship** (p. 193). AMBIGUOUS (A-27): personal combat allows exchanges only at the start of a Round, "never during a Round" (p. 87). It's unclear whether a Maneuver trade takes effect immediately for the remaining phases or from the next Turn. Recommendation: from the next phase.
- **Each Turn has four phases, and each ship performs one action in each phase. In each phase all ships act in initiative order before moving on to the next phase.** [p. 190]
  1. **Sensor Phase**
  2. **Pilot Phase (including movement)**
  3. **Gunner Phase**
  4. **Engineer Phase**

### 4.7 Declaring actions [p. 191, 202]
- Each phase, **the captain tells that phase's crew member** (sensor operator, pilot, gunner, engineer) which **one action** from the phase's list to take. [p. 191]
- **There are no fast and slow actions in space combat.** [p. 191]
- **There are no stunts for skill rolls.** The action descriptions define what extra successes do. [p. 191]
- **Every ship picks its action for each phase in secret; all picks are revealed at once, then resolved in initiative order.** Declaring happens **per phase** (the example declares, reveals and resolves the Sensor Phase, then the Pilot Phase, and so on). [p. 191, 202]
- **Reactive actions** (Go Dark, Launch Countermeasures; ERRATA p. 192: Evade is not an action) **break the normal initiative order**, like blocking in close combat. They resolve when triggered, but they must have been **declared as that position's action for the phase**. [p. 191]
- Order markers (Maps & Markers Pack) or written notes. [p. 191]
- When all four phases are done, a new Turn begins. [p. 191]

### 4.8 SPACE COMBAT ACTIONS (summary table) [p. 192, verified against the image]
ERRATA (p. 192): the summary table gains Maneuver and loses Evade. The full action descriptions on p. 193 are correct, so Evade is not an action.

| Phase | Action | Skill | Effect |
|---|---|---|---|
| 1. Sensor | Target Lock | COMTECH | Sensor lock on one target ship; allows Fire Weapon on it. |
| 1. Sensor | Go Dark | COMTECH | **Reactive.** Each success eliminates one success in an enemy Target Lock. Can also power down sensors and/or engines. |
| 1. Sensor | Power Up Sensors | NONE | Bring sensors back online. |
| 2. Pilot | Accelerate | PILOTING | Approach velocity +1 step. +2 if multiple successes are rolled. |
| 2. Pilot | Decelerate | PILOTING | Approach velocity −1 step. −2 if multiple successes are rolled. |
| 2. Pilot | Ram | PILOTING | Target must share your zone. Opposed roll. A hit damages both ships. |
| 2. Pilot | Dock | PILOTING | Target must share your zone and approach velocity. Opposed roll. Allows Open Airlock. |
| 2. Pilot | Maneuver (ERRATA p. 192: added to the table; described p. 193) | PILOTING (opposed) | Win → trade initiative cards with the enemy ship. |
| 3. Gunner | Fire Weapon | RANGED COMBAT | Requires sensor lock on the target (DECIDED A-24: not needed if the target's transponder is active). Inflicts weapon Damage, +1 for each additional success. |
| 3. Gunner | Launch Countermeasures | RANGED COMBAT | **Reactive.** Each success eliminates one success in an attack against you from SHORT range or more. |
| 4. Engineer | Emergency Repairs | COMTECH / HEAVY MACHINERY | Temporarily repairs a damaged component. Holds for one Shift only. |
| 4. Engineer | Power Up Engine | NONE | Two consecutive actions needed. |
| 4. Engineer | Open Airlock | HEAVY MACHINERY | Enables boarding. |
| 4. Engineer | Reactor Overload | HEAVY MACHINERY | Starts the ship's self-destruction at the end of the next Turn. |

### 4.9 Sensor operator actions (Sensor Phase) [p. 191–192]
**TARGET LOCK** (COMTECH)
- Prerequisites: sensors powered on, not "Sensor failure" (major 4: "Target Lock cannot be made"), and the target within Extreme range (≤ 8 zones). One target per action.
- Roll **COMTECH** + modifications (table below) + **Enhanced Sensors** (+1 each, p. 178) + other factors (Instrument malfunction −1, p. 198; help per p. 63, if the GM allows).
- **Success (≥1 success remaining after Go Dark cancellation)** → locked on. **You can target it with your armaments in this Turn.**
- **Subsequent Turns:** you must repeat Target Lock on that ship each Turn to fire on it, but **no roll is needed (automatic success) unless the target Goes Dark.** [p. 191]
  - AMBIGUOUS (A-28): it's unclear whether the auto-lock persists if you skipped Target Lock in an intermediate Turn, or after the target's signature changes. Recommendation: auto-lock if the ship was locked in the previous Turn and the target doesn't Go Dark this Turn.
- Range is measured **at the Sensor Phase**, before this Turn's movement. The Gunner Phase range is after movement. [p. 193, 202–203]

**TARGET LOCK modifiers** [p. 192] (verified against the image)
| Factor | Modification |
|---|---|
| Extreme Range | −2 |
| Long Range | −1 |
| Medium Range | +0 |
| Short Range | +1 |
| Contact Range | +2 |
| Malfunctioning Sensors | −2 |
| Target Ship Signature | Varies (add the target's Signature value) |
| Target Ship Sensors Offline | −1 (see A-25) |
| Target Ship Engines Offline | −2 (see A-25) |

"Malfunctioning Sensors −2" = minor damage "Sensor malfunction" (all COMTECH rolls using the ship's sensors −2, p. 199). Don't double-apply.

**GO DARK** (COMTECH, **reactive**)
- If an enemy ship tries to Target Lock you **this Turn**, you can try to break the lock: roll **COMTECH**. **Each of your successes eliminates one success rolled by the enemy** on its Target Lock. [p. 191]
- As part of Go Dark you may also **power down sensors (−1 Signature) and/or engines (−2 Signature), taking effect immediately**. [p. 191]
- Going Dark forces enemies who had an automatic lock to roll Target Lock again (example p. 203).
- AMBIGUOUS (A-29): if several enemies lock you, it's unclear whether you roll one Go Dark roll per incoming lock (as for countermeasures, "separate roll for each incoming attack") or one roll in total. Recommendation: one roll per incoming Target Lock. The Signature reduction applies before the enemy's lock roll is resolved, since it takes effect "immediately".

**POWER UP SENSORS** (no roll): if sensors are powered down, it takes **one Turn** (this action) to bring them back online. [p. 191]

### 4.10 Pilot actions (Pilot Phase) [p. 192–193]
General: all pilot rolls are **PILOTING + ship's Thrusters** (p. 188, 193). **No pilot actions at all** if: engines powered down (p. 190), **Engine failure** (major 7, "dead in space"), **Reactor breach** (major 10, "loses all propulsion"), **Bridge exposed to vacuum** (major 9, "can no longer be controlled"), the ship is **Disabled** (engines fail, p. 197), or the ship is docked to a bigger ship (p. 193).

- **ACCELERATE:** roll PILOTING + Thrusters. **1 success → approach velocity +1. Several successes → +1 or +2 (your choice).** Full Throttle talent +2. [p. 193]
- **DECELERATE:** the same, but −1 / (−1 or −2). [p. 193]
- **MANEUVER:** an **opposed PILOTING** roll (+ Thrusters) against the pilot of an enemy ship. **Uses your action, not the enemy's** (the enemy pilot still takes their own action). **Only you can push.** **Win → you can trade initiative cards with the enemy ship** (A-27). [p. 193]
- ERRATA (p. 192): **Evade is not an action** (removed from the summary table; the p. 193 pilot action descriptions are correct). Ambiguity A-30 is resolved.
- **RAM:** **target must be at Contact range (same zone) when the Turn starts (before movement)**. Opposed PILOTING (+ Thrusters). **On a win:** you hit; **each ship takes damage = half the *other* ship's full Hull (round up)**. **The target also takes +1 damage per success you rolled beyond the first.** [p. 193]
  - Rammer takes `ceil(target.Hull / 2)`. Target takes `ceil(rammer.Hull / 2) + (successes − 1)`.
  - AMBIGUOUS (A-31): whether "successes" means net successes after the opposed cancellation isn't stated (recommend net). It's also unclear whether Armor applies to ram damage (not stated; recommend yes, as "damage") and whether component damage applies (recommend yes, per single-attack thresholds; each ship's damage is a separate "attack").
- **DOCK:** only if the target is at **Contact range** and **both ships have the same approach velocity**. **Requires a Docking Station module** (= Docking Umbilical, p. 171) and no "Docking station damaged" (minor 36). Opposed PILOTING (+ Thrusters). **Win → your engineer may try to breach the enemy ship (Open Airlock).** **Once docked, the ships are joined: only the bigger ship of the two can perform pilot actions.** [p. 193]
  - AMBIGUOUS (A-32): "bigger" isn't defined (by class letter, then length?). It's unclear whether a cooperative dock needs a roll (presumably not), whether "same approach velocity" means equal per-ship velocity values (as tracked), and how the joined ships move (recommend: move together with the bigger ship's velocity).
- **Then: movement** of all ships (§4.4).

### 4.11 Gunner actions (Gunner Phase) [p. 196]
- **One gunner per ship can perform one action**, if the ship has one or more armaments. **Only one weapon may be launched per Turn, even with several weapons and gunners.** [p. 196]

**FIRE WEAPON** (RANGED COMBAT)
- Prerequisites: the target is **Target Locked this Turn**, or DECIDED (A-24): its transponder is active (no lock needed; a target running silent must be locked). Current range ≤ the weapon's Range. The weapon isn't "Armament damaged" (major 6). The ship isn't Disabled ("all weapon systems fail", p. 197). Ammo is available. Ship armaments can't target individuals (p. 176). A Tactical Nuke can only target a planetary surface.
- Roll **RANGED COMBAT + weapon Bonus + range modification** (table below). Also: Spaceship Gunner +2 (p. 79), Instrument malfunction −1, Armament malfunction −2 if this weapon is the affected one (p. 198), panic Tremble −2 (p. 201).
- Reactive defences cancel successes **(SHORT range or more only)**:
  - ERRATA (p. 192): Evade is not an action, so the target's pilot has no reactive defence. The Evade mention in the p. 196 Fire Weapon text is obsolete.
  - target gunner's **Launch Countermeasures**: each success cancels one attacker success. **Not against particle beams.**
- **Hit if ≥1 success remains. Damage = weapon Damage + 1 per extra remaining success** (i.e. Damage + (net successes − 1)). [p. 196]
- Particle beams: base Damage −1 per range category beyond Contact (§2.7).
- Then **ship armor** (§4.13) → hull and component damage (§4.14–4.15).

**SHIP WEAPON ATTACK modifiers** [p. 196] (verified against the image)
| Factor | Modification |
|---|---|
| Extreme Range | −2 |
| Long Range | −1 |
| Medium Range | +0 |
| Short Range | +1 |
| Contact Range | +2 |

**LAUNCH COUNTERMEASURES** (RANGED COMBAT, **reactive**)
- Launches defensive measures (Sensor Decoys, Sensor Drones, CIWS Laser Array). **Whenever offensive weapons target your ship this Turn from SHORT range (adjacent zone) or beyond**, your gunner rolls **RANGED COMBAT + the countermeasure's Bonus**. **Each success eliminates one success rolled by the attacker.** **Against multiple attacks, roll separately for each incoming attack.** [p. 196]
- It uses the ship's single gunner action, so a ship that counters can't fire that Turn.
- AMBIGUOUS (A-33): it presumably requires a countermeasure armament to be fitted, and it's unclear which one is used if there are several (recommend: declared with the action). Countermeasures don't affect particle beams (p. 176) or Contact-range attacks.

### 4.12 Engineer actions (Engineer Phase) [p. 196, 200]
- **One engineer per ship performs one action, however large the crew. Others can help (p. 63), up to +3.** [p. 196]
- **EMERGENCY REPAIRS** (HEAVY MACHINERY or COMTECH, or both, as the repair column of the damage table lists): a minor or major component damage is hastily repaired. [p. 196, 200]
  - **Each repair roll takes one Turn** (instead of one Shift).
  - **No spare parts are needed.**
  - **Temporary: the component fails again after one Shift.** Permanent repair is in §4.16.
  - **Components only, never the hull.**
  - Multi-roll repairs (e.g. "2 × HEAVY MACHINERY, COMTECH") need each listed roll to succeed, one roll per Turn. The example: HEAVY MACHINERY in Turn 1 is "work begun", COMTECH would follow. [p. 203]
  - Only one person can roll for the same component; others help (p. 200). Spaceship Mechanic +2 (p. 79). Tools apply (§4.16).
  - AMBIGUOUS (A-34): a failed roll presumably can be retried next Turn, with progress kept. Damage results with "–" in the repair column (Cargo damage, Crew injury, Critical crew injury, Chain reaction, Reactor detonation) can't be repaired as components.
- **POWER UP ENGINE** (no roll): if the engines were powered down, it takes **two Turns, i.e. two successive engineer actions**. Recommendation: any other engineer action in between resets the progress. [p. 196]
- **OPEN AIRLOCK** (HEAVY MACHINERY): only if **your pilot has successfully completed the Dock action**. **Success → you breach the enemy ship and your crew can board it (personal combat ensues).** [p. 196]
- **REACTOR OVERLOAD** (HEAVY MACHINERY): **success → the reactor overloads and blows up at the end of the following Turn: ship destroyed, all aboard killed.** **Another HEAVY MACHINERY roll (one Turn) halts the self-destruct.** [p. 196]
  - Recommendation: the stop roll is an Engineer-Phase action in the following Turn, before the end of that Turn.

### 4.13 Ship armor [p. 197]
- It works **exactly like body armor**: when the ship takes a hit, roll **dice equal to its Armor Rating**. **Each success reduces the damage by 1.** **This roll doesn't count as an action** for any crew member.
- **Railguns: armor counts as half, rounded up** → roll `ceil(Armor / 2)` dice. [p. 176]
- Damage after armor is what counts for hull and component damage (confirmed by the example: 4 damage − 1 = 3 → "three is less than half the Porphyrion's full Hull rating" → minor). [p. 202]

### 4.14 Hull damage [p. 197]
- The ship accumulates damage. **When total damage ≥ Hull rating → Disabled**: **engines and every weapon system fail; the whole ship suffers explosive decompression** (p. 107). A Disabled ship isn't blown apart and **can be repaired**, unless it took damage ≥ its full Hull in a **single attack** (destroyed).
- **Particle beam weapons cause no hull damage, only component damage.**
- AMBIGUOUS (A-36): there's no explicit cap on accumulated damage. The sheet has 10 damage boxes (p. 388), fewer than the R-class Hull 12. Since repairing one point removes Disabled, recommend capping accumulated damage at Hull (like Health 0 / vehicle HP).

### 4.15 Component damage [p. 197–199]
Every single attack that inflicts damage (after armor) also causes component damage according to the **single-attack** amount, where `half = ceil(Hull / 2)`:
- **1 ≤ damage < half → minor component damage**: roll **D66** (table below). **If the result doesn't apply to your ship, re-roll.**
- **half ≤ damage < Hull → major component damage**: roll **2D6** (table below).
- **damage ≥ Hull → the ship is completely destroyed and all crew and passengers are killed.**

**COMPONENT DAMAGE** [p. 197]
| Damage threshold | Component damage |
|---|---|
| 1 or more | Minor Damage |
| 1/2 Total HP or more | Major Damage |
| Total HP or more | Total loss of ship and crew |

- Edge case: **Hull 2 (Starcub, class C)**: half = 1, so any damaging hit is **major**, and 2+ in one attack destroys it.
- Particle beams: AMBIGUOUS (A-37). "No hull damage, only component damage", but it's unclear whether a particle-beam hit with damage ≥ Hull destroys the ship. Recommendation: apply minor/major thresholds only, with no destruction and no hull points. Armor is presumably rolled as normal (not stated otherwise).
- AMBIGUOUS (A-38): re-rolling non-applicable results is only stated for minor damage. Recommend re-rolling inapplicable major results too (e.g. "A.I. offline" on a ship without an AI). "Chain reaction" rolling 11 again is unresolved. Recommend: roll again.

**MINOR COMPONENT DAMAGE (D66)** [p. 198–199] (verified against the image). Ranges like 26–31 mean the D66 results 26 and 31.
| D66 | Damage | Effects | Repair roll |
|---|---|---|---|
| 11 | A.I. malfunction | COMTECH roll required to access the ship A.I. | COMTECH |
| 12 | Air scrubbers offline | All STAMINA rolls −2 for one Shift; after that the air is gone completely. | HEAVY MACHINERY |
| 13 | Airlock malfunction | Main airlock cannot be accessed. | HEAVY MACHINERY |
| 14–15 | Armament malfunction | One random armament: −2 to every attack made with it. | HEAVY MACHINERY |
| 16 | Artificial gravity malfunction | Running action requires MOBILITY roll. | HEAVY MACHINERY + COMTECH |
| 21 | Autodoc offline | Medical facilities cannot be used. | HEAVY MACHINERY |
| 22–23 | Cargo damage | One random cargo-hold item is destroyed. | – |
| 24 | Coffee maker malfunction | Galley coffee maker unusable. | HEAVY MACHINERY |
| 25 | Comms malfunction | All external communications offline. | COMTECH |
| 26–31 | Compartmental decompression | One random compartment decompresses suddenly (see p. 107). | HEAVY MACHINERY |
| 32–33 | Crew injury | One random crew member is hit: attack with six Base Dice, 1 damage per success. | – |
| 34 | Cryo chambers offline | Cryo chambers cannot be used. | HEAVY MACHINERY + COMTECH |
| 35 | Displacement drive malfunction | FTL rating doubled (DECIDED A-35: the printed "halved" is a misprint). | HEAVY MACHINERY + COMTECH |
| 36 | Docking station damaged | Docking with other vessels impossible. | HEAVY MACHINERY |
| 41 | Escape pod malfunction | EEVs cannot be used. | HEAVY MACHINERY |
| 42 | Hangar damage | One random craft in the hangar becomes Disabled. | HEAVY MACHINERY |
| 43–44 | Instrument malfunction | PILOTING, RANGED COMBAT, and COMTECH rolls on the ship get a −1 modification. | COMTECH |
| 45 | Intercoms disabled | All internal communications offline. | COMTECH |
| 46 | Ladderway damaged | MOBILITY roll required to move between levels of the ship. | HEAVY MACHINERY |
| 51 | Life support offline | All aboard not wearing a pressure suit are Freezing: STAMINA roll every Shift. | HEAVY MACHINERY |
| 52 | Navigation system offline | COMTECH roll with a −2 modification is required to plot any FTL travel. | COMTECH |
| 53–54 | Reactor leak | Reactor control area: weak radiation (1 Rad/Shift). | HEAVY MACHINERY |
| 55 | Refrigeration unit damaged | After one day, all refrigerated food aboard is lost. | HEAVY MACHINERY |
| 56–61 | Sensor malfunction | −2 to every COMTECH roll that uses the ship's sensors. | COMTECH |
| 62–64 | Thruster damage | Ship's Thrusters rating −2 steps. | HEAVY MACHINERY |
| 65 | Transponder offline | The ship's transponder stops working. | COMTECH |
| 66 | Waste disposal malfunction | Bad smells (flavour only). | HEAVY MACHINERY |

"X + Y" in the repair column = two rolls (one HEAVY MACHINERY and one COMTECH).
- DECIDED (A-35): the book prints "FTL rating halved" for Displacement drive malfunction, but a lower FTL rating is *faster*, so halving would be a bonus. The printed "halved" is treated as a misprint: **the FTL rating is DOUBLED** (FTL speed halved, like towing).

**MAJOR COMPONENT DAMAGE (2D6)** [p. 199] (verified against the image)
| 2D6 | Damage | Effects | Repair roll |
|---|---|---|---|
| 2 | A.I. offline | The ship A.I. is offline. | 3 × COMTECH |
| 3 | Escape pods heavily damaged | All EEVs unusable. | HEAVY MACHINERY, COMTECH |
| 4 | Sensor failure | Sensors totally out; no Target Lock possible. | 2 × COMTECH |
| 5 | Displacement drive failure | No FTL travel possible. | HEAVY MACHINERY, 2 × COMTECH |
| 6 | Armament damaged | One random armament can't be used. | HEAVY MACHINERY, COMTECH |
| 7 | Engine failure | Ship drifts powerless; no pilot actions possible. | 2 × HEAVY MACHINERY, COMTECH |
| 8 | Critical crew injury | One random crew member is Broken and gets a random critical injury. | – |
| 9 | Bridge exposed to vacuum | Ship becomes uncontrollable; everyone on the bridge suffers explosive decompression. | 2 × HEAVY MACHINERY, COMTECH |
| 10 | Reactor breach | No propulsion at all. Reactor control area: strong radiation (1 Rad/Turn); rest of ship: weak radiation. | 2 × HEAVY MACHINERY, COMTECH |
| 11 | Chain reaction | Roll twice on this table. | – |
| 12 | Reactor detonation | Total loss of the ship and all crew. | – |

### 4.16 Ship repairs (permanent) and spare parts [p. 200]
- Repairs require **time, skill, tools and often spare parts**. Tools are listed in Chapter 6, p. 135 (the text says "see Chapter 5"):

| Tool | Effect | Cost |
|---|---|---|
| Watatsumi DV-303 Bolt Gun | HEAVY MACHINERY +2 | $400 |
| Cutting Torch | HEAVY MACHINERY +2 (Power Supply 5) | $300 |
| Maintenance Jack | HEAVY MACHINERY +1 | $150 |
| Electronic Tools | COMTECH +1 | $250 |

  AMBIGUOUS (A-39): it isn't stated whether repairs without tools are possible (or at a penalty) and whether tool bonuses stack.
- **HULL REPAIRS:** one or several **HEAVY MACHINERY** rolls. **One roll per Shift.** **Only one person rolls**; others can help (p. 63). **Each success repairs 1 damage.** **A Disabled ship stops being Disabled once ≥1 point is repaired.** No spare parts are mentioned for hull repairs (AMBIGUOUS, A-40: assume none needed). [p. 200]
- **COMPONENT REPAIRS:** the damage tables list the required rolls and how many. **With several rolls listed: max one roll per Shift.** **A given component is repaired by one roller only**; others can help. **Spare parts are required:** **one mechanical spare part per HEAVY MACHINERY roll, one electronic spare part per COMTECH roll. The part is consumed if the roll succeeds.** (So you need the part in stock to attempt the roll; it's kept on failure.) [p. 200]
- **EMERGENCY REPAIRS** (in combat): see §4.12 (1 Turn per roll, no parts, holds 1 Shift, not hull). [p. 200]
- **EVA REPAIRS:** **hull repair and repair of external components require spacewalking**. If you can't, find a space station with a dry dock. EVA Specialist talent: +2 HEAVY MACHINERY/COMTECH when spacewalking (p. 78). AMBIGUOUS (A-41): which components are "external" isn't listed. [p. 200]
- **PUSHING REPAIRS:** if you spend a Shift repairing and push the roll, you don't have time to relieve stress (p. 104) before the next Shift starts (relevant if you repair again in the next Shift). [p. 200]
- **GETTING SPARE PARTS** [p. 200]:
  - Buy: a **mechanical part ≈ $100,000**, an **electronic part ≈ $200,000** (twice that) at an average spaceport ("costs can vary greatly").
  - Scavenge from a derelict: **one Shift + a skill roll per part** (HEAVY MACHINERY for mechanical, COMTECH for electronic), provided the wreck isn't totally destroyed. **Typical total yield: parts = the derelict's full Hull score, half mechanical and half electronic.** AMBIGUOUS (A-42): the split is unclear for odd Hull values, and it's unclear whether a failed scavenge roll uses up a part.

### 4.17 Ship destruction (all paths)
| Cause | Result | Page |
|---|---|---|
| Single attack damage (after armor) ≥ Hull | Ship destroyed, all crew and passengers killed | p. 197 |
| Major component damage 12 "Reactor detonation" (also possible via Chain reaction) | Total loss of the ship and all crew | p. 199 |
| Reactor Overload succeeds and isn't stopped | Explodes at the end of the following Turn, all aboard killed | p. 196 |
| Overcharging: third HEAVY MACHINERY failure | Reactor explodes, ship destroyed, all killed | p. 167 |
| Planetfall: third PILOTING failure | Crashes hard, all on board killed | p. 167 |
| Planetfall: Engine failure / Reactor breach during the landing | Ship crashes (outcome otherwise unspecified) | p. 167 |
| Accumulated damage ≥ Hull | **Disabled** (not destroyed): engines and weapons fail, explosive decompression. Repairable. | p. 197 |

### 4.18 Boarding, escape, end of combat
- **Boarding:** Dock (pilot, opposed) → Open Airlock (engineer, HEAVY MACHINERY) → breach → **personal combat**. Contact range = "ramming and boarding possible". Orbital mines are "very effective against unwanted boarding parties". [p. 189, 193, 196, 176]
- **Escape:** EEVs (§2.5). The panic result "Abandon Ship" sends a character to an EEV, or out through the airlock in a compression suit. Escape pods can be damaged (minor 41, major 3). [p. 173, 199, 201]
- **Personal combat during space combat:** both can happen at once. Recommended: play personal combat (Rounds) to its conclusion before resuming space combat (Turns). The GM may interleave a space Turn mid-fight. [p. 197]
- AMBIGUOUS (A-43): no explicit end-of-combat rule. The implied end conditions are: one side destroyed or Disabled, separation beyond 8 zones, boarding resolved, or an FTL escape. There are no rules for jumping to FTL during combat; the p. 187 anecdote implies it needs "enough distance".

### 4.19 Stress and panic in space combat [p. 193, 201]
- Stress and STRESS LEVELS work as in personal combat, but **Panic Rolls during space combat use the Space Combat Panic Roll table**. **You can't relieve stress during an active space combat.** [p. 193]

**SPACE COMBAT PANIC ROLL** (D6 + STRESS LEVEL; Nerves of Steel −2, p. 78) [p. 201] (verified against the image)
| Result | Effect |
|---|---|
| 1–6 | **Keeping it together.** No effect. |
| 7 | **Nervous twitch.** Your STRESS LEVEL, and that of all friendly PCs in the same compartment, +1. |
| 8 | **Tremble.** All skill rolls −2 until your panic stops. |
| 9 | **Ignore orders.** For the coming Turn you won't take any orders, accept help or discuss anything with anyone. STRESS LEVEL +1. |
| 10 | **Hysteric comm chatter.** Communications impossible. Your STRESS LEVEL −1, but the STRESS LEVEL of every friendly PC who hears it +1. |
| 11–12 | **Freeze.** Frozen for one Turn, losing your next action. Your STRESS LEVEL and that of all friendly PCs in the same compartment +1. |
| 13 | **Run to safety.** Abandon your station and escape to a safer part of the ship. STRESS LEVEL −1. Each friendly PC who sees it makes an immediate Panic Roll. |
| 14 | **Abandon ship.** Go to the closest escape pod and launch it; if there's no EEV, don a compression suit and exit via the airlock. Each friendly PC who sees you go makes an immediate Panic Roll. |
| 15+ | **Maximum overload.** The character wrecks their own station: next action lost, plus one fixed major component damage by position (Sensor Operator → Sensor failure; Pilot → Engine failure; Gunner → Armament damaged; Engineer → Reactor breach). |

- AMBIGUOUS (A-44): the **Captain** has no Maximum Overload mapping. For a character holding several positions, the GM picks. The general panic rules still apply: effects of 10+ replace the action, "more panic" escalation, duration until one Turn passes or COMMAND help or Broken (p. 104).

### 4.20 Talents relevant to ships and space combat [p. 76, 78–79]
| Talent | Effect | Page |
|---|---|---|
| Spaceship Commander | As captain in space combat, draw two initiative cards and choose one. | p. 79 |
| Spaceship Gunner | As gunner in space combat, +2 RANGED COMBAT. | p. 79 |
| Spaceship Mechanic | +2 to HEAVY MACHINERY or COMTECH when repairing damage to a spacecraft. | p. 79 |
| Full Throttle (Pilot) | +2 PILOTING for Accelerate or Decelerate actions. | p. 76 |
| Like the Back of Your Hand (Pilot) | +2 PILOTING with one specific chosen craft. Can be taken once per craft. | p. 76 |
| Reckless (Pilot) | Push AGILITY-based skill rolls (incl. PILOTING, RANGED COMBAT) twice. Each push +1 STRESS LEVEL. | p. 76 |
| EVA Specialist (General) | +2 HEAVY MACHINERY and COMTECH when spacewalking (EVA repairs). | p. 78 |
| Fast Reflexes (General) | Two initiative cards in the personal-combat draw (space applicability: A-26). | p. 78 |
| Nerves of Steel (General) | −2 to all Panic Rolls. | p. 78 |

### 4.21 Worked example (p. 202–203): facts the app can use as test cases
- Porphyrion (Conestoga: Hull 12, Armor 10, Thrusters +1) vs Lyssa (Corvus: Hull 6, Armor 5, Thrusters +1, transponder off).
- Setup: detected at 8 zones (Extreme). Porphyrion at −4, Lyssa at +4. Both velocities 2.
- Initiative: GM 6, Galt 8 → GM (Lyssa) acts first in every phase.
- **T1 Sensor:** both Target Lock. Lyssa COMTECH at −2 (Extreme) → 1 success → locked. Porphyrion locks too.
- **T1 Pilot:** Lyssa Accelerate (+1 Thrusters) → 1 success → velocity 3. Porphyrion Decelerate → 1 success → velocity 1. Movement: Lyssa +4 → +1. Porphyrion −4 → −3 (distance 4 = Long).
- **T1 Gunner:** Lyssa fires a Long Lance (+1) → 1 success → 4 damage. Porphyrion's armor roll gets 1 success → 3. 3 < 6 → minor. D66 = 45 → intercoms disabled. Porphyrion fires a Long Lance → 3 successes → 4 + 2 = 6 damage. Lyssa's armor gets 2 → 4. 4 ≥ 3 → major. 2D6 = 3 → escape pods heavily damaged.
- **T1 Engineer:** Lyssa Emergency Repairs, HEAVY MACHINERY 1 success ("work on the EEVs has begun"; COMTECH still needed). Porphyrion COMTECH 2 successes → intercoms fixed.
- **T2 Sensor:** Lyssa Go Dark. Porphyrion must roll Target Lock again at −1 (Long, 4 zones) → 1 success. Go Dark roll gets 0 → lock holds.
- **T2 Pilot:** Lyssa Accelerate → 2 successes → +2 → velocity 5. Porphyrion Decelerate → 0 successes → stays 1. Movement: Lyssa +1 → −4. Porphyrion −3 → −2 (they have passed; distance 2 = Medium).
- **T2 Gunner:** Lyssa Launch Countermeasures. Porphyrion fires a Long Lance → 4 successes. Countermeasures 3 successes → 1 left → hit, 4 damage. Armor 1 → 3. Lyssa total 4 + 3 = 7 ≥ Hull 6 → **Disabled**. (The 3 damage single hit is also ≥ half → major component damage would apply. The example doesn't roll it.)
- Note: the example omits the Long-range −1 on the T1 gunner rolls. Don't treat that omission as a rule.

### 4.22 ROUND PROCEDURE: implementable state machine
```
ENCOUNTER_SETUP
 S1  Assign ships to two sides. Per ship: transponder on/off; sensors on/off; engines on/off
     (may be powered down before the encounter) [p.190].
 S2  Detection: when any opposing ships are within 8 zones, they detect each other automatically.
     Place the two sides at -4 and +4 [p.190]. Ships with transponder ON are identified and can be
     fired on without a lock (DECIDED A-24); silent ships are "ping" and need a Target Lock.
 S3  GM sets the initial approach velocity per ship (default 2; 0 if vectors synched) and each
     ship's movement direction (towards the other side) [p.189].
 S4  Initiative: each ship's captain draws one card 1-10 (Spaceship Commander: draw 2, keep 1).
     Ascending order. Fixed for the encounter (except Maneuver trades) [p.87, 190, 79].
 S5  Assign crew to positions (Captain, Sensor Op, Pilot, Gunner(s), Engineer(s); AI may fill any)
     [p.188].

TURN n (repeat)
 T0  Start of Turn: record positions (for the Ram "start of Turn" check); clear this-Turn locks and
     reactive stances; keep "locked last Turn" flags for auto-lock.
 P1  SENSOR PHASE
     a. Each ship secretly declares: Target Lock(target) | Go Dark(+power down sensors/engines) |
        Power Up Sensors | (no action). Validate prerequisites.
     b. Reveal all declarations simultaneously.
     c. Apply Go Dark power-downs immediately (Signature reduction) (A-29).
     d. Resolve in initiative order: Target Lock -> if the target declared Go Dark, the target rolls
        COMTECH reactively and cancels successes; else if locked last Turn, auto-success; else roll
        COMTECH + range + target Signature + Enhanced Sensors + other mods. >=1 net success = locked
        for this Turn. Power Up Sensors -> sensors online (usable from next Turn).
 P2  PILOT PHASE
     a. Secret declare: Accelerate | Decelerate | Maneuver(target) | Ram(target) |
        Dock(target) | (none). ERRATA (p.192): no Evade action. Blocked entirely if engines off,
        Engine failure, Reactor breach, Bridge exposed, Disabled, or docked to a bigger ship.
     b. Reveal.
     c. Resolve in initiative order (ERRATA: no reactive pilot action remains).
        Ram/Dock: check Contact at start of Turn (Ram) / Contact + equal velocity + docking module
        (Dock).
     d. MOVEMENT: every ship moves velocity zones in its direction (docked pairs move together).
        Recompute pairwise distances and range bands. If off-map and distance <= 8: shift both back.
        If distance > 8: contact lost (A-23).
 P3  GUNNER PHASE
     a. Secret declare (one per ship): Fire Weapon(weapon, target) | Launch Countermeasures(device) |
        (none).
     b. Reveal.
     c. Resolve Fire Weapon in initiative order. For each attack:
        - check lock (DECIDED A-24: none needed if target transponder ON; silent target needs
          one), weapon range >= current range, weapon usable, ammo; consume ammo
        - attacker rolls RANGED COMBAT + Bonus + range mod + talents/damage mods (may push;
          facehugger -> Space Combat Panic Roll)
        - if range >= Short: target's Countermeasures roll (if declared and not a particle beam)
          cancels successes (ERRATA p.192: no Evade roll)
        - if net >= 1: damage = Damage (particle beam: -1 per band beyond Contact) + (net - 1)
        - armor: roll Armor dice (railgun: ceil(Armor/2)); subtract successes; final >= 0
        - if final >= Hull: DESTROYED (A-37 for particle beams)
          elif final >= ceil(Hull/2): major (2D6)
          elif final >= 1: minor (D66, re-roll n/a)
        - add final to accumulated hull damage (not for particle beams); if accumulated >= Hull:
          DISABLED
        - apply component effects immediately (e.g. Sensor failure, Engine failure, Thruster -2,
          crew injury, Reactor detonation)
 P4  ENGINEER PHASE
     a. Secret declare (one per ship): Emergency Repairs(component, skill) | Power Up Engine |
        Open Airlock | Reactor Overload | Stop Overload | (none). Helpers: up to +3.
     b. Reveal.
     c. Resolve in initiative order. Emergency repair success -> mark that roll done; when all
        listed rolls are done, the component works until (now + 1 Shift). Power Up Engine
        -> progress 1/2 or 2/2 (engines online). Open Airlock success -> boarding (switch to
        personal combat). Reactor Overload success -> overloadAt = end of Turn n+1.
 END OF TURN
     - Resolve any reactor overload whose deadline is this Turn -> ship destroyed.
     - Advance the clock by 1 Turn (5-10 min); expire emergency repairs older than 1 Shift.
     - Check end conditions (A-43). Next Turn.
```
Throughout: crew Stress Dice / facehugger → Space Combat Panic table. No stress relief during space combat. Panic effects can cancel or replace actions (Freeze, Ignore Orders, Run to Safety, Abandon Ship, Maximum Overload).

---

## 5. Other ship stat blocks in the book
A grep of the whole book for "Thrusters", "FTL rating", "Hull", "Signature" and "Armor rating", plus the campaign chapter and scenario pages, finds **no further spaceship stat blocks** beyond the six in §3.
- **Conestoga** stat block appears twice: p. 186 and p. 194–195. The p. 195 repeat omits "× 6" on Boosted Displacement Drives.
- **Corvus** stat block appears twice: p. 175 and p. 181. They're identical. The **Bison H** block appears twice: p. 182 and p. 185. They're identical.
- **UD-4L Cheyenne VTOL Dropship** is a **vehicle**, not a spaceship. It uses vehicle stats (vehicle combat rules, p. 114–115) [p. 140, description p. 144]:

| Vehicle | Passengers | Maneuverability | Speed | Hull | Armor | Cost |
|---|---|---|---|---|---|---|
| UD-4L Cheyenne VTOL Dropship | 15 | +2 | 4 | 10 | 7 | $3,100,000 |

  For a Vehicle Bay or hangar, the vehicle bay sizes list ATV, VTOL Gyrocar, APC and Daihotai Tractor (p. 172).
- Ship types named without stats: the Star System Encounters table (§1.5, p. 352). Novgorod Station has a Starcub shuttle in its hangar (p. 358, 365). Suggested campaign ships: Bison G (Space Truckers, p. 327), Conestoga (Colonial Marines, p. 328), Corvus (Explorers, p. 329).

---

## 6. Ship record sheet and space combat map
**SHIP SHEET** [p. 388] fields:
- NAME
- MANUFACTURER
- AI
- CREW
- PASSENGERS
- LENGTH
- FTL RATING
- SIGNATURE
- THRUSTERS
- HULL
- ARMOR
- DAMAGE (a track of **10 check boxes**)
- INTERNAL MODULES (list + SIZE column)
- LEASE COST
- MINOR COMPONENT DAMAGE (list + EFFECT)
- MAJOR COMPONENT DAMAGE (list + EFFECT)
- UPGRADES (list + EFFECT)
- ARMAMENTS (list + BONUS, DAMAGE, RANGE, SIZE)
- SHIP'S LOG (DATE, LOCATION)

Not on the sheet but needed by the rules: class, cost, hardpoint/module slot capacities, spare parts, ammo, power state, transponder.

**SPACE COMBAT MAP** [p. 386–387]: one track of 9 zones labelled −4 … 0 … +4.

---

## 7. Notable cross-references outside this file
- Dehydrated (p. 106).
- Decompression and explosive decompression (p. 107).
- Radiation (Rads), Freezing.
- Critical injuries (p. 99–101).
- Death Roll (p. 99).
- Panic (p. 104).
- Vehicles and vehicle repairs (p. 114–115).
- Medical gear: AutoDoc, MedPod (p. 135).
- Computer mainframes (p. 130).

---

## 8. Ambiguities / Game Mother discretion
| ID | Issue | Recommendation | Page |
|---|---|---|---|
| A-01 | Class C has FTL "–", yet the Starcub (C) has FTL 15, with no upgrade explaining it. | C-class custom ships: no FTL. Prebuilt Starcub: FTL 15 as printed. | p. 166, 183 |
| A-02 | Overcharging: whether a later success after a failure grants the −1 FTL; whether "once per week" is per engineer or per ship. | No bonus after a failure. Limit per ship. | p. 167 |
| A-03 | Whether Thrusters modify planetfall PILOTING. | Apply Thrusters (p. 170 general rule). | p. 167, 170 |
| A-04 | Planetfall success condition and the crash outcome of engine failure / reactor breach. | First successful roll lands. Crash outcome is GM's call. | p. 167 |
| A-05 | NDD: only awake travellers? Effect replacement? Epilepsy duration? | Only non-stasis travellers roll. A new result replaces the old. Effects end with the FTL travel unless stated. | p. 151 |
| A-06 | Only classes C/G/M/R have stats. | Designer limited to C/G/M/R. | p. 166, 172 |
| A-07 | Number of ships a hangar holds. | 1 ship of up to the listed class. | p. 171–172 |
| A-08 | "EEV II" = Class B or Class C. The Bison H uses Starcub shuttles as EEV II. | Let the user choose B or C. Keep the prebuilt text as printed. | p. 172, 182 |
| A-09 | Capacity limits (air, cryo, galley, EEV seats) vs crew/passengers. | Warn only. | p. 171–172 |
| A-10 | AI skill values = dice pools? Gunner without RANGED COMBAT? No HEAVY MACHINERY. | Treat as total dice pools. Non-9000 AIs can't gun or do HM actions without GM approval. | p. 130, 179, 188 |
| A-11 | No price for individual orbital mines. | GM-set (configurable). | p. 176–177 |
| A-12 | Do countermeasures get consumed? Does each missile shot use 1 missile? | Countermeasures reusable. 1 missile per Fire Weapon. | p. 176 |
| A-13 | Dry-dock duration and rush cost have no formula. | "1 week+" free text. GM sets. | p. 177 |
| A-14 | "Current biggest hardpoint/module": slot or installed item? Added Hardpoints on a class C? Boosted Drives without FTL? | Biggest slot. Not for C (no hardpoints). No Boosted Drives without an FTL drive. | p. 178 |
| A-15 | Maintenance: one or two minor damage rolls if both service rolls fail. | One D66 per failed or skipped roll. | p. 179 |
| A-16 | Are class stats fixed for custom ships? Do crew/length vary? Does class cost include core components? | Class row = fixed base. Crew/length informational. Class cost includes core components. | p. 166 |
| A-17 | No mandatory modules stated (EEVs legally required but not enforced). | Warnings for missing EEV capacity, Air Scrubbers, docking/hangar. | p. 171 |
| A-18 | No total cost formula. | Class + modules + AI + armaments + ammo + upgrades. | p. 166–178 |
| A-19 | Corvus "Reinforced Frame II" vs Hull 6 (one purchase). | Treat as ×1. | p. 175, 181 |
| A-20 | Conestoga front/rear blind spot (anecdote). | Not a rule. No facing. | p. 187 |
| A-21 | Prebuilt prices and the Conestoga's stats/slots/hardpoints don't follow the class template. | Load prebuilt ships as fixed records. Skip validation or flag "exceeds template". | p. 180–186 |
| A-22 | "One action per Turn" vs one person filling several positions. | One action per position (phase) per Turn. A multi-role person acts in each of their phases. | p. 188, 191 |
| A-23 | Negative velocity / reversing. Distance > 8 zones. Multi-ship tracking. | Velocity ≥ 0. Beyond 8 zones = contact lost (GM). Distances per pair. | p. 189, 193 |
| A-24 | Transponder-on target: targetable without Target Lock? Turning the transponder off mid-combat? | DECIDED: an active transponder replaces the Target Lock (fire without a lock); running silent requires a lock. Switching off: free choice (recommendation). | p. 190, 192, 196 |
| A-25 | Power-down Signature reduction **and** Target Lock table rows (−1/−2) would double count. | Apply once (use current Signature). | p. 190, 192 |
| A-26 | Fast Reflexes for a captain's initiative draw? | GM option, default off. | p. 78–79 |
| A-27 | When a Maneuver initiative trade takes effect. | From the next phase. | p. 87, 193 |
| A-28 | Auto-lock persistence (skipped Turns, target changes). | Auto only if locked in the immediately preceding Turn and the target doesn't Go Dark. | p. 191 |
| A-29 | Go Dark vs several incoming locks. When the Signature reduction applies. | One reactive roll per incoming lock. Reduction applies before enemy lock rolls. | p. 191 |
| A-30 | RESOLVED. Evade had no text description (timing, Thrusters, per-attack rolls, stacking). | ERRATA (p. 192): Evade is not an action; removed from this spec. | p. 192, 196 |
| A-31 | Ram: net or gross successes? Armor? Component damage? | Net successes. Armor applies. Component damage per thresholds. | p. 193 |
| A-32 | Dock: "bigger ship" definition, cooperative docking, equal velocities, joint movement. | Bigger = higher class, then length. Cooperative = no roll. Move together. | p. 193 |
| A-33 | Countermeasures: which device if several. Requires a fitted device. | Choose the device when declaring. Required. | p. 196 |
| A-34 | Emergency/permanent repair: retry after failure, progress kept. Non-repairable results. | Retry next Turn/Shift, keep completed rolls. "–" results can't be repaired. | p. 196, 200 |
| A-35 | "Displacement drive malfunction: FTL rating halved" is a speed-up under the rating semantics. | DECIDED: FTL rating DOUBLED (×2); the printed "halved" is a misprint. | p. 198 |
| A-36 | Cap on accumulated hull damage. The sheet has 10 boxes (R-class Hull is 12). | Cap at Hull. Allow > 10. | p. 197, 200, 388 |
| A-37 | Particle beam: can damage ≥ Hull destroy? Does armor apply? | Components only (minor/major), never destruction or hull points. Armor applies. | p. 176, 197 |
| A-38 | Re-roll inapplicable major results. Chain reaction rolling 11 again. | Re-roll inapplicable. 11 again = roll again. | p. 197, 199 |
| A-39 | Tools: required? Stacking? | Optional bonus. Use the best single tool per roll. | p. 135, 200 |
| A-40 | Spare parts for hull repairs. | Not needed. | p. 200 |
| A-41 | Which components are "external" (EVA needed). | GM decides (likely thrusters, sensors, comms, armaments, hull). | p. 200 |
| A-42 | Scavenging: odd Hull split. Failed roll consequences. | Floor/ceil split (GM). Failure = no part, Shift spent. | p. 200 |
| A-43 | No explicit end-of-combat or FTL-escape rule. | GM ends the encounter. Offer "contact lost" at > 8 zones. | p. 187–203 |
| A-44 | Maximum Overload has no Captain mapping. Multi-role persons. | GM picks the result. | p. 201 |

---

## 9. Time-related rules
| Rule | Duration / interval | Page |
|---|---|---|
| Space combat time unit | **Turn** (5–10 minutes), not Rounds | p. 188, 83 |
| Space combat Turn structure | 4 phases: Sensor → Pilot (+movement) → Gunner → Engineer | p. 190 |
| Initiative | Drawn once per encounter | p. 190, 87 |
| Power Up Sensors | 1 Turn (sensor operator action) | p. 190–191 |
| Power Up Engine | 2 Turns (two successive engineer actions) | p. 190, 196 |
| Reactor Overload | Explodes at the end of the **following** Turn. Stopping it takes 1 Turn (HEAVY MACHINERY) | p. 196 |
| Emergency repair | 1 Turn per roll. The fix holds **1 Shift** | p. 196, 200 |
| Permanent hull / component repair | 1 roll per **Shift** | p. 200 |
| Scavenging spare parts | 1 Shift per part | p. 200 |
| Regular maintenance | **Weekly**: 1 HEAVY MACHINERY + 1 COMTECH roll, each 1 Shift | p. 179 |
| Overcharging engines | 1 Shift of work. Effect lasts **1 week**. Max **once per week** | p. 167 |
| Planetfall | ≈ 1 Shift per attempt. After an abort, retry next Shift | p. 167 |
| FTL travel time | days = FTL rating × parsecs | p. 167 |
| NDD rolls | Every [FTL rating] days of FTL travel, +1 cumulative | p. 151 |
| NDD Paranoia | Until the FTL travel ends | p. 151 |
| NDD Epilepsy | Stress Die each Shift. A seizure disables for a full Turn | p. 151 |
| NDD Dementia | D6 days after the FTL travel ends | p. 151 |
| Rude awakening, Broken | Death Roll every Turn | p. 151 |
| Hypersleep (any length) | Dehydrated on waking unless Hydr8tion was taken | p. 151, 106 |
| Air scrubbers offline | −2 STAMINA for 1 Shift, then total air loss | p. 198 |
| Life support offline | STAMINA roll each Shift (Freezing) | p. 198 |
| Reactor leak / breach | 1 Rad/Shift (leak). 1 Rad/Turn in the reactor area (breach) | p. 199 |
| Refrigeration unit damaged | Food lost after 1 day | p. 199 |
| Panic: Ignore Orders / Freeze | The coming Turn / one Turn | p. 201 |
| Stress relief | Not possible during active space combat. Pushing repairs blocks relief before the next Shift | p. 193, 200 |
| Dry-dock refit | A week or more | p. 177 |
| Lease | 2% of total price per **year** | p. 168 |
| Living expenses / salaries | Per **week** | p. 160, 162 |
| EEV endurance | A: semi-stasis ≤ 1 week. B: 10 days. C: crew in stasis 50+ years, FTL 1.4 parsecs. D: 3 people 2 years or 20 in cryo 50 years. Corvus lifeboat: up to 3 months | p. 173, 180 |
| Starcub range | 13 parsecs before refuelling | p. 183 |
| Star system encounter rolls | Before and after hyperspace. In-system daily / weekly / monthly | p. 351 |

---

## 10. State the app must track

### 10.1 Per ship: design (static)
- Identity: name, model/number, class (C/G/M/R or "custom template" / "exception"), manufacturer, role, prebuilt flag.
- AI mainframe (model and its skill values) or none.
- Crew count, passengers, troop capacity, length.
- Base class values. Upgrade counts (Added Hardpoints by size, Armored Bulkheads, Boosted Displacement Drives, Enhanced Sensors, External Modules by size, Overpowered Thrusters, Planetfall Capacity, Reinforced Frame, Stealth Technology).
- Derived stats: FTL, Signature, Thrusters, Hull, Armor, Target Lock bonus, planetfall capability.
- Module slot capacities per size (class + external). Installed modules (type, size, capacity/variant, e.g. EEV class, Hangar contents, Vehicle Bay contents, Medlab equipment). Slot assignment.
- Hardpoint capacities per size (class + added). Installed armaments (type, size, Bonus, Damage, Range, special rule flags: armor-piercing, particle beam, countermeasure, surface-only).
- Price (listed for prebuilt, computed for custom), lease cost per year, validation warnings.
- Ship's Log entries (date, location) (sheet field).

### 10.2 Per ship: runtime (persistent between sessions)
- Hull damage (0…Hull), **Disabled** flag, **Destroyed** flag.
- Active component damage entries: table (minor/major), result id, the random target chosen (which armament, compartment, crew member, hangar vessel, cargo item), repair rolls required vs completed, emergency-repair status and expiry time.
- Spare parts inventory: mechanical and electronic counts. Tools aboard.
- Ammo: missiles per launcher, nukes, mines.
- Power state: sensors on/off, engines on/off, engine power-up progress (0/1/2). Transponder on/off (and "Transponder offline" damage).
- FTL modifiers: towing (Tractor Hitch in use), overcharge active-until date, last overcharge date. Navigation offline (−2 to plot). Drive failure.
- Maintenance: current week, HEAVY MACHINERY service done/failed, COMTECH service done/failed.
- Docked-with ship id. Airlock breached flag. Location / in hypersleep. Current FTL trip (parsecs, days elapsed, NDD roll count per awake traveller).

### 10.3 Per crew position (in a space combat)
- Position type (Captain, Sensor Operator, Pilot, Gunner[n], Engineer[n]) and the assigned character(s) or AI. One character may hold several positions.
- For the acting character: relevant skill dice (COMTECH, PILOTING, RANGED COMBAT, HEAVY MACHINERY), attribute, talents (Spaceship Commander/Gunner/Mechanic, Full Throttle, Like the Back of Your Hand + which craft, Reckless, EVA Specialist, Nerves of Steel, Fast Reflexes), STRESS LEVEL, current panic effect and its duration, Health / Broken, pushes used this roll.
- Helpers assigned (Engineer; max 3 / +3).
- Gunners: which weapon or countermeasure each is assigned to.
- Declared action this phase (hidden until reveal) and its parameters (target, weapon, component, skill).
- Whether the person is following orders (disobedience, Ignore Orders panic).

### 10.4 Per space combat (encounter)
- Sides and the member ships of each.
- Turn number. Current phase (Sensor/Pilot/Gunner/Engineer). Sub-state (declaring / revealed / resolving / movement).
- Initiative card per ship (captain's draw, alternate card if a talent applies). Resolution order. Pending initiative trades (Maneuver).
- Track positions (zone −4…+4 with map offset handling), per-ship direction, per-ship approach velocity. Pairwise distance and range band. Positions at start of Turn (for Ram).
- Detection and identification status per pair (ping vs identified; transponder visible).
- Target locks: lock(attacker → target) for this Turn, and "locked last Turn" flags for auto-lock.
- Reactive stances this Turn: Go Dark (and power-downs applied), Launch Countermeasures (device). ERRATA (p. 192): no Evade.
- Attack log: per attack, the roll, cancellations, damage, armor, final damage, component result.
- Reactor overload timers (ship, deadline Turn). Emergency repair expiries (in Shifts relative to the game clock).
- Docking and boarding state (docked pairs, airlock breached → hand off to personal combat).
- Game clock (Turns elapsed, converted to Shifts for repair expiry).
- End-of-encounter status (destroyed / disabled / contact lost / boarded / fled).
