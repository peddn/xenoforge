# 03 – Combat, Damage & Injuries

> Official errata and project decisions in [00-errata-und-festlegungen.md](00-errata-und-festlegungen.md) take precedence over this file.

Source: ALIEN RPG Core Rulebook (Free League, 1st ed. 2019), Chapter 5 "Combat & Panic", book pp. 80–102 (PDF 84–106).
All page references are BOOK pages. Book pp. 80, 84, 102 are full-page art (no rules); p. 81 is the chapter title page.

Conventions used in this file

- "success" = the success symbol (a 6 on any die, Base or Stress). "facehugger" = the facehugger symbol (a 1 on a Stress Die). The source text drops these icons; every occurrence below was checked against the rendered page.
- "modification +N / –N" = roll N more / N fewer Base Dice; modifications are summed; if there are not enough Base Dice to remove, remove Stress Dice (cross-ref, p. 62).
- "opposed roll" = both sides roll; the acting side must roll at least one success AND more successes than the opponent; each opponent success cancels one of yours; only the acting side can push (cross-ref, p. 64).
- Skill names are written in CAPITALS as in the book (CLOSE COMBAT, RANGED COMBAT, MOBILITY, OBSERVATION, STAMINA, MEDICAL AID, COMMAND, MANIPULATION, COMTECH, PILOTING); attributes likewise (STRENGTH, AGILITY, WITS, EMPATHY).
- Rules from outside pp. 80–102 are marked "cross-ref" (e.g. "[cross-ref: p. 104]"). They are included only where the combat flow depends on them. Another spec file should own their full text.
- "AMBIGUOUS:" marks my interpretation. All of them are listed again in the section "Ambiguities / Game Mother discretion".
- "ERRATA:" marks an official errata change to the printed rule; "DECIDED:" marks a binding project decision.

---

## 1. Time & space

### 1.1 Maps & zones

- A conflict is typically played on a map of the location (ship, colony, …). [p. 82]
- The map is split into **zones**, usually one room, corridor or patch of ground each. Zone size ranges from a few steps across to about 25 meters; cramped areas have smaller zones than open terrain. [p. 82]
- Official material marks zones on location maps; in random encounters the GM sketches or describes the area. [p. 82]
- Playing without maps ("theater of the mind") is allowed, which is useful in close-quarters conflicts with few combatants. [p. 82]
- App note: positions are not tracked by the app. Every rule below that depends on zone or range needs the range category (and zone features) to be **entered by the user** for each roll. [p. 82, p. 83]

### 1.2 Borders & line of sight

- Two adjacent zones share a border that is either **open** or **blocked** (wall or bulkhead). A blocked border may contain a door or hatch (shown on the map) that lets you move between the zones. [p. 83]
- Open borders don't block vision or movement. [p. 83]
- A blocked border normally blocks line of sight, even with a door or hatch in it, unless you actively stand at the doorway and peek through. [p. 83]

### 1.3 Range categories [p. 83]

| Range | Description |
|---|---|
| Engaged | Right next to you |
| Short | A few meters away, in the same zone as you |
| Medium | Up to 25 meters away, in an adjacent zone |
| Long | Up to about 100 meters (four zones) away |
| Extreme | Up to about one kilometer |

- There are exactly five range categories. Ordered: Engaged < Short < Medium < Long < Extreme. [p. 83]

### 1.4 Zone features [p. 83]

The book gives these as examples, so the GM can invent others.

| Feature | Effect |
|---|---|
| Cluttered | Full of debris or machinery. Moving into the zone requires a MOBILITY roll. Failure: you still enter, but fall prone. |
| Dark | Dimly lit. OBSERVATION rolls in the zone get –2. Ranged attacks *into* the zone get –2 and cannot pass *through* the zone. |
| Cramped | Crawlspace or narrow tunnel. Crawl only, no running. Individuals next to you block both movement past them and shots at targets behind them. |

- DECIDED: The Dark zone is called dimly lit but gives –2, which matches Darkness –2 in the Ranged Fire Modifications table, not Dim light –1 (p. 95). The light level is chosen **once per shot**: a Dark zone counts as **Darkness –2**; **Dim light –1** is the separate, lesser option; **never both**. [p. 83, p. 95]

### 1.5 Measuring time [p. 83]

| Unit | Duration | Primary use |
|---|---|---|
| Round | 5–10 seconds | Combat |
| Turn | 5–10 minutes | Stealth |
| Shift | 5–10 hours | Recovery |

- The exact duration of a Round, Turn and Shift varies with the situation. The GM tracks time and decides when another Round, Turn or Shift has passed. [p. 83]
- A day normally has four Shifts: Morning, Day, Evening, Night. [p. 83]
- Stealth mode is played in Turns. [p. 85]
- When the enemy is revealed, stealth mode is over and combat starts. You stop counting Turns and start counting Rounds. [p. 86]
- A Round is over when all participants in the combat have acted once. Then a new Round begins. [p. 87]
- AMBIGUOUS: The book does not say how many Rounds make a Turn, or when or how the game goes back from Rounds to Turns after a fight. Suggested: the GM ends combat manually and the clock goes back to Turns. Do **not** convert elapsed Rounds into Turns automatically; let the GM advance the Turn manually (offer a hint such as "~60 Rounds ≈ 1 Turn" at most). [p. 83, p. 86]

---

## 2. Stealth mode (Turns)

### 2.1 Movement & exploration

- Stealth mode is played in Turns. [p. 85]
- In one Turn you can move **two zones** on the map. You explore them, scanning for enemies and getting a superficial description of the two zones from the GM. A team can explore individually or as a group. [p. 85]
- To examine something in a zone more closely (for example, to access a data terminal), you must stay **one entire Turn (or longer)** in a single zone. The GM has final say. [p. 85]

### 2.2 Enemy movement

- In stealth mode the GM handles enemy movement **secretly**, **each Turn, after the PCs have moved**. [p. 85]
- NPCs move under the PC rules: **a human NPC moves at most two zones per Turn**. Non-humans may be faster or have special rules. [p. 85]
- The book recommends that the GM track hidden NPCs on a hidden copy of the map, or on paper. [p. 85]

### 2.3 Active and passive enemies

- The scenario or GM makes each enemy **ACTIVE** or **PASSIVE**. [p. 85]
- ACTIVE enemies are aware of you and actively stalking you. [p. 85]
- PASSIVE enemies are unaware of you; you can ambush them. [p. 85]

### 2.4 Detection

- In stealth mode, **PASSIVE** enemies in your zone or within your line of sight are detected automatically. [p. 85]
- If you make no attempt to move quietly, PASSIVE enemies automatically detect you as well, and initiative is drawn (combat starts). [p. 85]
- To sneak past PASSIVE enemies undetected, or to make a sneak attack, roll **MOBILITY against their OBSERVATION** (opposed). You get a modification based on the distance to the enemy (Stealth table, 2.5). [p. 85, p. 86]
- **ACTIVE enemies sneaking up on you:** a nearby ACTIVE enemy may approach for a sneak attack. The GM warns you that a threat is closing in and rolls the enemy's MOBILITY vs your OBSERVATION. **If you lose, the enemy gets a free attack on you** (see sneak attacks, 3.11). [p. 85]
- An **ACTIVE enemy that stays hidden and doesn't attack you can't be spotted**, unless a motion tracker detected it first or the GM rules it obvious (e.g. you search the exact place it hides). [p. 85]
- AMBIGUOUS: The book does not say what happens when you *win* against an ACTIVE enemy's sneak attempt. Suggested, by analogy with the PC sneak-attack rule ("opponent spots you at your starting distance", p. 90): the enemy is spotted at its starting distance, initiative is drawn and there is no free attack. [p. 85, p. 90]

### 2.5 Stealth table (MOBILITY modification for sneaking) [p. 86]

| Range | MOBILITY roll |
|---|---|
| Short (same zone) | –1 |
| Medium (adjacent zone) | 0 |
| Behind open door/hatch | +2 |
| Long | +1 |
| Extreme | +3 |

- This table has no Engaged row and has a "Behind open door/hatch" row. The Sneak Attacks & Ambushes table (3.11) has an Engaged row (–2) and no door row. AMBIGUOUS: Suggested: use this Stealth table to sneak *past* enemies, and the Sneak Attacks & Ambushes table for sneak attacks and ambushes, where the range is the distance you want to reach. [p. 85, p. 86, p. 90]

### 2.6 Group stealth

- Groups roll once per side: **the character with the lowest MOBILITY skill level rolls for the sneaking group, and the target with the highest OBSERVATION skill level rolls for the targets**. The book states this as "the usual rules for stealth" in the Ambush rule. [p. 90]
- AMBIGUOUS: The book applies this group rule explicitly only to ambushes. Suggested: apply it to every group stealth roll (sneaking past, and an ACTIVE enemy sneaking up on a group of PCs). [p. 90]

### 2.7 Motion trackers [p. 86]

- You can use a motion tracker (for example, the M314) **once per Turn**. **Each use requires a Power Supply roll** (cross-ref, p. 34). [p. 86]
- It automatically registers any **large moving object** in range (**indoors: LONG, up to four zones; outdoors: EXTREME**) and tells you its zone. [p. 86]
- It detects **only moving objects**. If the GM decides the enemy stays still, it doesn't register. [p. 86]
- Detection by tracker ≠ spotting. Engaging an enemy in combat requires sharing its zone or having line of sight to it. For a small or well-hidden enemy, the GM may additionally demand an OBSERVATION roll to spot it. [p. 86]
- A tracker detection lets you later spot an ACTIVE hidden enemy (2.4). [p. 85]
- Map markers for "pings" are optional table props. [p. 86]
- AMBIGUOUS: The book does not cover using a tracker during combat (Rounds). Suggested: treat it as the fast action "Use item", allowed once per Turn of game time, with the Power Supply roll each use. [p. 86, p. 88]

---

## 3. Actions & initiative

### 3.1 Start of combat

- Once the enemy is revealed, stealth mode ends and combat begins: Turns stop, Rounds start. Initiative is drawn first, **before any action dice are rolled; sneak attacks are the exception** (3.11). [p. 86]

### 3.2 Drawing the initiative [p. 87]

- Use **ten cards numbered 1–10**: the ten special initiative cards of the ALIEN card deck, or a normal deck with the **ace counting as 1**. [p. 87]
- **Every player taking part in the conflict, voluntarily or involuntarily, draws one card. The GM draws one card per NPC.** [p. 87]
- The number on the card sets the order of action, **lowest to highest**: number 1 acts first, number 2 second, and so on until everyone has acted. Cards are placed openly. [p. 87]
- A Round ends once every participant has acted once; then the next Round starts. [p. 87]
- **The Round order stays the same for the whole conflict.** Initiative is drawn only once, at the start of the first Round. [p. 87]
- Ties cannot happen: each card is unique and only 10 exist. [p. 87]
- Related rules outside this section (cross-refs):
  - Talent Fast Reflexes: draw two initiative cards, choose one, and **shuffle the other back into the deck before others draw**. So these characters must draw first. [cross-ref: p. 78]
  - Talent Spaceship Commander: two cards in space combat only. [cross-ref: p. 79]
  - Xenomorphs with a Speed rating: **the GM draws as many initiative cards as their Speed rating**. At each of its initiatives the Xenomorph performs one fast and one slow action. [cross-ref: p. 112]
- AMBIGUOUS: The book has no rule for **newcomers joining an ongoing combat**. Suggested: the newcomer draws from the cards still undrawn, starts acting from the next Round (or the current Round if its number has not yet passed, at GM choice), and keeps that card. [p. 87]
- AMBIGUOUS: The book does not say what to do if more than 10 cards are needed (Speed creatures plus many combatants). Suggested: group more NPCs (3.4). If that is impossible, the GM assigns order manually. [p. 87]

### 3.3 Changing the initiative

- You **never draw a new initiative card during a fight**. [p. 87]
- You can **exchange your initiative card (and so your initiative for the Round) with another player character**. [p. 87]
  - This is only allowed **at the start of the fight or at the start of a Round, never during a Round**. [p. 87]
  - Both PCs must be **able to speak to each other**. [p. 87]
  - AMBIGUOUS: The book only names exchanges between two player characters. Suggested: PC↔NPC exchanges happen only through stunts (below). [p. 87]
- Close and ranged combat stunts let you **swap initiative scores with your enemy, effective from next Round; you can't return to your old initiative.** [p. 92, p. 95]

### 3.4 Non-player characters (initiative groups)

- Normally the GM draws one card per NPC. [p. 87]
- With many NPCs, **when there are more than ten combatants**, the GM groups the NPCs: **NPCs with identical stats share one group**, and the GM draws one card for each group. [p. 87]
- A group's NPCs all act at the same position in the Round order; the GM sets the order inside the group. [p. 87]

### 3.5 Slow & fast actions (action budget)

- At your slot in the Round you get **one slow action plus one fast action, OR two fast actions**. [p. 87]
- A slow action normally involves a skill roll. A fast action takes less time and may or may not need a roll. [p. 87]
- State which two actions you perform, describe them and roll. Some actions give your opponent the chance to perform a **reactive action** (block, overwatch fire). [p. 87]
- So a fast action may use the slow slot, but a slow action can never use the fast slot. Two slow actions in one Round are impossible ("you can't crawl twice in the same round"). [p. 87, p. 89]

### 3.6 Slow actions table [p. 88]

| Action | Prerequisite | Skill |
|---|---|---|
| Crawl | You are prone | – |
| Close combat attack | – | Close Combat |
| Shoot firearm | Firearm | Ranged Combat |
| Burst of full auto fire | Firearm | Ranged Combat |
| Throw weapon | Thrown weapon | Ranged Combat |
| Reload | Firearm | – |
| First aid | Broken or dying victim | Medical Aid |
| Stop panic | Panicking character | Command |
| Give orders | Character who can hear you | Command |
| Persuade | Your opponent can hear you | Manipulation |
| Use signature item | Signature item | – |
| Climb into space suit | Space suit | Mobility |
| Start engine | Vehicle | – |

- ERRATA: "Enter/exit vehicle" was printed here as a slow action; the official errata for p. 88 makes it a **fast** action, so the row moved to the fast actions table (3.7). [p. 88]
- DECIDED: "Start engine" stays a **slow** action, as printed in this table. Documented alternative (GM setting): p. 113 calls it a fast action. [p. 88, p. 113]

### 3.7 Fast actions table [p. 88]

| Fast action | Prerequisite | Skill |
|---|---|---|
| Run | No enemy at Engaged range | – |
| Move through door/hatch | – | – |
| Get up | You are prone | – |
| Draw weapon | – | – |
| Block attack | Attacked in close combat | Close Combat |
| Pick up item | – | – |
| Shove | Enemy at Engaged range | Close Combat |
| Grapple attack | You've grappled an opponent | Close Combat |
| Retreat | Enemy at Engaged range | Mobility |
| Aim | Ranged weapon | – |
| Seek cover | Cover in same zone | – |
| Assume overwatch position | Ranged weapon | – |
| Enter/exit vehicle | Vehicle | – |
| Grab the wheel | Vehicle | – |
| Drive | Vehicle | Piloting |
| Use item | Varies | Varies |

- ERRATA: "Enter/exit vehicle" is a fast action per the official errata for p. 88 (printed in the slow actions table, 3.6). [p. 88]

Other action types stated elsewhere in this section:
- Overwatch fire = a normal attack = slow action. [p. 97]
- Help another character = one action of the same kind as the helped action. [p. 89]
- Break free from a grapple = slow action. [p. 93]
- Open an unlocked door or hatch = fast action. [p. 89]
- Picking up a dropped object in combat = fast action. [p. 92]
- Drawing a melee weapon from sheath or belt = fast action. Drawing a gun from belt or holster, or a rifle from its sling = fast action. [p. 91, p. 94]
- Firing a weapon = slow action. [p. 94]
- Stop panic: COMMAND roll, slow action in combat. [cross-ref: p. 104]

### 3.8 Reactive actions & marking actions

- Reactive actions (blocking, overwatch fire) **break the initiative order**, so they can happen outside your own slot. [p. 89, p. 92]
- They **count against your two actions for the Round** (3.5). [p. 92, p. 97]
- Table tip: turn your initiative card 90° left for a fast action and 90° right for a slow action, 180° once both actions are used. [p. 89]
- App: track used actions per combatant per Round, independently of whose slot is current. [p. 89, p. 92]

### 3.9 Helping others [p. 89]

- Helping another PC or NPC perform an action **costs you one action of the same kind (slow or fast)**. [p. 89]
- You must state that you are helping **before any dice are rolled**. [p. 89]
- Helping **breaks the initiative order** in the Round. [p. 89]
- Each helper gives +1, with at most 3 helpers (maximum +3). The help must make sense in the story, and the helper must be able to support the action (GM's call). [cross-ref: p. 63]

### 3.10 Movement [p. 89]

- **Running:** a fast action. Takes you into an adjacent zone, **or** switches you between SHORT and ENGAGED range to an enemy or PC in your current zone. **No roll needed unless you run into a Cluttered zone** (MOBILITY roll, 1.4). [p. 89]
  - Since run is a fast action and two fast actions are allowed, you can run twice in a Round. [p. 87, p. 89; derived]
- **Crawling:** while prone you can't run, only crawl. Crawl = run, but as a **slow action**, so at most one crawl per Round. **Cramped** zones allow no movement except crawling. [p. 89]
- **Close combat:** if an **active enemy is at ENGAGED range**, you can't just walk away. You must **Retreat** (4.6). [p. 89]
- **Doors & hatches:** opening an unlocked door or hatch is a fast action. A locked one can be broken down: a **typical metal door or hatch breaks after 10 points of damage**. Sturdier doors take more force and also have an Armor Rating (6.2). Some electronically locked doors or hatches open with a COMTECH roll. [p. 89]
- **Vehicles:** different movement rules (cross-ref, p. 113). [p. 89]

### 3.11 Ambushes & sneak attacks [p. 90]

**Sneak attack**
1. You stalk a target that is unaware of you. Roll **MOBILITY versus the target's OBSERVATION** (opposed). The modification depends on how close you want to get (table below). To attack in close combat you usually have to get to ENGAGED range. [p. 90]
2. **Failure:** your opponent spots you **at your starting distance**, and initiative is drawn. [p. 90]
3. **Success:** you get **one free action (slow or fast, but not both) before initiative is drawn**. [p. 90]
4. **The target cannot block a sneak attack.** [p. 90]
5. Sneak attacks are always individual: one attacker against one target. [p. 90]

**Ambush** (a special kind of sneak attack)
- You lie in wait and attack when the enemy comes close. Roll MOBILITY as above, **with a +2 modification**, because the target is the one moving. [p. 90]
- **Groups can ambush, and can be ambushed.** Normal stealth rules apply: **the attacker with the lowest MOBILITY skill level rolls for the attacking side; the target with the highest OBSERVATION skill level rolls for the targets.** [p. 90]
- AMBIGUOUS: The book does not say whether the ambush +2 replaces the range modification or adds to it. Suggested: add it (total = range modification + 2). [p. 90]
- AMBIGUOUS: The book does not say how many free actions a successful group ambush gives. Suggested: each ambusher gets one free action (slow or fast, not both). Each ambusher's attack is against one target, which cannot block. [p. 90]

**Sneak Attacks & Ambushes table** [p. 90]

| Range | Modification |
|---|---|
| Engaged | –2 |
| Short | –1 |
| Medium | 0 |
| Long | +1 |
| Extreme | +3 |

- AMBIGUOUS: The book does not say whether a ranged sneak attack at ENGAGED range counts as firing at an "unwitting enemy", which gives +3 instead of –3 (p. 94). Suggested: yes, because the target is unaware. [p. 90, p. 94]

---

## 4. Close combat

### 4.1 Basics [p. 91]

- Attacking with bare fists or a melee weapon uses **CLOSE COMBAT**. [p. 91]
- Close combat usually happens at **ENGAGED** range. You can fight unarmed or with a weapon. [p. 91]
- Drawing a melee weapon from sheath or belt is a **fast action**. [p. 91]
- **Close combat attacks require standing.** A prone character must first Get up (fast action). [p. 91]
- **While you are prone, standing enemies get +2 on all close combat attacks against you.** [p. 91]
- A close combat attack is a slow action (table, p. 88). Exceptions: the grapple attack is fast (4.4), and the free attacks from retreat and from sneak attacks follow their own rules. [p. 88, p. 93]
- A skill at 0 defaults to the attribute alone. Example: CLOSE COMBAT 0 → roll STRENGTH 3 only → 3 Base Dice. [p. 93]
- For weapon stats (Bonus, Damage, Range) see the weapon tables. For example, Unarmed Attack has Damage 1 and "Armor doubled"; Knife has Damage 2. [cross-ref: pp. 118–126; p. 126]

### 4.2 Resolution & close combat stunts [p. 92]

- If the CLOSE COMBAT roll succeeds (≥1 success), the attack hits and inflicts **damage equal to the weapon's Damage rating**. Armor may reduce it (6.2). [p. 92]
- **For every extra success** (beyond the first), choose one stunt: [p. 92]
  1. **+1 damage.** Can be chosen multiple times if you roll several extra successes. [p. 92]
  2. **Out-maneuver:** swap initiative scores with your enemy, **effective from next Round**. Permanent: you can't return to your old initiative. [p. 92]
  3. **Knock or pull an object away** from your opponent (a weapon or other item of your choice). Picking up a dropped item is a fast action in combat. [p. 92]
  4. **Opponent falls prone.** Only against **humanoid** opponents. [p. 92]
  5. **Grapple:** you pin the enemy in a tight clinch (4.4). Only against **humans and synthetics**. [p. 92]
- AMBIGUOUS: Only the +1 damage stunt is explicitly repeatable. Suggested: each other stunt can be chosen at most once per attack. [p. 92]

### 4.3 Blocking [p. 92]

- If you are attacked in close combat, you can **block** to avoid being hit. [p. 92]
- Blocking is a **fast action** and uses a **CLOSE COMBAT** roll. [p. 92]
- **You must declare the block before the attacker rolls.** [p. 92]
- **For each success on the block roll, choose one effect:** [p. 92]
  - **DECREASE DAMAGE:** remove one of the enemy's successes. **If the enemy has no successes left, the attack misses.** Can be chosen multiple times. [p. 92]
  - **COUNTERATTACK:** deal damage to the attacker equal to **your weapon's Damage rating**. You **cannot** spend extra successes to raise the counterattack damage. [p. 92]
  - **DISARM:** you disarm your enemy. [p. 92]
- **Reactive action:** blocking breaks the normal initiative order but **counts against your two actions in the Round**. **Each block costs you one action later in the Round. If you have already used both actions, you can't block.** Saving your fast action is therefore wise if you expect to be attacked. [p. 92]
- **Blocking unarmed:** without a weapon you can block only **unarmed attacks by other humans**. Blocking an **armed** close combat attack or a **Xenomorph attack** needs a sturdy weapon or tool. [p. 92]
- **Cannot be blocked:** sneak attacks [p. 90], grapple attacks [p. 93], the free attack after a failed retreat [p. 93]. [p. 90, p. 93]
- **Can be blocked:** shoves. [p. 93]
- **NPCs and blocking:** NPCs typically don't block. The GM can override this when dramatically appropriate. [p. 93]
- Talent Flyweight: block with AGILITY instead of STRENGTH. [cross-ref: p. 78]
- AMBIGUOUS: The book doesn't say when the block is rolled. Suggested: the attacker rolls (and pushes, if wanted) first, then the blocker rolls and spends successes against the final attacker result. [p. 92]
- AMBIGUOUS: The book doesn't say whether a block can use the slow slot after the fast action is spent. Suggested: yes, because a fast action may always replace the slow action ("two fast actions"). So a character can block at most twice per Round, and only with unused actions. [p. 87, p. 92]
- AMBIGUOUS: The book doesn't say whether COUNTERATTACK or DISARM can be chosen more than once, or whether armor reduces counterattack damage. Suggested: each at most once per block, and the counterattack is normal damage that armor can reduce. [p. 92]
- AMBIGUOUS: The book doesn't say which object DISARM removes. Suggested: the weapon used in the attack. [p. 92]

### 4.4 Grappling [p. 93]

- When you grapple an opponent as a stunt: [p. 93]
  - **Both you and your opponent fall to the ground.** [p. 93]
  - The opponent **drops any weapon** they were holding and **cannot move**. [p. 93]
  - The opponent **can do nothing except try to break free**: a **slow action**, successful only if they **win an opposed CLOSE COMBAT roll** against you. [p. 93]
  - While grappling, **you can only release the opponent or make a grapple attack**: a **normal unarmed attack**, except it is a **fast action** and **can't be blocked**. [p. 93]
- Xenomorphs that grab or attach to you follow special rules (cross-ref, p. 112). [p. 93]
- AMBIGUOUS: The book doesn't say whether releasing costs an action. Suggested: free (no action). [p. 93]
- AMBIGUOUS: The book doesn't say whether two grapple attacks per Round are allowed. Suggested: yes, since there are two fast actions. [p. 87, p. 93]
- AMBIGUOUS: The book doesn't say what state the two characters are in after the grapple ends. Suggested: both are still prone (on the ground) and need Get up. The grappler is not "standing", so the +2 against prone targets does not apply to grapple attacks. The grappled character cannot block (break free is their only action). [p. 91, p. 93]

### 4.5 Shoving [p. 93]

- A **fast action** against an opponent at **ENGAGED** range, using a **CLOSE COMBAT** roll. The GM can modify the roll for certain opponents. [p. 93]
- Success: the opponent is shoved to **SHORT** range from you. Useful for switching to ranged combat. [p. 93]
- **Shoves can be blocked.** [p. 93]

### 4.6 Retreat [p. 93]

- If an **active enemy is at ENGAGED range**, you must make a **MOBILITY roll** (fast action, table p. 88) to move away to **SHORT** range. [p. 93]
- **On failure** you move anyway, but the enemy gets a **free close combat attack** on you. It **uses up none of the enemy's actions for the Round**, and **you can't block it**. [p. 93]
- AMBIGUOUS: The book doesn't say what happens with several engaged enemies. Suggested: one MOBILITY roll; on failure, each active enemy at Engaged range gets a free attack. [p. 93]

### 4.7 Example (close combat, block disallowed) [p. 93]

- Nguyen (NPC) attacks Captain Charlize with a knife. Gina (Charlize's player) draws initiative **6**; the GM draws **1**, so Nguyen acts first. [p. 93]
- Nguyen's CLOSE COMBAT is **0**, so the roll defaults to his **STRENGTH 3** = **3 Base Dice**. [p. 93]
- Before the roll, Gina declares a block, but Charlize is **unarmed**, so the GM rules it out (an unarmed character can't block an armed attack). [p. 93]
- The GM rolls 3 Base Dice and gets **one success** → **2 points of damage** to Charlize's Health (the knife's Damage rating is 2, cross-ref p. 126). [p. 93]

---

## 5. Ranged combat

### 5.1 Basics [p. 94]

- Shooting or throwing at a distance uses **RANGED COMBAT**. [p. 94]
- You **must be able to see the target** and need a **ranged weapon**, even if it is just something to throw. [p. 94]
- Drawing a gun (belt/holster) or a rifle (sling) is a **fast action**. **Shooting is a slow action.** [p. 94]
- Range: the weapon tables give each weapon's **maximum range category**. You cannot attack beyond it. [p. 94]
- Zone restrictions: Dark zones (–2 into the zone, and you cannot shoot through them) and Cramped zones (you cannot shoot past individuals next to you) (1.4). [p. 83]

### 5.2 Aiming [p. 94]

- Aiming is a **fast action** and gives **+2** to the attack roll. [p. 94]
- You **lose the aim** if you do **anything else except shoot your weapon** after aiming, **or if you are hurt**. Aiming again costs another fast action. [p. 94]
- AMBIGUOUS: The book doesn't say whether aim lasts into later Rounds. Suggested: aim persists across Rounds until you take any action other than shooting (including a block or a reactive action), or until you take damage. [p. 94]
- AMBIGUOUS: "Hurt" is not defined. Suggested: ≥1 point of damage after the armor roll. [p. 94]
- AMBIGUOUS: The book doesn't say whether the aim bonus covers more than one shot. Suggested: it applies to the next shot only, since that shot is "something else" for any later shot. [p. 94]

### 5.3 Range & target size [p. 94]

- **MEDIUM –1**, **LONG –2**. [p. 94]
- **ENGAGED –3** (it's hard to draw a bead on someone that close), **except** against a **defenseless or unwitting** enemy, where you get **+3** instead. [p. 94]
- A Broken character is defenseless (p. 99), so firing at a Broken target at Engaged range gets +3 (derived). [p. 94, p. 99]
- **Large target** (e.g. a vehicle) **+2**. **Small target** (small hatch, hand-held item, Chestburster) **–2**. [p. 94]

### 5.4 Ranged fire modifications table [p. 95]

| Factor | Modification |
|---|---|
| Aimed shot | +2 |
| Engaged range | –3/+3 |
| Short range | – (none) |
| Medium range | –1 |
| Long range | –2 |
| Extreme range | –3 |
| Large target | +2 |
| Small target | –2 |
| Dim light | –1 |
| Darkness | –2 |

- Extreme range –3 appears only in the table, not in the body text. [p. 95]
- DECIDED: pick **one** light level per shot: Dim light –1 **or** Darkness –2. A Dark zone (1.4) counts as Darkness –2; Dim light is the separate, lesser option. Never apply both. [p. 83, p. 95]
- Full auto adds +2 (5.6). Gear bonuses (weapon Bonus) and talent modifiers come from other chapters. [cross-ref: pp. 75–79, 118–126]

### 5.5 Resolution & ranged stunts [p. 95]

- On a hit, you inflict **damage equal to the weapon's Damage rating**. [p. 95]
- **For every extra success, choose one stunt:** [p. 95]
  1. **+1 damage.** Can be chosen multiple times. [p. 95]
  2. **Pin down:** **PCs must make an immediate Panic Roll. NPCs instead miss their next slow action.** [p. 95]
  3. **Reposition:** swap initiative scores with your enemy, **effective from next Round**. Permanent: you can't return to your old initiative. [p. 95]
  4. **Target drops a weapon or other hand-held item** of your choice. [p. 95]
  5. **Opponent falls prone or is pushed back** (e.g. through an airlock). [p. 95]
- Being pinned down by a ranged attack is a Panic Roll trigger. Androids never make Panic Rolls. [cross-ref: p. 104; p. 111]
- AMBIGUOUS: The book doesn't say whether non-damage stunts repeat. Suggested: at most once each per attack. [p. 95]
- AMBIGUOUS: The book doesn't say when the pinned NPC's lost slow action happens. Suggested: the NPC's next unused slow action, which is this Round's if the NPC hasn't acted yet, otherwise next Round's. [p. 95]
- AMBIGUOUS: The book doesn't cover swapping initiative with an NPC that shares a group card. Suggested: GM decides; the NPC can be split out of the group with its own card. [p. 87, p. 95]

### 5.6 Full auto fire [p. 96]

Only weapons listed as fully automatic can do this. It is a normal ranged attack (slow action "Burst of full auto fire", p. 88) with these differences:
- **+2** to the roll. [p. 96]
- **Your STRESS LEVEL immediately increases by one**, and **you add that Stress Die to the roll** for the burst. [p. 96]
- **Each additional success beyond the first can go to secondary targets within SHORT range of the primary target.** The **first success assigned to a given secondary target deals damage equal to the weapon's Damage rating**. **Further successes assigned to that secondary target can be used for stunts.** [p. 96]
- Talent Machine Gunner: your STRESS LEVEL does not increase when firing full auto. [cross-ref: p. 78]
- NPCs don't have a STRESS LEVEL (cross-ref, p. 62). AMBIGUOUS: For NPCs, the full-auto stress step does not apply; they get only the +2 and the secondary targets. [p. 96]
- AMBIGUOUS: The book doesn't say whether you must use successes on secondary targets. Suggested: assigning them is optional; any successes not assigned stay with the primary target as normal stunts. The book sets no limit on the number of secondary targets, and each one rolls its own armor/cover against the damage it takes. [p. 96]

### 5.7 Cover [p. 96]

- You must find cover **before** someone shoots at you; unlike blocking, it is not a reaction. **Taking cover behind something in the same zone is a fast action** ("Seek cover", p. 88). [p. 96]
- Cover has an **Armor Rating** and works just like armor. **When you are hit by a ranged attack, roll Base Dice equal to the cover's Armor Rating. Each success reduces the damage by one.** [p. 96]
- **Cover and armor combine: add the Armor Ratings** (one combined roll). [p. 96]
- **NPCs and cover:** NPCs typically don't take cover. The GM can override this when dramatically appropriate. [p. 96]

**Common types of cover** [p. 96]

| Cover | Armor Rating |
|---|---|
| Shrubbery | 2 |
| Furniture | 3 |
| Door | 4 |
| Inner bulkhead | 5 |
| Outer bulkhead | 6 |
| Armored bulkhead | 7+ |

- Cover protects only against **ranged** attacks ("when you're hit by a ranged attack"). [p. 96]
- AMBIGUOUS: The book doesn't say how long cover lasts. Suggested: until the character moves (run, crawl, retreat, get shoved or pushed back) or the GM says the cover is gone. [p. 96]
- AMBIGUOUS: The book doesn't say how Armor Piercing (cross-ref, p. 118: "any Armor Rating counts as half … rounding fractions up") applies to combined cover and armor. Suggested: halve the combined total. [p. 96]

### 5.8 Ammo [p. 96]

- You don't count individual bullets. [p. 96]
- **Whenever you fire a weapon and roll one or more facehuggers on your Stress Dice, your magazine is empty after the attack and you must reload. You also make a Panic Roll.** [p. 96]
- **Reloading is a slow action.** [p. 96]
- **Track how many full reloads of ammunition you carry for the weapon.** [p. 96]
- **Androids and ammo:** androids have no STRESS LEVEL, so they never run out of ammo (intentional). **NPCs run out of ammo when the GM finds it dramatically appropriate.** [p. 96]
- Order of events: the attack is resolved and the magazine is empty afterwards ("after the attack"). But the Panic Roll comes from the same skill roll: a Panic result of 10+ cancels the action and replaces it with the panic action, even if successes were rolled. Facehuggers also forbid pushing that roll. [cross-ref: p. 104; p. 61]
- The ammo rule also applies to pushed rolls: Stress Dice are re-rolled when pushing. [cross-ref: p. 60–61]
- AMBIGUOUS: The book doesn't say whether thrown weapons trigger the empty-magazine rule. Suggested: no; the rule only applies to "a weapon with a limited magazine" (cross-ref, p. 61). Thrown attacks still trigger the Panic Roll on a facehugger. [p. 96]
- AMBIGUOUS: The book doesn't say what happens when you have no reloads left. Suggested: the weapon can't fire until a reload is found. [p. 96]

### 5.9 Overwatch [p. 97]

- **Assume overwatch position:** a **fast action**, aimed in a **specified direction**. You need a **ranged weapon** and **no enemies within ENGAGED range**. [p. 97]
- **From assuming overwatch until your time to act in the next Round**, you may shoot at a target in the chosen direction. [p. 97]
- You may shoot **at any point in the turn order**, and **your shot resolves ahead of all other actions, including already declared ones**. Example: an enemy in your direction declares a shot → you fire first. **The enemy may not change its attack after your overwatch shot.** [p. 97]
- **Overwatch fire is a normal attack (a slow action)**, so you must keep your slow action in the Round free for it. [p. 97]
- **Both sides on overwatch:** if you and an enemy are on overwatch against each other and both fire, an **opposed RANGED COMBAT roll** decides which attack goes first. **This roll is not an action for either of you.** [p. 97]
- **Losing overwatch:** it holds only while you do nothing except shoot in the chosen direction. **Any other action ends it.** It also ends **immediately** if: [p. 97]
  - you are **attacked in close combat** (whether or not you are hit), or [p. 97]
  - you **suffer damage**. [p. 97]
- AMBIGUOUS: The book doesn't say which Round's slow action an overwatch shot uses. Suggested: firing in the same Round you assumed overwatch uses that Round's slow action (if unused). Firing in the next Round, before your slot, uses the next Round's slow action, which leaves you only a fast action at your slot. [p. 97]
- AMBIGUOUS: The book doesn't say whether overwatch carries past your next slot. Suggested: the window closes at your next time to act. At that point you may assume overwatch again (a fast action) to stay on overwatch. Alternative reading (the book also says overwatch is kept while you do nothing other than shooting): it continues while you only shoot in that direction. Make this a GM setting. [p. 97]
- AMBIGUOUS: The book doesn't say whether overwatch gives the +2 aim bonus. Suggested: no, unless you also took the Aim action. [p. 97]

### 5.10 Example (thrown improvised weapon) [p. 97]

- Charlize pushes Nguyen back and climbs onto a gantry. Drawing her sidearm would escalate, so she looks for something to throw. [p. 97]
- Next Round: she picks up a power tool (**fast action**) and hurls it (**RANGED COMBAT**): **3 Base Dice for AGILITY + 1 Base Die for RANGED COMBAT** = 4 dice → **two successes**. [p. 97]
- The tool deals **1 point of damage** (the first success, so the improvised weapon has Damage 1). The **extra success** is spent on the stunt **"pushed back"**: Nguyen staggers toward the airlock and trips over the entrance. [p. 97]

---

## 6. Damage

### 6.1 Health [p. 98]

- Damage covers everything from exhaustion to bleeding cuts and broken bones. Your **Health** decides how much damage you can take. [p. 98]
- **Starting / maximum Health = STRENGTH.** Talents can modify maximum Health. [p. 98]
- **Health can't go below zero.** [p. 98]

### 6.2 Armor [p. 98]

- Armor has an **Armor Rating**. You can wear **only one suit of armor at a time**. [p. 98]
- **When you take damage from an attack, roll Base Dice equal to your Armor Rating. Each success reduces the damage by one point.** [p. 98]
- **The armor roll is not an action.** [p. 98]
- It works the same way for cover (combine with armor by adding the ratings, ranged attacks only) [p. 96], sturdy doors [p. 89] and vehicles. [cross-ref: p. 114]
- Weapon specials: "Armor Piercing" means any Armor Rating counts as half, rounding up. "Armor Doubled" doubles the Armor Rating. Unarmed Attack is "Armor doubled". [cross-ref: p. 118; p. 126]
- **Armor degradation: pp. 80–102 have no rule for it.** Armor Rating is not reduced by taking hits. The only armor degradation is in specific creature rules, e.g. Xenomorph Acid Splash: each armor success reduces that armor's Armor Rating by one. [cross-ref: p. 298]
- AMBIGUOUS: The book doesn't say whether the armor roll can be pushed. Suggested: no, since it is not a skill roll and not an action. It has no Stress Dice. [p. 98]

### 6.3 Recovery [p. 98]

- **Once no longer Broken, you regain 1 lost Health point per Turn (5–10 minutes) of rest.** [p. 98]
- Critical injuries can still affect you after all Health is restored. [p. 98, p. 101]
- AMBIGUOUS: "Rest" is not defined. Suggested: no actions or skill rolls for that Turn, like resting to relieve stress (cross-ref, p. 104). [p. 98]

### 6.4 Broken [p. 98]

- **At zero Health you are Broken**, which in effect takes you out of the action. [p. 98]
- **Immediately roll for a critical injury.** [p. 98]
- Unless dead, you **can only crawl and mumble in pain**: **no other actions** and **no skill rolls**. [p. 98]
- **Each further attack that causes damage gives you another critical injury.** Health stays at 0. [p. 98]
- **A Broken character is defenseless** (relevant for coup de grâce, and for +3 at Engaged range when shooting). [p. 99]
- Being Broken ends a panic effect. [cross-ref: p. 104]
- Synthetics and Xenomorphs: different Broken and critical rules. [cross-ref: pp. 111–112; Xenomorph crit table p. 291]
- AMBIGUOUS: The book doesn't say whether a Broken character can still make a Death Roll, which is a STAMINA roll. Suggested: yes. Death Rolls are explicitly required for Broken characters with fatal injuries, so they are an exception to "can't roll for any skills". [p. 98, p. 99]
- AMBIGUOUS: A hit that armor reduces to 0 damage does not count as "an attack that causes damage", so it gives no crit (derived). [p. 98]

### 6.5 Getting back up [p. 98]

Being Broken is not fatal in itself; only critical injuries can kill. There are two ways back up, assuming you are not dead: [p. 98]
- **First aid:** another character uses **MEDICAL AID** on you, a **slow action**. Medical equipment can give bonuses. The book says "see Chapter 5", but gear is Chapter 6. **On a success, you get back up immediately and regain Health equal to the number of successes rolled.** First aid on someone who isn't Broken has no effect. [p. 98]
- **On your own:** Broken with **nobody around to help** → after **one Turn (5–10 minutes) you automatically regain 1 Health** and can then get up unaided. [p. 98]
- AMBIGUOUS: The book doesn't say whether "no one around to help" still applies if helpers are present but fail. Suggested: the one-Turn self-recovery happens whenever a full Turn passes without a successful first aid. [p. 98]
- AMBIGUOUS: The book doesn't say whether getting back up also removes prone. Suggested: "get back up" means the character is standing (not prone) and no longer Broken. [p. 98]

### 6.6 Coup de grâce [p. 99]

- A Broken character is **defenseless**. [p. 99]
- If the target is a **human being** and you want to **kill it outright**, you must **fail an EMPATHY roll** (attribute only, no skill, **no Stress Dice**). [p. 99]
- **Whether or not you succeed in killing the target, your STRESS LEVEL increases by one** (killing in cold blood is not easy). [p. 99]
- The book names a **"Cold-Blooded" talent** that removes this drawback when killing defenseless enemies. [p. 99]
- DECIDED: No talent called "Cold-Blooded" exists in the talent list (pp. 75–79); the reference means **Merciless** (cross-ref, p. 78), whose own text only waives the EMPATHY roll. **Merciless skips both the EMPATHY roll and the +1 stress.** [p. 99]
- AMBIGUOUS: The book doesn't give the coup de grâce's action type or effect. Suggested: a slow action (an attack) at a range the weapon allows. If the EMPATHY roll fails, the target dies outright with no damage or crit roll. If it succeeds, you can't bring yourself to do it. [p. 99]
- AMBIGUOUS: Non-human targets (androids, creatures) need no EMPATHY roll according to the text ("If it's a human being"). The book doesn't say whether the stress increase still applies to them. Suggested: no. [p. 99]

### 6.7 Broken NPCs [p. 99]

- NPCs can be Broken in the same ways as PCs. [p. 99]
- NPCs can give first aid to PCs and the other way round. [p. 99]
- Dice are usually **not** rolled when an NPC aids another NPC; the GM decides. [p. 99]
- The GM can decide that a **minor NPC who is Broken simply dies**. [p. 99]

### 6.8 Critical injuries: when and how [p. 99]

- While you still have Health, damage represents fatigue, bruises and small cuts. Critical injuries are much more dangerous and can maim or kill. [p. 99]
- **When knocked down to zero Health, roll D66 on the critical injury table.** [p. 99]
- Also **each further damaging attack while Broken** gives another crit. [p. 98]
- D66: roll two D6. The first die is the tens digit and the second the ones digit, giving 11–66. [cross-ref: p. 22]
- Talent Killer: when your enemy suffers a critical injury, you may swap the two D66 digits. Humans only. [cross-ref: p. 78]
- Suffering a critical injury triggers a **Panic Roll**. Androids never panic. [cross-ref: p. 104; p. 111]

### 6.9 Critical injuries table [p. 100]

"Fatal: Yes, –N" means the injury is fatal and the **MEDICAL AID roll to save your life gets –N** [p. 99]. "Time limit" is the interval for Death Rolls [p. 99]. Healing time is in days [p. 101].

| D66 | Injury | Fatal | Time limit | Effects | Healing time |
|---|---|---|---|---|---|
| 11 | Winded | No | – | None. | – |
| 12 | Stunned | No | – | None. | – |
| 13 | Crippling pain | No | – | STRESS LEVEL increases one step. | – |
| 14 | Sprained ankle | No | – | Until a MEDICAL AID roll is made: MOBILITY –2, can't run. | – |
| 15 | Blood in eyes | No | – | Until a MEDICAL AID roll is made: OBSERVATION –2, RANGED COMBAT –2. | – |
| 16 | Concussion | No | – | MOBILITY –2. | D6 days |
| 21 | Severed ear | No | – | OBSERVATION –2. | D6 days |
| 22 | Broken toes | No | – | Run is a slow action. | D6 days |
| 23 | Broken hand | No | – | Can't use hand. | D6 days |
| 24 | Knocked out teeth | No | – | MANIPULATION –2. | D6 days |
| 25 | Impaled thigh | No | – | Run is a slow action. | 2D6 days |
| 26 | Slashed shoulder | No | – | Can't use arm. | D6 days |
| 31 | Broken nose | No | – | MANIPULATION and OBSERVATION –1. | D6 days |
| 32 | Crotch hit | No | – | 1 damage on each MOBILITY or CLOSE COMBAT roll. | D6 days |
| 33 | Broken ribs | No | – | MOBILITY and CLOSE COMBAT –2. | 2D6 days |
| 34 | Gouged eye | No | – | RANGED COMBAT and OBSERVATION –2. | 2D6 days |
| 35 | Busted kneecap | No | – | Can't run, only crawl. | 2D6 days |
| 36 | Broken arm | No | – | Can't use arm. | 2D6 days |
| 41 | Broken leg | No | – | Can't run, only crawl. | 2D6 days |
| 42 | Crushed foot | No | – | Can't run, only crawl. | 3D6 days |
| 43 | Crushed elbow | No | – | Can't use arm. | 3D6 days |
| 44 | Punctured lung | Yes | One Day | STAMINA –2, MOBILITY –2. | D6 days |
| 45 | Bleeding gut | Yes | One Shift | 1 damage on each MOBILITY or CLOSE COMBAT roll. | D6 days |
| 46 | Ruptured intestines | Yes | One Shift | Disease (Virulence 6). | 2D6 days |
| 51 | Busted kidney | Yes | One Day | MOBILITY –2; can't run, only crawl. | 2D6 days |
| 52 | Arm artery cut | Yes, –1 | One Turn | Arm unusable. | D6 days |
| 53 | Leg artery cut | Yes, –1 | One Turn | Run is a slow action. | D6 days |
| 54 | Severed arm | Yes, –1 | One Shift | Can't use arm. | Permanent |
| 55 | Severed leg | Yes, –1 | One Shift | Can't run; crawl only. | Permanent |
| 56 | Cracked spine | No | – | Paralyzed below the neck. Becomes permanent without MEDICAL AID in time. | 3D6 days |
| 61 | Ruptured jugular | Yes, –1 | One Round | STAMINA –1. | 2D6 days |
| 62 | Ruptured aorta | Yes, –2 | One Round | STAMINA –2. | 3D6 days |
| 63 | Disemboweled | Yes | – | Instant death. | – |
| 64 | Crushed skull | Yes | – | Instant death. | – |
| 65 | Pierced head | Yes | – | Instant death. | – |
| 66 | Impaled heart | Yes | – | Instant death. | – |

Parsing notes for implementation (checked against the rendered table):
- Fatal values are exactly: `No`, `Yes`, `Yes, –1`, `Yes, –2`. Fatal with a time limit (Death Roll applies): 44, 45, 46, 51, 52, 53, 54, 55, 61, 62. Instant death, no Death Roll: 63–66 [p. 99]. Medical Aid modifier to save life: –1 for 52, 53, 54, 55, 61; –2 for 62; 0 otherwise. [p. 99, p. 100]
- Time limits: One Round (61, 62), One Turn (52, 53), One Shift (45, 46, 54, 55), One Day (44, 51). [p. 100]
- Crits 14 and 15 have no healing time: they last **until a MEDICAL AID roll is made**. [p. 100]
- Crits 11, 12, 13 have no lasting effect beyond the immediate one (13: +1 STRESS LEVEL). [p. 100]
- "Permanent" (54, 55) never heals. [p. 100]
- 46 "Disease with Virulence 6": apply the disease rules (cross-ref, p. 109: sickness rolls against Virulence; first point of damage one Shift after infection; no Health recovery while sick; new Sickness Roll each Shift). [p. 100]
- 32 and 45: **1 damage every time** the character rolls MOBILITY or CLOSE COMBAT. [p. 100]
- "Can't use arm/hand": one-handed only; no two-handed weapons. AMBIGUOUS: GM rules on exact gear restrictions. [p. 100]
- "To run becomes a slow action" (22, 25, 53): Run uses the slow slot, so you can run at most once per Round. [p. 100]
- "Can't run, only crawl" (35, 41, 42, 51, 55): movement only by Crawl (slow action). [p. 100]
- Attribute/skill modifiers from crits (e.g. MOBILITY –2) are modifications to rolls of that skill or attribute (STAMINA is a skill). AMBIGUOUS: The book doesn't say whether modifiers from several crits stack. Suggested: yes, they stack. [p. 100]
- AMBIGUOUS: Crit 56 (Cracked spine) is **not** fatal and has **no time limit**, yet "if not given MEDICAL AID in time, the effect is permanent". "In time" is undefined, so the GM decides. Suggested default: before the injury's healing time starts. [p. 100]
- AMBIGUOUS: The book doesn't say what happens if the same crit is rolled twice. Suggested: record both, GM's call. (Only synthetics have a shift-down rule, cross-ref p. 111.) [p. 100]
- AMBIGUOUS: The book doesn't say when healing dice (D6/2D6/3D6) are rolled. Suggested: roll and record them when the injury is suffered. [p. 100, p. 101]

### 6.10 Death [p. 99]

- A crit listed as fatal **forces a Death Roll when its time limit runs out**. [p. 99]
- **Death Roll = a roll for STAMINA. You cannot push it, and you roll no Stress Dice.** [p. 99]
- **If the Death Roll fails, you die.** If it succeeds, you linger on, but **must make another Death Roll each time the listed amount of time passes again**. [p. 99]
- **Saving your life:** you need **first aid from someone before any Death Roll fails**. First aid = **slow action**, **MEDICAL AID** roll. The worst crits impose a negative modification on that MEDICAL AID roll (the "Yes, –N" column). [p. 99]
- **Self-treatment:** if you recover a Health point by yourself (after one Turn, 6.5) before failing a Death Roll, you can try to give yourself first aid, at **–2**. [p. 99]
- **One attempt per healer:** each character who tries to treat you gets **only one try**. A second chance needs **better medical equipment**. [p. 99]
- **Broken and fatally injured:** Broken plus one or more fatal crits → **two separate MEDICAL AID rolls**: one to get you up, one to save your life, **in any order**. [p. 99]
- **Instant kill:** results **63–66 kill outright. No Death Roll is allowed.** [p. 99]
- Handling death: the book suggests treating PC death as a dramatic peak. In Cinematic play, the GM usually has another character or NPC ready. This has no mechanical effect. [p. 101]
- AMBIGUOUS: The book doesn't define "when the listed time runs out" precisely. Suggested: start a timer when the injury is inflicted. The first Death Roll comes when one full unit of the time limit (Round/Turn/Shift/Day) has passed since then, and later rolls every unit after that. For "One Round", that is when the Round order next reaches the injured character's initiative slot, a full Round later. Let the GM resolve it manually if needed. [p. 99]
- AMBIGUOUS: The book doesn't say whether several fatal crits need several life-saving rolls. The wording ("a fatal critical injury (or several) … another to save your life") suggests **one** successful life-saving roll covers all fatal injuries. The book doesn't say which modifier applies then. Suggested: the worst (most negative) modifier among the untreated fatal crits. Make this a GM setting. [p. 99]
- AMBIGUOUS: "Each character … can try only once" is not scoped. Suggested: one attempt per healer per patient per dying episode (until saved or dead). The healer can retry only with better medical equipment than before. [p. 99]
- AMBIGUOUS: A fatal crit suffered later starts its own timer. A successful life-saving roll stops all Death Rolls, but the crit effects continue for their healing time. [p. 99, p. 101]
- AMBIGUOUS: The book doesn't say whether the MEDICAL AID roll to save a life can be pushed. Suggested: yes, since only the Death Roll is explicitly unpushable. Pushing raises the healer's stress as usual (cross-ref, p. 60–61). [p. 99]

### 6.11 Healing critical injuries [p. 101]

- Each critical injury has its effect for the healing time shown, **measured in days**. [p. 101]
- **Care:** if someone **tends to you daily** during healing (of one or several crits) **and makes a MEDICAL AID roll**, the **remaining healing time is halved**. An **earlier roll to save your life doesn't count**; a new roll is required. [p. 101]
- **Health points:** you can recover all lost Health and still suffer the effects of a critical injury. [p. 101]
- AMBIGUOUS: The book doesn't say how many care halvings are allowed or how to round. Suggested: one successful care roll halves the remaining time of every crit being treated, once per healing process, rounding up. The book's "daily" tending means the carer must keep tending each day. [p. 101]
- AMBIGUOUS: The book doesn't say what a "MEDICAL AID roll is made" means for crits 14 and 15. Suggested: any successful MEDICAL AID roll on the character, which in combat is a slow action. It needs no specific healing time. [p. 100]

### 6.12 Permanent mental trauma table [p. 101]

Trigger (cross-ref, p. 104): if you rolled **13 or higher on a Panic Roll** during a session, make an **EMPATHY roll** (attribute only, no skill) **after the session**. DECIDED: **if the roll fails**, you develop a permanent mental trauma: roll D6 on this table (publisher FAQ / later printings; p. 104 as printed in this edition says on success; the 13+ threshold is unchanged). Androids that are revived after System Shutdown also suffer one (cross-ref, p. 111).

| D6 | Effect |
|---|---|
| 1 | **Phobia:** fear of something linked to what made you panic (GM picks). STRESS LEVEL +1 when within SHORT range of it; staying near it longer than one Round → Panic Roll. |
| 2 | **Alcoholism:** each Shift without alcohol → STRESS LEVEL +1. Relieving stress (p. 104) requires drinking alcohol. |
| 3 | **Nightmares:** roll EMPATHY whenever you sleep. Success → nightmare: STRESS LEVEL +1 and no stress relief for one full day afterwards. |
| 4 | **Depression:** roll EMPATHY once per day. Failure → bad day: STRESS LEVEL +1 and no stress relief until the next day. |
| 5 | **Drug use:** each Shift without a recreational drug (p. 137) → STRESS LEVEL +1. Relieving stress (p. 104) requires taking your drug of choice. |
| 6 | **Amnesia:** you no longer remember who you or the other characters are. Roleplay it. |

---

## 7. Cross-references needed by the combat engine (outside pp. 80–102; pointers only)

- Dice basics, D6/D3/2D6/D66 (p. 22). Pushing (p. 60). Stress Dice and the facehugger (p. 61). Modifications (p. 62). Help from others: +1 per helper, max +3 (p. 63). Opposed rolls (p. 64). NPCs never push and have no STRESS LEVEL (p. 62). [cross-ref: pp. 22, 60–64]
- Panic Roll triggers include: facehugger on a Stress Die, being **pinned down**, **suffering a critical injury**, being attacked by an unknown alien creature, and witnessing certain panic effects. Resolution is D6 + STRESS LEVEL. A result of 10+ cancels the current action. Panic stops through COMMAND (slow action), when you are **Broken**, or after one Turn. Relieving stress happens per Turn of rest. Permanent mental trauma trigger (p. 103–104). [cross-ref: pp. 103–104]
- Talents that change combat: Fast Reflexes, Flyweight, Killer, Machine Gunner, Merciless and others (pp. 75–79). [cross-ref: pp. 75–79]
- Falling, explosions (Blast Power; Engaged +1 damage; Blast Power ≥7 reaches MEDIUM at Blast Power –6), fire (Intensity) (p. 108). Disease (p. 109). Other hazards (pp. 106–110). [cross-ref: pp. 106–110]
- Synthetics: own D6 crit table, no Death Rolls, no stress or panic, no pushing (p. 111). Xenomorphs: Speed (several initiative cards, one slow and one fast action per card), signature attacks, D6 crit table on p. 291, natural Armor Rating (p. 112). [cross-ref: pp. 111–112, 291]
- Vehicles (p. 113–114). Weapon stats and specials (Armor Piercing, Armor Doubled, Blast, Full auto) (p. 118 ff.). Close combat weapons (p. 126). Armor items (p. 128). Motion tracker M314 (p. 134). Power Supply roll (p. 34). [cross-ref: pp. 34, 113–134]

---

## 8. Combat procedure (state machine)

States: `STEALTH` (clock = Turns) → `COMBAT_SETUP` → `ROUND_START` → `ACTING(slot)` ↔ `REACTION` → `ROUND_END` → … → `COMBAT_END` → `STEALTH`/`DOWNTIME`. Asynchronous sub-machines: `ATTACK`, `DAMAGE`, `CRIT`, `DYING`.

### 8.1 Stealth mode (one iteration = one Turn) [pp. 85–86]
1. PCs declare movement (≤2 zones each, individually or as a group), or that they stay to examine something (≥1 full Turn in one zone). [p. 85]
2. Optional: each motion tracker is used (once per tracker per Turn) → Power Supply roll → GM reports the zones of large *moving* objects within LONG (indoors) or EXTREME (outdoors) range. [p. 86]
3. GM moves NPCs secretly (human NPCs ≤2 zones per Turn; non-humans per their rules). [p. 85]
4. Detection checks: [p. 85]
   - a. PASSIVE enemy in the same zone or in line of sight → PCs detect it. [p. 85]
     - PCs not moving quietly → enemy detects them → go to 8.2 (no free actions). [p. 85]
     - PCs try to sneak past → opposed MOBILITY (group: lowest skill level) vs OBSERVATION (group: highest skill level) + Stealth table modification. Win → stay undetected, remain in STEALTH. Lose → enemy spots them at their starting distance → 8.2. [pp. 85–86, 90]
     - PCs try a sneak attack or ambush → opposed MOBILITY vs OBSERVATION + Sneak Attacks & Ambushes table (+2 extra for an ambush). Win → mark as "free action pending" → 8.2. Lose → spotted at starting distance → 8.2 (no free actions). [p. 90]
   - b. ACTIVE enemy attempts a sneak attack → GM announces "a threat is moving in" → opposed enemy MOBILITY vs PC OBSERVATION. If the PC loses → the enemy gets a free attack (which can't be blocked) → 8.2. If the PC wins → AMBIGUOUS: spotted, go to 8.2 without a free attack. [p. 85]
   - c. ACTIVE enemy hiding and not attacking → cannot be spotted unless a tracker detected it first or the GM deems it obvious. [p. 85]
5. If nothing triggers, advance the Turn clock by 1 and repeat. [p. 85]

### 8.2 Combat setup [pp. 86–87, 90]
1. Switch clock to Rounds (Round = 1). Stop counting Turns. [p. 86]
2. Resolve **free actions** from successful sneak attacks or ambushes, **before initiative is drawn**. Each is one slow OR one fast action, by one attacker against one target, and the target cannot block. Free attacks go through 8.5 (ATTACK) with "unblockable" set. [p. 90]
3. **Draw initiative** from a 10-card deck (1–10): [p. 87]
   - Fast Reflexes holders draw 2, keep 1, return the other and reshuffle **before others draw**. [cross-ref: p. 78]
   - Each PC draws 1. The GM draws 1 per NPC. If combatants > 10, NPCs with identical stats form groups and draw 1 card per group. Speed-N creatures draw N cards (cross-ref, p. 112). [p. 87]
4. PCs who can speak to each other may exchange cards ("at the start of the fight"). [p. 87]
5. Go to 8.3. [p. 87]

### 8.3 Round start [pp. 87, 92, 95]
1. Apply initiative swaps from stunts made in the previous Round ("taking effect next Round"). They are permanent. [p. 92, p. 95]
2. PCs who can speak to each other may exchange initiative cards (start of Round only). [p. 87]
3. Reset every combatant's (and every Speed card's) action budget: `slowSlot = free, fastSlot = free`. A fast action may use the slow slot. [p. 87]
4. Apply carried-over penalties, e.g. NPC pinned down → its slow slot is consumed (AMBIGUOUS, 5.5). [p. 95]
5. Check timers that fall due (Death Rolls for "One Round" crits, etc.; see 6.10 AMBIGUOUS timing) and the overwatch windows that close at each holder's slot. [p. 97, p. 99]
6. Set the slot pointer to the lowest card value. → 8.4. [p. 87]

### 8.4 A combatant's slot ("time to act") [pp. 87–93]
For each card value in ascending order (NPC group: GM orders its members):
1. Close the overwatch window of this combatant if it was assumed in an earlier Round (AMBIGUOUS, 5.9). [p. 97]
2. Determine the allowed actions:
   - Dead → skip. [p. 99]
   - Broken → only **Crawl** (slow); no skill rolls. [p. 98]
   - Grappled → only **Break free** (slow, opposed CLOSE COMBAT; must win). [p. 93]
   - Grappling → only **Grapple attack** (fast, unblockable unarmed attack) or **release**. [p. 93]
   - Prone → no Run, no close combat attack. Options: **Get up** (fast) or **Crawl** (slow). [p. 89, p. 91]
   - Enemy at Engaged → no Run; use Retreat. Overwatch can't be assumed. [p. 88, p. 89, p. 97]
   - Crit effects (can't run, run becomes slow, can't use arm, etc.). [p. 100]
   - Panic effects. [cross-ref: p. 104]
3. With the remaining budget (reactions earlier this Round may already have used some), the player declares one slow + one fast action, or two fast actions. Validate each against the prerequisites in the tables (3.6 and 3.7). [p. 87, p. 88]
4. Resolve each action. Attacks → 8.5. Movement → Cluttered zone MOBILITY roll (failure: in the zone but prone). Help → declare before the roll and deduct the helper's action. [pp. 83, 89]
5. Any action other than shooting ends Aim and Overwatch. [p. 94, p. 97]
6. Advance to the next card value. When every card has acted → 8.7. [p. 87]

### 8.5 Attack (from declaration to damage) [pp. 90–97]
1. **Declare**: attacker, target, attack type (close / grapple attack / shove / shoot / full auto / throw / overwatch shot / free attack), weapon, range category (user input), and relevant conditions (Dark zone, light, target size, defenseless/unwitting, prone target, cover). [pp. 91–96]
2. **Validate**:
   - Close combat: attacker standing; target normally at Engaged. [p. 91]
   - Ranged: can see the target; range ≤ weapon's max range; not shooting through a Dark zone; not shooting past adjacent individuals in a Cramped zone; magazine not empty. [pp. 83, 94, 96]
   - Full auto: weapon has full auto. [p. 96]
   - Shove / Retreat: enemy at Engaged. [p. 88]
   - Overwatch shot: target in the chosen direction; slow slot free. [p. 97]
3. **Interrupt window** (overwatch): any combatant on overwatch whose direction covers the attacker may shoot now. That shot resolves before the declared action, and the attacker cannot change its declared attack. Mutual overwatch → opposed RANGED COMBAT (not an action) decides who goes first. [p. 97]
4. **Block window** (close combat only, not sneak attacks, grapple attacks or retreat free attacks): the defender may declare a block **before the attacker rolls**. Requires an unused action, and the unarmed-block restrictions apply. The block consumes 1 action. [p. 92, p. 93]
5. **Close combat attack on an overwatcher** → the overwatch is lost immediately. [p. 97]
6. **Build the attacker's pool**: [pp. 93–96]
   - Base Dice = attribute + skill (skill 0 → attribute only) + gear bonus. [p. 93; cross-ref: pp. 118–126]
   - Modifications: close combat +2 against a prone target if the attacker is standing; ranged modifications table (aim +2, range, size, light, Engaged –3 or +3; DECIDED: one light level per shot, Dim light –1 or Darkness –2, a Dark zone counts as Darkness, never both); full auto +2; helpers +1 each (max +3); crit penalties; talents. [pp. 83, 89, 91, 94–96, 100]
   - Full auto (PC): STRESS LEVEL +1 **before** rolling (unless Machine Gunner); this new Stress Die is included. [p. 96]
   - Stress Dice = current STRESS LEVEL (PCs only; synthetics and NPCs have none). [p. 96; cross-ref: p. 61, p. 62, p. 111]
7. **Roll**. The attacker may push (not NPCs, not synthetics; cross-ref). If any Stress Die shows a facehugger: no push → **Panic Roll** (a result of 10+ cancels the attack). If the attack is firing a weapon with a magazine → **magazine empty after the attack**. [p. 96; cross-ref p. 61, p. 104]
8. **Block roll** (if declared): CLOSE COMBAT. Spend each success on: remove 1 attacker success (repeatable) / counterattack (blocker's weapon Damage, not increasable) / disarm. [p. 92]
9. **Hit check**: attacker net successes ≥ 1 → hit. Otherwise miss (a counterattack or disarm from the block still applies). [p. 92, p. 95]
10. **Allocate successes**: the first success = hit for weapon Damage. Each extra success = one stunt from the relevant list (close 4.2 / ranged 5.5). Full auto: extra successes may go to secondary targets within SHORT of the primary (first per target = Damage, more = stunts). [pp. 92, 95, 96]
11. **Apply stunts**: initiative swap (queued for next Round), drop or knock away an object (mark it on the ground), prone or pushed back, grapple (set both prone; target drops its weapon and can't move), pin down (PC → immediate Panic Roll; NPC → loses next slow action). [pp. 92–93, 95]
12. For each target that takes damage (including a counterattack on the attacker) → 8.6. [p. 92, p. 98]

### 8.6 Damage → Broken → crit [pp. 96, 98–100]
1. Incoming damage D = Damage rating + damage stunts. [p. 92, p. 95]
2. Armor roll (not an action): Base Dice = Armor Rating (+ cover Armor Rating if the attack is ranged and the target is in cover). Apply weapon specials (Armor Piercing halves, rounding up; Armor Doubled ×2, cross-ref p. 118). Each success −1 damage. D' = max(0, D − successes). [p. 96, p. 98]
3. If D' = 0 → done (no crit even if Broken). [p. 98]
4. D' > 0 → target loses Aim and Overwatch. [p. 94, p. 97]
5. If the target is not Broken: Health = max(0, Health − D'). If Health reaches 0 → **Broken** → immediately go to 8.6.7. [p. 98]
6. If the target was already Broken → go to 8.6.7 (another crit). [p. 98]
7. **Crit** (humans; synthetics and Xenomorphs use their own tables): roll D66 (Killer: attacker may swap digits) → look up 6.9: [p. 99, p. 100]
   - 63–66 → **Dead** (no Death Roll). [p. 99]
   - Apply the effect (13: STRESS LEVEL +1; record penalties and restrictions; 46: start disease). Roll and store the healing time. [p. 100]
   - Fatal with a time limit → enter DYING: store the time limit, the Medical Aid modifier, and the inflicted-at timestamp; schedule the first Death Roll. [p. 99]
   - PC → Panic Roll. [cross-ref: p. 104]
8. Minor NPC Broken → GM may declare it dead. [p. 99]

### 8.7 Round end [p. 87]
1. When all participants have acted once → Round ends. Advance the clock by 1 Round (5–10 s). [p. 83, p. 87]
2. If the GM ends combat → 8.9. Otherwise → 8.3. [p. 87]

### 8.8 Dying, first aid, getting up (runs across Rounds, Turns, Shifts, Days) [pp. 98–99]
1. **Death Roll due** (time limit elapsed since infliction, or since the last Death Roll) → STAMINA roll, no push, no Stress Dice. Fail → Dead. Success → schedule the next Death Roll one more time-limit interval later. [p. 99]
2. **First aid to save a life**: slow action by a healer, MEDICAL AID + equipment bonus + the crit's modifier (–1/–2). Allowed only before a failed Death Roll and only once per healer, unless they have better equipment. Success → the fatal condition is cleared (no more Death Rolls); the effects and healing time remain. [p. 99]
3. **First aid to get up** (Broken): slow action, MEDICAL AID. Success → no longer Broken, Health = number of successes, the character gets up immediately. No effect on a non-Broken character. Broken + fatal → both rolls are needed, in either order. [p. 98, p. 99]
4. **On your own**: Broken and no help for one Turn → +1 Health, may get up. Now (if dying and before a failed Death Roll) the character may try self-first-aid to save its life at –2. [p. 98, p. 99]
5. **Coup de grâce** on a Broken human: the attacker must fail an EMPATHY roll (attribute only, no Stress Dice). Then the target dies. The attacker gets STRESS LEVEL +1 either way. DECIDED: with **Merciless**, no EMPATHY roll and no stress (6.6). [p. 99]

### 8.9 Combat end [pp. 83, 98, 101]
1. The GM ends combat (no rule-defined end; AMBIGUOUS). Return to Turns (stealth or free play). [p. 83]
2. Initiative cards are discarded. Aim, overwatch, cover, grapples and pending initiative swaps are cleared. AMBIGUOUS: Suggested: prone and Broken states persist. [p. 87]
3. Non-Broken characters recover 1 Health per Turn of rest. Broken characters follow 8.8. [p. 98]
4. Pending Death Rolls continue on their timers (Turn, Shift, Day). [p. 99]
5. Crit healing in days; Care halves the remaining time (6.11). Disease per p. 109. [p. 101]
6. After the session: permanent mental trauma check (6.12). [p. 101; cross-ref p. 104]

### 8.10 Worked examples from the book (numbers as printed)

- **Close combat, block ruled out** [p. 93]: initiative Gina/Charlize **6**, GM/Nguyen **1** → Nguyen acts first. Nguyen CLOSE COMBAT **0** → defaults to **STRENGTH 3** → **3 Base Dice**. Gina declares a block before the roll, but Charlize is unarmed against a knife → not allowed. Result: **1 success** → **2 damage** to Charlize's Health (knife Damage 2). [p. 93]
- **Throwing an improvised weapon** [p. 97]: next Round, Charlize picks up a power tool (**fast action**) and throws it (**RANGED COMBAT**, slow): **3 Base Dice (AGILITY) + 1 Base Die (RANGED COMBAT)** → **2 successes** → **1 point of damage** + **1 extra success** spent on "pushed back" (Nguyen staggers toward the airlock and trips). [p. 97]

---

## 9. Ambiguities / Game Mother discretion

Each item is my interpretation, or a gap the GM must fill. Suggested app behavior: implement the suggestion as the default, and let the GM override.

1. **Rounds ↔ Turns conversion and returning to Turns** after combat are undefined. The GM switches modes and advances clocks manually. [p. 83, p. 86]
2. **Dark zone –2 vs "Dim light –1 / Darkness –2"**: DECIDED: one light level per shot. A Dark zone counts as Darkness –2; Dim light –1 is the separate, lesser option; never both. [p. 83, p. 95]
3. **Winning against an ACTIVE enemy's sneak attempt**: the enemy is spotted at its starting distance and initiative is drawn, with no free attack. [p. 85]
4. **Group stealth rule** (lowest MOBILITY vs highest OBSERVATION) applies to all group stealth rolls, not only ambushes. [p. 90]
5. **Stealth table vs Sneak Attacks & Ambushes table**: the Stealth table is for sneaking past, the other for sneak attacks and ambushes. [p. 86, p. 90]
6. **Motion tracker in combat**: treat as the fast action "Use item", once per Turn of game time, with a Power Supply roll. [p. 86]
7. **Newcomers joining combat**: draw from the undrawn cards. The GM decides whether they act this Round or next. [p. 87]
8. **More than 10 initiative cards needed** (Speed creatures + many combatants): group more NPCs, otherwise the GM orders manually. [p. 87]
9. **Initiative exchange is PC↔PC only.** Changes involving NPCs happen only through stunts. [p. 87]
10. **Ambush +2** is added to the range modification. [p. 90]
11. **Group ambush free actions**: each ambusher gets one free action (slow or fast). [p. 90]
12. **Ranged sneak attack at Engaged** counts as "unwitting", so +3. [p. 90, p. 94]
13. **Non-damage stunts** (close and ranged) can be chosen at most once per attack. [p. 92, p. 95]
14. **Block timing**: the attacker rolls (and pushes) first, then the blocker rolls and spends successes. [p. 92]
15. **Blocking with the slow slot**: allowed (a fast action may replace the slow one), so at most 2 blocks per Round. [p. 87, p. 92]
16. **COUNTERATTACK / DISARM** can be chosen at most once per block. Armor applies to counterattack damage. Disarm removes the weapon used in the attack. [p. 92]
17. **Grapple**: releasing is free. Two grapple attacks per Round are allowed. Both stay prone after the grapple ends. A grappled character can't block. A grapple attacker isn't "standing", so no +2. [p. 91, p. 93]
18. **Retreat from several engaged enemies**: one roll; on failure each active engaged enemy gets a free attack. [p. 93]
19. **Aim persistence and "hurt"**: aim lasts across Rounds until any non-shooting action, or until ≥1 damage is taken after armor. It applies to one shot. [p. 94]
20. **Pinned-down NPC**: loses its next unused slow action (this Round's if still unused). [p. 95]
21. **Initiative swap with a grouped NPC**: GM decides, and may give that NPC its own card. [p. 87, p. 95]
22. **NPC full auto**: no stress step, because NPCs have no STRESS LEVEL. [p. 96; cross-ref p. 62]
23. **Full auto distribution** is optional and unlimited in number of secondary targets. Each target rolls its own armor/cover. [p. 96]
24. **Cover duration**: until the character moves or the cover is gone. **Armor Piercing vs combined cover + armor**: halve the combined total. [p. 96]
25. **Ammo rule and thrown weapons**: thrown weapons never "empty a magazine", but a facehugger still triggers a Panic Roll. **No reloads left** → the weapon can't fire. [p. 96]
26. **Which Round's slow action an overwatch shot uses**: the slow action of the Round in which it is fired. [p. 97]
27. **Overwatch duration**: by default the window ends at your next slot (re-assume with a fast action). GM setting for "persists while only shooting". [p. 97]
28. **Overwatch gives no aim bonus** unless you also aimed. [p. 97]
29. **Armor roll can't be pushed** and uses no Stress Dice. [p. 98]
30. **"Rest" for Health recovery**: no actions or skill rolls during the Turn. [p. 98]
31. **Death Roll while Broken** is allowed despite "can't roll for any skills". [p. 98, p. 99]
32. **Hit reduced to 0 damage by armor** → no crit for a Broken target. [p. 98]
33. **"No one around to help"**: self-recovery happens after any full Turn without a successful first aid. [p. 98]
34. **Getting back up** clears both Broken and prone. [p. 98]
35. **Coup de grâce**: DECIDED: the "Cold-Blooded" talent named in the text (it does not exist) means **Merciless** (p. 78), which skips both the EMPATHY roll and the stress. The action type is a slow attack. A failed EMPATHY roll means the target dies outright. Non-human targets need no roll and give no stress. [p. 99]
36. **Death Roll timing**: one full time-limit interval after infliction, then repeating. "One Round" falls at the victim's slot in the next Round. [p. 99]
37. **Several fatal crits**: one life-saving roll covers all of them, using the worst modifier (GM setting). [p. 99]
38. **"Each healer can try only once"**: per patient per dying episode; a retry needs better equipment. [p. 99]
39. **Saved from a fatal crit**: Death Rolls stop, but effects continue for the healing time. Later fatal crits start new timers. [p. 99, p. 101]
40. **Pushing the life-saving MEDICAL AID roll** is allowed (only the Death Roll is explicitly unpushable). [p. 99]
41. **Cracked spine (56) "in time"** is undefined (it is not fatal and has no time limit). The GM decides; suggested default: before healing starts. [p. 100]
42. **Duplicate crits** (same result twice): record both, GM's call. [p. 100]
43. **Healing dice** are rolled when the injury is suffered. **Care** halves the remaining time once per healing process, rounding up, and needs daily tending. [p. 100, p. 101]
44. **Crits 14/15 "until MEDICAL AID roll is made"**: any successful MEDICAL AID roll on the character. [p. 100]
45. **Crit penalties stack**. "Can't use arm/hand" gear limits are the GM's ruling. [p. 100]
46. **End of combat state**: aim, overwatch, cover, grapples and pending swaps are cleared; prone and Broken persist. [p. 87]

Pure GM-discretion points stated by the book itself:
- How long a Round, Turn or Shift lasts, and when one has passed. [p. 83]
- Time needed to examine something in a zone. [p. 85]
- Whether an enemy is ACTIVE or PASSIVE. [p. 85]
- Movement of non-human NPCs. [p. 85]
- Whether a still enemy registers on the motion tracker, and whether an OBSERVATION roll is needed to spot a small or hidden enemy. [p. 86]
- Order within an NPC initiative group. [p. 87]
- NPCs blocking. [p. 93]
- NPCs taking cover. [p. 96]
- NPC ammo. [p. 96]
- Modifiers to shoves against certain opponents. [p. 93]
- NPC-on-NPC first aid. [p. 99]
- Minor Broken NPCs dying. [p. 99]
- The object of a Phobia. [p. 101]

---

## 10. Time-related rules

### 10.1 Units [p. 83]

| Unit | Duration | Use |
|---|---|---|
| Round | 5–10 seconds | Combat. One Round = every participant acts once. [p. 83, p. 87] |
| Turn | 5–10 minutes | Stealth mode. [p. 83, p. 85] |
| Shift | 5–10 hours | Recovery. Four per day: Morning, Day, Evening, Night. [p. 83] |
| Day | 4 Shifts | Crit healing times and the "One Day" time limit. [p. 83, p. 101] |

- Combat starts when the enemy is revealed: Turns stop, Rounds start. [p. 86]
- The GM decides when each unit has passed. [p. 83]

### 10.2 Everything with a duration or frequency

| Rule | Duration / frequency | Page |
|---|---|---|
| Stealth movement | 2 zones per Turn (PCs and human NPCs) | p. 85 |
| Examine something in a zone | ≥1 entire Turn in that zone (GM decides) | p. 85 |
| GM enemy movement | each Turn, after the PCs move | p. 85 |
| Motion tracker | once per Turn; Power Supply roll each use | p. 86 |
| Initiative | drawn once, at the start of Round 1; never redrawn | p. 87 |
| Initiative exchange between PCs | at the start of the fight or of a Round only | p. 87 |
| Stunt initiative swap | takes effect next Round; permanent | p. 92, p. 95 |
| Action budget | per Round: 1 slow + 1 fast, or 2 fast | p. 87 |
| Crawl | at most once per Round (slow action) | p. 89 |
| Aim | lasts until any other action or until hurt | p. 94 |
| Overwatch | from assuming it until your time to act in the next Round; ended by any other action, a close combat attack against you, or damage | p. 97 |
| Pinned-down NPC | misses its next slow action | p. 95 |
| Full auto stress | +1 STRESS LEVEL immediately, on each burst | p. 96 |
| Retreat free attack | immediate; not counted against the enemy's actions | p. 93 |
| Health recovery (not Broken) | +1 Health per Turn of rest | p. 98 |
| Broken, unattended | +1 Health after one Turn, then may get up | p. 98 |
| Self first aid when dying | possible once the +1 Health has been recovered on your own (after one Turn), before a failed Death Roll; –2 | p. 99 |
| Death Roll | when the crit's time limit runs out, then again each time the same interval passes | p. 99 |
| Crit time limits | One Round: 61, 62. One Turn: 52, 53. One Shift: 45, 46, 54, 55. One Day: 44, 51 | p. 100 |
| Instant death | 63–66, immediate | p. 99, p. 100 |
| Crit healing times | D6 days: 16, 21, 22, 23, 24, 26, 31, 32, 44, 45, 52, 53. 2D6 days: 25, 33, 34, 35, 36, 41, 46, 51, 61. 3D6 days: 42, 43, 56, 62. Permanent: 54, 55. None: 11, 12, 13. Until a MEDICAL AID roll: 14, 15 | p. 100 |
| Cracked spine (56) | effect becomes permanent if MEDICAL AID isn't given "in time" (undefined) | p. 100 |
| Care | daily tending + MEDICAL AID roll → remaining healing time halved | p. 101 |
| Disease (crit 46, Virulence 6) | damage one Shift after infection, Sickness Roll every Shift (cross-ref) | p. 100; p. 109 |
| Phobia | Panic Roll if you stay near the object for more than a single Round | p. 101 |
| Alcoholism / Drug use | must consume every Shift, or +1 STRESS LEVEL | p. 101 |
| Nightmares | EMPATHY roll each sleep; on success +1 stress and no stress relief for a full day | p. 101 |
| Depression | EMPATHY roll every day; on failure +1 stress and no stress relief until the next day | p. 101 |
| Permanent mental trauma check | after a session with a Panic Roll result ≥13 (cross-ref) | p. 101; p. 104 |
| Panic ends on its own | after one Turn (cross-ref) | p. 104 |

---

## 11. State the app must track

### 11.1 Per combatant
- **Identity and kind**: PC / NPC / NPC group member / synthetic / Xenomorph. Kind decides the crit table, Stress Dice, pushing, panic and ammo behavior. [p. 96, p. 99; cross-ref pp. 62, 111, 112]
- **Minor NPC flag** (GM may let a Broken minor NPC die). [p. 99]
- **ACTIVE / PASSIVE** (enemies); **detected** status (seen / tracker ping / hidden). [p. 85, p. 86]
- **Attributes and skills** used here: STRENGTH, AGILITY, EMPATHY, WITS; CLOSE COMBAT, RANGED COMBAT, MOBILITY, OBSERVATION, STAMINA, MEDICAL AID, COMMAND, MANIPULATION, COMTECH, PILOTING. [pp. 85–101]
- **Talents** affecting combat (Fast Reflexes, Flyweight, Killer, Machine Gunner, Merciless, …). [cross-ref: pp. 75–79]
- **Health**: current / max (max = STRENGTH ± talents); floor 0. [p. 98]
- **STRESS LEVEL** (PCs only). [p. 96]
- **Status flags**: Broken, Dead, Prone, Panicking (cross-ref), Defenseless (= Broken). [pp. 91, 98, 99]
- **Grapple link**: grappling → target id / grappled by → attacker id. [p. 93]
- **Initiative card value(s)** (several for Speed creatures); the pending swap for next Round. [p. 87, p. 92, p. 95]
- **Action budget for the current Round**, per card: slot usage (slow slot used as slow / as fast / free; fast slot used / free); log of actions including reactions taken outside own slot; whether the slot has been reached this Round. [p. 87, p. 89, p. 92]
- **Pending penalties**: "miss next slow action" (pinned NPC). [p. 95]
- **Aim**: active (weapon id). [p. 94]
- **Overwatch**: active, direction (text), Round assumed, expires at own next slot. [p. 97]
- **Cover**: in cover?, cover type and Armor Rating. [p. 96]
- **Armor**: worn suit and its Armor Rating (only one). [p. 98]
- **Weapons and items**: in hand / holstered (draw = fast action); per weapon Damage, Bonus, max range, flags (full auto, armor piercing, armor doubled, thrown, blast); **magazine-empty flag**; **number of full reloads carried**. Items dropped or knocked away (on the floor). [pp. 91–96]
- **Free action pending** (successful sneak attack or ambush). [p. 90]
- **Critical injuries list**, each with: D66 code, name, fatal (bool), Medical Aid modifier (0/–1/–2), time limit unit, inflicted-at timestamp, next Death Roll due, Death Rolls made, "life saved" flag, effect modifiers (skill → value), restrictions (can't run, run is slow, crawl only, can't use arm/hand, paralyzed), "damage on MOBILITY/CLOSE COMBAT roll", healing time (rolled days), remaining days, permanent flag, "until MEDICAL AID" flag, care-halving applied flag. [pp. 99–101]
- **First aid attempt log** (per healer: patient, purpose save-life/get-up, equipment level, result) to enforce "one try per healer unless better equipment". [p. 99]
- **Broken-since timestamp** and **"recovered Health on own"** flag (enables self-aid at –2). [p. 98, p. 99]
- **Coup de grâce** stress effect source (attacker's STRESS LEVEL). [p. 99]
- **Disease state** (from crit 46). [cross-ref: p. 109]
- **Permanent mental traumas** (D6 results) and the session flag "Panic Roll ≥13 rolled this session" (cross-ref, p. 104). [p. 101]

### 11.2 Per combat / per scene
- **Mode**: STEALTH (Turns) / COMBAT (Rounds) / DOWNTIME (Shifts/Days). [p. 83, p. 86]
- **Clock**: Round counter (within combat), Turn counter, Shift counter (with Morning/Day/Evening/Night), Day counter. Timestamps for timers. [p. 83]
- **Initiative deck**: cards 1–10 drawn / undrawn; assignment card → combatant or NPC group; current slot pointer; queued stunt swaps for next Round; PC↔PC exchanges allowed only at the start of the fight or a Round. [p. 87]
- **NPC groups** (members with identical stats; one card each group). [p. 87]
- **Combatant count** (>10 → grouping rule). [p. 87]
- **Per-attack context inputs** (no map): range category, target size, light (one per shot: normal / Dim light –1 / Darkness –2; DECIDED: a Dark zone sets Darkness, never both), Dark/Cluttered/Cramped zone flags, target defenseless/unwitting, target prone, attacker standing, cover. [pp. 83, 91, 94–96]
- **Timers queue**: Death Rolls (per crit), self-recovery after one Turn for Broken characters, overwatch expiry, healing days, disease Shifts, Health recovery per Turn of rest. [pp. 97–101]
- **Reaction windows**: pending declared action (for overwatch interrupts), declared blocks before the attack roll. [p. 92, p. 97]
- **Event log** (rolls, successes, facehuggers, stunts chosen, damage before and after armor, crits) for review and undo.
