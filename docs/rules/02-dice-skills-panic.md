# 02 — Dice Engine, Skills, Stress & Panic

> Official errata and project decisions in [00-errata-und-festlegungen.md](00-errata-und-festlegungen.md) take precedence over this file.

Source: ALIEN RPG Core Rulebook (Free League, 1st ed. 2019). All page references are **book pages** (PDF page − 4).
Core scope: book pp. 18–23, 56–71, 103–105 (+ trauma table p. 101), 211–213. Rules elsewhere in the book that feed the dice engine, stress or panic are included as cross-references, with their pages.

---

## 0. Conventions and glossary

- **Base Die**: a six-sided die. A 6 shows the **success** symbol. 1–5 are blank for skill rolls. [p. 22]
- **Stress Die**: a six-sided die of a different colour. A 6 shows the **success** symbol and a 1 shows the **facehugger** symbol, which triggers panic. [p. 22, 58]
- In this spec, **success** means a 6 on any die (Base or Stress) and **facehugger** means a 1 on a Stress Die. A 1 on a Base Die has no meaning. [p. 22, 58]
- You can play with ordinary d6s in two colours. [p. 22, 58]
- **Dice pool**: all Base Dice plus all Stress Dice rolled together in one skill roll. [p. 58]
- **Modification**: a signed integer that adds or removes Base Dice. [p. 62]
- **Stunt**: a bonus effect bought with each success beyond the first. [p. 58, 64–71]
- **STRESS LEVEL**: an integer per PC. It is the number of Stress Dice the PC adds to every skill roll. [p. 61, 103]
- **Panic Roll**: D6 + current STRESS LEVEL, read on the Panic table. [p. 104–105]
- **Slow action / fast action**: the combat action economy. On your initiative you get one slow + one fast action, or two fast actions. [p. 87]
- **Time units**: Round 5–10 s (combat); Turn 5–10 min (stealth); Shift 5–10 h (recovery). Usually 4 Shifts/day: Morning, Day, Evening, Night. GM rules when each Round, Turn or Shift has elapsed. [p. 83]
- **Act**: one of three acts of a Cinematic scenario (Setup, Tilt, Showdown). **Game session**: one evening of play. [p. 216, 31]
- **Range terms** used here: ENGAGED (right next to you), SHORT (a few metres, same zone), MEDIUM (up to 25 m, adjacent zone), LONG (up to ~100 m, four zones), EXTREME (up to ~1 km). [p. 83]

---

## 1. Game modes (rule differences only)

- There are two modes, **Cinematic play** (a one-session scenario, high lethality) and **Campaign play** (a continuous campaign with PCs you create). In the book, mode-specific rules are marked with a CIN or CAM symbol. [p. 19]
- No dice-engine rule in pp. 18–23 differs by mode. These are the mode differences that touch this spec's scope:

| Rule | Cinematic | Campaign | Page |
|---|---|---|---|
| Story Points (one automatic success per point spent) | Available. Max 3 per player. | Not available. | p. 61, 31, 383 |
| Personal Agenda reward | 1 Story Point per Act if the GM judges the PC advanced the Agenda | 1 bonus XP per session for concrete action despite risk or cost | p. 31, 216 |
| Signature item stress relief (−1 STRESS LEVEL) | once per **Act** | once per **game session** | p. 104 |
| Talent The Long Haul (ignore all facehuggers from a single roll) | once per Act | once per game session | p. 76 |
| XP question "Did you make a Panic Roll?" | n/a (no XP) | +1 XP after the session | p. 35 |

- Story Points belong to the **player**, not the PC, and carry over to the next Cinematic scenario. They are capped at 3. [p. 383]

---

## 2. Tools of the game (book pp. 21–22)

### 2.1 Dice notation [p. 22]
| Notation | Procedure | Range |
|---|---|---|
| D6 | Roll one Base Die and read the number, **ignoring symbols** | 1–6 |
| D3 | Roll a D6, divide by 2, **round up** (1–2→1, 3–4→2, 5–6→3) | 1–3 |
| 2D6 | Roll two D6 and add them | 2–12 |
| D66 | Roll two D6. The first is the tens digit, the second the ones digit. | 11–66 (36 outcomes, digits 1–6 only) |
| (3D6, used for healing times) | Roll three D6 and add them | 3–18 |

- You roll dice only for a **dramatic action** where the outcome is uncertain and might decide whether the PC lives or dies. [p. 22, 61]

### 2.2 Cards [p. 22]
- An optional custom card deck has two uses: (1) **drawing initiative** in combat, and (2) tracking key gear, weapons and vehicles. It is not required. [p. 22]
- Initiative, for cross-reference (full rules belong in the combat spec): use ten cards numbered 1–10, or a normal deck with the ace as 1. Each PC draws one card. The GM draws one per NPC, or one per group of NPCs with identical stats when there are more than 10 combatants. Order runs **lowest to highest**. Initiative is drawn only once, at the start of the first Round. Two PCs may swap cards at the start of the fight or of a Round, never during a Round, and only if they can talk to each other. [p. 87]
- Tip: turn the card 90° per action taken (left for a fast action, right for a slow action). [p. 89]
- Talent Fast Reflexes: draw 2 cards, keep one, and shuffle the other back before others draw. [p. 78] Talent Spaceship Commander: in space combat, draw 2 and keep one. [p. 79] Xenomorphs with Speed N draw N cards. [p. 112]

---

## 3. Rolling dice (skill rolls)

### 3.1 When to roll
- Describe what you attempt first, then roll, then interpret the result. If you push, describe how. The GM stops you only if you narrate beyond what you rolled. [p. 60]
- Roll only when it is really necessary (dramatic situations or tough challenges). Otherwise the GM lets the action simply happen. [p. 61]
- NPCs roll only for actions that **directly affect a PC** (attacking or saving one). Otherwise the GM decides without dice. [p. 62]

### 3.2 Building the dice pool
- **Base Dice** = attribute score + skill level + net modification. [p. 58, 62]
- Each of the 12 skills is tied to one of four attributes: STRENGTH, AGILITY, WITS, EMPATHY. Skill levels run **0–5**. [p. 58]
- Attribute range for Campaign PCs: 2–4, or 5 in the career's key attribute. Androids get +3 to two attributes, up to a max of 8. Starting skill points: 10, max 3 per career skill, max 1 in any other skill. (This belongs to character creation; listed here only for pool bounds.) [p. 28]
- **Rolls without skill**: at skill level 0 you still roll, using Base Dice = attribute + gear modifiers, plus any Stress Dice. [p. 58, 28]
- **Stress Dice** = the PC's current STRESS LEVEL. You **must** add them to every skill roll, including the re-roll when pushing. [p. 61, 103]
- **Gear** can add Base Dice (a gear bonus works as a positive modification). Some actions simply require a specific piece of gear. [p. 59, 62]
- Roll all dice together. [p. 58]

### 3.3 Success, stunts, failure
- **Success**: at least one success (a 6) among all dice, Base or Stress. Otherwise the action **fails**. [p. 58]
- **Stunts**: if you roll more than one success, each success beyond the first buys one stunt from the skill's list (see §4). [p. 58, 64]
- **Failure**: something goes wrong. The GM has the final say on consequences and may add further dramatic consequences. Failure must never stop the story completely: there must be a way forward, perhaps at a cost in time, risk or UA dollars. [p. 59]
- After a failure you have one last chance: you may **push** the roll (§3.4). [p. 59]

### 3.4 Pushing the roll
- **Who may push**: PCs only. NPCs never push. [p. 62] Androids (PC or NPC) **cannot push** skill rolls. [p. 29, 111] In an opposed roll, only the attacker may push (§3.14). [p. 64] Some rolls are explicitly unpushable: Death Roll, Resilient talent roll, explosions. [p. 99, 76, 108]
- **When**: usually after a failure, but you may also push a **successful** roll to get more successes (for example, more damage). [p. 60]
- **Restriction**: if the roll shows one or more facehuggers on Stress Dice, you **cannot push**. You must make a Panic Roll immediately instead. [p. 61, 104]
- **Stress cost**: when you push, your STRESS LEVEL immediately goes up by 1, and you record it **before** re-rolling. [p. 61]
- **What is re-rolled**: grab **all dice that did not show a success** (Base and Stress) and roll them again. Dice showing a success stay locked. [p. 60]
- **Extra Stress Die**: since the re-roll is a skill roll and the Stress Dice in a skill roll always equal the current STRESS LEVEL, the push adds **one new Stress Die**. Book example: STRESS LEVEL 2 → push → 8 Base + 3 Stress Dice are re-rolled. [p. 61] The chance table on p. 59 also says the pushed column includes "the added Stress Die from pushing". [p. 59]
- **How often**: once. If the push fails you live with the consequences. [p. 60] Some talents allow a **second push** for skill rolls based on one attribute (§7), and each push costs +1 STRESS LEVEL. [p. 60, 74–77]
- After a push, all successes count: those locked from the earlier roll plus those in the re-roll. A facehugger in the pushed roll triggers a Panic Roll. [p. 61]

### 3.5 Stress Dice in the roll
- Stress Dice make success **more** likely ("sharp and alert"). A facehugger on any Stress Die during a skill roll means you risk panic. [p. 61, 103]
- Androids and NPCs have no STRESS LEVEL, so they roll no Stress Dice. [p. 29, 62, 105]

### 3.6 Panic triggered by the roll
- If one or more facehuggers come up on Stress Dice in a skill roll: no push is allowed, and you must **immediately** make a Panic Roll (§5.4). [p. 61, 104]
- A Panic Roll can make the skill roll fail no matter how many successes you rolled. Specifically, a Panic result of **10 or higher** cancels the action and replaces it with the forced panic action, even if you rolled successes. [p. 61, 104]
- AMBIGUOUS (strongly implied, A40): with a Panic result of 1–9, the skill roll keeps its normal outcome, because only 10+ is said to cancel the action. [p. 104]
- **Ammo**: if the action is firing a weapon with a limited magazine, one or more facehuggers also **empty the magazine** after the attack, in addition to the Panic Roll. Reloading is a slow action. Track full reloads. [p. 61, 96]
- Androids have no STRESS LEVEL, so they never run out of ammo this way. NPCs run out of ammo when the GM finds it dramatically appropriate. [p. 96]

### 3.7 Story Points (Cinematic only)
- Spend 1 Story Point to get **one automatic success** in a dice roll. You may spend it after a failed roll, or after a successful roll to get an extra success (for example, for a stunt). [p. 61]
- You gain them by following your Personal Agenda: the GM evaluates at the end of each Act and awards 1 point without saying why. Max **3** at any time. [p. 61, 31, 216]
- DECIDED: Cinematic play only; **max 1 Story Point per roll**; **skill rolls only**. Note: the book does not limit points per roll or roll types.

### 3.8 NPCs, androids, machines
- NPCs use skills like PCs, except that they **never push**. They have no STRESS LEVEL, never make Panic Rolls, and panic only when the GM decides. [p. 62, 105]
- The GM may roll a fixed pool for an NPC, creature or AI (for example, "Mother's 8 Base Dice", or a creature attacking with ten Base Dice). [p. 64, 295+]
- Androids: no pushing, no STRESS LEVEL, no Panic Rolls, no signature item. [p. 29, 111]
- Machines with a skill (Pauling MedPod: MEDICAL AID level 10; AutoDoc: level 6) roll the **skill level only** and cannot push. [p. 136]

### 3.9 Only one chance
- Once you have rolled (and pushed), you cannot roll again for the **same goal**. You must try something different, wait until circumstances change substantially, or let another PC try. **This does not apply in combat** (you may attack the same enemy repeatedly). [p. 62]

### 3.10 Group rolls
- When the PCs face a challenge together, they **do not roll separately**. They choose the best-suited PC, and the others may help (§3.13). A failure counts for everyone, and no one may try once each. Not in combat, where each PC attacks freely. [p. 62]
- **Group stealth**: when sneaking side by side, only the PC with the **lowest skill level** (MOBILITY) rolls, and the result applies to the whole group. [p. 67]
- **Group observation**: when scouting together, only one PC rolls and the result applies to all. The players choose who. [p. 68]

### 3.11 Modification
- Modification +N gives N extra Base Dice. −N gives N fewer Base Dice. [p. 62]
- All modifications on one roll are **summed** (+2 and −1 make +1). [p. 62]
- If a negative net modification exceeds the available Base Dice, remove **Stress Dice** for the remainder. [p. 62]
- If no dice remain, the action **cannot succeed**. [p. 62]
- Sources include gear, the difficulty of the action, help from others [p. 62], rules such as aimed shots, range and negotiating position [p. 63], and talents [p. 63]. Other sources in the book include panic effect Tremble, critical injuries, zone features, drugs and alcohol.

### 3.12 Difficulty [p. 63]
The GM normally does not rate difficulty, since you only roll in challenging situations. The table is guidance for when the GM wants to stress external factors.

| Difficulty | Modification |
|---|---|
| Trivial | +3 |
| Simple | +2 |
| Easy | +1 |
| Average | 0 |
| Demanding | −1 |
| Hard | −2 |
| Formidable | −3 |

### 3.13 Help from others
- PCs or NPCs can help with a skill roll. Help must be **declared before the roll**, must make sense in the story, and the helper must be able to support the action. The GM has the final say. [p. 63]
- Each helper gives **+1**. At most **3 helpers**, so the maximum help modification is **+3**. [p. 63]
- In combat, helping counts as the same type of action (fast or slow) as the action being supported. [p. 63]
- NPCs can help each other the same way. Acting in groups is a handy way to run many NPCs. [p. 63]

### 3.14 Opposed rolls
- To win, the active party ("attacker") must roll **at least one success and more successes than the adversary**. Each adversary success **cancels** one of the attacker's successes. [p. 64]
- **Only the attacker may push.** [p. 64]
- The two sides may roll different skills or the same skill. Common pairings: MANIPULATION vs MANIPULATION; MOBILITY (sneak) vs OBSERVATION; STAMINA vs STAMINA (arm wrestling). The GM may call for an opposed roll whenever appropriate. [p. 64, 66, 70]
- A tie means the attacker fails. Book example: Charlize rolls 4 Base Dice (WITS 4, no COMTECH) against Mother's 8 Base Dice. Both get 2 successes, they cancel, and the attempt fails. [p. 64]
- Stunts in an opposed roll (explicit for MANIPULATION): each success beyond what you needed to win buys one stunt, so stunts = attacker − defender − 1. [p. 70]

### 3.15 Chance of success (reference table) [p. 59]
| Dice | Chance of success | Pushed roll (incl. the added Stress Die) |
|---|---|---|
| 1 | 17% | 29% |
| 2 | 31% | 50% |
| 3 | 42% | 64% |
| 4 | 52% | 74% |
| 5 | 60% | 81% |
| 6 | 67% | 87% |
| 7 | 72% | 90% |
| 8 | 77% | 93% |
| 9 | 81% | 95% |
| 10 | 84% | 96% |

- The first column equals 1 − (5/6)^n. AMBIGUOUS: the pushed column does not match a simple model. Rerolling n dice gives 1 − (5/6)^(2n) = 31%, 52%, … and rerolling n+1 dice gives 42%, 60%, …. The book's assumptions are not stated, so use it only for display, not as a test oracle. [p. 59]

### 3.16 Other roll types the engine must support (cross-references)
| Roll type | Dice | Stress Dice? | Pushable? | Facehugger effect | Page |
|---|---|---|---|---|---|
| Skill roll (incl. skill 0) | attr + skill + mods | yes (= STRESS LEVEL) | yes (PC, once; twice with talent) | Panic Roll; no push | p. 58–61 |
| Coup de grâce EMPATHY roll | EMPATHY only | **no** (explicit) | not stated | – | p. 99 |
| Death Roll | STAMINA (attr + skill) | **no** (explicit) | **no** | – | p. 99 |
| Permanent trauma / Nightmares / Depression EMPATHY roll | EMPATHY only | AMBIGUOUS | AMBIGUOUS | – | p. 101, 104 |
| Resilient talent roll | STRENGTH only | not stated | **no**, not an action | – | p. 76 |
| Armor roll / cover roll | Base Dice = Armor Rating; each success removes 1 damage | no | no | – | p. 96, 98 |
| Supply roll | **Stress Dice** = Supply rating, **max 6** | n/a | no | each facehugger lowers Supply by 1; AMBIGUOUS (A33): **not** a Panic trigger (not a skill roll) | p. 34 |
| Radiation damage roll | dice = current Rads; each success = 1 damage | – | – | – | p. 110 |
| Permanent radiation check | 1 Stress Die when about to heal a Rad | – | – | facehugger → the Rad becomes permanent | p. 110 |
| NDD Epilepsy check | 1 Stress Die per Shift | – | – | facehugger → seizure (disabled a full Turn) | p. 151 |
| Ovomorph release at SHORT range | 1 Stress Die per Round | – | – | facehugger → Facehugger released | p. 300 |
| Fixed NPC/creature/AI pool | N Base Dice | no | no | – | p. 64, 295+ |
| Panic Roll | D6 + STRESS LEVEL (+ mods) | – | – | – | p. 104 |

---

## 4. The skills

### 4.0 Overview [p. 58]
| Skill | Attribute | Typical opposed by | Stunt basis |
|---|---|---|---|
| Heavy Machinery | STRENGTH | – | each extra success beyond the first |
| Stamina | STRENGTH | STAMINA (e.g. arm wrestle) | each extra success |
| Close Combat | STRENGTH | – (block is a reaction); CLOSE COMBAT to break a grapple | each extra success |
| Mobility | AGILITY | OBSERVATION (stealth) | each extra success |
| Ranged Combat | AGILITY | – | each extra success |
| Piloting | AGILITY | – | each extra success |
| Observation | WITS | MOBILITY (spotting a sneaker) | each extra success = one question |
| Comtech | WITS | – (GM may oppose with an AI's pool) | each extra success beyond the first |
| Survival | WITS | – | each extra success |
| Command | EMPATHY | target's MANIPULATION (Pull Rank) | no stunt list; orders use **every** success |
| Manipulation | EMPATHY | MANIPULATION | each success beyond what was needed to win |
| Medical Aid | EMPATHY | – | no stunt list; Recovery uses **every** success |

General stunt rules: choose one stunt per qualifying success, and it must suit the situation. The book says a stunt may be chosen several times only for "+1 damage". AMBIGUOUS whether other stunts can repeat; the suggested default is no. [p. 64–71]

### 4.1 HEAVY MACHINERY (STRENGTH) [p. 64]
- **Use**: using, repairing, jury-rigging or breaking any kind of heavy machinery. [p. 64]
- **Failure**: the machine won't cooperate, and the noise may attract unwanted company. [p. 64]
- **Success**: the machine bends to your will. [p. 64]
- **Stunts** (each extra success beyond the first, one applicable stunt): [p. 64]

| Stunt | Effect |
|---|---|
| Follow-up bonus | +1 modification to a later skill roll related to this one |
| "You got this" | You need not roll to overcome the **exact same** challenge again |
| Quick | Done in **half** the normal time |
| Break it | You break it permanently |
| Quiet | You act quietly |
| Show off | You show off |

- Cross-references: EVA Specialist talent +2 to HEAVY MACHINERY when spacewalking [p. 78]. Spaceship Mechanic +2 when repairing spacecraft [p. 79]. Maintenance jack +1, bolt gun / cutting torch +2 in relevant situations [p. 135]. Ship repairs use 1 mechanical spare part per HEAVY MACHINERY roll [p. 200].

### 4.2 STAMINA (STRENGTH) [p. 65]
- **Use**: when physical endurance or vigour is tested, e.g. surviving the cold vacuum of space or resisting a deadly contagion. [p. 65]
- **Failure**: you give in to the pain and suffer the consequences. [p. 65]
- **Success**: you push on a little longer. [p. 65]
- **Stunts** (each extra success): [p. 65]

| Stunt | Effect |
|---|---|
| Share | Give one success to another PC in the same situation as you |
| Follow-up bonus | +1 modification to a later related skill roll |
| Hardened | You need not roll to overcome the exact same challenge in the future |
| Impress | You impress someone |

- Cross-references: the Death Roll is a STAMINA roll with no push and no Stress Dice [p. 99]. Stoic talent lets you roll STAMINA with WITS instead of STRENGTH [p. 79]. Also used for vacuum, freezing, starving, exhaustion, drowning and suffocation [p. 106–110].

### 4.3 CLOSE COMBAT (STRENGTH) [p. 66; combat version p. 92]
- **Use**: attacking someone in close combat (Chapter 4/5). [p. 66]
- **Failure**: you stumble and miss, and now it's the opponent's turn. [p. 66]
- **Success**: hit for damage equal to the weapon's Damage rating (p. 118). [p. 66]
- **Stunts** (each extra success, choose one of these): [p. 66, 92]

| Stunt | Effect (p. 66) | Combat-chapter wording (p. 92) |
|---|---|---|
| +1 damage | Inflict one more point of damage. **May be chosen multiple times.** | same |
| Out-maneuver | Swap initiative scores with the opponent from the **next Round** (the text says "next turn"). You can't go back to your earlier initiative. | "taking effect next Round" |
| Disarm | Knock or pull a weapon or other object from the opponent; you choose which. Picking up a dropped object in combat is a fast action. | same |
| Knock down | Opponent is knocked to the ground | "falls prone"; **humanoid opponents only** |
| Grapple | Hold the opponent in a grapple. To break free they must win an opposed CLOSE COMBAT roll against you, and they can take no other action until then, or until you are Broken or let go. **Humans and synthetics only.** | "pin your enemy in a tight clinch, see Grappling" |

- Grappling detail (cross-reference): both fall to the ground, and the opponent drops any held weapon and cannot move. Breaking free is a **slow** action that succeeds if they win an opposed CLOSE COMBAT roll. While grappling, your only action (besides releasing) is a grapple attack: an unarmed attack that is a **fast** action and cannot be blocked. [p. 93]
- **Weapons**: clubs, knives, power drills, etc. [p. 66, 127] **Blocking**: you can block close combat attacks (p. 92: a fast reactive action). [p. 66]
- Cross-references: Hard Hitter +2 if you sacrifice your fast action [p. 78]. Subdue +2 to grapple instead of damage, and extra successes have no effect [p. 74]. Flyweight / Dodge are substitutes for blocking [p. 78, 75].

### 4.4 MOBILITY (AGILITY) [p. 66]
- **Use**: getting out of a hazardous situation (risky climb, dangerous jump, sneaking past a lurking enemy). **Stealth** is an **opposed roll vs the opponent's OBSERVATION**. It has specific uses in stealth mode and combat (retreat, cluttered zones, climbing into a space suit, over-encumbered moving). [p. 66, 85, 89, 93, 34, 107]
- **Failure**: you fail and suffer the consequences. [p. 66]
- **Success**: you survive the sticky situation. [p. 66]
- **Stunts** (each extra success): [p. 66]

| Stunt | Effect |
|---|---|
| Share | Give one success to another PC in the same situation as you |
| Follow-up bonus | +1 modification to a later related skill roll |
| Impress | You impress someone |

- **Group stealth**: one roll, by the PC with the lowest skill level; outcome counts for everyone. [p. 67]
- Cross-references: Stealthy talent +2 to move undetected [p. 79]. Zero-G Training +2 in zero-G [p. 79].

### 4.5 PILOTING (AGILITY) [p. 67]
- **Use**: any difficult and dangerous manoeuvre at the helm of any spacecraft. Also driving ground vehicles. [p. 67]
- **Failure**: coming in too hot, you crash and burn. [p. 67]
- **Success**: you pull off the manoeuvre by the skin of your teeth. [p. 67]
- **Stunts** (each extra success): [p. 67]

| Stunt | Effect |
|---|---|
| Follow-up bonus | +1 modification to a later related skill roll |
| Show off | You show off |

- Cross-references: Full Throttle +2 to Accelerate/Decelerate in spacecraft [p. 76]. Like the Back of Your Hand +2 with one specific craft [p. 76]. Drive is a fast action using PILOTING [p. 88].

### 4.6 RANGED COMBAT (AGILITY) [p. 67; combat version p. 95]
- **Use**: firing all types of ranged weapons (Chapter 4/5). [p. 67]
- **Failure**: the shot misses, may hit something else, and gunfire may attract attention. [p. 67]
- **Success**: hit for the weapon's Damage rating (p. 118). [p. 67]
- **Stunts** (each extra success, choose one of these): [p. 67, 95]

| Stunt | Effect (p. 67) | Combat-chapter wording (p. 95) |
|---|---|---|
| +1 damage | One more point of damage. **May be chosen multiple times.** | same |
| Pin down | Target must make an **immediate Panic Roll** | **PCs** make an immediate Panic Roll. **NPCs instead miss their next slow action.** |
| Reposition | Swap initiative scores with the enemy from the next Round (text: "next turn"). You can't go back. | "taking effect next Round" |
| Disarm | Target drops a weapon or other hand-held object; you choose which | same |
| Knock back | Opponent falls to the ground or is pushed back (e.g. through an airlock) | same |

- **Taking cover**: see p. 86/96 (cover Armor Rating). [p. 67]
- Cross-references (combat spec): ranged modifications [p. 95]. Aimed shot +2 [p. 94–95]. Full auto: +2, STRESS LEVEL +1 immediately, and the extra success goes to secondary targets [p. 96]. Ammo / facehugger [p. 96]. Rapid Fire / Weapon Specialist / Spaceship Gunner [p. 79].

### 4.7 OBSERVATION (WITS) [p. 68]
- **Use**: spotting someone sneaking (opposed by their MOBILITY), or learning more about an unknown threat you've spotted. [p. 68]
- **Failure**: you can't make it out, or mistake it for something else. The **GM feeds you false information**. [p. 68]
- **Success**: you make out what it is and whether it looks like a threat. The exact information is up to the GM. [p. 68]
- **Stunts**: each extra success answers one of these questions: [p. 68]

| # | Question |
|---|---|
| 1 | Is it coming for me? |
| 2 | Are there more of them close by? |
| 3 | How do I get in/past/away? |

- **Group observation**: when scouting together, one PC (players' choice) rolls for all. [p. 68]
- Cross-references: Investigator, Analysis and Breakthrough (auto-pass one OBSERVATION roll per session) [p. 74, 77]. Watchful +2 vs sneak attacks [p. 79]. Dark zone −2 [p. 83]. Ranged/critical-injury penalties [p. 100].

### 4.8 COMTECH (WITS) [p. 69]
- **Use**: any challenging attempt to program, repair, decrypt or manipulate any computer or communications technology (androids, mainframes, etc.). [p. 69]
- **Failure**: it won't work, and you may have triggered an alarm. [p. 69]
- **Success**: the code works. [p. 69]
- **Stunts** (each extra success **beyond the first**): [p. 69]

| Stunt | Effect |
|---|---|
| Follow-up bonus | +1 modification to a later related skill roll |
| Solved | No need to roll for the exact same challenge in the future |
| Quick | Done in half the normal time |
| New info | You get new or unexpected information (GM's choice) |
| Hide tracks | You hide your tracks |
| Show off | You show off |

- Cross-references: opposed vs an AI pool (example p. 64). EVA Specialist / Spaceship Mechanic +2 [p. 78–79]. MedPod programming is a Simple (+2) COMTECH roll, and AutoDoc an unmodified one [p. 136]. 1 electronic spare part per COMTECH repair roll [p. 200].

### 4.9 SURVIVAL (WITS) [p. 70]
- **Use**: in a hazardous planetside environment (heat, cold, sandstorms, acid rain, extreme weather), figuring out how to stay alive. [p. 70]
- **Failure**: no safe haven. Unless rescued, you're on borrowed time. [p. 70]
- **Success**: you find a safe haven to weather the storm. [p. 70]
- **Stunts** (each extra success): [p. 70]

| Stunt | Effect |
|---|---|
| Share | Give one success to another PC **in the same trouble** as you |
| Follow-up bonus | +1 modification to a later related skill roll |
| Impress | You impress someone |

### 4.10 MANIPULATION (EMPATHY) [p. 70–71]
- **Use**: making another person see things your way through lies, threats or persuasion. It is an **opposed roll vs MANIPULATION** ("it takes a liar to spot a liar"), modified by **negotiating position**. [p. 70]
- **Failure**: the adversary won't listen and won't comply. They may start to dislike you or attack you if provoked. [p. 70]
- **Success**: adversary **complies, or attacks you physically at once**. If complying, they may ask a favour in return (GM sets it; should be something you can reasonably meet). You accept or decline the deal. [p. 70]
- **Stunts**: for each success **in excess of what you need to win** the opposed roll, choose one: [p. 70]

| Stunt | Effect |
|---|---|
| No favour | Opponent complies without demanding a favour in return |
| Extra | Opponent does more than asked (e.g. gives useful information; details by GM) |
| Impressed | Opponent is impressed and will try to help you later (details by GM) |

- **Being manipulated**: NPCs and other PCs can MANIPULATE you. On their success you **must either attack or propose a deal**; the GM (or that player) decides whether the deal is accepted. [p. 70]
- **Manipulate a group**: address the leader or spokesperson. You get **−1** if the opponent has more people on their side. If the leader agrees, the group usually follows. With no leader, each opponent acts individually (harder). [p. 70]
- **Not mind control**: the request must be somewhat reasonable, or the GM can disallow it. [p. 71]
- **Negotiating position** (the GM decides which apply, ±1 each, summed): [p. 71]

| +1 each | −1 each |
|---|---|
| You have more people on your side | Your opponent has more people on their side |
| What you ask for costs your opponent nothing | You ask for something valuable or dangerous |
| Your opponent has suffered damage | Your opponent has nothing to gain by helping you |
| You have helped your opponent previously | You are having trouble understanding each other |
| You present your case very well (GM decides) | You are talking via radio or shouting at a distance |

- Persuade is a **slow action** in combat, and the opponent must hear you. [p. 88]
- Cross-references: Authority (use COMMAND instead of MANIPULATION) [p. 74]. Take Control (WITS instead of EMPATHY) [p. 75]. Menacing (STRENGTH instead of EMPATHY when threatening; on success the opponent can't demand anything in return but may still attack) [p. 78]. Hothead +2 to opposed MANIPULATION when someone tries to give you orders, and usable vs Pull Rank [p. 78]. Personal Safety (straight MANIPULATION roll, not opposed, not an action, +1 STRESS LEVEL) [p. 75]. Knocked-out teeth / broken nose critical injuries [p. 100].

### 4.11 MEDICAL AID (EMPATHY) [p. 71]
- **Recovery**: a person at Health 0 is **Broken**. On a successful MEDICAL AID roll they get back up and **immediately recover Health equal to the number of successes rolled** (all successes, not extras). [p. 71, 98]
- **Save a life**: treating a fallen character with a **critical injury**. A failed roll can mean death. [p. 71, 99]
- No stunt list. [p. 71]
- Cross-references (damage spec): first aid is a **slow action** [p. 88, 98]. First aid on someone not Broken has no effect [p. 98]. Saving a life from a fatal critical injury must happen before they fail a Death Roll. Some critical injuries apply a negative modification ("Yes, −1/−2" column) [p. 99–100]. Self-treatment is possible only after recovering 1 Health on your own, at −2 [p. 99]. Each character may try only **once** per patient, and a second chance needs better medical equipment [p. 99]. If both Broken and fatally injured, two separate rolls are needed, in either order [p. 99]. Care: a daily MEDICAL AID roll halves the remaining healing time [p. 101]. Personal Medkit +2 (single use) and Surgical Kit +1 vs fatal critical injuries [p. 136]. Field Surgeon +2 vs dying [p. 75].

### 4.12 COMMAND (EMPATHY) [p. 71]
- **Stop panic**: if another character's Panic Roll makes them lose control, a COMMAND roll can snap them out of it (see §5.9). **Slow action** in combat. [p. 71, 104, 88]
- **Give orders**: in combat, as a **slow action**, give orders to another character who can hear you (comm radio is fine). Roll COMMAND. **Each success** gives them **+1** to their roll when carrying out that order. [p. 71]
- **Officers**: Officer-career PCs with Pull Rank may order PCs and NPCs via COMMAND (cross-reference p. 76: COMMAND vs the target's MANIPULATION; on success the target must obey even if harmful; **+1 STRESS LEVEL** to the officer each time; one roll covers one specific action; it **cannot stop panic actions**). [p. 71, 76]
- No stunt list. [p. 71]
- Cross-references: Field Commander lets you give orders as a fast action (twice per Round) [p. 76]. Counselor talent [p. 78]. Authority [p. 74].

---

## 5. Stress & Panic

### 5.1 STRESS LEVEL basics
- STRESS LEVEL measures mounting tension. It **usually starts at 0** and rises during play. [p. 30, 103]
- It is tracked **only for PCs**, never for NPCs, and never for androids. [p. 105, 29]
- Whenever you make a skill roll you **must** add Stress Dice equal to your current STRESS LEVEL. [p. 103]
- No maximum is stated. AMBIGUOUS: no cap; the Panic table tops out at 15+. Minimum 0 is implied. [p. 103–105]

### 5.2 STRESS LEVEL increase triggers (complete list found in the book)

**Core list: "+1 whenever one of the following happens"** [p. 103]

| # | Trigger | Amount | Notes / cross-ref |
|---|---|---|---|
| 1 | You push a skill roll | +1 per push | Applied before the re-roll [p. 61]. With a second push from a talent, +1 again [p. 74–77]. |
| 2 | You fire a burst of full auto fire | +1 | Immediate. Add the Stress Die to that roll [p. 96]. **Machine Gunner** talent: no increase [p. 78]. |
| 3 | You suffer one or more points of damage | +1 | |
| 4 | You go without sleep, food or water | +1 | Detailed by the conditions below [p. 106–107] |
| 5 | You perform a coup de grâce | +1 | Whether or not the kill succeeds. A "Cold-Blooded" talent avoids it [p. 99]. Merciless skips the EMPATHY roll [p. 78]. |
| 6 | A Scientist in your team fails the Analysis talent roll | +1 | p. 77 says "other PCs within SHORT range". A success instead gives −1 to them. |
| 7 | A member of your own crew attacks you | +1 | |
| 8 | A person nearby is revealed to be an android | +1 | |
| 9 | You encounter certain creatures or locations (scenario/GM) | +1 (scenarios may give +2) | see the table below |

**Other stress-increase rules elsewhere in the book**

| Source | Effect | Page |
|---|---|---|
| Panic 7 Nervous Twitch | Self +1, and all friendly PCs in SHORT range +1 | p. 105 |
| Panic 9 Drop Item | Self +1 | p. 105 |
| Panic 10 Freeze | Self +1, and all friendly PCs in SHORT range +1 | p. 105 |
| Panic 11 Seek Cover | Self −1, but all friendly PCs in SHORT range **+1** | p. 105 |
| Panicking NPCs | May raise nearby PCs' STRESS LEVEL (GM judgement) | p. 105 |
| GM introduces horrific situations or fearsome beings | Stress at GM discretion | p. 211 |
| Critical injury #13 Crippling pain | +1 ("increases one step") | p. 100 |
| Talent Pull Rank (user) | +1 each use | p. 76 |
| Talent Personal Safety (user) | +1 each use | p. 75 |
| Dehydrated | Every Shift: 1 damage and +1 | p. 106 |
| Starving | Daily STAMINA roll. On failure: 1 damage and +1. | p. 106 |
| Freezing | STAMINA roll per day (above freezing), per Shift (sub-zero) or per Turn (deep space). On failure: 1 damage and +1. | p. 107 |
| Exhausted | No direct stress, but it blocks relief (see 5.10). AMBIGUOUS (A35): the generic trigger "go without sleep" (p. 103) applies. | p. 106, 103 |
| Neversleep Pills | +1 per dose | p. 137 |
| Coffee | +1 | p. 137, 139 |
| Trauma: Phobia | +1 when within SHORT range of the phobia object | p. 101 |
| Trauma: Alcoholism | +1 each Shift without alcohol | p. 101 |
| Trauma: Nightmares | +1 on a nightmare (EMPATHY roll **succeeds** when sleeping) | p. 101 |
| Trauma: Depression | +1 on a bad day (daily EMPATHY roll **fails**) | p. 101 |
| Trauma: Drug Use | +1 each Shift without your drug | p. 101 |
| Space combat panic table | various (see §5.12) | p. 201 |

- Book example of a condition: Hayes is Freezing with STAMINA 0 and STRENGTH 2, so he rolls 2 Base Dice and gets no success. He takes 1 damage and his STRESS LEVEL goes 0 → **1** (only +1 in total, not +2). Next roll he adds 1 Stress Die. [p. 108]

**Creature-specific stress triggers** (signature attacks, rolled by the GM on D6) [p. 295–323]

| Creature (attack #) | Effect | Page |
|---|---|---|
| Facehugger #1–2 Skittering Menace | victim +1 **and** Panic Roll | p. 303 |
| Facehugger #3 Tail Lash | victim +1 (besides damage) | p. 303 |
| Stalker/Scout/Drone #1 Hypnotizing Gaze | victim +1 and Panic Roll | p. 309 |
| Stalker/Scout/Drone #2 Playing With Its Prey | victim +1 and Panic Roll | p. 309 |
| Soldier/Sentry #1 Assessing the Threat | everyone within MEDIUM range +1 | p. 311 |
| Charger #2 Charge | even if the MOBILITY roll succeeds: falls down and +1 | p. 314 |
| Harvester #1 Sonar Sweep | all PCs up to LONG range +1 | p. 320 |
| Harvester #6 Pulverize | all who witness +1 and Panic Roll | p. 320 |

**Scenario examples (Hope's Last Day)** show that a scenario can grant arbitrary amounts to chosen PCs: a gunshot and scream +1 [p. 375]; seeing Reynolds' body +1, cleaning the keycard +1 [p. 379]; hearing an admission of being facehugged +1, CCTV carnage +1, seeing the Drone +1 [p. 380]; stacked bodies **+1 to all who see, +2 to the one who opened the door**, witnessing the bar +1, searching it +1 more [p. 381]; Osterman taken +1 [p. 383]. The app needs a GM tool to apply **±N to selected PCs**.

### 5.3 Panic Roll triggers (complete list found in the book)

**Core list** [p. 104]

| # | Trigger |
|---|---|
| 1 | One or more facehuggers on Stress Dice in a skill roll (and the roll then can't be pushed) |
| 2 | You witness a friendly character suffering a certain panic effect (table results 12, 13, 14, and Overkill) |
| 3 | You are pinned down by a ranged attack (Ranged Combat "pin down" stunt) |
| 4 | You suffer a critical injury |
| 5 | You're attacked by a strange alien creature you have never seen before |
| 6 | A truly horrifying event occurs (scenario/GM) |

**Other Panic Roll triggers elsewhere in the book**

| Source | Who rolls | Page |
|---|---|---|
| Panic 12 Scream | every friendly character who **hears** the scream | p. 105 |
| Panic 13 Flee | every friendly character who **sees** you run | p. 105 |
| Panic 14 Berserk | every friendly character who **witnesses** the rampage | p. 105 |
| Overkill effect | all **other PCs in SHORT range** | p. 104 |
| Ranged stunt "pin down" | the PC target (NPCs instead lose their next slow action) | p. 67, 95 |
| Trauma: Phobia | staying close to the phobia object for more than a single Round | p. 101 |
| NDD Psychosis (FTL travel) | every friendly character who witnesses the rampage | p. 151 |
| Chestburster birth | people witnessing it for the first time | p. 305, 383 |
| Creature attacks: Bloodburster #4, #5, #6; Neomorph #1, #3, #4, #5; Facehugger #1–2, #4 (all sub-results); Chestburster #4, #5, #6; Stalker/Scout/Drone #1, #2, #3, #4 (victim **and all friendly characters in the same zone**); Soldier/Sentry #2 (all targets in SHORT range); Beastly Bite #3 and Crushing Blow #4 (Praetorian/Charger/Queen); Harvester #2 (those not stunned in SHORT range), #6 (witnesses); Lion Worm #1 (all PCs in SHORT range), #2; Scorpionid #2 | as stated per attack | p. 295, 296, 303, 306, 309, 311, 314, 320, 321, 323 |

- AMBIGUOUS (A32): NPCs **never** make Panic Rolls, so "friendly character" in practice means friendly PCs (and not androids). [p. 105, 29]

### 5.4 Resolution [p. 104]
- **Panic Roll = D6 + current STRESS LEVEL**, read on the table below. [p. 104]
- Modifiers: Nerves of Steel talent gives **−2** to all Panic Rolls. [p. 78] No other Panic Roll modifiers appear in the book.
- The D6 is a plain D6 (a Base Die, number only). [p. 22]

### 5.5 Panic table [p. 105]
| Result | Name | Effect | Duration | Own STRESS LEVEL | Others' STRESS LEVEL | Forced action (10+) | Makes others Panic Roll |
|---|---|---|---|---|---|---|---|
| 1–6 | Keeping it together | No effect. | – | – | – | no | no |
| 7 | Nervous Twitch | – | immediate | **+1** | all friendly PCs in SHORT range **+1** | no | no |
| 8 | Tremble | All skill rolls using **AGILITY** get **−2** | **until your panic stops** (§5.9) | – | – | no | no |
| 9 | Drop Item | Drop a weapon or other important item. The GM chooses which. | immediate | **+1** | – | no | no |
| 10 | Freeze | Frozen for **one Round**, losing your **next slow action** | 1 Round | **+1** | all friendly PCs in SHORT range **+1** | **yes** | no |
| 11 | Seek Cover | Your **next action** must move you away from danger to a safe spot if possible. You **may** make a retreat roll (p. 93) if an enemy is at ENGAGED range. After **one Round** you act normally. | next action / 1 Round | **−1** | all friendly PCs in SHORT range **+1** | **yes** | no |
| 12 | Scream | Scream for **one Round**, losing your **next slow action** | 1 Round | **−1** | – | **yes** | **every friendly character who hears** the scream (immediate) |
| 13 | Flee | Run to a safe spot and stay there. No attacks, nothing risky. **No** retreat roll if an enemy is ENGAGED as you flee. | until panic stops | **−1** | – | **yes** | **every friendly character who sees you run** (immediate) |
| 14 | Berserk | Attack the **closest person or creature, friend or foe**, at once; keep going until **you or it is Broken**. | until you or the target is Broken (or panic stops) | – | – | **yes** | **every friendly character who witnesses** the rampage (immediate) |
| 15+ | Catatonic | You collapse and can't talk or move, staring blankly | until panic stops | – | – | **yes** | no |

- Results **7+** count as "suffering from a panic effect" for More Panic. [p. 104]
- A Panic Roll can **lower** stress (results 11, 12 and 13). [p. 104]

### 5.6 Panic actions [p. 104]
- On a result of **10 or higher** you are forced to perform a specific action (the table effect). [p. 104]
- Panic Roll caused by a **skill roll for an action** → that action is **cancelled**, replaced at once by the forced panic action, **successes notwithstanding**. [p. 104]

### 5.7 More panic [p. 104]
- If you are suffering from a panic effect (7+) and must make another Panic Roll, the **new effect replaces** the previous one. [p. 104]
- If the new result is **lower** than the previous effect, it is automatically raised to **one step more severe** than the previous effect (e.g. previous 10 Freeze, new 8 → 11 Seek Cover). [p. 104]
- AMBIGUOUS: what happens on an equal result; whether 15+ can escalate; whether expired one-Round or immediate effects still count as "suffering". See §9.

### 5.8 Overkill [p. 104, 74]
- Only with the Colonial Marine talent **Overkill**. On a Panic Roll result of **11 or higher**, and **if any enemies are in sight**, you may replace the effect with the **Overkill effect**. [p. 104, 74]
- Overkill effect: immediately attack your enemies, and don't stop until **you or all enemies in sight are Broken**. All **other PCs in SHORT range** must make an immediate Panic Roll. [p. 104]
- No STRESS LEVEL change is stated for Overkill. [p. 104]

### 5.9 Stopping panic [p. 104]
- Some panic effects are immediate or last one Round (7, 9, 10, 11, 12). Others remain until **one** of these happens: [p. 104]
  1. Another character comes to your aid and makes a **COMMAND** roll (a **slow action** in combat). AMBIGUOUS (A25): the roll presumably must succeed ("return them to their senses"). [p. 104, 71]
  2. You are **Broken**. [p. 104]
  3. **One Turn passes.** [p. 104]
- The Pull Rank talent **cannot** stop actions triggered by Panic Rolls. [p. 76]

### 5.10 Relieving stress [p. 104; GM p. 213]
- **Rest**: each **full Turn** (5–10 min) spent resting in a safe area secured from enemies (as far as you know) gives **−1** STRESS LEVEL. You may make **no skill rolls** while resting, and an interrupted rest doesn't count. [p. 104]
- **Panic Roll**: some results reduce stress (11, 12, 13). [p. 104]
- **Drugs**: certain drugs reduce stress (listed below). [p. 104]
- **Signature item**: once per **Act** (Cinematic) or once per **game session** (Campaign), interact with your signature item to reduce STRESS LEVEL by **one step**. This typically needs a **slow action** ("Use signature item" is listed as a slow action). [p. 104, 88]
- **Helping each other calm down** is named as a player tool (via talents). [p. 213]
- **Blockers**: some conditions block stress relief. [p. 104]

| Blocker | Scope | Page |
|---|---|---|
| Dehydrated | cannot relieve stress (effects wear off within one Shift after drinking) | p. 106 |
| Starving | cannot relieve stress (wears off within one Shift after eating) | p. 106 |
| Exhausted | cannot relieve stress (ends after sleeping at least one Shift) | p. 106 |
| Freezing | cannot relieve stress until warm | p. 107 |
| Neversleep Pills | cannot relieve stress during the day without sleep | p. 137 |
| Trauma Alcoholism / Drug Use | cannot relieve stress without drinking / consuming the drug of choice | p. 101 |
| Trauma Nightmares | cannot relieve stress for a full day after a nightmare | p. 101 |
| Trauma Depression | after a bad day: no relief until the next day | p. 101 |
| Active space combat | cannot relieve stress | p. 193 |
| Pushing a ship-repair roll during a Shift | no time to relieve stress before the next Shift | p. 200 |

**Stress-reducing substances and items**

| Item | Effect | Page |
|---|---|---|
| Naproleve ($20) | **STRESS LEVEL → 0** immediately. Every extra dose within one Shift: −1 to all AGILITY-based skill rolls for the rest of that Shift. | p. 136–137 |
| Beer ($4) | −1 per drink; −1 to all WITS-based skills for the next Shift | p. 137, 139 |
| Hard liquor ($10–$500/bottle) | −1 and −1 to WITS-based skills **per glass** | p. 137, 139 |
| Colony Specialty Meal ($20–$300) | −1, and Food Supply +1 | p. 137, 139 |
| Recreational drugs / X-drugs | "Varies" (no fixed values) | p. 136–137 |
| (Coffee, Neversleep) | **+1** (see 5.2) | p. 137, 139 |

**Talents that reduce stress** (cross-reference)

| Talent | Effect | Page |
|---|---|---|
| Banter (Marine) | You and everyone in SHORT range drop **2** steps (instead of 1) per Turn in a safe place. Several Marines with Banter don't stack. | p. 74 |
| Calming Presence (Medic) | Once per Turn, −1 to another character within SHORT range, on top of normal recovery. Both must be in a relatively safe place. Not on yourself. | p. 75 |
| Counselor (general) | Once per Turn, a COMMAND roll: each success reduces another character's (SHORT range) STRESS LEVEL by one extra step, on top of the default one step. Not on yourself. | p. 78 |
| Merciless (general) | −1 each time you cause an enemy to be Broken | p. 78 |
| Analysis (Scientist) | Success: all other PCs within SHORT range −1 (failure: +1) | p. 77 |

### 5.11 Permanent mental trauma [p. 104, 101]
- Any Panic Roll result of **13+** during a game session → **EMPATHY roll after the session** (attribute only, no skill). [p. 104]
- DECIDED: if that roll **fails**, you develop a permanent mental trauma: roll a **D6** on the table below. Threshold stays 13+. Source: publisher's FAQ and later printings. Note: the 1st printing has trauma on a success. [p. 104]

| D6 | Trauma | Effect |
|---|---|---|
| 1 | Phobia | Terrified of something related to what made you panic (the GM decides what). **+1 STRESS LEVEL** when within SHORT range of it. Staying close for more than a single Round forces a **Panic Roll**. |
| 2 | Alcoholism | Must drink alcohol **every Shift** or get +1 STRESS LEVEL. You **cannot relieve stress** without drinking alcohol. |
| 3 | Nightmares | Make an **EMPATHY roll when you sleep**. **If it succeeds**: a nightmare, +1 STRESS LEVEL, and **no stress relief for a full day** afterwards. |
| 4 | Depression | **Every day** make an EMPATHY roll. **If it fails**: a bad day, +1 STRESS LEVEL, and no stress relief until the next day. |
| 5 | Drug Use | Must use a recreational drug (p. 137) **every Shift** or get +1 STRESS LEVEL. You **cannot relieve stress** without your drug of choice. |
| 6 | Amnesia | You no longer recall who you or the other characters are. Roleplayed. |
[p. 101]

### 5.12 NPCs and panic [p. 105]
- STRESS LEVELs are only tracked for PCs. NPCs never make Panic Rolls and panic when the GM deems it appropriate. Panicking NPCs can raise nearby PCs' STRESS LEVEL (GM judgement). [p. 105]

### 5.13 Space combat panic table (cross-reference; authoritative in the space-combat spec) [p. 193, 201]
In space combat stress works as in personal combat, but Panic Rolls use **this** table, and stress **cannot be relieved** during active space combat. [p. 193]

| Result | Name | Effect |
|---|---|---|
| 1–6 | Keeping it together | – |
| 7 | Nervous Twitch | Self +1, and all friendly PCs **in the same compartment** +1 |
| 8 | Tremble | **All** skill rolls −2 until your panic stops |
| 9 | Ignore Orders | For the coming **Turn** you take no orders, accept no help and discuss nothing. Self +1. |
| 10 | Hysteric Comm Chatter | Communications impossible. Self −1. Every friendly PC who hears it +1. |
| 11–12 | Freeze | Frozen for one **Turn**, losing your next point of action. Self +1, and all friendly PCs in the same compartment +1. |
| 13 | Run to Safety | Abandon your station, retreat to a safer part of the ship. Self −1. Each friendly PC who sees it: immediate Panic Roll. |
| 14 | Abandon Ship | Run to the nearest escape pod and launch it (if there is no EEV: put on a compression suit and leave via the airlock). Every friendly PC who sees you flee makes an immediate Panic Roll. |
| 15+ | Maximum Overload | You overload and critically damage your station. Next action lost. Major component damage table result by your position: Pilot → Engine failure; Gunner → Armament damaged; Engineer → Reactor breach; Sensor Operator → Sensor failure. |
[p. 201]

---

## 6. Game Mother: Stress and Horror (book pp. 211–213)
- Stress and panic are the GM's main horror tools. The GM can introduce stress when PCs meet horrific situations or fearsome beings, but most stress comes from the PCs' own actions (pushing). [p. 211]
- Three horror tools: fear of the unknown, loss of control, horrible choices. These are guidance only, with no mechanics. [p. 211]
- Three stages of fear: dread, terror, horror. These are guidance only. When horror hits, "the game mechanics take over": draw initiative. [p. 212]
- Relieving stress: players manage stress with signature items, safe places to hide, and helping each other calm down (p. 104). The GM should allow calmer periods, then strike again. [p. 213]
- Splitting the team: allow it, and cut back and forth between groups. Keep all players at the table and trust them to separate player and PC knowledge. Consider what communications the PCs have. There are **no stress mechanics** for splitting up. [p. 213]
- Agendas, secrets and PvP: secret notes are fine. If a PC acts in direct opposition to the group, call PvP: play the current scene to its end, then the PC becomes an NPC. [p. 213]

---

## 7. Cross-reference: talents that hook into the dice engine, stress or panic [p. 74–79]

| Talent | Hook | Page |
|---|---|---|
| Past the Limit (Marine), True Grit (Roughneck) | push **STRENGTH**-based skill rolls twice; +1 stress per push | p. 74, 76 |
| Nimble (Kid), Reckless (Pilot) | push **AGILITY**-based skill rolls twice | p. 75, 76 |
| Cunning (Company Agent), Inquisitive (Scientist) | push **WITS**-based skill rolls twice | p. 75, 77 |
| Compassion (Medic), Influence (Officer) | push **EMPATHY**-based skill rolls twice | p. 75, 76 |
| The Long Haul (Roughneck) | once per Act (CIN) / session (CAM): ignore **all facehuggers** from a single roll | p. 76 |
| Nerves of Steel | −2 to all Panic Rolls | p. 78 |
| Overkill (Marine) | replace a Panic result of 11+ with the Overkill effect | p. 74, 104 |
| Machine Gunner | full auto gives no stress increase | p. 78 |
| Merciless | coup de grâce without the EMPATHY roll; −1 stress per enemy Broken | p. 78 |
| Banter, Calming Presence, Counselor, Analysis | stress reduction (see 5.10) | p. 74, 75, 78, 77 |
| Pull Rank, Personal Safety | +1 stress to the user per use | p. 76, 75 |
| Breakthrough (Scientist) | once per session, auto-pass one OBSERVATION roll (GM may veto) | p. 77 |
| Resilient (Roughneck) | when taking damage, STRENGTH roll (attribute only, no push, not an action); each success removes 1 damage | p. 76 |
| Attribute substitutes: Authority (COMMAND for MANIPULATION), Take Control (WITS for MANIPULATION), Menacing (STRENGTH for MANIPULATION when threatening), Stoic (WITS for STAMINA), Flyweight (AGILITY to block), Dodge (MOBILITY to block, reduce damage only) | change the pool's attribute or skill | p. 74, 75, 78, 79 |
| Flat modifications (+2 unless noted): Field Surgeon, Full Throttle, Like the Back of Your Hand, EVA Specialist, Hard Hitter, Hothead, Stealthy, Watchful, Weapon Specialist, Spaceship Gunner, Spaceship Mechanic, Zero-G Training, Subdue; Rapid Fire −2 | modifications | p. 74–79 |

---

## 8. Dice-engine algorithm

### 8.1 Data model
```
Die        = { type: 'BASE' | 'STRESS', face: 1..6, locked: bool }
isSuccess(d)    = d.face == 6                      // any die type          [p. 22, 58]
isFacehugger(d) = d.type == 'STRESS' && d.face == 1                         [p. 22, 58]

Actor      = { kind: 'PC' | 'NPC', isAndroid: bool, stressLevel: int>=0 (PC non-android only),
               attributes, skills, talents, activePanic: PanicState | null, ... }
Modifier   = { source: 'gear'|'difficulty'|'help'|'rule'|'talent'|'panic'|'injury'|'drug'|'order'|..., value: int }
RollRequest= { actor, attribute, skill (nullable), modifiers[], helpers[] (max 3 counted),
               kind: 'SKILL' | 'ATTRIBUTE_ONLY' | 'DEATH' | 'FIXED_POOL' | 'ARMOR' | 'SUPPLY' | ...,
               opposedRole: null | 'ATTACKER' | 'DEFENDER',
               weaponHasLimitedMagazine: bool, isActionInCombat: bool }
```

### 8.2 Skill roll: step by step
```
1. VALIDATE
   - If the "only one chance" rule applies (same goal already rolled and pushed, outside combat) → reject. [p. 62]
   - Help must be declared now (before rolling). Count helpers = min(declaredHelpers, 3). [p. 63]

2. BUILD POOL
   base   = attributeValue + skillLevel                        // skillLevel 0 allowed [p. 58]
   netMod = sum(modifiers.value) + helpersCount (×1 each)      // signed sum [p. 62–63]
   baseDice   = base + netMod
   stressDice = (actor is PC && !actor.isAndroid && kind == SKILL) ? actor.stressLevel : 0   [p. 61, 103]
   if baseDice < 0:                                            // overflow removes Stress Dice [p. 62]
       stressDice = max(0, stressDice + baseDice)
       baseDice = 0
       removedStress = deficit (remember it, see AMBIGUOUS A2)
   if baseDice + stressDice == 0: return { outcome: FAIL, reason: 'NO_DICE' }   [p. 62]

3. ROLL all dice (unlocked).                                     [p. 58]

4. EVALUATE
   successes   = count(isSuccess) over ALL dice (locked + unlocked)
   facehuggers = count(isFacehugger) over the dice rolled this time
   (Long Haul: once per Act/session the player may declare facehuggers = 0 for this roll) [p. 76]

5. IF facehuggers > 0 (PC with stress):                          [p. 61, 104]
     - mark pushAllowed = false
     - if weaponHasLimitedMagazine: magazineEmpty = true (after the attack resolves)  [p. 61, 96]
     - panic = resolvePanicRoll(actor, trigger='SKILL_ROLL')      // §8.3
     - if panic.finalValue >= 10 (or Overkill chosen): outcome = CANCELLED_BY_PANIC
             (the action is replaced by the forced panic action, regardless of successes) [p. 104]
       else: continue to step 7 with the current successes (no push possible)

6. ELSE offer PUSH if all of:
     actor is PC, !isAndroid, opposedRole != 'DEFENDER', kind == SKILL,
     pushesUsed < maxPushes  (1; or 2 if the actor has the push-twice talent for this roll's attribute) [p. 60, 74–77]
   If the player pushes:
     a. actor.stressLevel += 1   (persist BEFORE re-rolling)       [p. 61]
     b. lock every die showing a success                          [p. 60]
     c. add ONE new unlocked Stress Die                            [p. 61 example: 2 → 3 Stress Dice]
        (equivalently: stressDice = new stressLevel − removedStress, see A2)
     d. re-roll all unlocked dice (Base and Stress)                [p. 60]
     e. pushesUsed += 1; GOTO 4

7. STORY POINT (Cinematic only, kind == SKILL): after the roll the player may spend ONE point → +1 success.
   DECIDED: max 1 per roll, skill rolls only (old note: "each point adds +1"). [p. 61]

8. OUTCOME
   SIMPLE roll:   success iff successes >= 1;  stuntsAvailable = successes − 1        [p. 58]
                  (Medical Aid Recovery: healthRecovered = successes; Command orders: bonus = +successes) [p. 71]
   OPPOSED roll:  net = attackerSuccesses − defenderSuccesses
                  attacker wins iff attackerSuccesses >= 1 && net >= 1 (a tie means the attacker fails)  [p. 64]
                  stuntsAvailable = net − 1                                       [p. 70]
   GROUP roll:    one roll; a failure applies to every participant                [p. 62]
```

### 8.3 Panic Roll
```
resolvePanicRoll(pc, trigger):
  if pc.kind != 'PC' or pc.isAndroid: return NONE            // NPC panic is GM fiat [p. 105, 29]
  d = D6()                                                    // Base Die number [p. 22]
  value = d + pc.stressLevel + panicMods(pc)                  // Nerves of Steel: −2 [p. 104, 78]
  prev = pc.activePanic                                       // value >= 7 and not yet stopped
  if prev != null and value < prev.value:
       value = min(prev.value + 1, 15)                        // "one step more severe" [p. 104] (cap: A7)
  if value >= 13: pc.session.panic13Plus = true               // trauma check after session [p. 104] (A8)
  effect = TABLE[clamp(value)]                                // 1–6, 7, …, 14, 15+ [p. 105]
  if value >= 11 && pc.has('Overkill') && enemiesInSight && player chooses:
       effect = OVERKILL                                      // [p. 104]
  apply effect:
     stress deltas (self / friendly PCs in SHORT range)      // 7, 9, 10, 11, 12, 13
     queue Panic Rolls for witnesses/hearers                  // 12, 13, 14, Overkill → recursive, each a separate resolvePanicRoll
     set forced action / lost slow action                     // 10–15+, Overkill
     set modifiers (Tremble: AGILITY −2)                      // 8
  if value >= 7: pc.activePanic = { value, effect, startedAtRound, startedAtTurn }
  return { raw: d, finalValue: value, effect }

stopPanic(pc, reason ∈ {COMMAND_SUCCESS, BROKEN, TURN_PASSED}):   [p. 104]
  pc.activePanic = null; remove Tremble modifier; clear ongoing forced behaviour
```
- Order of side effects within one event (e.g. a hit that causes damage and a critical injury): AMBIGUOUS (A18). Suggested default: apply damage → +1 stress → critical injury → Panic Roll using the updated STRESS LEVEL.

### 8.4 Worked examples from the book (use as unit tests)
1. **Rolls without skill** (Jeffries): STRENGTH 3, STAMINA 0 → **3 Base Dice**, 0 Stress Dice. [p. 59]
2. **Push** (Nat): STRENGTH 5 + HEAVY MACHINERY 3 = **8 Base Dice**, STRESS LEVEL 2 = **2 Stress Dice** (10 dice). There are no successes and no facehuggers, so the push is allowed. STRESS LEVEL → **3**. She re-rolls **8 Base + 3 Stress**. Two Base Dice show a success and there are no facehuggers, so it succeeds. [p. 61]
3. **Gear + help** (Charlize): WITS 4 + OBSERVATION 2 = 6, plus the Tracker's +1 and Hayes' help +1 = **8 Base Dice**. [p. 63]
4. **Opposed** (Charlize vs Mother): 4 Base Dice (WITS, no COMTECH) vs 8 Base Dice. **2 vs 2 → cancelled → failure.** [p. 64]
5. **Condition** (Hayes, Freezing): STAMINA 0, STRENGTH 2 → 2 Base Dice and no success → 1 damage, STRESS LEVEL **0 → 1** (+1 in total); the next roll has 1 Stress Die. [p. 108]
6. **Vacuum** (Charlize): STAMINA roll with 2 Base Dice gives one success, so she can act. [p. 107]
7. **Initiative**: Gina draws 6, the GM draws 1, so Nguyen acts first. [p. 93]

### 8.5 Additional unit-test cases derived from the rules
| Case | Input | Expected |
|---|---|---|
| Help cap | 4 declared helpers | +3 only [p. 63] |
| Modification overflow | attr 2 + skill 0, mod −3, STRESS 2 | 0 Base, 1 Stress Die [p. 62] |
| No dice | attr 2 + skill 0, mod −3, STRESS 0 | auto-fail, no roll [p. 62] |
| Mod sum | +2 and −1 | +1 [p. 62] |
| Facehugger blocks push | 0 successes, 1 facehugger | no push; Panic Roll [p. 61] |
| Panic 1–9 keeps the result | 2 successes, 1 facehugger, panic 8 | success with 1 stunt; Tremble active [p. 104–105] |
| Panic 10+ cancels | 3 successes, 1 facehugger, panic 11 | action cancelled; Seek Cover [p. 104] |
| Panic value | STRESS 4, D6 = 5 | 9 Drop Item; STRESS → 5 [p. 104–105] |
| Nerves of Steel | STRESS 4, D6 = 5 | 7 Nervous Twitch [p. 78] |
| More panic step-up | active 10, new 8 | 11 Seek Cover [p. 104] |
| More panic higher | active 10, new 13 | 13 Flee [p. 104] |
| More panic from low | active 14, new 3 | 15+ Catatonic [p. 104] |
| Push keeps successes | 1st roll: 1 success on a Base Die, push | that die locked; others + 1 new Stress Die re-rolled; STRESS +1 [p. 60–61] |
| D3 | D6 = 5 | 3 [p. 22] |
| D66 | first 3, second 6 | 36 [p. 22] |
| Supply roll | Supply 8 | roll 6 Stress Dice; −1 Supply per facehugger; no Panic [p. 34] |
| Death Roll | any STRESS | no Stress Dice, no push [p. 99] |
| Android | any skill roll | 0 Stress Dice, no push, never Panic [p. 29] |
| NPC defender in opposed roll | – | NPC can't push; attacker PC may [p. 62, 64] |
| Trauma check, roll fails (DECIDED) | Panic 13+ this session; post-session EMPATHY roll fails | permanent trauma; roll D6 on the trauma table [p. 104, 101] |
| Trauma check, roll succeeds (DECIDED) | Panic 13+ this session; post-session EMPATHY roll succeeds | no trauma [p. 104] |
| No trauma check | highest Panic this session 12 | no EMPATHY roll [p. 104] |
| Story Point limit (DECIDED) | CIN skill roll, player tries to spend 2 | only 1 accepted, +1 success [p. 61] |
| Story Point roll type (DECIDED) | Panic / Supply / armor / Death Roll | Story Point not allowed |

---

## 9. Ambiguities / Game Mother discretion

- **A1 – STRESS LEVEL bounds.** No maximum is stated. The minimum of 0 is implied ("usually starts at zero"). Suggest: floor 0, no cap. [p. 103]
- **A2 – Push after modification removed Stress Dice.** The rules don't say whether a push then adds one Stress Die or rebuilds the pool from the full STRESS LEVEL. Suggest: add exactly one new Stress Die. [p. 61–62]
- **A3 – Zero dice and Story Points.** "No chance to succeed" might conflict with spending a Story Point for an automatic success. Suggest: a GM toggle, default no. [p. 61, 62]
- **A4 – Stunt repetition.** Only "+1 damage" is explicitly repeatable. Suggest: every other stunt at most once per roll. [p. 64–70]
- **A5 – Opposed-roll sequencing.** The rules don't say whether the attacker pushes after seeing the defender's result. A defender PC adds Stress Dice (every skill roll does), but whether a defender's 10+ panic cancels a reactive roll is unclear. Suggest: both roll, then the attacker may push, and the defender's panic follows normal rules. [p. 64, 104]
- **A6 – "Give one success to another PC in the same situation"** (STAMINA, MOBILITY, SURVIVAL). Timing and target are unclear: can the success go to a PC who already rolled and failed? Suggest: yes, to any PC rolling against the same situation. [p. 65, 66, 70]
- **A7 – More Panic edge cases.** (a) An equal result gets no step-up ("lower" only) and the same effect re-applies. (b) Escalation beyond 15+ stays Catatonic. (c) The book doesn't say whether results 7 and 9 (immediate) or expired one-Round results (10–12) still count as "suffering from a panic effect". Suggest: a 7+ effect counts until a stop condition (COMMAND, Broken, one Turn) clears it, configurable. [p. 104]
- **A8 – Which value counts as "13 or higher" for trauma.** The raw D6 + stress, the value after Nerves of Steel, or the value after More Panic step-up? Suggest: the final value after all modifiers and adjustments. [p. 104]
- **A9 – Attribute-only EMPATHY rolls** (trauma check, Nightmares, Depression). Stress Dice and pushing are not specified. The coup de grâce explicitly excludes Stress Dice. Suggest: no Stress Dice, no push. [p. 99, 101, 104]
- **A10 – Freeze/Scream triggered by a skill roll.** Is the cancelled action the "lost next slow action", or is a further slow action lost? Suggest: the cancelled action counts as the lost slow action if it was a slow action; otherwise the next slow action is lost. [p. 104–105]
- **A11 – Berserk end.** "Until you or the target is Broken". The general stop conditions (COMMAND, Broken, one Turn) also apply, since Berserk is an ongoing effect. [p. 104–105]
- **A12 – Flee with an enemy ENGAGED.** "Not allowed to make a retreat roll" probably means you move and the enemy gets the free close combat attack of a failed retreat (p. 93). [p. 93, 105]
- **A13 – Overkill.** It is optional. Its interaction with More Panic and with stopping panic isn't stated. Suggest: treat it as an ongoing panic effect of value ≥ 11 that the normal stop conditions end, and that also ends when you or all enemies in sight are Broken. [p. 104]
- **A14 – Which relief methods a blocker blocks.** Conditions "block your ability to relieve stress". Clearly that covers rest; probably the signature item; unclear for drugs, Panic results and talents. Suggest: block rest, signature item and Calming Presence/Counselor/Banter; allow drugs and panic results; GM override. [p. 104, 106–107]
- **A15 – Counselor "in addition to the default one step".** It is unclear whether the target must be resting. Suggest: no, the default step is granted by the talent use itself; GM discretion. [p. 78]
- **A16 – Stacking simultaneous triggers.** E.g. a crew member attacks and damages you (+1 for the crew attack, +1 for damage?). The Hayes example shows a condition's damage plus its stress is **+1 total**, not +2. Suggest: each distinct core trigger stacks, but condition damage doesn't add a separate damage trigger. [p. 103, 108]
- **A17 – Damage absorbed by armor.** Stress only if at least 1 point is actually "suffered" after armor and cover. [p. 103, 98]
- **A18 – Order of side effects.** Damage stress before the critical-injury Panic Roll (see §8.3). [p. 98–104]
- **A19 – Analysis talent scope.** "A Scientist in your team" (p. 103) vs "other PCs within SHORT range" (p. 77). Suggest: SHORT range. [p. 77, 103]
- **A20 – "Cold-Blooded talent"** (p. 99) does not exist in the talent list. The closest is Merciless (p. 78), which skips the EMPATHY roll but doesn't explicitly waive the +1 stress. GM decides. [p. 78, 99]
- **A21 – Push-twice talents with attribute substitution** (Take Control, Menacing, Stoic, Authority). Which attribute is the roll "based on"? Suggest: the attribute actually used for the pool. [p. 74–79]
- **A22 – Command "give orders" bonus vs the +3 help cap.** The cap is stated only for helpers. Suggest: orders are a separate, uncapped modification. [p. 63, 71]
- **A23 – Manipulate a group −1** and the negotiating-position factor "opponent has more people" (−1) are the same factor. Don't double count. [p. 70–71]
- **A24 – Group stealth "lowest skill level".** Read literally as the lowest MOBILITY skill level, not total dice. Ties are the players' choice. [p. 67]
- **A25 – Stop Panic COMMAND roll.** Success is required (implied). It's a normal skill roll by the helper, with Stress Dice, pushable, and can trigger the helper's own panic. It can't be used on yourself ("another character"). [p. 71, 104]
- **A26 – Chance-of-success "pushed" column.** It doesn't match a simple probability model. Display only. [p. 59]
- **A27 – "Taking effect next turn"** in the skill-chapter stunts means **next Round** (combat chapter wording). [p. 66–67, 92, 95]
- **A28 – Pin-down vs NPCs.** p. 67 says the target makes a Panic Roll, but NPCs never make Panic Rolls. Use p. 95: NPCs miss their next slow action. [p. 67, 95, 105]
- **A29 – Signature item outside combat.** "Typically requires a slow action". Outside combat there's no action economy, so the GM decides. [p. 104]
- **A30 – Story Points with panic.** A Story Point can't undo a 10+ panic cancellation ("even if you rolled successes"). Spending one after a push is not restricted (DECIDED: still max one per roll, see §3.7). [p. 61, 104]
- **A31 – The Long Haul.** If facehuggers are ignored, presumably the roll can be pushed and the magazine isn't emptied. Not stated. [p. 76]
- **A32 – "Friendly character" vs "friendly PCs".** Only PCs (non-android) can make Panic Rolls or have stress, so both phrases resolve to friendly human PCs. [p. 105, 29]
- **A33 – Facehuggers on non-skill Stress Dice rolls** (supply, radiation, epilepsy, Ovomorph). No Panic Roll, because the trigger is "in a skill roll". [p. 34, 104, 110, 151, 300]
- **A34 – Rest "full Turn" in a "safe area (as far as you know)".** The GM decides when a Turn of rest is complete and whether it was interrupted. [p. 104]
- **A35 – "Go without sleep, food or water" (+1).** The generic trigger on p. 103 vs the detailed condition schedules on pp. 106–107. Suggest: use the condition rules for timing. Exhausted has no explicit stress, so the GM applies the generic +1 once per day without sleep. [p. 103, 106–107]
- **A36 – Phobia "for more than a single round"** (lowercase). Read as one Round. [p. 101]
- **A37 – Trauma in Cinematic play.** The 13+ check "after the session" has no mode restriction. It's meaningful mainly in Campaign play. GM decides. [p. 104]
- **A38 – Multiple 13+ results in one session** lead to a single EMPATHY roll (the text uses the singular). [p. 104]
- **A39 – NPC Panic.** Entirely GM fiat. The app should offer a manual "NPC panics" flag and optional "+N stress to nearby PCs". [p. 62, 105]
- **A40 – Panic 1–9 after a skill roll.** The skill roll keeps its normal success or failure. This is strongly implied but not stated outright: only 10+ "cancels" the action, and p. 61 says panic "can" cause failure. [p. 61, 104]
- **A41 – Second push (talent) after a facehugger.** If the first push shows a facehugger, a second push is impossible and a Panic Roll follows. This applies the general rule, which is only stated for pushing in general and not for second pushes specifically. [p. 61, 104]

---

## 10. Time-related rules

| Unit | Rule | Page |
|---|---|---|
| Definitions | Round 5–10 s (combat); Turn 5–10 min (stealth); Shift 5–10 h (recovery); 4 Shifts per day; the GM tracks time | p. 83 |
| Round | Freeze (10): frozen one Round, lose next slow action | p. 105 |
| Round | Seek Cover (11): next action forced; act normally after one Round | p. 105 |
| Round | Scream (12): one Round, lose next slow action | p. 105 |
| Round | Initiative swap stunts take effect next Round; no going back | p. 66–67, 92, 95 |
| Round | Pin down: NPCs miss their next slow action | p. 95 |
| Round | Initiative is drawn once at the start of the first Round; swaps only at the start of a fight or Round | p. 87 |
| Round | Each combatant: 1 slow + 1 fast, or 2 fast actions per Round. Stop panic, give orders, first aid, persuade and use signature item are slow actions. | p. 87–88 |
| Round | Phobia: more than a single Round near the object → Panic Roll | p. 101 |
| Round | Vacuum: STAMINA each Round, −1 cumulative from the 2nd Round (cross-ref) | p. 107 |
| Turn | Ongoing panic effects end when **one Turn passes** (or COMMAND, or Broken) | p. 104 |
| Turn | Rest: −1 STRESS LEVEL per full Turn of uninterrupted rest in a safe area | p. 104 |
| Turn | Banter: −2 per Turn in a safe place | p. 74 |
| Turn | Calming Presence and Counselor: once per Turn each | p. 75, 78 |
| Turn | Freezing in deep space: STAMINA every Turn (failure: 1 damage, +1 stress) | p. 107 |
| Turn | Space combat panic: Ignore Orders "for the coming Turn"; Freeze one Turn | p. 201 |
| Turn | Analysis requires studying for at least one Turn | p. 77 |
| Turn | NDD Epilepsy: a seizure disables you for a full Turn | p. 151 |
| Shift | Dehydrated: every Shift 1 damage and +1 stress; effects wear off within one Shift of drinking | p. 106 |
| Shift | Starving: effects wear off within one Shift of eating | p. 106 |
| Shift | Freezing sub-zero: STAMINA once per Shift | p. 107 |
| Shift | Alcoholism / Drug Use trauma: consume every Shift or +1 stress | p. 101 |
| Shift | Naproleve: extra doses in the same Shift give −1 AGILITY until the end of the Shift | p. 137 |
| Shift | Beer/booze: −1 WITS-based skills for the next Shift | p. 139 |
| Shift | Coffee: postpones lack-of-sleep drawbacks for one Shift | p. 139 |
| Shift | NDD Epilepsy: roll a Stress Die each Shift | p. 151 |
| Shift | Pushed ship repair: no stress relief before the next Shift | p. 200 |
| Day | Starving: daily STAMINA roll | p. 106 |
| Day | Exhausted: you need ≥ 1 Shift of sleep per day; daily STAMINA roll at −(days without sleep) | p. 106 |
| Day | Freezing above zero: daily STAMINA | p. 107 |
| Day | Neversleep: no sleep needed for one day, and no stress relief that day | p. 137 |
| Day | Nightmares: no relief for a full day after a nightmare. Depression: daily EMPATHY roll, no relief until the next day after a bad day. | p. 101 |
| Act (CIN) | Signature item −1 once per Act; The Long Haul once per Act; Story Point award at the end of each Act | p. 104, 76, 31, 216 |
| Session (CAM) | Signature item once per session; The Long Haul once per session; Breakthrough once per session; Hidden Stash item each session | p. 104, 76, 77, 78 |
| After session | Trauma check if any Panic result was 13+ (DECIDED: trauma on a failed EMPATHY roll); XP questions incl. "Did you make a Panic Roll?" (CAM) | p. 104, 35 |
| Between scenarios | Story Points persist (max 3) | p. 383 |
| Immediate | Push: +1 stress before the re-roll. Full auto: +1 immediately. Panic from facehuggers: immediately, before anything else. Witness Panic Rolls: "immediate". | p. 61, 96, 104–105 |
| Before roll | Help must be declared before rolling | p. 63 |
| After attack | Facehugger with a limited magazine: magazine empty after the attack | p. 96 |

---

## 11. State the app must track

**Global / session**
- Game mode (Cinematic / Campaign); current Act (1–3, Cinematic); session id/number. [p. 19, 216]
- Game clock: Round counter (in combat), Turn counter, Shift (Morning/Day/Evening/Night), day. The GM advances it. [p. 83]
- Combat state: in combat yes/no; initiative order (cards 1–10 per PC / NPC / NPC group); actions used per combatant per Round (slow/fast). [p. 87–89]
- Space combat active (blocks stress relief; switches the panic table). [p. 193, 201]
- Per-roll "goal" registry for the Only One Chance rule (outside combat). [p. 62]
- Roll log: every die (type, face, locked, which pass), modifiers with sources, helpers, pushes, facehuggers, Panic Rolls (raw D6, stress, modifiers, adjusted value, effect), Story Points spent.

**Per player**
- Story Points (0–3, Cinematic, belong to the player). [p. 61, 383]

**Per PC**
- Attributes, skill levels (0–5), talents (incl. push-twice attribute, Nerves of Steel, Overkill, The Long Haul usage, Breakthrough usage, etc.). [p. 28, 74–79]
- `isAndroid` (no stress, no push, no panic, no signature item). [p. 29]
- **STRESS LEVEL** (int ≥ 0), with a change history (source and page) for auditability. [p. 103]
- Health / Broken status (Broken ends panic; you can't roll skills while Broken). [p. 98, 104]
- **Active panic**: result value (7–15+), effect id, started-at (Round/Turn), source trigger, ongoing modifiers (Tremble: AGILITY −2), forced-action flags (lost next slow action, must Seek Cover next action, Fleeing, Berserk target, Catatonic, Overkill). [p. 104–105]
- `panic13PlusThisSession` flag → post-session EMPATHY check (DECIDED: failure → trauma). [p. 104]
- Permanent traumas (Phobia object, Alcoholism, Nightmares, Depression, Drug Use + drug of choice, Amnesia) and their timers: last drink / drug Shift, nightmare no-relief until, depression no-relief until. [p. 101]
- Signature item (name) and "used this Act/session" flag. [p. 33, 104]
- Rest state: resting since (Turn), in a safe area, interrupted flag. [p. 104]
- Conditions: Dehydrated, Starving, Exhausted (days without sleep), Freezing (+ cold level → roll interval); relief-block flags with expiry (Neversleep day, space combat). [p. 106–107, 137, 193]
- Drug/alcohol effects with expiry: Naproleve doses this Shift (AGILITY −1 per extra dose), drinks (WITS −1 until the end of the next Shift), coffee (sleep drawbacks postponed until). [p. 137, 139]
- Weapon magazine state and full reloads carried. [p. 96]
- Consumable Supply ratings (air, food, water, power), for Supply rolls with Stress Dice. [p. 34]
- Personal Agenda (for the Story Point / XP award) and XP. [p. 31, 35]
- Position data or GM-selectable groups to resolve "friendly PCs in SHORT range", "who hears", "who sees", "in the same zone/compartment" for stress spreads and witness Panic Rolls. [p. 104–105, 201]
- Stunt carry-overs: pending "+1 to a later related roll" modifiers, and "no need to roll this exact challenge again" flags. [p. 64–70]
- Pending order bonuses from COMMAND (+N to a specified order). [p. 71]

**Per NPC**
- Skill/attribute or fixed dice pools; no stress; GM "panicking" flag; "misses next slow action" (pin down); initiative card or group. [p. 62, 87, 95, 105]

**Per roll (transient, for the push/panic state machine)**
- Dice array with locked flags; pushes used / pushes allowed; push blocked (facehugger); opposed role; group / help participants; cancelled-by-panic flag; magazine-emptied flag; Story Points applied; successes and stunts available / chosen.
