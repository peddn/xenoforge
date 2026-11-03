# 06 — Creatures: Alien Species

> Official errata and project decisions in [00-errata-und-festlegungen.md](00-errata-und-festlegungen.md) take precedence over this file.

ALIEN RPG Core Rulebook, 1st ed. (Free League, 2019, ISBN 978-91-88805-55-3).
Scope: book p. 112 ("Xenomorphs", player section) and book pp. 282–323 (Chapter 11, "Alien Species").
All page refs are BOOK pages (PDF page − 4). Every stat block and attack table below was checked against the rendered page image.
Core rules that creature entries invoke (armor, blocking, grappling, crits, disease, fire, drowning, etc.) are summarized in §1.12 only as far as needed. Their full spec belongs to the other spec files.

---

## 0. Conventions used in this file

- **Success** = a 6 on any die (the book's crosshair icon). **Facehugger symbol** = a 1 on a Stress Die. [p. 58 ff., used on p. 298, 300, 303, 309, 311]
- **"N BD, Dmg D"** = the attack rolls N Base Dice; its Damage rating is D.
- **AP** = armor piercing.
- **Two different triggers.** Implement both of them separately:
  - **"hits" / "is successful" / "if it hits"**: at least 1 success is left after any block.
  - **"causes (any) damage" / "inflicts any damage"**: at least 1 point of damage is left after the target's armor roll.
- **Time units** [p. 83]: combat uses Rounds (5–10 seconds), stealth uses Turns (5–10 minutes), recovery uses Shifts (5–10 hours). Usually 4 Shifts per day (Morning, Day, Evening, Night). The book treats 24 hours as four Shifts [p. 296] and 24–36 hours as four to six Shifts [p. 301].
- **Range bands** [p. 83]:
  - ENGAGED: right next to you.
  - SHORT: a few meters, same zone.
  - MEDIUM: up to 25 m, adjacent zone.
  - LONG: up to about 100 m (four zones).
  - EXTREME: up to about 1 km.
- **"Xenomorph"** in the general rules means every xeno breed made by the xenovirus pathogen: Neomorphs, Praetomorphs, Ultramorphs and Xenomorph XX121 (P. praepotens) [p. 290]. The "Other Extrasolar Species" (Swarm, Harvesters, Lion Worms, Tanakan Scorpionids) are a separate group [p. 316]. See A13.
- `AMBIGUOUS (An):` marks my interpretation. Every An is listed again in §11.
- `ERRATA:` marks a change made by the official errata. `DECIDED:` marks a project decision that settles an An.

---

## 1. General Xenomorph rules

### 1.1 Skills and dice [p. 290]
- Xenomorphs have no Attributes. They use specific skills and roll a number of Base Dice equal to the skill level only. In this book the only skills given are MOBILITY and OBSERVATION. [p. 290]
- Skill levels can be much higher than human levels. [p. 290]
- Xenomorphs **never push rolls**. [p. 290]
- AMBIGUOUS (A6): Creatures have no STRESS LEVEL, roll no Stress Dice and never make Panic Rolls. This is implied by "Base Dice equal to their skill level only" and "never push". Stress and Panic effects in attack tables only ever target the victims.

### 1.2 Speed [p. 291; p. 112]
Speed has two uses:
1. **Stealth mode** (p. 85): ERRATA: the creature moves **two zones per point of Speed each Turn** (the printed p. 291 says one). Humans move two zones per Turn, so they count as Speed 1. [p. 291, errata]
   - ERRATA: (A1) resolved. This now matches p. 85, where PCs and human NPCs move **two** zones per Turn in stealth mode. Implement 2 × Speed zones per Turn. No GM option.
2. **Combat**: the creature acts once for each point of Speed in each Round. [p. 291; p. 112]
   - Draw one initiative card per point of Speed.
   - At each of these initiative points the creature performs **one fast and one slow action**.
- Speed can be changed by the Xeno crit "Wounded": Speed −1, to a minimum of 1, and the creature loses "the lowest initiative". [p. 291] See A8.
- Speed 0 (eggs) [p. 301]: the egg draws no initiative and takes no actions. Implied.

### 1.3 Signature attacks [p. 291; p. 112]
- Most Xenomorphs attack only in close combat, at ENGAGED range. [p. 291]
- They **do not make ordinary close-combat attacks**. **Each time** a Xenomorph attacks, the GM rolls a D6 on that creature's signature attack table and resolves the result. [p. 291; p. 112]
  - The result is random. No rule lets the GM choose an entry.
  - Exceptions:
    - Entries that force a follow-up on the creature's next initiative: Final Embrace, Headbite after Ready to Kill, Tail Strike after Pincer Grab, the Lion Worm squeeze, Stripped to the Bone.
    - The Charge entry, which is re-rolled for non-Chargers.
    - Last Breath. See A12.
- The table entry sets the dice, Damage and effects. The creature's skills are **not** used for signature attacks.
- Attacks per Round: one signature attack per initiative slot (Speed), plus any free or extra attacks that entries or abilities grant.
  - Free and extra attacks: Neomorph Leaping Attack, Stalker Feral Hunger, Lion Worm Feral Rage, Harvester Hidden Behemoth.
- AMBIGUOUS (A2): A signature attack uses the creature's **slow action**. A close-combat attack is a slow action [p. 88], and signature attacks replace close-combat attacks. Moving to ENGAGED (run, a fast action) and attacking (slow) therefore fit into one initiative slot. Sprint is a slow action, so a creature that sprints cannot also attack in the same slot.
- DECIDED: (A3) **Damage scaling.** The book never says whether extra successes add damage on signature attacks. Project decision:
  - Damage = the entry's Damage rating + 1 per success beyond the first. This mirrors the normal close-combat "+1 damage" stunt [p. 92]. The book's own wording "base Damage" also supports it ("five Base Dice, Damage 1 (… six Base Dice, base Damage 2)" [p. 303]; acid splash "with base Damage 1" [p. 298]).
  - No other stunts: creatures never pick other stunts.
  - Default for every game; the GM may switch a game to "base Damage only" (see 00-errata-und-festlegungen.md, F-03).

### 1.4 Blocking signature attacks [p. 291; p. 92–93; p. 75]
- Unless an entry says otherwise, a signature attack **can be blocked** (p. 92). Blocking it requires some form of weapon or blunt instrument. [p. 291]
  - An unarmed character can only block unarmed attacks by humans. [p. 92]
- Block procedure [p. 92]:
  - Declare the block **before** the attacker rolls. It is a fast action and uses CLOSE COMBAT.
  - For each success, choose one: Decrease Damage (remove one attacker success; if none are left, the attack misses), Counterattack (damage = the blocker's weapon Damage; no extra successes added), or Disarm.
  - Blocking is reactive: it breaks the initiative order, but it uses one of the blocker's two actions for the Round. A character who has used both actions cannot block.
- **Cannot be blocked**:
  - the extra attack from the Neomorph Leaping Attack [p. 296];
  - any sneak attack [p. 90];
  - the free attack an enemy gets when you fail a retreat [p. 93].
- Talent **Dodge** [p. 75]: works like blocking, but rolls MOBILITY and can only reduce damage. It "can even dodge a creature's signature attack".
- NPCs (which includes creatures) typically don't block. The GM may override this. [p. 93]
- AMBIGUOUS (A4): Many entries have no attack roll (automatic effects such as Hypnotizing Gaze, Playing With Its Prey, Tail Grapple, Call the Guard). A block has no successes to remove there, so it has no effect. Default: a block cannot stop roll-less entries.
- AMBIGUOUS (A5): Counterattack against a creature does normal damage. That damage goes through the creature's armor and can trigger an Acid Splash. Disarm has no effect against creatures.

### 1.5 Armor Rating [p. 290; p. 98; p. 118]
- Most Xenomorphs have a natural Armor Rating (thick hide). [p. 290]
- Notation `X (Y vs fire)` means Armor X, or Armor Y against fire attacks. The Scorpionid uses "(3 against bullets)" instead [p. 322]. The Harvester's underbelly has 5 [p. 318].
  - **Note:** the Imp is `0 (3 vs fire)`, which is *higher* against fire. Do not assume the fire value is always lower. [p. 306]
- Armor roll (for creatures and PCs alike) [p. 98]:
  - Roll Base Dice equal to the Armor Rating.
  - Each success reduces the damage by 1.
  - The roll is not an action.
- **Armor Piercing** (general rule): the Armor Rating counts as half, **rounding fractions up**. [p. 118]
  - Neomorph AP entries say "halving any Armor Rating" [p. 296]. Same rule.
  - The Swarm's entries say "Armor is halved (rounded down)" [p. 317]. That rounding is different; implement it per entry.

### 1.6 Health, zero Health and "Critical Injuries on Xenomorphs" [p. 290–291; p. 112]
- When a Xenomorph is reduced to 0 Health, it **is not Broken** like a human. Instead, roll D6 on this table. [p. 290; p. 112 points to p. 291]

**Critical Injuries on Xenomorphs** [p. 291]

| D6 | Crit | Effect |
|---|---|---|
| 1 | Rise Again | Falls to the ground, apparently dead (a ruse). **If it is attacked again, or on its next initiative**, it regains 1 Health and rises. |
| 2 | Wounded | Speed −1 (minimum 1; it loses its lowest initiative). Regains 1 Health. **Start of every Round: roll D6; 1–3 → tries to escape.** |
| 3 | Desperate Action | **At once** takes one fast + one slow action, out of turn order. Roll D6. **1–3:** tries to escape; on success, regains **half its Health**. **4–6:** attacks the closest opponent at once. No damage dealt → it **dies**; damage dealt → regains 1 Health. |
| 4 | Last Breath | Mortally wounded. **On its next initiative** it tries to kill the nearest victim, then it dies. If it is wounded again before then, it dies instantly. |
| 5–6 | Torn Apart | Instant death. Severely mutilated: it can only be Analyzed (Analysis talent, p. 77) with OBSERVATION −2. |

- AMBIGUOUS (A7) **Rise Again**:
  - When it is "attacked again", the triggering attack is resolved first and the creature then rises with 1 Health. If that attack deals damage, it is back at 0 Health and you roll on the table again (A14). GM option: it rises before the attack.
  - When it rises "on its next initiative", it acts normally on that initiative.
- AMBIGUOUS (A8) **Wounded**: "lowest initiative" means the creature's lowest-numbered remaining card (its earliest action). The D6 escape check repeats at the start of every later Round of the fight.
- AMBIGUOUS (A9) **"tries to escape"**: there is no escape mechanic. Recommendation: the creature spends its actions moving away (run or Sprint). The escape succeeds when it leaves the PCs' line of sight or the GM rules so. This mirrors the Chestburster and Bloodburster ESCAPE entries [p. 295, 306].
- AMBIGUOUS (A10) **Desperate Action, "half its Health"**: half of its **maximum** Health, rounded down, minimum 1. GM may override.
- AMBIGUOUS (A11) **Desperate Action, failed escape**: no consequence is stated. It stays at 0 Health; any further damage triggers a new D6 roll (A14).
- AMBIGUOUS (A12) **Last Breath, "try to kill the nearest victim"**: make one signature attack (D6 table) against the nearest victim. GM option: pick the table's deadliest entry.
- AMBIGUOUS (A14): Several results restore 1 Health. If the creature is later reduced to 0 Health again, roll on this table again.
- A creature at 0 Health takes no further damage. Only the table above decides death. Implied.
- No rule lets creatures recover Health other than the effects above.
- AMBIGUOUS (A13) **Which creatures use this table**:
  - The general rules cover the Neomorph line and every XX121 stage, eggs included.
  - The **Swarm** has its own rule: at 0 Health it leaves [p. 316].
  - The book does not say what Harvesters, Harvester juveniles, Lion Worms and Scorpionids do at 0 Health. Default: use this table. GM option: at 0 Health the creature dies.

### 1.7 Special abilities [p. 290; p. 112]
- Some Xenomorphs have special actions beyond their signature attacks. These are listed under each creature below.

### 1.8 Movement in combat (as used by creatures)
- **Run** [p. 88–89]:
  - Fast action. Moves one zone, or between SHORT and ENGAGED range of a target in the same zone.
  - Prerequisite: no enemy at ENGAGED range.
  - No roll is needed unless the zone is Cluttered.
- **Sprint** (special ability of Neomorphs, of all XX121 stages except Stage I, and of Scorpionids in water) [p. 294, 295, 297, 299, 322]:
  - Slow action, as if two run actions were used.
  - Covers two zones, **or** goes straight from an adjacent zone to ENGAGED range of a target.
- **Retreat** [p. 93]:
  - Anyone with an active enemy at ENGAGED range must roll MOBILITY to move away to SHORT range.
  - On a failure they still move, but the enemy gets a free close-combat attack. That attack is not one of the enemy's actions and cannot be blocked.
  - AMBIGUOUS (A19): When a PC fails to retreat from a creature, the creature's free attack is one signature attack (D6 roll). When a creature retreats, it rolls its own MOBILITY skill. ESCAPE results ("moving two zones away in a single action") ignore the retreat roll.
- **Line of sight** [p. 83]: a blocked border blocks line of sight, even if it has a door, unless someone is standing in the doorway. This matters for ESCAPE results, which end once the PCs lose line of sight.

### 1.9 Multiple creatures and initiative [p. 87; p. 291; p. 314]
- Initiative uses ten cards, numbered 1–10. Lowest acts first. Initiative is drawn once, at the start of the first Round, and the order stays the same for the whole conflict. [p. 87]
- The GM draws one card per NPC. If there are many NPCs and **more than ten combatants**, NPCs with identical stats form a group. Each group draws one card and all its members act at that point, in an order the GM picks. [p. 87]
- Creatures draw **one card per point of Speed**. [p. 291]
- AMBIGUOUS (A20): For a group of identical creatures with Speed > 1, the group draws Speed cards. If more cards are needed than exist, the GM must group creatures further. The book gives no rule for running out of the ten cards.
- Mid-combat arrivals: Call the Guard reinforcements arrive in the zone next Round, then draw initiative and act [p. 314]. This explicitly overrides the draw-once rule.
- Exchanging initiative only works between PCs [p. 87]. Not relevant to creatures.
- Hive mind is fluff: six or more Stage IV XX121s together "begin to operate in unison" [p. 311]. There is no mechanic.

### 1.10 Stress and Panic caused by creatures
- **Panic Roll triggers** [p. 104]:
  - Being attacked by an alien creature of a kind you have never seen.
  - "You suffer a critical injury." Every fixed crit from a signature attack therefore causes a Panic Roll.
  - "You witness a friendly character suffering from a certain panic effect."
  - A horrifying event, as set by the scenario or GM.
- **STRESS LEVEL +1 triggers** [p. 103]:
  - "You suffer one or more points of damage."
  - Meeting specific creatures or places that the scenario or GM designates.
- Creature-specific triggers (details in each table):
  - Chestburster birth: everyone who witnesses it **for the first time** makes an immediate Panic Roll [p. 305].
  - Ready to Kill: the victim **and all friendly characters in the same zone** make Panic Rolls [p. 309].
  - Pulverize: everyone who witnesses it gets +1 STRESS LEVEL and a Panic Roll [p. 320].
  - Assessing the Threat: everyone within MEDIUM range gets +1 STRESS LEVEL [p. 311].
  - Sonar Sweep: all PCs within LONG range get +1 STRESS LEVEL [p. 320].
  - Many other entries cause Panic Rolls or +1 STRESS LEVEL.
- AMBIGUOUS (A15): An entry that says "inflicts critical injury #X …, triggering an immediate Panic Roll" causes **one** Panic Roll, not one for the crit and another for the entry. If the same attack also makes the victim Broken, the fixed crit **replaces** the random crit for being Broken [p. 98]. GM option: apply both.
- Androids [p. 111]:
  - They have no STRESS LEVEL and never make Panic Rolls, so ignore stress and panic effects for them.
  - They are immune to vacuum, cold and disease, and are "typically unaffected by alien pathogens". This covers Neomorph motes and Scorpionid venom.
  - They use their own D6 crit table on p. 111.
  - AMBIGUOUS (A16): Fixed human crits (#35/#53/#56/#61/#64/#66) and "random critical injury" results against an android are rolled on the **synthetic** crit table instead (GM option: narrate). Androids are not "potential hosts" for facehugging or ovomorphing. Neomorphs only attack androids if provoked [p. 315].

### 1.11 Other creature-related modifiers
- Firing at a small target, "such as … a Chestburster", gives −2 [p. 94].
  - AMBIGUOUS (A50): Apply the −2 to all Stage II–III XX121 (facehuggers and the burster family) and to the Bloodburster. GM decides.
- Sneak attack and ambush by a creature [p. 85, 90]:
  - The GM rolls the creature's MOBILITY against the target's OBSERVATION.
  - Range modifiers: ENGAGED −2, SHORT −1, MEDIUM 0, LONG +1, EXTREME +3. An ambush adds +2.
  - On a success the creature gets **one free action (slow or fast, not both) before initiative is drawn**, and the target cannot block it.
  - On a failure the creature is spotted at its starting distance and initiative is drawn.
  - The Watchful talent gives +2 to OBSERVATION when trying to spot a sneak attack [p. 79].
  - A group ambush uses the lowest MOBILITY among the attackers against the highest OBSERVATION among the targets [p. 90].
- Stealth mode basics [p. 85]: ACTIVE enemies stalk the PCs; PASSIVE enemies are unaware of them. Enemy movement is resolved secretly each Turn, after the PCs move.
- Motion trackers (once per Turn) detect large **moving** objects within LONG range indoors and EXTREME range outdoors. They do not detect a creature that stays still [p. 86]. XX121 has no heat signature, so infrared and thermal sensors don't see it [p. 299].

### 1.12 Core rules referenced by creature entries (summary only)

| Rule | Mechanic (short) | Page |
|---|---|---|
| Opposed roll | The attacker wins only if they roll successfully **and** get more successes. Each defender success cancels one attacker success. Only the attacker may push. | p. 64 |
| Prone | A prone character cannot run (only crawl, which is a slow action). Getting up is a fast action. Standing enemies get +2 on close-combat attacks against a prone character. | p. 88–89, 91 |
| Grappling ("grabbed"/"grappled") | Both combatants fall to the ground. The victim drops any held weapon and cannot move. The victim's only possible action is breaking free: a slow action that succeeds if the victim wins an opposed CLOSE COMBAT roll. (Creature entries give the creature's dice for this roll.) | p. 93 |
| Broken (humans) | At 0 Health. Immediately roll D66 for a critical injury. The character can only crawl and mumble, with no other actions and no skill rolls. Each further attack that causes damage → another critical injury. | p. 98 |
| Death Roll | STAMINA roll. No push, no Stress Dice. Failure = death. | p. 99 |
| Instant kill | Crits #63–66 kill outright, with no Death Roll. | p. 99 |
| Critical injuries used by creature tables | #35 Busted kneecap (not fatal; can't run, only crawl; 2D6 days). #53 Leg artery cut (fatal −1, time limit one Turn; running becomes a slow action; D6 days). #56 Cracked spine (not fatal; paralyzed from the neck down; permanent if MEDICAL AID isn't given in time; 3D6 days). #61 Ruptured jugular (fatal −1, time limit **one Round**; STAMINA −1; 2D6 days). #64 Crushed skull (instant death). #66 Impaled heart (instant death). | p. 100 |
| Fire | Roll Intensity Base Dice; 1 damage per success; armor protects. If it deals damage, the target burns and is attacked again at the start of each new Round with Intensity +1, until an attack deals no damage. Anyone at ENGAGED can put it out with MOBILITY (slow action). Broken by fire → Death Roll every Round. | p. 108 |
| Explosions | Roll Blast Power Base Dice for each person within SHORT range; 1 damage per success; +1 damage at ENGAGED. | p. 108 |
| Falling (humans) | Height in meters / 2, rounded down. | p. 108 |
| Disease | Sickness Roll = STAMINA opposed by Virulence. On a failure: 1 damage one Shift after infection; no Health recovery; a new Sickness Roll at the start of each Shift (failure = 1 damage); if Broken, a Sickness Roll after every Shift (failure = death). A success cures. A carer may roll MEDICAL AID against the Virulence instead. | p. 109 |
| Drowning | STAMINA roll every Round underwater, during the character's turn and before their actions; failure = 1 damage. Broken while drowning → Death Roll each **Turn**. | p. 110 |
| Radiation | Rads accumulate. Each new Rad → roll dice equal to total Rads; 1 damage per success. | p. 110 |
| Armor Piercing | The Armor Rating counts as half, rounded up. | p. 118 |
| Androids | See §1.10. Synthetic crit table. | p. 111 |

---

## 2. Xenomorph XX121 special abilities [p. 298–299]

These are common to **all varieties of Xenomorph XX121**: Ovomorph, Facehuggers, the Chestburster family, Stage IV–VI. They do **not** apply to Neomorphs, the Swarm or the other extrasolar species. [p. 298]

### 2.1 Acid Splash [p. 298–299]
- **Trigger:** an XX121 is wounded.
  - Only damage that **draws blood** causes a splash. A **fire attack does not**. [p. 298]
  - Each XX121 type has its own **Acid Splash rating** (see its stat block). The bigger the creature and the more damage it took, the bigger the splash.
- **Who is hit:** "its attacker and anyone within ENGAGED range". [p. 298]
  - The example hits "everyone within ENGAGED range" after a Smart Gun hit [p. 299].
  - AMBIGUOUS (A17): Hit everyone at ENGAGED range of the creature. That includes a close-combat attacker. A ranged attacker who is not at ENGAGED range is not hit (per the example). Roll separately for each victim (cf. Explosions, p. 108).
- **Strength:** Acid Splash rating **+ the damage the Xeno took from that attack** (after its armor) = Base Dice. **Base Damage 1.** [p. 298]
  - Example: Acid Splash 8 and 5 damage → a 13 Base Dice attack on everyone within ENGAGED range [p. 299].
  - Per A3, each extra success adds +1 damage. This is consistent with "base Damage 1".
- **Armor:** armor reduces the damage normally. **For each success rolled on the armor roll, that armor's Armor Rating drops permanently by 1**, as the acid eats through it. [p. 298]
- **Environment:** GM discretion: the acid can hit nearby ammo or grenades, or eat through floors, cables, a hull, etc. [p. 298]
- **Continuing burn:** a victim **damaged by the initial splash** takes a new acid attack **at the start of every Round**, as with fire (p. 108). [p. 298]
  - At the start of each new Round the number of Base Dice is **halved, rounding down**.
  - The first time any acid attack roll inflicts no damage, the acid has burned out and the effect ends.
  - Example: 13 BD splash → 6 → 3 → 1 → 0 (ended).
  - AMBIGUOUS (A17): Armor applies to the continuing attacks (as with fire), and their armor successes also degrade the Armor Rating.
  - AMBIGUOUS (A18): If several splashes hit one victim, track each burn separately.
- **Broken by acid:** a victim who is Broken by the splash, or who takes further acid splash damage while already Broken, makes a **Death Roll every Round** until they die or someone saves them with a MEDICAL AID roll. [p. 298]
- Eggs explode with acid when destroyed. "The eggs are essentially vats of acid, and when exploding will cause acid damage accordingly." [p. 300] Use the egg's Acid Splash rating.
- **Die for the Queen** (Soldier/Sentry entry 5) uses a fixed **12 Base Dice** acid splash against everyone within **SHORT** range [p. 311]. Apply the rest of the acid rules (armor erosion, continuing burn, Death Roll).

### 2.2 Other XX121 abilities [p. 299]
- **Sprint:** every XX121 variety **except Stage I** (the egg) can sprint as a slow action. See §1.8.
- **Resistance to vacuum:** total protection. The creature can act indefinitely in vacuum with no ill effect.
- **Falling:** falling damage = distance fallen in meters **÷ 4, rounded down**, then reduced by Armor. Humans use ÷ 2 [p. 108].
- **Immune to the cold:** cold has no effect, even the cold of space.
- **Radiation:** the creature can take **10 Rads** before it feels any ill effect. The first 10 points are ignored for radiation rolls.
- **No respiration:** it does not need to breathe.
  - AMBIGUOUS (A49): It is therefore immune to drowning, suffocation and airborne effects that work through breathing. GM decides.
- **No heat signature:** it does not register on infrared or thermographic sensors. Motion trackers still work.

---

## 3. Neomorph line [p. 293–297]

General [p. 293]:
- **Neomorph blood is not acidic. Neomorphs have no Acid Splash.**
- The life cycle has five stages. Each stage has its own termination protocol.
- Neomorphs are more animalistic and less intelligent than XX121.
- Behaviour [p. 315]: feral Neomorphs attack anything living that is not of their type. They find androids a curiosity and only attack them if provoked. Neomorphs and XX121 fight each other.
- XX121 special abilities (§2) do not apply to Neomorphs. Each Neomorph stage with stats has its own **Sprint**.

### 3.1 Stage I — Neomorphic Egg Sacs [p. 293]
- Bird's-egg-sized fungal pods, found in **clutches of 2D6**; an infected area often has several clutches. They release the motes. [p. 293]
- No stat block.
- **Termination:** severe temperature changes. Fire destroys them; liquid nitrogen makes them inert **until they thaw**. [p. 293]
- Origin: when fungus interacts with the decaying corpse of an adult Neomorph, it becomes new spore sacs [p. 296]. No timing is given.

### 3.2 Stage II — Neomorphic Motes [p. 294]
- **Trigger:** egg sacs are **disturbed** and release a "puff of pollen" (a swarm of motes). The motes target the **nearest potential host** and enter through an unprotected orifice (eyes, ears, nose, throat) or an open wound. [p. 294]
  - AMBIGUOUS (A32): The GM decides what counts as "disturbed" and who is "nearest". A sealed helmet or suit (no unprotected orifice) prevents exposure.
- **Infection** [p. 294]:
  - Exposure is handled as a **disease with Virulence 9**: a Sickness Roll, i.e. STAMINA opposed by 9 Base Dice (p. 109).
  - **Failed first Sickness Roll → Stage III within one Shift. No more Sickness Rolls.**
  - If the first roll succeeds, there is no infection. Implied by the disease rules.
  - AMBIGUOUS (A32): The book doesn't say whether the normal disease effects apply before Stage III (1 damage at outbreak, no Health recovery). Default: they do not apply; the patient simply dies at Stage III. Whether MEDICAL AID can roll this first Sickness Roll instead of the patient is also unstated. Default: the patient rolls. Androids are immune [p. 111].
  - AMBIGUOUS (A28): "Within one Shift". Default timer: Stage III at exactly +1 Shift after the failed roll. GM may trigger it earlier.
- No stat block, no attack table.
- **Termination:** motes avoid smoke and are killed by fire. [p. 294]

### 3.3 Stage III — Neomorphic Bloodburster [p. 294–295]

| Stat | Value |
|---|---|
| Speed | 3 |
| Health | 2 |
| Skills | Mobility 9, Observation 6 |
| Armor Rating | 3 (zero against fire damage) |
| Acid Splash | — (none) |

[stats p. 295]

- **Birth** [p. 294]:
  - On entering Stage III the patient **dies at once**: the Bloodburster's birth kills them.
  - Needs **one Round** to get out of its sack (sluggish). Once free, it is enraged and attacks any living creature it meets.
  - **After just a few Turns**, it enters Stage IV (juvenile Neomorph).
- **Sprint** [p. 294]: slow action, equal to two run actions (two zones, or from an adjacent zone directly into ENGAGED range).
- **Containment** [p. 294]: **fire or any serious damage** taken before it is free (the sluggish Round) makes it **flee and hide**, growing quickly into a juvenile.
- AMBIGUOUS (A33):
  - During the sluggish Round, the Bloodburster takes no actions.
  - "Serious damage" means ≥1 damage after armor.
  - "A few Turns" means D6 Turns, taken from the ESCAPE entry.

**Bloodburster Attacks** [p. 295]

| D6 | Name | Roll | Effect |
|---|---|---|---|
| 1–3 | ESCAPE | — | Moves **two zones away in one action**, into the nearest air duct if there is one. Once the PCs lose line of sight, **combat ends; switch to stealth mode**. **After D6 Turns** it becomes a Juvenile Neomorph (Stage IV) and stalks the PCs. |
| 4 | TERRIFYING HISS | — | Jumps onto the victim, showing its teeth and hissing. The victim makes an **immediate Panic Roll**. |
| 5 | LEG BITE | 6 BD, Dmg 2 | If it causes damage: automatic **critical injury #53**, Broken or not; triggers an immediate Panic Roll. |
| 6 | THROAT BITE | 8 BD, Dmg 1 | If it causes damage: automatic **critical injury #61**, Broken or not; triggers an immediate Panic Roll. |

### 3.4 Stage IV — Neophyte (Juvenile Neomorph) [p. 295]

| Stat | Value |
|---|---|
| Speed | 3 |
| Health | 4 |
| Skills | Mobility 10, Observation 6 |
| Armor Rating | 4 (2 against fire damage) |
| Acid Splash | — (none) |

[stats p. 295]

- **Evolving:** **after one Shift**, the juvenile enters Stage V and becomes an adult Neomorph. [p. 295]
- **Sprint:** as above. [p. 295]
- Behaviour [p. 295]:
  - It targets one prey in a pack and devotes everything to taking it down, even tracking it if it escapes.
  - It is hard to frighten away.
  - Under fire it ignores damage to itself "until it becomes critical".
- AMBIGUOUS (A34): The book gives **no Neophyte attack table**. Use the **Neomorph Attacks** table (§3.5). Its heading is generic, and the juvenile attacks with "sharp-tipped tail and teeth", which matches entries 2, 5 and 6.

### 3.5 Stage V — Adult Neomorph [p. 296–297]

| Stat | Value |
|---|---|
| Speed | 2 |
| Health | 6 |
| Skills | Mobility 9, Observation 8 |
| Armor Rating | 6 (3 against fire damage) |
| Acid Splash | — (none) |

[stats p. 297]

- **Sprint:** as above. This one is printed in the stat block. [p. 297]
- **Short lifespan:** an adult usually dies naturally **within 24 hours (four Shifts)**. Fungus acting on the corpse makes new spore sacs (Stage I). [p. 296]
  - AMBIGUOUS (A28): Default timer: dies at +4 Shifts after it becomes an adult. GM may end it earlier.
- It hunts with stealth tactics. It is "known to get up and attack even when [it seems] to be dead", which fits crit result 1, Rise Again. [p. 296]

**Neomorph Attacks** [p. 296] (used by the Adult Neomorph, and by the Neophyte per A34)

| D6 | Name | Roll | Effect |
|---|---|---|---|
| 1 | TERRIFYING HISS | — | Leans in close, hissing. The victim makes an **immediate Panic Roll**. |
| 2 | TAIL SLASH | 10 BD, Dmg 2, **AP** | Armor piercing: halves any Armor Rating. |
| 3 | DEADLY GRAB | 8 BD, Dmg 1 | **If it hits:** at once drags the victim **one zone over (MEDIUM range)**, then lets go. Victim: **prone**, **drops hand-held items**, **immediate Panic Roll**. |
| 4 | LEAPING ATTACK | 8 BD, Dmg 1 | **If successful:** the victim is thrown to the ground (prone) and makes an immediate Panic Roll. The Neomorph immediately makes an **extra attack: 10 BD, Dmg 2**. The extra attack does not count as an action and **cannot be blocked**. |
| 5 | THROAT BITE | 8 BD, Dmg 1 | If it causes damage: automatically **critical injury #61** (even if the victim is not Broken), which triggers an immediate Panic Roll. |
| 6 | TAIL SPIKE | 7 BD, Dmg 1, **AP** | Armor piercing: halves Armor. If it causes damage: automatically **critical injury #66** (even if the victim is not Broken). This **kills the victim outright**. |

- AMBIGUOUS (A37), Deadly Grab: the Neomorph moves into the new zone with the victim. It ends at the GM's choice of SHORT or ENGAGED range from them. The book only says it "drags … before releasing".

---

## 4. Xenomorph XX121 life cycle [p. 297–315]

All stages have the §2 special abilities: Acid Splash, Sprint (except Stage I), vacuum, falling, cold, radiation, no respiration, no heat signature. [p. 298–299]

### 4.1 Stage I — Ovomorph (Egg) [p. 300–301]

| Stat | Ovomorph | The Queen's Egg |
|---|---|---|
| Speed | 0 | 0 |
| Health | 2 | 3 |
| Skills | None | None |
| Armor Rating | 2 (0 vs fire) | 3 (1 vs fire) |
| Acid Splash | 4 | 5 |

[stats p. 301]

- Eggs come from either a Queen or **ovomorphing** (transforming a living host). Queen-laid eggs are hardier: use "The Queen's Egg" stats for them and "Ovomorph" for ovomorphed eggs. [p. 300]
- **Opening the egg** [p. 300]:
  - A potential host **within ENGAGED range** of an egg must make a **MOBILITY roll each Round** to avoid disturbing it. If the roll fails, the egg opens and releases a Facehugger.
  - An egg can release its Facehugger **even at SHORT range**. **Roll a Stress Die each Round; on a 1 (facehugger symbol), the Facehugger is released.**
  - AMBIGUOUS (A21):
    - The checks happen at the start of each Round. In stealth mode, check once per Turn instead, or switch to Rounds when a host comes close (GM).
    - The SHORT-range Stress Die is rolled once per egg per Round while at least one potential host is at SHORT range (not at ENGAGED).
    - Each host at ENGAGED range rolls MOBILITY separately.
    - The released Facehugger then joins combat and draws initiative.
- **Containment** [p. 300]:
  - A rifle bullet is likely to deflect off the hide. Implement this through the armor stat; no extra rule.
  - Fire sterilizes eggs and explosives work. Exploding eggs cause acid damage (Acid Splash).
  - **Extreme cold can make an egg dormant**, which suspends its opening checks.
- **Ovomorphing** [p. 301; also p. 309, 312]:
  - A Drone isolated from a hive collects hosts, usually incapacitating them by partially crushing their skulls. It cocoons them in secreted resin and injects genetic material with its tail barb.
  - The host turns into an egg (Ovomorph stats) holding a new Facehugger.
  - The process typically takes **24–36 hours (four to six Shifts)**. [p. 301]
  - Any adult alien can start ovomorphing; only Queens lay eggs en masse. [p. 312]
  - Stage IV Headbite: GM option to leave the victim barely alive so the Xenomorph can begin ovomorphing. [p. 309]
  - AMBIGUOUS (A52): The book has no rule for saving a host during ovomorphing. GM decides. Default timer: 4 Shifts plus a GM option of +0–2 Shifts, i.e. 4 + D3−1 Shifts if randomized.

### 4.2 Stage II — Facehugger, Praeto-Facehugger, Royal Facehugger [p. 302–303]

| Stat | Facehugger | Praeto-Facehugger | Royal Facehugger |
|---|---|---|---|
| Speed | 2 | 2 | 2 |
| Health | 2 | 2 | 3 |
| Mobility | 8 | 6 | 8 |
| Observation | 8 | 8 | 10 |
| Armor Rating | 2 (0 vs fire) | 2 (0 vs fire) | 4 (2 vs fire) |
| Acid Splash | 4 | 4 | 5 |

[stats p. 303]

- **Variants and the seed each carries** [p. 302]:
  - The major difference between types is the seed: Praetomorph, Xenomorph Drone, or Queen.
  - Standard Facehugger → Drone line.
  - Praeto-Facehugger → Praetomorph line. It is slightly bigger but otherwise works the same, and is less dexterous.
  - Royal Facehugger → specifically produces **Queens**. It is hypothesized to carry **two** spores: a Queen and a protector.
  - AMBIGUOUS (A30): Whether a Royal Facehugger yields two embryos (a Queenburster plus another burster) is the GM's choice. Default: one Queenburster.
- **Facehugging** [p. 302]:
  - A Facehugger **automatically facehugs any immobilized potential host within ENGAGED range**.
  - In combat, a **successful signature attack #6** (The Final Embrace) does it.
  - The victim is impregnated and **rendered unconscious**.
  - **Within one Shift the Facehugger falls off dead and the victim regains consciousness**, usually ravenous.
  - **One Shift after that, a Chestburster (Stage III) is born and the host dies.**
  - AMBIGUOUS (A22): "Immobilized" means Broken, unconscious, paralyzed (Capture for the Hive), grappled/grabbed, or cocooned. GM decides.
  - AMBIGUOUS (A28): Default timers: falls off at +1 Shift; chestburster at +2 Shifts from facehugging. The GM can shorten the first interval ("within"). The Queenburster takes longer, and the Imp is faster (§4.3, A29).
- **Removal and detection** [p. 302, 305]:
  - Pulling it off the face usually kills the host.
  - Every surgical removal of an embryo so far has killed the patient. Depending on how far gestation has progressed, the alien also dies.
  - **The core book gives no roll to detect or remove an embryo.** See A27.
- **Containment** [p. 302]: its skin protects it from most melee attacks (via Armor). It is helpless against pulse rounds and laser tools (GM flavour; no number). Fire works.
- Facehugger acid matters here. Wounding a Facehugger that is on someone's face triggers an Acid Splash (Acid Splash 4 or 5) on everyone at ENGAGED range, host included. Implied by §2.1.

**Facehugger Attacks** [p. 303] (all three variants)

| D6 | Name | Roll | Effect |
|---|---|---|---|
| 1–2 | SKITTERING MENACE | — | It has chosen its host and skitters forward. The victim gets **+1 STRESS LEVEL** and makes an **immediate Panic Roll**. |
| 3 | TAIL LASH | 5 BD, Dmg 1 (**Royal: 6 BD, base Dmg 2**) | Aside from any damage, the victim gets **+1 STRESS LEVEL**. |
| 4 | TAIL GRAPPLE | — (roll D6 for effect) | Leaps and catches the victim from behind. Roll D6. **1–2:** legs caught; the target falls **prone** and makes a Panic Roll. **3–4:** arms tangled in the tail; the victim **cannot use any held items** and makes a Panic Roll. **5–6:** tail wrapped around the neck, choking; the victim **suffers the effects of DROWNING** (p. 110) and makes a Panic Roll. |
| 5 | FACE GRAPPLE | Opposed: 6 BD vs target's CLOSE COMBAT | Not an action for the victim. **If the Facehugger wins:** the target suffers **THE FINAL EMBRACE** on the Facehugger's **next initiative**. **If the victim wins:** they throw the beast to the floor, but it **attacks the same target again on its next initiative**. |
| 6 | THE FINAL EMBRACE | 6 BD (no Damage) | Its acid destroys any helmet or respirator in the way, so helmets and respirators give no protection. **If it gets ≥1 success, the victim is facehugged and immediately Broken.** (Then apply the facehugging timeline above.) |

- AMBIGUOUS (A26), Tail Lash: the +1 STRESS LEVEL applies whether or not the attack hits, and it is **in addition to** the normal +1 for taking damage [p. 103]. GM option: one +1 total.
- AMBIGUOUS (A25), Tail Grapple:
  - The book gives no duration and no way out. Default: the effect lasts until the Facehugger acts again, is killed, or the victim (or an ally at ENGAGED range) succeeds at a break-free roll (opposed CLOSE COMBAT against the Facehugger's 6 BD, slow action, per the p. 93 analogue).
  - The 5–6 result ("choking") uses drowning: a STAMINA roll every Round, failure = 1 damage.
  - There is no hit roll, so A4 applies.
- AMBIGUOUS (A24), Face Grapple:
  - A tie (including 0 vs 0) means the Facehugger did not win, so the victim wins [p. 64].
  - "Suffer THE FINAL EMBRACE" means you resolve entry 6, **including its 6 BD roll**. GM option: automatic.
  - "Attacks the same target again" means a new D6 roll on this table against the same target.
  - With Speed 2, the "next initiative" can come later in the **same Round**.
- AMBIGUOUS (A23), Final Embrace "immediately Broken": set Health to 0 and mark the victim Broken and unconscious. Default: **no** random crit roll for this Broken state, because a crit could kill the host. GM option: roll per p. 98.

### 4.3 Stage III — Chestburster, Bambi-Burster, Imp, Queenburster [p. 305–306]

| Stat | Chestburster | Bambi Burster | Imp | Queenburster |
|---|---|---|---|---|
| Speed | 2 | 2 | 2 | 2 |
| Health | 2 | 2 | 2 | 3 |
| Mobility | 8 | 10 | 8 | 8 |
| Observation | 6 | 6 | 8 | 6 |
| Armor Rating | 2 (0 vs fire) | 2 (0 vs fire) | **0 (3 vs fire)** | 4 (2 vs fire) |
| Acid Splash | 4 | 4 | 4 | 5 |

[stats p. 306]

- **Types** [p. 305]:
  - Chestburster: from a human or other bipedal host.
  - **Bambi-Burster:** from a quadrupedal host.
  - **Imp:** the immature Praetomorph, from a Praeto-Facehugger. It matures "substantially faster" than traditional Chestbursters. If it doesn't feel threatened on emergence, it may mimic the nearest creature.
  - **Queenburster:** an embryonic Queen that **gestates over days instead of hours**.
- **Birth** [p. 305]:
  - The birth **kills the host outright**.
  - Everyone who witnesses a birth **for the first time** makes an **immediate Panic Roll**.
  - **Within one Shift, the Chestburster evolves to Stage IV.**
  - After birth it seeks a hiding place and raw metals to eat. Once fed, it cocoons in resin and metamorphoses.
- **Variable Gestation** [p. 306]:
  - Factors such as cancer-prone hosts or compromised immune systems can speed up or slow down gestation.
  - **Infection during hypersleep can slow the process upon revival.**
  - **Being placed in stasis while infected arrests the infection/incubation until the afflicted is awakened**, so the timer pauses.
  - "It is ultimately up to the Game Mother … to decide how rapid any xenovirus gestation period is."
  - AMBIGUOUS (A53): All gestation timers must be editable and pausable by the GM.
- Ranged attacks against a Chestburster get −2 (small target) [p. 94].
- AMBIGUOUS (A29): The Queenburster's "days" and the Imp's "substantially faster" are not quantified. Defaults:
  - Queenburster: D6 days of gestation after the Facehugger falls off. GM sets it.
  - Imp: 1 Shift total, i.e. born when the Facehugger falls off. GM sets it.
  - Every gestation timer must be editable and pausable (stasis).
- AMBIGUOUS (A31): Which Stage IV each burster becomes:
  - Imp → Stalker. Stated: the Stalker is the adult Praetomorph [p. 308].
  - Queenburster → Queen. Implied [p. 305, 312].
  - Chestburster (Drone seed [p. 302]) → Drone.
  - Bambi-Burster → Scout ("Runner"). Supported by lore on p. 256, not by a rule.

**Chestburster Attacks** [p. 306] (one table for all four types, per the stat block grouping)

| D6 | Name | Roll | Effect |
|---|---|---|---|
| 1–3 | ESCAPE | — | Moves **two zones away in one action**, preferably into the closest vent, sewer or duct. It has escaped once the PCs lose line of sight. **Switch to stealth mode**; it seeks to hide and grow. |
| 4 | TERRORIZING HISS | — | Jumps toward a target, hissing. The target makes an **immediate Panic Roll**. |
| 5 | LEG SLASH | 6 BD, Dmg 2 | If it causes damage: automatically **critical injury #53** (even if the victim is not Broken), which triggers an immediate Panic Roll. |
| 6 | THROAT BITE | 8 BD, Dmg 1 | If it causes damage: immediately **critical injury #61** (even if the victim is not Broken), which triggers an immediate Panic Roll. |

### 4.4 Stage IV — Stalker, Scout, Drone [p. 308–309]

| Stat | Stalker | Scout | Drone |
|---|---|---|---|
| Speed | 2 | 3 | 2 |
| Health | 6 | 5 | 7 |
| Mobility | 8 | 8 | 10 |
| Observation | 6 | 10 | 8 |
| Armor Rating | 10 (5 vs fire) | 8 (4 vs fire) | 8 (4 vs fire) |
| Acid Splash | 8 | 8 | 8 |

[stats p. 309]

**Stalker** (adult Praetomorph, P. linesteres) [p. 308]
- An evolutionary dead end: it never metamorphoses further and has no Queen stage.
- More fire-resistant (reflected in its armor).
- Praetomorphs tolerate no other alien species [p. 315].
- **FERAL HUNGER [Stalker only]:** if the Stalker makes an attack that **causes damage**, it immediately makes a **second attack (8 BD, Dmg 1)** against the same target, as a **free action**. [p. 308]
- AMBIGUOUS (A38): The second attack cannot itself trigger Feral Hunger again. It can be blocked, because the rule doesn't say otherwise (A57).

**Scout** ("Runner") [p. 308]
- Fast; usually moves on all fours.
- It looks for hosts and alerts the hive. If no hive exists, it captures and cocoons hosts for ovomorphing until a Queenburster is produced.
- Once a Queen is born and the hive is secure, it metamorphoses (→ Sentry, p. 310).

**Drone** ("Lurker") [p. 308]
- A stealthy ambush hunter.
- **SILENT ASSASSIN [Drone only]:** when the Drone makes a stealth-mode roll or a sneak attack **from within the same zone**, its target gets **−2 to their OBSERVATION test**. [p. 308]
- Its development **arrests at Stage IV** unless a Queen's pheromones make it molt into a **Worker or Soldier**, whichever the hive needs. Until then it eats, gathers hosts for ovomorphing, and sleeps, often near warm places such as a reactor core. [p. 308]

**Stalker, Scout & Drone Attacks** [p. 309]

| D6 | Name | Roll | Effect |
|---|---|---|---|
| 1 | HYPNOTIZING GAZE | — | The victim is mesmerized. They get **+1 STRESS LEVEL** and make an **immediate Panic Roll**. |
| 2 | PLAYING WITH ITS PREY | — | Not a killing attack. The target is **knocked to the ground (prone)** and **drops all hand-held items**, but **takes no damage**. The victim gets **+1 STRESS LEVEL** and makes an **immediate Panic Roll**. |
| 3 | DEADLY GRAB | 10 BD, Dmg 1 | **If it hits:** at once **drags the victim into an adjacent zone** and drops them. Victim: **prone**, **drops all hand-held items**, **immediate Panic Roll**. |
| 4 | READY TO KILL | 10 BD (no Damage given) | **If it hits:** the victim **counts as grabbed** (p. 93). To break loose they must win an **opposed CLOSE COMBAT roll against 10 BD**. **The victim and all friendly characters in the same zone** make Panic Rolls. **Unless the victim breaks free, the Xenomorph uses HEADBITE (entry 6) on them on its next initiative.** |
| 5 | CAPTURE FOR THE HIVE | 10 BD, Dmg 1 | If it causes **any** damage: pulled punch, **only 1 damage** is dealt, and **paralyzing venom** applies. Victim rolls **STAMINA**: **successes = Rounds they stay standing**, then **unconscious for one Shift**. An adrenaline shot (**MEDICAL AID roll with a Medkit**) removes the paralysis. |
| 6 | HEADBITE | 9 BD, Dmg 2 | If it causes any damage: automatically **critical injury #64**, killing the victim in one blow. **GM option:** the victim instead remains just alive enough for the Xenomorph to start ovomorphing. |

- AMBIGUOUS (A36), Ready to Kill:
  - It deals **no damage**: it only grabs. GM option: Dmg 1.
  - The grabbed victim follows p. 93: they drop any held weapon, cannot move, and their only action is the break-free attempt (a slow action, opposed CLOSE COMBAT against 10 BD).
  - The Xeno is not otherwise restricted.
  - With Speed 2–3, the Xeno's next initiative may come before the victim's turn. In that case the victim gets no chance to break free before the Headbite.
  - The forced Headbite still rolls its 9 BD and can be blocked.
- AMBIGUOUS (A37), Deadly Grab: the Xeno moves into the neighboring zone with the victim.
- AMBIGUOUS (A39), Capture for the Hive:
  - 0 successes means the victim falls unconscious immediately.
  - The adrenaline shot can be given before or during the unconsciousness, and ends both the countdown and the unconsciousness.
  - "Pulls its punch" applies after the armor roll: any damage ≥1 becomes exactly 1.
  - The +1 Stress for taking damage still applies.

### 4.5 Stage V — Soldier, Worker, Sentry [p. 310–311]

| Stat | Soldier | Worker | Sentry |
|---|---|---|---|
| Speed | 2 | 1 | 3 |
| Health | 8 | 4 | 8 |
| Mobility | 8 | 4 | 12 |
| Observation | 8 | 4 | 10 |
| Armor Rating | 10 (5 vs fire) | 5 (2 vs fire) | 12 (5 vs fire) |
| Acid Splash | 10 | 6 | 10 |

[stats p. 310]

- **Soldier** ("Warrior") [p. 310]:
  - The adult form of a Drone. Attacks en masse.
  - Its paralytic tail barb is used to carry prey back to the hive for impregnation.
  - It hibernates in the hive walls until the Queen calls [p. 315].
- **Worker** ("Weaver") [p. 310]:
  - Builds and maintains the hive.
  - **Ignores threats within the hive and runs if threatened.** Protection is left to Soldiers and Sentries.
  - "Non-aggressive… offer little resistance."
  - AMBIGUOUS (A35): The book gives **no attack table for Workers**. Default: a Worker never attacks and always flees. GM option: use the Soldier & Sentry table.
- **Sentry** ("Defender") [p. 310]:
  - Matures from Scouts once a hive and a source of hosts are secured.
  - Climbs walls and ceilings very efficiently.
  - Backs off or attacks as the Queen signals. Will sacrifice itself.
  - **Sentries do not leave the inner hive unless the Queen does.**

**Soldier & Sentry Attacks** [p. 311]

| D6 | Name | Roll | Effect |
|---|---|---|---|
| 1 | ASSESSING THE THREAT | — | Pauses, hissing quietly. **Everyone within MEDIUM range gets +1 STRESS LEVEL.** |
| 2 | ONE FOR ALL | — (targets roll MOBILITY) | Jumps at the biggest enemy group. **Each target within SHORT range rolls MOBILITY; on a failure: prone and 1 damage.** **All** targets make immediate Panic Rolls. |
| 3 | ALL-OUT ATTACK | 12 BD, Dmg 2, **AP** | A wild attack with every claw, fang and the tail. |
| 4 | CAPTURE FOR THE HIVE | 12 BD, Dmg 1 | As the Stage IV entry: if it causes any damage, only 1 point is inflicted and the paralyzing venom takes effect. The victim rolls STAMINA; successes = the number of Rounds they stay up, then they are **unconscious for one Shift**. Adrenaline shot = MEDICAL AID roll with a Medkit. **Otherwise the Xeno captures them for the egg chamber.** |
| 5 | DIE FOR THE QUEEN | — (acid 12 BD) | Jumps at an enemy and tears itself open. **Immediate 12 Base Dice acid splash on every target within SHORT range of the Xeno** (§2.1). **The Xeno dies.** |
| 6 | HEADBITE | 10 BD, Dmg 2 | If it causes any damage: automatically **critical injury #64**, killing the victim. |

- AMBIGUOUS (A40), One For All:
  - "Targets" means enemies of the Xeno (PCs and their allies).
  - The 1 damage applies only to those who **fail** the MOBILITY roll. It is not reduced by armor, because there is no attack roll. GM option: armor applies.
  - Panic Rolls apply to everyone within SHORT range, whether or not they passed.
  - Die for the Queen hits everyone within SHORT range, including other Xenos, at the GM's choice.
- AMBIGUOUS (A58), Assessing the Threat: "everyone within MEDIUM range" as written. Androids get no Stress [p. 111].

### 4.6 Stage VI — Praetorian, Charger / Crusher, Queen [p. 312–314]

| Stat | Praetorian | Charger / Crusher | Queen |
|---|---|---|---|
| Speed | 2 | 1 | 2 |
| Health | 10 | 16 | 14 |
| Mobility | 5 | 4 | 8 |
| Observation | 8 | 5 | 12 |
| Armor Rating | 12 (5 vs fire) | 18 (6 vs fire) | 14 (5 vs fire) |
| Acid Splash | 10 | 10 | 10 |

[stats p. 313]

- Only possible once an existing Queen produces Royal Jelly and her Soldiers and Sentries are exposed to it. [p. 312]
- **Praetorian** [p. 312]:
  - Appears when a hive passes **300 individuals**: Workers feed the Queen's chosen warriors Royal Jelly until they transform. It is the penultimate form of a Soldier, a royal guard and army commander.
  - It is theorized that a Praetorian can molt into a new Queen if the hive loses its Queen.
  - Taking out a Praetorian "will temporarily disorient the Soldiers it coordinates". This is GM flavour; no mechanic is given.
- **Charger / Crusher** [p. 312]:
  - A Sentry force-fed Royal Jelly and raw metals **for weeks** cocoons itself and transforms.
  - It **cannot climb or jump very well**. It is rare and appears only in long-established hives.
  - Charging with head shield lowered: almost invulnerable. Its body, though, has only regular Soldier armor.
  - AMBIGUOUS (A48): The GM may use Armor 10 (5 vs fire), the Soldier's value, for attacks to its body or rear, and 18 for frontal attacks.
- **Queen** [p. 312]:
  - At least 6 m tall. The only creature that lays eggs en masse, from an egg sac while suspended in a resin cradle.
  - Communicates with the hive (fluff).
- **Royal Jelly** [p. 313]:
  - A Queen secretion that accelerates metamorphosis in P. praepotens and M. noxhydria.
  - Extremely addictive to humans. Large quantities raise strength, stamina and aggression. No numbers are given.

**Praetorian, Charger & Queen Attacks** [p. 314]

| D6 | Name | Roll | Effect |
|---|---|---|---|
| 1 | CALL THE GUARD | — | Summons **D6 Sentries (Queen)** or **D6 Soldiers (Praetorian or Charger)**. They **arrive in the zone next Round, then draw initiative and may act**. |
| 2 | CHARGE | — (target rolls MOBILITY, no action) | **Charger only. The Praetorian or Queen re-rolls.** The target must make a MOBILITY roll (not an action) or be crushed: **immediately Broken and suffering a random critical injury**. **Even if the roll succeeds**, the victim falls down (prone) and gains **+1 STRESS LEVEL**. |
| 3 | BEASTLY BITE | 10 BD, Dmg 1 | If it causes any damage: **critical injury #61** even if the victim isn't Broken, which triggers a Panic Roll. |
| 4 | CRUSHING BLOW | — (target rolls MOBILITY −2, no action) | Victim rolls **MOBILITY at −2** (not an action). Failure: crushed, **three critical injuries** at once (three rolls on the crit table, all applied, Broken or not). Victim **knocked to the ground**; **immediate Panic Roll**. |
| 5 | TAIL SPIKE | **10 BD (14 BD for a Queen)**, Dmg 1, **AP** | If it causes any damage: automatically **critical injury #66**, killing the victim outright. |
| 6 | HEADBITE | 10 BD, Dmg 2 | If it causes any damage: immediately **critical injury #64**, killing the victim. |

- AMBIGUOUS (A42), Charge, "immediately Broken": set Health to 0. The random critical injury is the one for being Broken, so roll it once, not twice.
- AMBIGUOUS (A41), Crushing Blow: "knocked to the ground and … Panic Roll" applies whatever the MOBILITY result. GM option: only on a failure. Note that the three crits (on a failure) each count as a Panic trigger anyway; per A15, make one Panic Roll.
- AMBIGUOUS (A43), Call the Guard:
  - The GM decides whether guards are available (for example, no hive nearby means none come).
  - The arriving group draws its cards at the start of the following Round, using Speed per creature or per group (A20).
  - Default count: roll the D6 regardless; the GM may cap it.

### 4.7 Stage VII and beyond; other Xenomorph types [p. 314]
- Empress, Queen Mother, Palatine and "Alien King" are hypothetical and **unsubstantiated**. They have no stats.
- Other xenomorph types (xenocobras, squid-like starfish, the "Deacon") are reports only. They have no stats.
- "Ultramorphs" are named on p. 290. They have no stats.

### 4.8 The Alien Hive, hive mind and species interaction [p. 311, 315]
- **Hive** [p. 315]:
  - Resin nest with confusing tunnels. The walls make it easier for Xenos to climb, and Soldiers hide in ridges and pockets.
  - Hosts are cocooned and impregnated in the walls until they birth a Chestburster and die.
  - Soldiers lie dormant in the walls and ceiling until the Queen calls them.
  - The Queen's chamber is central and guarded.
  - Some hives hold diluted pools of A0-3959X.91-15, used to alter hosts.
  - There are no numeric rules. Hive environment effects (Dark, Cluttered, Cramped zone features, p. 83) are for the GM to apply.
- **Hive mind** [p. 311]: six or more Stage IV XX121 operate in unison. **Add a Queen and they molt to Stage V.** No timing is given (A54).
- **Species interaction** [p. 315]:
  - Creatures of different types generally fight each other. A Facehugger and a Soldier cooperate; a Soldier and a Neomorph fight.
  - Neomorphs attack anything living not of their type, and androids only if provoked.
  - XX121 attack any creature they perceive as a threat.
  - Praetomorphs tolerate no other alien species.
  - XX121 from different hives fight to kill each other's Queen. When a hive wins:
    - the losing hive's eggs and Praetorians are destroyed;
    - its Scouts, Workers and Drones are assimilated;
    - its Soldiers are bound to the new Queen or often destroyed.
  - Abominations hate everyone until they mature. They are not statted in the core book.

### 4.9 XX121 evolution paths (summary)

| From | To | Condition / timing | Page |
|---|---|---|---|
| Host (cocooned) | Ovomorph | Ovomorphing by any adult alien; 24–36 h (4–6 Shifts) | p. 301, 312 |
| Queen | Queen's Egg | Laid en masse | p. 300, 312 |
| Egg | Facehugger released | Host at ENGAGED fails MOBILITY (each Round), or Stress Die = 1 with a host at SHORT range (each Round) | p. 300 |
| Facehugger on host | Falls off and dies; host wakes | Within 1 Shift | p. 302 |
| Impregnated host | Chestburster born; host dies | 1 more Shift (Queenburster: days; Imp: faster) | p. 302, 305 |
| Chestburster (any) | Stage IV | Within 1 Shift (after hiding and eating metals) | p. 305 |
| Imp | Stalker | Praetomorph adult; dead end | p. 308 |
| Drone | Worker or Soldier | Queen's pheromones; otherwise arrests at Stage IV | p. 308, 310 |
| Scout | Sentry | Once a Queen is born and the hive/hosts are secure | p. 308, 310 |
| Soldier | Praetorian | Hive > 300; fed Royal Jelly | p. 312 |
| Sentry | Charger | Force-fed Royal Jelly + metals for weeks | p. 312 |
| Queenburster | Queen | First molt | p. 305, 312 |
| Praetorian | Queen | Theorized, if the hive loses its Queen | p. 312 |

---

## 5. Engineers and Agent A0-3959X.91-15 [p. 284–289]
- **Engineers** (Homo Genitor / "Mala'kak" / "Ossians"):
  - Lore only. "No living Engineers are known to exist." [p. 284]
  - Physical traits: averaging 2.7 m, up to 4.5 m. [p. 285]
  - **No stat block in the core book.**
- Engineer technology [p. 288–289]: flute, biosuit, rebreather, sarcophagi, holography, orrery, juggernauts, motherships, dreadnoughts, steatite ampules. These are descriptions only, with **no numbers**.
  - The biosuit "act[s] as armor against ballistic weaponry" and lets the wearer survive a toxic environment "for at least one hour sans helmet". No Armor Rating is given.
- **Agent A0-3959X.91-15** ("black goo") [p. 287]:
  - Atomizes when released.
  - Direct contact makes a living thing die horribly, spawn monsters, or turn into one.
  - **No mechanics** are given. The GM handles it. Neomorph egg sacs result from it (§3.1).

---

## 6. Other extrasolar species [p. 316–323]

These use the same Speed and signature-attack mechanics (§1.2–1.5). They have **no Acid Splash** and no XX121 abilities. Their crit handling is AMBIGUOUS (A13).

### 6.1 The Swarm [p. 316–317]
Space locusts, about 15 cm each. They consume everything except metals.

| Stat | Value |
|---|---|
| Speed | 1 per zone covered |
| Health | 5 per zone covered |
| Skills | None |
| Armor Rating | Impervious to all attacks except fire, explosions and electricity |

[stats p. 317]

- **Swarm rules** [p. 316]:
  - Size is measured in the number of zones the Swarm covers.
  - **Speed = number of zones covered. Health = 5 × number of zones covered.**
  - **The Swarm cannot be killed.** Victims can only flee or try to drive it off.
  - **If its Health drops to 0, it has had enough and goes looking for easier pickings**, i.e. it leaves.
  - It does **not** use the Xeno crit table.
- **Containment** [p. 316]: fire, electrical conductivity and lethal pesticides.
- Its attacks hit characters **inside the Swarm area** (every zone it covers). It does not need ENGAGED range.
- AMBIGUOUS (A44):
  - Attacks other than fire, explosions or electricity deal 0 damage to the Swarm.
  - Fire, explosion and electricity attacks hit it with **no** armor roll (Armor 0).
  - Speed and maximum Health are fixed when the encounter starts. If the GM changes its zone coverage mid-fight, recompute both and add or remove initiative cards at the start of the next Round.
  - CONSUME can raise Health above the starting maximum (no cap is stated). GM option: cap at 5 × zones.

**Swarm Attacks** [p. 317]

| D6 | Name | Roll | Effect |
|---|---|---|---|
| 1 | CONSUME | — | It consumes everything in its path. **Swarm Health +2.** |
| 2 | PHEROMONES | — | **+1 to the Swarm's next roll on this table (cumulative).** |
| 3 | SUFFOCATING SWARM | — (victims roll STAMINA) | **Everyone inside the Swarm area** makes a **STAMINA** roll. **Failure:** the victim **cannot act during the next Round**, and the Swarm performs a **STRIPPED TO THE BONE** attack against them **as its next action**. |
| 4 | DESPERATE HUNGER | 8 BD, Dmg 1 (vs armor) | **Everyone within the Swarm who is wearing armor** suffers an 8 BD, Dmg 1 attack. **Any damage is immediately subtracted from the Armor Rating** (not from Health). Those without armor have their clothes shredded but take **no damage**. |
| 5 | FEEDING FRENZY | 6 BD, Dmg 1 | **All characters inside the Swarm** suffer the attack. **Armor is halved (rounded down).** |
| 6 | STRIPPED TO THE BONE | 8 BD, Dmg 1 | One target. **Armor is halved (rounded down).** If it inflicts any damage, the victim **suffers a critical injury, even if not Broken** (a random D66). **Every later Round, one Swarm action is automatically this attack again on the same target, at the Swarm's next initiative.** Ends when the victim is stripped to the bone or rescued, or the Swarm is driven off. |

- AMBIGUOUS (A44), continued:
  - **Pheromones:** keep a running bonus. Each PHEROMONES result adds +1, and the bonus applies to every later roll until a roll comes up something other than PHEROMONES, then resets to 0. Results above 6 count as 6. GM option: the bonus never resets.
  - **Suffocating Swarm:** if several victims fail, each becomes a forced target, one per Swarm action in the order of its initiative cards.
  - **Desperate Hunger:** no armor roll is made against this attack, and the loss of Armor Rating is permanent until repaired.
  - **Stripped to the Bone:** "stripped to the bone" means dead. Several victims can be locked at once (from Suffocating Swarm).

### 6.2 Harvesters and Harvester Juveniles [p. 318–320]

| Stat | Harvester | Harvester Juvenile |
|---|---|---|
| Speed | 1 | 1 |
| Health | 15 | 4 |
| Mobility | 4 | 8 |
| Observation | — (none listed) | — (none listed) |
| Armor Rating | 15 (except the underbelly, with 5) | 4 |

[stats p. 318]

- Traits [p. 318]:
  - Bursts out of the ground. Threatened: stampedes. Cornered: charges. **Doesn't know when to retreat**.
  - Navigates by sonar.
  - Can breathe otherwise poisonous (high-CO2) atmospheres.
- **HIDDEN BEHEMOTH** [p. 318]:
  - At rest it sleeps just under the surface.
  - Someone walking over that ground gets an **OBSERVATION roll** to spot the weakened earth.
  - **If the roll fails, the ground collapses under them and the startled Harvester gets a free attack.**
- **HARVESTER JUVENILES** [p. 318]:
  - A mother carries **up to 2D6 young** under her armor.
  - **If she comes under attack and is in genuine danger, the young leap to her defense and attack too, rolling 4 Base Dice, Damage 1.**
  - Juveniles have **no D6 table**: every attack they make is 4 BD, Dmg 1.
- **Underbelly** [p. 318]: Armor 5 there. "Good luck getting close enough".
- AMBIGUOUS (A45):
  - **Underbelly:** the GM decides when an attack reaches it (for example, a prone or upended Harvester, or an attacker underneath).
  - **Hidden Behemoth:** the free attack is one D6 signature roll against the person who fell in, and it happens before initiative.
  - **Juveniles:** the GM decides when the mother is in "genuine danger" (default: Health ≤ 7, i.e. half). When they join, the juveniles draw initiative as a group (Speed 1 each).
  - **No Observation listed:** use 0 dice, or GM sets it.
  - **"Doesn't know when to retreat":** if the GM uses the Xeno crit table (A13), ignore its "tries to escape" results.

**Harvester Attacks** [p. 320]

| D6 | Name | Roll | Effect |
|---|---|---|---|
| 1 | SONAR SWEEP | — | **Add +1 to the next roll on this table (maximum 6).** All PCs **up to LONG range** get **+1 STRESS LEVEL**. |
| 2 | SONAR PULSE | — (victims roll STAMINA, no action) | **Everyone within SHORT range** makes a **STAMINA roll** (not an action) or is **stunned and loses their next action**. **Those who are not stunned make a Panic Roll.** |
| 3 | CHARGE | — (targets roll MOBILITY, no action); failures take 10 BD, Dmg 1 | Rushes the player it sees as the biggest threat; the charge **carries it into the next zone**, hitting **that player and everyone in that direction**. **Each target rolls MOBILITY (not an action)** to dodge. **Failure:** a **10 BD, Dmg 1** attack and **knocked prone**. |
| 4 | STAMPEDE | 6 BD, Dmg 2 | **All targets within SHORT range** suffer a crushing attack of 6 BD, Dmg 2 and are **knocked prone if hit**. |
| 5 | BURROWING CLAWS | 10 BD, Dmg 2 | Leaps on one victim. |
| 6 | PULVERIZE | Opposed: victim's CLOSE COMBAT vs 8 BD (no action) | The victim is dragged into the mouth. They make a last-ditch **opposed CLOSE COMBAT roll against 8 BD** (not an action) to dodge. **If they fail, they die.** **All who witness this get +1 STRESS LEVEL and make a Panic Roll.** |

- AMBIGUOUS (A45), continued:
  - **Charge:** the Harvester ends in the next zone. "In that direction" means everyone in the line between it and the target, through to the next zone (GM picks).
  - **Sonar Pulse:** "everyone" includes NPCs.
  - **Pulverize:** a tie (or 0 vs 0) counts as a failure for the victim, because the victim must "win" the opposed roll [p. 64]. The witness Stress and Panic apply only if the victim dies (GM option: always).
  - **Sonar Sweep:** the +1 is not cumulative. The cap of 6 is explicit here and for the Lion Worm; for the Swarm it is only implied (A56).
  - **Sonar Sweep stress:** applies to all PCs as written. Androids are unaffected (A58).

### 6.3 Lion Worms [p. 320–321]

| Stat | Value |
|---|---|
| Speed | 2 |
| Health | 8 |
| Skills | Mobility 12, Observation 8 |
| Armor Rating | 4 |

[stats p. 321]

- Traits [p. 320]:
  - A relentless hunter that attacks first and **retreats underground when confused**.
  - Burrows only through soft earth.
  - Disoriented by released pollen or pollutants.
  - Easily distracted by the pheromones of another Lion Worm: it seeks out and attacks the "offender".
- **AMBUSH PREDATOR** [p. 321]: in stealth mode, the Lion Worm's prey get **−2 to OBSERVATION** if they are in the Lion Worm's territory.

**Lion Worm Attacks** [p. 321]

| D6 | Name | Roll | Effect |
|---|---|---|---|
| 1 | INTIMIDATING DISPLAY | — | A bluff; **on its next action it makes a tactical retreat**. **Panic Roll for every PC within SHORT range.** After the retreat, **stealth mode resumes** and the worm keeps hunting the PCs. **Its next attack gets +1 on this table (maximum 6).** |
| 2 | PLAY WITH ITS PREY | 8 BD (no Damage) | **If it hits:** the target is knocked to the ground (prone) and **drops all hand-held items**, but is otherwise unharmed. The victim makes an **immediate Panic Roll**. |
| 3 | ACRID PHEROMONES | — (victims roll STAMINA) | **All PCs within SHORT range** make a **STAMINA** roll. **If it fails, they are stunned and lose their next slow action.** |
| 4 | FERAL RAGE | 10 BD, Dmg 2 | If it causes any damage, it immediately makes a **second attack as a free action, with the same dice and Damage (10 BD, Dmg 2)**. |
| 5 | CRIPPLING ATTACK | 8 BD, Dmg 1 | If it does any damage: automatically **critical injury #56** (Cracked spine), even if the victim isn't Broken. |
| 6 | DEATH ROLL | 10 BD, Dmg 1 | **If it hits:** victim is **grappled** and disoriented, **losing their next slow action**. **The worm's next action: a squeeze for 10 BD, Dmg 1 again.** |

- AMBIGUOUS (A46):
  - **Play With Its Prey:** the Panic Roll happens only on a hit.
  - **Feral Rage:** the second attack does not chain.
  - **Death Roll:**
    - The squeeze happens once, on the worm's next action.
    - After that, while the victim is still grappled, the GM may repeat the squeeze instead of rolling on the table.
    - Breaking free follows p. 93: opposed CLOSE COMBAT, but the book gives no dice for the worm. Use 10 BD by analogy, or its Mobility 12, as the GM chooses.
    - Losing the next slow action means the victim cannot even try to break free on their next turn.
  - **Intimidating Display:** the +1 lasts until the worm's next attack roll.

### 6.4 Tanakan Scorpionids [p. 322–323]

| Stat | On Land | In Water |
|---|---|---|
| Speed | 1 | 3 |
| Health | 5 | 5 |
| Mobility | 6 | 10 |
| Observation | 6 | 10 |
| Armor Rating | 6 (3 against bullets) | 6 (3 against bullets) |

[stats p. 322]

- Traits [p. 322]:
  - Hides in dunes and rocks by day and hunts at night. Aggressive if provoked. A strong swimmer.
  - Its shell protects it against knives and other melee weapons, but not against bullets.
  - Its venom kills quickly without an antidote. (Fluff; the mechanic is Tail Strike below.)
- **UNDERWATER SPRINT** [p. 322]:
  - Slow action, counts as two run actions: two zones, or straight from an adjacent zone to ENGAGED range.
  - Only **if it is in water or its movement starts in water**.
  - It uses this to sneak-attack prey on the shore.
- AMBIGUOUS (A47), stat profile and armor:
  - Use the "In Water" profile while the Scorpionid is in water, and "On Land" otherwise.
  - Speed changes take effect from the next Round: add or remove initiative cards.
  - "Against bullets" means firearm projectile attacks. The GM decides for other ranged weapons.

**Scorpionid Attacks** [p. 323]

| D6 | Name | Roll | Effect |
|---|---|---|---|
| 1 | RETREAT | — | It decides to **flee the fight as quickly as possible**. |
| 2 | PINNED DOWN | 10 BD, Dmg 1 | **If it hits:** target goes **prone**, **drops hand-held items**, **immediate Panic Roll**. |
| 3 | PINCER GRAB | 8 BD, Dmg 1 | **If it hits:** the victim is **grappled** (p. 93). To break loose they must win an **opposed CLOSE COMBAT roll against 8 BD**. **Grasp kept → automatic TAIL STRIKE (entry 5) on the victim at its next initiative.** **If this attack hits underwater**, it also pulls the victim deeper and **one zone away from the group**, and the victim suffers **drowning** effects (p. 110). |
| 4 | FLAILING STRIKE | 8 BD, Dmg 1 | **If it hits:** victim is thrown **into the next zone** and takes an **immediate critical injury** (random D66, Broken or not), ending **prone** and **dropping hand-held items**. |
| 5 | TAIL STRIKE | 10 BD, Dmg 1 | If it inflicts any damage, it poisons the victim with **venom counting as a disease with Virulence 9** (p. 109). |
| 6 | IMMOBILIZE | 8 BD, Dmg 2 | If it causes any damage: **immediate critical injury #35** (Busted kneecap), Broken or not. |

- AMBIGUOUS (A47), continued:
  - **Pincer Grab:** "this attack" means the Pincer Grab itself. Drowning lasts while the victim is held underwater (until they break free or are rescued).
  - **Tail Strike venom:** apply the full disease rules (p. 109): a Sickness Roll against 9, the first damage one Shift later, and Sickness Rolls each Shift. Androids are immune.
  - **Retreat:** the Scorpionid uses its actions to move away (Sprint if it is in water). It leaves combat once it is out of sight (A9).

---

## 7. Master stat table (quick reference)

| Creature | Spd | HP | Mob | Obs | Armor | Armor vs fire (or special) | Acid | Sprint | Attack table | Page |
|---|---|---|---|---|---|---|---|---|---|---|
| Neomorphic Bloodburster | 3 | 2 | 9 | 6 | 3 | 0 | — | Yes | Bloodburster | 295 |
| Neophyte (juvenile) | 3 | 4 | 10 | 6 | 4 | 2 | — | Yes | Neomorph (A34) | 295 |
| Adult Neomorph | 2 | 6 | 9 | 8 | 6 | 3 | — | Yes | Neomorph | 296–297 |
| Ovomorph | 0 | 2 | — | — | 2 | 0 | 4 | No | none | 301 |
| Queen's Egg | 0 | 3 | — | — | 3 | 1 | 5 | No | none | 301 |
| Facehugger | 2 | 2 | 8 | 8 | 2 | 0 | 4 | Yes | Facehugger | 303 |
| Praeto-Facehugger | 2 | 2 | 6 | 8 | 2 | 0 | 4 | Yes | Facehugger | 303 |
| Royal Facehugger | 2 | 3 | 8 | 10 | 4 | 2 | 5 | Yes | Facehugger (Tail Lash 6 BD/Dmg 2) | 303 |
| Chestburster | 2 | 2 | 8 | 6 | 2 | 0 | 4 | Yes | Chestburster | 306 |
| Bambi Burster | 2 | 2 | 10 | 6 | 2 | 0 | 4 | Yes | Chestburster | 306 |
| Imp | 2 | 2 | 8 | 8 | 0 | **3** | 4 | Yes | Chestburster | 306 |
| Queenburster | 2 | 3 | 8 | 6 | 4 | 2 | 5 | Yes | Chestburster | 306 |
| Stalker | 2 | 6 | 8 | 6 | 10 | 5 | 8 | Yes | Stage IV (+Feral Hunger) | 309 |
| Scout | 3 | 5 | 8 | 10 | 8 | 4 | 8 | Yes | Stage IV | 309 |
| Drone | 2 | 7 | 10 | 8 | 8 | 4 | 8 | Yes | Stage IV (+Silent Assassin) | 309 |
| Soldier | 2 | 8 | 8 | 8 | 10 | 5 | 10 | Yes | Soldier & Sentry | 310–311 |
| Worker | 1 | 4 | 4 | 4 | 5 | 2 | 6 | Yes | none (A35) | 310 |
| Sentry | 3 | 8 | 12 | 10 | 12 | 5 | 10 | Yes | Soldier & Sentry | 310–311 |
| Praetorian | 2 | 10 | 5 | 8 | 12 | 5 | 10 | Yes | Stage VI | 313–314 |
| Charger / Crusher | 1 | 16 | 4 | 5 | 18 | 6 | 10 | Yes | Stage VI (incl. Charge) | 313–314 |
| Queen | 2 | 14 | 8 | 12 | 14 | 5 | 10 | Yes | Stage VI (Tail Spike 14 BD) | 313–314 |
| Swarm | 1/zone | 5/zone | — | — | impervious | fire, explosions, electricity only | — | — | Swarm | 316–317 |
| Harvester | 1 | 15 | 4 | — | 15 | underbelly 5 | — | — | Harvester | 318–320 |
| Harvester Juvenile | 1 | 4 | 8 | — | 4 | — | — | — | fixed 4 BD, Dmg 1 | 318 |
| Lion Worm | 2 | 8 | 12 | 8 | 4 | — | — | — | Lion Worm | 321 |
| Scorpionid (land) | 1 | 5 | 6 | 6 | 6 | 3 vs bullets | — | — | Scorpionid | 322–323 |
| Scorpionid (water) | 3 | 5 | 10 | 10 | 6 | 3 vs bullets | — | Underwater Sprint | Scorpionid | 322–323 |

---

## 8. Creature turn procedure (implementation flow)

### 8.1 Encounter start (stealth → combat)
1. **Sneak or ambush check (optional, before initiative)** [p. 85, 90]:
   - If an ACTIVE creature tries a sneak attack, roll the creature's MOBILITY (Base Dice) against the target's OBSERVATION (with Stress Dice).
   - Apply the range modifier: ENGAGED −2, SHORT −1, MEDIUM 0, LONG +1, EXTREME +3. Add +2 for an ambush.
   - Creature modifiers: Drone Silent Assassin (target −2 OBSERVATION when attacking from the same zone); Lion Worm Ambush Predator (prey −2 OBSERVATION in its territory). Watchful talent: target +2.
   - Hidden Behemoth: an OBSERVATION roll for whoever walks over the Harvester. On a failure the ground collapses and the Harvester gets a free attack.
   - **On a success**, the creature gets **one free action (slow OR fast) before initiative**, and the target cannot block it. If the action is an attack, run §8.4 (one D6 signature roll).
   - **On a failure**, the creature is spotted at its starting distance. Go to step 2.
2. **Draw initiative** [p. 87, 291]:
   - Each creature draws **Speed** cards: for Scorpionids, the Speed of the current environment; for the Swarm, the number of zones it covers.
   - Speed 0 creatures (eggs) draw none.
   - With more than 10 combatants, group NPCs that have identical stats (A20).
   - Store the cards per creature in ascending order.
3. **First-sighting panic** [p. 104]: when a creature first attacks a PC who has never seen that kind of creature, that PC makes a Panic Roll. Track this per PC and per creature type.

### 8.2 Start of each Round (in this order)
1. **Arrivals:** guards from a Call the Guard made in the previous Round enter the zone and draw initiative now [p. 314].
2. **Ongoing burns:**
   - **Acid:** each active acid effect attacks its victim with `dice = floor(previous_dice / 2)`. Armor applies (A17), and each armor success reduces Armor Rating by 1. If the attack deals 0 damage, or dice reach 0, the effect ends. If the victim is Broken by it, or takes acid damage while Broken, set the per-Round Death Roll flag [p. 298].
   - **Fire:** Intensity +1 each Round [p. 108]. This applies both to creatures set on fire and to PCs.
3. **Per-Round Death Rolls:** victims flagged by acid or fire make their Death Roll [p. 298, 108].
4. **Wounded creatures** (Xeno crit 2): roll D6; on 1–3 the creature tries to escape this Round [p. 291].
5. **Egg checks** [p. 300]:
   - Every potential host at ENGAGED range of an unopened, non-dormant egg rolls MOBILITY; a failure opens the egg.
   - Every egg with a potential host at SHORT range (and none at ENGAGED) rolls one Stress Die; a 1 opens the egg.
   - A released Facehugger draws initiative (A21).
6. **Paralysis countdowns** (Capture for the Hive): decrement the "Rounds still standing" counter. At 0 the victim falls unconscious; start a 1-Shift timer [p. 309, 311].
7. **Continuing locks:** a Swarm with Stripped-to-the-Bone targets has its next action(s) reserved [p. 317]. A Suffocating Swarm victim "cannot act during the next Round" [p. 317].

### 8.3 On each of a creature's initiative cards (lowest number first)
1. **Skip** if the creature is dead, has escaped, or the card was removed (Wounded, A8).
2. **Resolve the crit state first** [p. 291]:
   - Rise Again pending → it regains 1 Health and stands up, then acts normally (A7).
   - Last Breath pending → it makes one attack against the nearest victim (A12), then it **dies**. End.
3. **Resolve forced follow-ups**, if any. They replace the D6 roll for this action:
   - Facehugger after a won Face Grapple → THE FINAL EMBRACE against that target [p. 303].
   - Facehugger after a lost Face Grapple → a new attack (D6 roll) against the **same** target [p. 303].
   - Stage IV after Ready to Kill, with the victim still grabbed → HEADBITE (9 BD, Dmg 2) against them [p. 309].
   - Scorpionid after Pincer Grab, with its grasp retained → TAIL STRIKE (10 BD, Dmg 1) [p. 323].
   - Lion Worm after Death Roll → squeeze (10 BD, Dmg 1) [p. 321].
   - Lion Worm after Intimidating Display → **tactical retreat** (its next action). Stealth mode resumes for it, and its next attack roll gets +1 [p. 321].
   - Swarm with locked victim(s) → STRIPPED TO THE BONE against each locked victim, one per action [p. 317].
   - Creature trying to escape (Wounded or Desperate Action) → it spends its actions moving away (A9).
4. **Behaviour:** the creature takes **one fast + one slow action** [p. 291]. Typical options:
   - Run (fast): one zone, or SHORT ↔ ENGAGED. Not allowed with an enemy at ENGAGED; it must retreat instead [p. 89, 93].
   - Sprint (slow): two zones, or from an adjacent zone into ENGAGED [p. 299 etc.].
   - Signature attack (slow, A2): go to §8.4.
   - Behaviour constraints:
     - Workers flee [p. 310].
     - Sentries stay in the inner hive unless the Queen leaves [p. 310].
     - The Charger can't climb or jump well [p. 312].
     - Neomorphs ignore androids unless provoked [p. 315].
5. After the attack, use the remaining fast action if possible (for example, a run).

### 8.4 Signature attack resolution
1. **Roll the D6 entry:**
   - `r = D6 + pending_bonus`, capped at 6. Bonuses: Swarm Pheromones (cumulative, A44), Harvester Sonar Sweep (+1), Lion Worm after Intimidating Display (+1).
   - Clear one-shot bonuses after use.
   - If the entry is invalid for this creature (CHARGE for a Praetorian or Queen), re-roll [p. 314].
2. **Choose target(s):** "the victim" means one target at ENGAGED range, chosen by the GM (A51). Area entries use the range or area stated:
   - MEDIUM: Assessing the Threat.
   - SHORT: One For All, Die for the Queen, Stampede, Sonar Pulse, Acrid Pheromones, Intimidating Display.
   - LONG: Sonar Sweep.
   - Swarm area: Suffocating Swarm, Desperate Hunger, Feeding Frenzy.
   - "In that direction": Harvester Charge.
   - Same zone as the victim: the Panic in Ready to Kill.
3. **Entry type:**
   - **a) No roll (automatic effect):** apply the effect. A block cannot stop it (A4).
   - **b) Target-roll entry:**
     - The target rolls the stated skill, and **it is not an action**: MOBILITY (Charge, Crushing Blow at −2, One For All, Harvester Charge) or STAMINA (Sonar Pulse, Acrid Pheromones, Suffocating Swarm).
     - Apply the success or failure branch.
   - **c) Opposed entry (Face Grapple, Pulverize):**
     - The creature's Base Dice and the target's skill are rolled at the same time; it is not an action for the victim.
     - Each target success cancels one creature success. The side that "wins" must have successes remaining.
     - On a tie, the side that needed to win loses (A24, A45).
   - **d) Attack-roll entry (N BD, Dmg D):**
     1. Before the roll, the target may declare a **block** (fast action; needs a weapon or blunt instrument; needs an unused action) or a **Dodge** (talent). Not allowed if the entry says "cannot be blocked", or for sneak or retreat free attacks.
     2. Roll N Base Dice and count successes `s`.
     3. Block: the blocker rolls CLOSE COMBAT, and each block success removes one of `s` (or counterattacks or disarms) [p. 92].
     4. **Hit** if `s ≥ 1` → fire the "if it hits" effects. Otherwise it misses; stop. (Exception: Swarm Desperate Hunger on unarmored targets: clothes are shredded, no damage.)
     5. DECIDED: damage `dmg = D + (s − 1)` (A3; no other stunts; GM alternative `dmg = D`, F-03).
     6. **Armor roll:** the target rolls `A` Base Dice. `A` is the Armor Rating, halved **rounding up** if AP [p. 118], or halved **rounding down** for Swarm entries [p. 317]. `dmg −= armor successes` (minimum 0). Desperate Hunger: no armor roll (A44).
     7. If `dmg ≥ 1` → **"causes damage"**:
        - Subtract from Health (Desperate Hunger: from Armor Rating instead).
        - The victim gets **+1 STRESS LEVEL** for taking damage [p. 103].
        - Special damage caps: Capture for the Hive sets dmg = 1.
        - Fire the "if it causes damage" effects: fixed crit #N even if not Broken; venom disease; paralysis; Feral Hunger / Feral Rage extra attack.
        - Apply the fixed or random crit. If the victim also hits 0 Health, they are Broken; per A15 the fixed crit replaces the random Broken crit.
        - Instant-kill crits (#63–66) → the victim dies, with no Death Roll [p. 99].
     8. If `dmg = 0` after armor, effects triggered by "hits" still apply, but effects triggered by "causes damage" do not.
4. **Apply status effects** from the entry: prone; dropped hand-held items; dragged to the next zone (the creature moves too, A37); grabbed or grappled (store the break-free dice: 10 for Ready to Kill, 8 for Pincer Grab); stunned and loses next action / next slow action; cannot act next Round; drowning; Stress +1; forced follow-up flags (§8.3 step 3).
5. **Panic Rolls:** collect all Panic triggers for each character from this entry (entry text, crit, witnessing) and make **one** Panic Roll per character per triggering event (A15). Androids are exempt [p. 111].
6. **Extra or free attacks** that don't count as actions, resolved immediately:
   - Neomorph Leaping Attack: 10 BD, Dmg 2, **unblockable**.
   - Stalker Feral Hunger: 8 BD, Dmg 1, same target, only if the triggering attack caused damage.
   - Lion Worm Feral Rage: 10 BD, Dmg 2, only if it caused damage.
   - These do not chain (A38, A46).
7. **Self-effects:**
   - Die for the Queen → the creature dies after the acid attack.
   - Escape results → move the creature two zones in **one** action (no retreat roll, A55). If the PCs lose line of sight, the creature leaves combat (Bloodburster/Chestburster: stealth mode ensues; Bloodburster growth timer = D6 Turns).
   - Retreat (Scorpionid) or tactical retreat (Lion Worm) → the creature leaves combat when out of sight.
   - Call the Guard → schedule D6 arrivals for the next Round.
   - Consume → Swarm Health +2.
   - Pheromones → bonus +1.

### 8.5 When a creature takes damage (PC or NPC attacks a creature)
1. **Creature armor roll:**
   - Choose the Armor Rating: the fire value vs fire; "vs bullets" for the Scorpionid; underbelly 5 for the Harvester when it applies; impervious for the Swarm except fire, explosions and electricity.
   - Halve it, rounding up, if the attack is AP.
   - Roll that many Base Dice; each success = −1 damage.
   - Creatures never push and take no stress.
2. Subtract the damage from Health (minimum 0).
3. **State reactions:**
   - **Last Breath** pending and damage ≥1 → the creature dies instantly [p. 291].
   - **Rise Again** pending and it is attacked → it rises with 1 Health after this attack (A7).
   - **Bloodburster** still in its sluggish birth Round and takes fire or serious damage → it runs off and hides; start the "becomes Neophyte" timer [p. 294].
   - **Harvester** mother in genuine danger → her 2D6 juveniles join (A45).
   - **Swarm** Health 0 → it leaves [p. 316].
4. **Acid Splash** (every XX121 stage, eggs included):
   - Only if damage ≥1 and the damage type draws blood, i.e. **not fire**.
   - Each character at ENGAGED range of the creature (A17) suffers `AcidSplash + damage_taken` Base Dice, base Damage 1.
   - Each armor success also reduces that armor's Armor Rating by 1, permanently.
   - If the victim takes damage, start a continuing-acid effect with `dice = AcidSplash + damage_taken`. It is halved at the start of each new Round.
   - If the victim is Broken by it, set the per-Round Death Roll flag [p. 298].
5. **Zero Health:**
   - Swarm → it leaves.
   - Any other creature, if it is reduced to 0 by this attack → roll D6 on the Critical Injuries on Xenomorphs table [p. 291]. Others than Xeno lines: A13.
   - Desperate Action → resolve its immediate fast + slow action now, out of turn order.

### 8.6 Ending combat
- Combat ends, for the escaping creature, when an ESCAPE result breaks line of sight [p. 295, 306]. Return to stealth mode, which is played in Turns [p. 85].
- An adult Neomorph past 4 Shifts dies [p. 296]. A Neophyte at 1 Shift and an escaped Bloodburster at D6 Turns change stage (swap the stat block).

---

## 9. Time-related rules

| Event | Duration / timing | Unit | Page |
|---|---|---|---|
| Creature movement in stealth mode | ERRATA: **2 zones** per point of Speed **per Turn**; humans 2 zones per Turn (Speed 1) (A1, resolved) | Turn | p. 291 (errata) |
| Creature actions in combat | Speed × (1 fast + 1 slow) **per Round** | Round | p. 291 |
| Egg opening (host at ENGAGED) | MOBILITY roll **each Round**; a failure opens the egg | Round | p. 300 |
| Egg opening (host at SHORT) | Stress Die **each Round**; a 1 opens the egg | Round | p. 300 |
| Egg dormancy | Extreme cold makes the egg dormant (no duration) | — | p. 300 |
| Ovomorphing (host → egg) | 24–36 hours = **4–6 Shifts** | Shift | p. 301 |
| Facehugger attached | Facehugger dies and falls off **within 1 Shift**; the host is unconscious until then | Shift | p. 302 |
| Chestburster gestation | Born **1 Shift after** the Facehugger falls off; the host dies | Shift | p. 302 |
| Queenburster gestation | "**Days** instead of hours" (unquantified, A29) | Day | p. 305 |
| Imp gestation | "Substantially faster" (unquantified, A29) | — | p. 305 |
| Gestation modifiers | Hypersleep infection slows it on revival; stasis **pauses** it until the host wakes; GM may speed it up or slow it down | — | p. 306 |
| Chestburster → Stage IV | **Within 1 Shift** of birth | Shift | p. 305 |
| Drone → Worker/Soldier, Scout → Sentry | When a Queen is present (pheromones) or the hive is secured (event-driven) | — | p. 308, 310–311 |
| Sentry → Charger | Force-fed **for weeks** | Week | p. 312 |
| Soldier → Praetorian | Hive > 300 individuals; Royal Jelly (event-driven) | — | p. 312 |
| Neomorph motes infection | One Sickness Roll (Virulence 9) on exposure. A failure → Stage III **within 1 Shift**. No further Sickness Rolls. | Shift | p. 294 |
| Bloodburster birth | The host dies immediately on entering Stage III | — | p. 294 |
| Bloodburster sluggish | **1 Round** to free itself from the sack | Round | p. 294 |
| Bloodburster → Neophyte | "A few Turns"; after an ESCAPE result: **D6 Turns** | Turn | p. 294–295 |
| Neophyte → Adult | **1 Shift** | Shift | p. 295 |
| Adult Neomorph lifespan | Dies of natural causes **within 24 hours (4 Shifts)** | Shift | p. 296 |
| Acid splash continuing burn | Attack at the **start of each Round**, dice halved (rounded down) each new Round; ends when an attack deals no damage | Round | p. 298 |
| Broken by acid | Death Roll **every Round** until death or MEDICAL AID | Round | p. 298 |
| Capture for the Hive paralysis | Stays up for (STAMINA successes) **Rounds**, then unconscious **1 Shift**; ended early by adrenaline (MEDICAL AID + Medkit) | Round / Shift | p. 309, 311 |
| Ready to Kill → Headbite | On the Xeno's **next initiative**, unless the victim breaks free | Initiative | p. 309 |
| Face Grapple → Final Embrace | On the Facehugger's **next initiative** | Initiative | p. 303 |
| Pincer Grab → Tail Strike | On the Scorpionid's **next initiative**, if it keeps its grasp | Initiative | p. 323 |
| Death Roll (Lion Worm) squeeze | On the worm's **next action** | Initiative | p. 321 |
| Intimidating Display retreat | On the worm's **next action**; +1 on its next attack roll | Initiative | p. 321 |
| Call the Guard arrival | **The following Round**, then they draw initiative | Round | p. 314 |
| Wounded (Xeno crit 2) | Roll D6 at the **start of every Round**; 1–3 → tries to escape | Round | p. 291 |
| Rise Again / Last Breath | Trigger on the creature's **next initiative** (or when attacked/wounded) | Initiative | p. 291 |
| Swarm Stripped to the Bone | Repeats **each Round**, on the Swarm's next initiative | Round | p. 317 |
| Suffocating Swarm failure | Victim cannot act during **the next Round** | Round | p. 317 |
| Sonar Pulse / Acrid Pheromones stun | Loses **next action** / **next slow action** | Action | p. 320–321 |
| Lion Worm Death Roll disorientation | Victim loses **next slow action** | Action | p. 321 |
| Scorpionid venom | Disease, Virulence 9: damage 1 Shift after infection, Sickness Roll each Shift | Shift | p. 323, p. 109 |
| Fixed crits #61 / #53 from bite and slash entries | Fatal crit time limits before the Death Roll: #61 (Ruptured jugular) one Round; #53 (Leg artery cut) one Turn | Round / Turn | p. 100 |
| Facehugger Tail Grapple choke | Drowning: STAMINA every Round (duration A25) | Round | p. 303, 110 |

---

## 10. State the app must track

### 10.1 Per creature instance
- `type`; the stat block (Speed, max Health, Mobility, Observation, Armor, Armor-vs-fire / vs-bullets / underbelly, Acid Splash, has Sprint, attack table id); current stage (for life-cycle swaps).
- `health_current` (0..max; the Swarm can exceed max, A44).
- `speed_current`, which can be reduced by the Wounded crit; for the Swarm = zones covered; for the Scorpionid it depends on the environment.
- `initiative_cards[]` (one per point of Speed); `actions_used_this_slot` (fast/slow).
- `zone`; `range_to[target]` (ENGAGED/SHORT/…); `in_water` (Scorpionid); `zones_covered` (Swarm); `in_territory` (Lion Worm).
- `awareness`: ACTIVE / PASSIVE / hidden (stealth); `in_combat` / `escaped`.
- `crit_state`: none / `rise_again_pending` / `wounded` (start-of-Round escape check) / `last_breath_pending` / `dead`. Plus `xeno_crit_rolls[]` (history).
- `trying_to_escape` (from Wounded or Desperate Action).
- `pending_forced_action`: {kind: FinalEmbrace | ReAttackSameTarget | Headbite | TailStrike | Squeeze | TacticalRetreat | StrippedToTheBone, target_id, due: next_initiative}.
- `grabbing[target_id]` with `break_free_dice` (Ready to Kill 10, Pincer Grab 8, Lion Worm Death Roll A46).
- `table_roll_bonus` (Swarm Pheromones cumulative; Harvester Sonar Sweep +1 one-shot; Lion Worm +1 one-shot).
- `on_fire` (Intensity), for fire attacks against it.
- Bloodburster: `sluggish_until_round`; `grow_to_neophyte_at` (D6 Turns).
- Neophyte: `becomes_adult_at` (+1 Shift). Adult Neomorph: `dies_at` (+4 Shifts).
- Chestburster family: `born_at`, `evolves_at` (+≤1 Shift), `target_stage_iv` (A31).
- Egg: `opened` (bool), `dormant` (cold), `queen_laid` (stat choice), `facehugger_variant` held (standard / praeto / royal).
- Harvester: `juveniles_count` (2D6), `juveniles_deployed`.
- Swarm: `stripped_targets[]`.
- Charger: `charging` / head-shield state (A48).
- Group membership (identical stats sharing initiative, A20).
- Encounter-level: `seen_creature_types` per PC (first-sighting Panic, p. 104); `reinforcements_due_next_round` (Call the Guard).

### 10.2 Per affected or infected character
- **Facehugged / impregnated (XX121):**
  - `facehugged_at`, `facehugger_variant` (standard, praeto, royal), `host_type` (biped / quadruped), `unconscious`, and Broken via Final Embrace.
  - `facehugger_falls_off_at` (+≤1 Shift).
  - `embryo_type` (Chestburster / Bambi / Imp / Queenburster) and `chestburst_at` (fall-off + 1 Shift; Queenburster days; Imp faster; GM-editable).
  - `gestation_paused` (stasis / hypersleep), with paused time accounted for.
  - `ovomorphing_started_at` / `ovomorph_complete_at` (+4–6 Shifts), if being ovomorphed.
- **Neomorph motes:** `mote_exposure_at`, `sickness_roll_result`, `stage_iii_at` (+≤1 Shift → death + Bloodburster spawn).
- **Acid effects:** a list of {`dice_current`, `started_round`}. The Death-Roll-every-Round flag (acid).
- **Armor degradation:** `armor_rating_current` per armor item, reduced by acid armor successes and Swarm Desperate Hunger.
- **Paralysis (Capture for the Hive):** `rounds_standing_left` (from STAMINA successes), then `unconscious_until` (+1 Shift); can be cleared by adrenaline.
- **Grabbed / grappled by** creature_id, with the break-free dice.
- **Status flags:** prone; dropped items; `arms_tangled` (cannot use held items); `choking` (drowning rolls); `stunned_lose_next_action` / `lose_next_slow_action`; `cannot_act_next_round` (Suffocating Swarm); dragged / thrown zone changes.
- **Pending forced attacks** targeting them (Final Embrace, Headbite, Tail Strike, Squeeze, Stripped to the Bone).
- **Disease:** Scorpionid venom (Virulence 9), using the full p. 109 disease state.
- **Fixed crits received** (#35/#53/#56/#61/#64/#66), with their time limits. STRESS LEVEL changes and Panic Roll triggers.
- `has_seen[creature_type]`; witnessed a first chestburster birth (one-time Panic Roll).
- Android flag: immune to disease, motes, vacuum and cold; no stress or panic; synthetic crit table.

### 10.3 Per egg clutch / environment
- Neomorph egg sac clutches (2D6 sacs each): disturbed, destroyed, or inert (liquid nitrogen until thawed).
- Hive state (population for the Praetorian > 300 threshold; Queen present; Royal Jelly). Event-driven; GM-set.

---

## 11. Ambiguities / Game Mother discretion

- **A1** Stealth movement. ERRATA: resolved. The official errata for p. 291 sets 2 zones per point of Speed per Turn, and humans move 2 zones per Turn (Speed 1). This matches p. 85. No GM toggle. [p. 85, 291]
- **A2** A signature attack is the creature's slow action (implied by p. 88 and p. 291).
- **A3** DECIDED: signature attack damage = the entry's Damage + 1 per success beyond the first. No other stunts. GM alternative: base Damage only (F-03). Supported by "base Damage" [p. 298, 303] and the close-combat stunt [p. 92].
- **A4** Entries with no attack roll cannot be stopped by blocking.
- **A5** Block counterattack against a creature: normal damage (it can cause an Acid Splash); disarm has no effect.
- **A6** Creatures have no Stress and no Panic.
- **A7** Rise Again: the triggering attack resolves first; the creature acts normally on the initiative when it rises.
- **A8** Wounded: "lowest initiative" = the lowest-numbered card. The escape D6 repeats every Round.
- **A9** "Tries to escape" / Scorpionid Retreat: the creature moves away and escapes once out of line of sight. No roll is defined.
- **A10** Desperate Action escape: regains half of **maximum** Health, rounded down, minimum 1.
- **A11** A failed Desperate Action escape has no stated consequence.
- **A12** Last Breath: one signature attack against the nearest victim. GM may pick the deadliest entry.
- **A13** The Xeno crit table applies to the Neomorph and XX121 lines. For Harvesters, Juveniles, Lion Worms and Scorpionids it is not stated (default: use the table; option: die at 0). The Swarm leaves at 0.
- **A14** Re-roll on the Xeno crit table each time the creature is reduced to 0 Health again.
- **A15** Fixed crit + "triggering a Panic Roll" = one Panic Roll. A fixed crit replaces the random Broken crit when both would happen.
- **A16** Androids: fixed human crits → synthetic crit table. Not hosts for facehugging or ovomorphing. Immune to motes and venom.
- **A17** Acid Splash: hits everyone at ENGAGED range of the creature (a ranged attacker not at ENGAGED is spared); roll per victim; armor applies to continuing burns and those armor successes also erode it; "draws blood" excludes only fire (GM decides for other non-blood damage types).
- **A18** Several acid splashes on one victim are tracked as separate burns.
- **A19** A failed retreat from a creature gives it a free signature attack (D6 roll). Creatures retreating roll their own MOBILITY.
- **A20** Initiative groups for creatures with Speed > 1, and what happens when the ten cards run out.
- **A21** Egg checks: start of Round; once per Turn in stealth mode; one Stress Die per egg; MOBILITY per host at ENGAGED range.
- **A22** "Immobilized potential host" = Broken, unconscious, paralyzed, grabbed or cocooned (GM).
- **A23** Final Embrace "immediately Broken": Health 0, unconscious, no random crit by default.
- **A24** Face Grapple: a tie means the victim wins; "suffer THE FINAL EMBRACE" still rolls 6 BD; "attacks again" = a new D6 roll against the same target; "next initiative" can be in the same Round.
- **A25** Tail Grapple: effects last until the Facehugger next acts, dies, or the victim breaks free (opposed CLOSE COMBAT against 6 BD). No duration is given in the book.
- **A26** Tail Lash: +1 STRESS LEVEL even on a miss, and in addition to the damage stress.
- **A27** No rule exists to detect or remove a facehugger or an embryo. Removal kills the host "in most cases" [p. 302, 305]. Any detection or surgery roll is the GM's call.
- **A28** "Within one Shift" timers (facehugger falls off, mote → Stage III, Chestburster → Stage IV, adult Neomorph death "within 24 h"): the default fires at the full interval, and the GM can fire earlier.
- **A29** Queenburster gestation ("days") and Imp gestation ("substantially faster") are unquantified. Defaults: D6 days and 1 Shift total, both GM-editable.
- **A30** A Royal Facehugger producing two embryos is GM option. Default: one.
- **A31** Stage IV form for each burster: Imp → Stalker and Queenburster → Queen are stated or implied. Chestburster → Drone and Bambi → Scout are interpretation.
- **A32** Neomorph motes: what counts as "disturbed" and "nearest"; sealed suits protect; no disease effects before Stage III; the patient rolls the Sickness Roll; androids are immune.
- **A33** Bloodburster: no actions during the sluggish Round; "serious damage" = ≥1 damage; "a few Turns" = D6 Turns.
- **A34** The Neophyte uses the Neomorph Attacks table (none is printed for it).
- **A35** Workers have no attack table. Default: they never attack and they flee.
- **A36** Ready to Kill: no damage; the grabbed victim follows p. 93; the Xeno is not restricted; the forced Headbite still rolls and can be blocked.
- **A37** Deadly Grab (Neomorph and Stage IV): the creature moves along with the dragged victim.
- **A38** The Feral Hunger extra attack does not chain.
- **A39** Capture for the Hive: 0 successes = immediately unconscious; adrenaline cures at any time; the 1-damage cap applies after armor.
- **A40** One For All: 1 damage only to those who fail MOBILITY, with no armor roll; Panic for all targets.
- **A41** Crushing Blow: prone and Panic regardless of the MOBILITY result. GM option: only on a failure.
- **A42** Charge (Charger): Broken = Health 0 plus one random crit (not two).
- **A43** Call the Guard: whether guards are available, and how they draw initiative.
- **A44** Swarm:
  - attacks other than fire, explosions or electricity deal 0;
  - fire, explosions and electricity face no armor roll;
  - no cap stated on Consume Health;
  - Pheromones bonus accumulates and resets after a non-Pheromones result; results above 6 count as 6;
  - several Suffocating Swarm victims each become forced targets;
  - Desperate Hunger has no armor roll and its armor loss is permanent;
  - zone or Speed changes mid-fight.
- **A45** Harvester:
  - when the underbelly can be hit;
  - Hidden Behemoth free attack = one D6 roll;
  - the "genuine danger" threshold for juveniles;
  - no Observation stat;
  - ignore escape crits;
  - Charge path and end zone;
  - Pulverize tie = death, and witness effects apply on a death;
  - Sonar Pulse also hits NPCs.
- **A46** Lion Worm:
  - Play With Its Prey panic only on a hit;
  - Feral Rage doesn't chain;
  - the Death Roll squeeze happens once (GM may repeat while the victim is grappled);
  - break-free dice are not given (use 10 BD or Mobility 12);
  - the Intimidating Display +1 lasts until the next attack roll.
- **A47** Scorpionid: which stat profile applies is set by the environment; what counts as "bullets"; Pincer Grab's "this attack hits underwater" refers to the Pincer Grab; drowning lasts while the victim is held; the venom uses the full disease rules.
- **A48** Charger: its body has only Soldier armor (10, 5 vs fire), per the containment text. GM decides when it applies.
- **A49** XX121 "No respiration" → immune to drowning, suffocation and inhaled agents (GM).
- **A50** The −2 small-target ranged modifier (stated for the Chestburster) is extended to other tiny creatures (GM).
- **A51** Target selection: the book never says which character a creature attacks. Entries say "the victim" or "a target"; only a few name one (Harvester Charge: the biggest threat; One For All: the largest group; Last Breath: the nearest; Neophyte: one chosen prey). Default: the GM picks the target, and the app offers "nearest at ENGAGED" as a suggestion.
- **A52** There is no rule to rescue a host during ovomorphing. Default timer: 4–6 Shifts, GM-set.
- **A53** Variable gestation (immune system, hypersleep, stasis) is fully GM-controlled. The app must allow editing and pausing timers. [p. 306]
- **A54** Hive mind, the Stage IV → V molt and Stage VI transformations have no numeric timing except "weeks" for the Charger.
- **A55** ESCAPE results (two zones in one action) ignore the retreat MOBILITY roll and the free attack.
- **A56** The "+1 to next roll" bonuses: the Harvester and the Lion Worm cap at 6 explicitly; the Swarm's cap is implied by the table's size.
- **A57** Free extra attacks (Feral Hunger, Feral Rage, the Lion Worm squeeze) can be blocked, because only the Neomorph Leaping Attack extra attack is stated as unblockable.
- **A58** Area Stress effects (Assessing the Threat, Sonar Sweep): "everyone" / "all PCs" as written. Androids are unaffected.
