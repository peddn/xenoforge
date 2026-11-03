# 01 — Characters (ALIEN RPG Core Rulebook, 1st ed. 2019)

> Official errata and project decisions in [00-errata-und-festlegungen.md](00-errata-und-festlegungen.md) take precedence over this file.

Scope: game modes as they affect characters, character creation (Cinematic and Campaign), attributes, skills, talents (all), Health, Stress, Personal Agenda / Story Points, Buddy & Rival, gear / signature item / cash, encumbrance, consumables, experience, all nine careers, androids/synthetics as PCs, other character-creation methods, character sheet.

Conventions used in this file

- `[p. N]` = **book page** (printed page number = PDF page − 4).
- **Success** = the success symbol, a 6 on any Base Die or Stress Die. **Facehugger** = the facehugger symbol, a 1 on a Stress Die. (The extracted text drops these icons; every place where they occur in this file was checked against the rendered page.)
- `CIN` = rule applies to Cinematic play only, `CAM` = Campaign play only. The book marks mode-specific rules with these symbols [p. 19].
- "AMBIGUOUS (A#):" marks my interpretation. All of them are listed again in the section "Ambiguities / Game Mother discretion".
- "Derived:" marks a consequence that follows directly from combining two stated rules.

---

## 0. Dice notation used by character rules

- **D6**: roll one Base Die and read the number, ignoring symbols [p. 22].
- **D3**: roll a D6, divide by 2, round up [p. 22].
- **2D6**: roll two D6 and add them [p. 22].
- **D66**: roll two D6. The first die is the tens digit and the second the ones digit, giving 11–66 [p. 22].
- Base Dice and Stress Dice both have the success symbol on the 6. Stress Dice also have the facehugger symbol on the 1 [p. 22].
- **Skill roll** (context only): Base Dice = attribute + skill level, plus a number of Stress Dice equal to the current STRESS LEVEL. You need at least one success [p. 58, 61].
- **Push** (context only): re-roll all dice that did not show a success. This is allowed once, and the STRESS LEVEL rises by 1 *before* the re-roll. You cannot push if a facehugger was rolled; you make a Panic Roll instead [p. 60–61]. Some talents allow a second push [p. 60].

---

## 1. Game modes and how they affect characters

| Topic | Cinematic play (CIN) | Campaign play (CAM) | Page |
|---|---|---|---|
| Length | One scenario, designed for a single session, with 3 Acts | Many sessions (at least 5–6 recommended) | [p. 19, 216, 217] |
| Where PCs come from | **Pre-generated** by the scenario. Players decide among themselves who plays whom | Players **create** their PCs with the Chapter 2 rules | [p. 26] |
| Surplus PCs | More characters than players is usual. The leftover characters become NPCs run by the GM | n/a | [p. 26] |
| Choosing PCs | If the players can't agree, roll dice for it | n/a | [p. 26] |
| Personal Agenda | Set by the scenario. A new hidden handout for **each Act**. It is not shown to others and not written on the sheet | The player picks one from the career list or invents one | [p. 31] |
| Agenda reward | 1 **Story Point** at the end of each Act if the PC furthered the Agenda | 1 **XP** at the end of each session (see §11 and §16) | [p. 31, 35] |
| Buddy / Rival | Set by the scenario | Chosen by the player | [p. 31] |
| Starting gear | Set by the scenario | Chosen from the career list | [p. 33] |
| Name | Set by the scenario | Chosen from the career list or invented | [p. 30] |
| Signature item stress relief | Once per **Act** | Once per **game session** | [p. 104] |
| The Long Haul talent | Once per **Act** | Once per **game session** | [p. 76] |
| PvP | The GM may "call PvP". The traitor PC becomes an NPC afterwards (see §12) | Handled within the group, with no specific rules | [p. 32] |
| Development | Story Points (max 3, belong to the **player**) | XP spent on skills and talents | [p. 61, 35–36, 383] |
| Vehicle / ship | The scenario may give the group one | The group may start with one (Chapter 7) | [p. 33, 327] |

- Campaign frameworks (decide before creating characters, as a group): Space Truckers, Colonial Marines, Frontier Colonists [p. 26, 326]. The suggested careers for each are in §21.

---

## 2. Character creation methods found in the book

| # | Method | Where | Summary |
|---|---|---|---|
| M1 | **Campaign PC creation** (10 steps) | [p. 27–36, careers p. 38–55] | Full point-buy method, specified in §3–§17 |
| M2 | **Pre-generated Cinematic PCs** | [p. 26, 30, 31, 33, 215–216] | The scenario supplies the PCs with stats, background, gear, Buddy/Rival and per-Act Agendas. Players pick by agreement or roll dice. Leftovers become NPCs. Hope's Last Day (the mini-scenario in the book) has 5 pre-generated PCs. Players choose them or select randomly, and leftover PCs are played as NPCs [p. 370–371]. *(Another spec extracts those PCs.)* |
| M3 | **Android PC** (a variant of M1 or M2) | [p. 28–29, 111] | Any career. +3 to two attributes after the 14-point buy, max 8 in those two. Other drawbacks apply. See §20. Pre-generated androids use "normal android rules" [p. 371] |
| M4 | **Session Zero** (CAM, recommended) | [p. 217] | The group creates PCs together and picks a framework. Discussion questions include "Who are your buddies and who are your rivals?". Procedural only, with no extra mechanics |
| M5 | **Replacement PC after PvP** (CIN) | [p. 32] | A PC who "turned traitor" (and survived) becomes a GM-controlled NPC. That player gets a new PC to play, if one is available (i.e. a leftover pre-generated character) |
| M6 | **Quick NPC creation** (GM) | [p. 214] | 14 attribute points "just like PCs" (not absolute). No NPC attribute above 5 **except androids**. Skills only where needed: 2 = fairly competent, 3–5 = expert. Usually no talents. Needs Name, Appearance, Trait, Goal. No consumables, gear or Stress tracking for NPCs [p. 214, 105] |
| M7 | **Typical NPC stat blocks** (GM) | [p. 354–355+] | Ready-made NPCs to pick or adapt. They include three android NPCs (Covert, Curious, Refurbished) |

- No random PC-generation tables exist (no rolled careers or attributes). The only random elements in M1 are cash, "D6 doses"-type gear quantities, and optionally choosing pre-generated PCs by dice in M2 [p. 26, 39–55, 371].
- The career pages offer name, appearance, agenda and signature-item **lists** to pick from, not tables to roll on [p. 39–55].

---

## 3. Campaign PC creation — step order [p. 27]

1. Choose your career.
2. Spend points on your attributes.
3. Spend points on your skills.
4. Choose your career talent.
5. Choose your name.
6. Decide on your appearance.
7. Decide on your Personal Agenda.
8. Choose your Buddy and Rival.
9. Pick your gear and a signature item.
10. Roll for cash.

Derived values: Health = STRENGTH [p. 30]. STRESS LEVEL = 0 [p. 30]. Carrying capacity = STRENGTH × 2 [p. 34].

---

## 4. Career [p. 27]

- There are nine core careers: Colonial Marine, Colonial Marshal, Company Agent, Kid, Medic, Officer, Pilot, Roughneck, Scientist [p. 27, 38–55].
- The career determines the **key attribute** (which may go to 5), the **key skills** (which may go to 3 at creation), the **three career talents** to choose from, the starting gear list, the cash formula, and the suggested names, appearance, agendas and signature items [p. 27–29, 33].
- Career talents can only be learned by characters of that career. This applies at creation and later [p. 74].

---

## 5. Attributes [p. 27–28]

- There are four attributes: **STRENGTH** (raw muscle power), **AGILITY** (body control, speed, motor skills), **WITS** (perception, intelligence, sanity), **EMPATHY** (charisma, empathy, manipulation). The scale is 1–5 [p. 27].
- Attributes are used in rolls and also determine how much damage you can take before you are Broken [p. 27].
- **Starting points (CAM): 14 points** in total across the four attributes [p. 28].
- **Min 2, max 4** per attribute. Exception: the career's **key attribute may be 5** [p. 28].
- **Androids:** after the 14 points are assigned, add **+3 to two attributes of choice**. Those two may reach **max 8** [p. 28, 29]. See §20.
- Validation (human): sum = 14; each value is in 2..4; the key attribute is in 2..5.
- Validation (android): the base passes the human validation; then exactly two distinct attributes get +3. Max 8 in those two, which a key attribute at 5 reaches. AMBIGUOUS (A1): the book does not say whether the +3 must go to two *different* attributes. "Two attributes of their choice" implies two distinct ones.
- There is **no rule for raising attributes** after creation. XP can only buy skills and talents [p. 36]. AMBIGUOUS (A2): attributes are therefore fixed for life.
- NPCs: no NPC attribute above 5 except androids [p. 214].

---

## 6. Skills [p. 28, 58]

### 6.1 The 12 skills and their attributes [p. 58]

| Attribute | Skills |
|---|---|
| STRENGTH | Heavy Machinery, Stamina, Close Combat |
| AGILITY | Mobility, Ranged Combat, Piloting |
| WITS | Observation, Comtech, Survival |
| EMPATHY | Command, Manipulation, Medical Aid |

### 6.2 Rules

- Skill level scale is **0–5** [p. 28, 58].
- **No skill level**: you can still roll, using the attribute plus gear modifiers (and Stress Dice) [p. 28, 58].
- **Starting points (CAM): 10 points** in total [p. 28].
- **Career key skills (3 per career): up to 3** points each at creation [p. 28].
- **Any other skill: max 1** point at creation [p. 28].
- Skills can be increased with XP (§16) [p. 28, 36].
- Validation: sum = 10; key skills ≤ 3; non-key skills ≤ 1. AMBIGUOUS (A3): the book implies all 10 points must be spent ("distribute a total of 10 points").
- Example (Roughneck Nat): Heavy Machinery 3, Stamina 2, Close Combat 2, Comtech 1, Observation 1, Medical Aid 1, for a total of 10 [p. 28].

---

## 7. Talents at creation [p. 29, 74]

- Talents are not rated: you have them or you don't [p. 74].
- There are two types: **career talents** (only for that career) and **general talents** (anyone) [p. 74].
- **CAM: you start with exactly 1 talent**, chosen from your career's **three** career talents [p. 29, 74].
- More talents can be learned during play with XP (§16) [p. 29, 36, 74].
- CIN: pre-generated PCs come with the talents the scenario gives them [p. 26]. (No rule restricts these.)
- Full talent list and effects: §19.

---

## 8. Health [p. 30, 98]

- **Starting / maximum Health = STRENGTH** [p. 30, 98].
- Talents can modify maximum Health (Tough: +2) [p. 30, 79].
- Taking damage reduces Health. At **0 Health you are Broken** and roll a critical injury [p. 30, 98].
- Health cannot go below 0. Each further damaging attack while Broken gives another critical injury [p. 98].
- Recovery (context only): when no longer Broken, recover 1 Health per Turn of rest. Broken with nobody to help: automatically recover 1 Health after one Turn and get back up. First aid (MEDICAL AID, slow action) gets you up with Health = number of successes [p. 98]. Full rules belong to the combat spec.
- Androids: Health = STRENGTH, including the +3 bonus. Derived from the android NPC blocks, e.g. Android, Covert: STRENGTH 8, HEALTH 8 [p. 355]. Androids don't heal; they are repaired (§20) [p. 111].
- Character sheet: 10 Health boxes [p. 389]. That is the theoretical max: android STRENGTH 8 + Tough 2.

---

## 9. Stress [p. 30, 103–104]

- **STRESS LEVEL starts at 0** ("usually") [p. 30, 103].
- It increases by pushing and by stressful events (context, full list on p. 103): push a skill roll; fire full auto; suffer ≥1 damage; go without sleep, food or water; perform a coup de grâce; a Scientist in your team fails to use Analysis; a crew member attacks you; **a person nearby is revealed to be an android**; GM- or scenario-determined creatures and locations [p. 103].
- Add Stress Dice equal to the STRESS LEVEL to every skill roll. A facehugger on any Stress Die in a skill roll means a Panic Roll [p. 61, 103].
- **Relieving stress**: −1 per full Turn resting in a safe area. No skill rolls while resting, and an interrupted rest doesn't count [p. 104]. Signature item: see §13. Some conditions block relief [p. 104, 106].
- **Androids have no STRESS LEVEL** (§20) [p. 29, 111].
- NPCs never track STRESS LEVEL and never make Panic Rolls [p. 105].
- Character sheet: 10 Stress boxes [p. 389]. AMBIGUOUS (A4): no maximum STRESS LEVEL is stated in the rules.

---

## 10. Name and appearance [p. 30]

- CAM: each career lists **3 male and 3 female** typical names. Choose one or make up your own [p. 30].
- CIN: the scenario provides names [p. 30].
- Appearance: a few words. The career gives suggestions, but the choice is free [p. 30].

---

## 11. Personal Agenda and Story Points

### 11.1 Cinematic (CIN)

- Scenario fixes the Agendas. GM gives each PC a **new** one at the **start of every Act** (3 Acts) [p. 31, 216].
- Agendas are **hidden**: not shown to other players and **not written on the character sheet** [p. 31]. They should remain hidden until the end of the scenario [p. 216].
- Events in a scenario can change a PC's Personal Agenda [p. 216].
- **End of each Act**: the GM evaluates each PC. If the PC took specific action to further the Agenda during the Act ("in spite of personal risk or significant sacrifice" [p. 216]), the GM awards **1 Story Point**. The GM announces the award but **does not say why** [p. 31, 216].
- Hope's Last Day is a single Act (the "third Act" only). Story Points are evaluated and handed out after the scenario ends (epilogue). Players may then reveal their Agendas [p. 370, 383].

### 11.2 Story Points (CIN only) [p. 61, 383]

- **Spend 1 Story Point → 1 automatic success in a dice roll** [p. 61]. DECIDED: skill rolls only, max 1 per roll (see A5 below).
- You can spend it **after a failed roll** (to turn it into a success) or **after a successful roll** to get an extra success [p. 61].
- **Max 3 Story Points** held at any time [p. 61, 383].
- **Story Points belong to the player, not the PC.** Players may keep them for the **next Cinematic scenario** [p. 383]. Derived: they survive PC death and PC swaps.
- Earned only by following the Personal Agenda (see above) [p. 61].
- DECIDED (A5): **at most one Story Point per roll**, and **only on skill rolls** (never on Panic Roll, Supply roll, armor roll, Death Roll). Note: the book is silent on both points.
- AMBIGUOUS (A5, still open): whether spending one cancels a Panic Roll triggered by the same roll (nothing says it does).
- AMBIGUOUS (A6): no starting Story Points are given. Assume 0 for a new player, plus whatever was carried over.
- DECIDED: Story Points exist **only in Cinematic play**; none in Campaign play. Note: the book only mentions them for Cinematic scenarios [p. 61].
- Character sheet: 3 Story Point boxes [p. 389].

### 11.3 Campaign (CAM) [p. 31, 35]

- Pick one of the **three suggested agendas** for your career, or write your own [p. 31].
- **Session end**: group reviews every PC's agenda. PC acted concretely toward its agenda this session, accepting risk or cost → **+1 bonus XP** [p. 31].
- AMBIGUOUS (A7): this "bonus XP" [p. 31] is very probably the **same** XP as the XP question "Did you risk or sacrifice something to further your Personal Agenda?" [p. 35], since p. 31 points to p. 35. Treat it as one XP, not two.
- No rule covers changing a Campaign agenda. AMBIGUOUS (A8): allow free change with GM consent.

---

## 12. Buddies, Rivals and PvP

- A PC can have **one Buddy and one Rival among the other PCs**, and only one of each [p. 31].
- CAM: you choose one PC as Buddy and **another** PC as Rival (two different PCs) [p. 31].
- CIN: the scenario chooses for you [p. 31].
- **Changing (CAM)**: after the end of a game session you may change your Buddy and Rival freely [p. 36].
- XP ties: "risk your life for your Buddy PC" and "challenge or stand up to your Rival PC" are each worth 1 XP (§16) [p. 35].
- AMBIGUOUS (A9): it is not stated whether having a Buddy or Rival is mandatory. With only 2 PCs you cannot have both a Buddy and a Rival, because they must be different PCs.
- **PvP (CIN)**: the GM can "call PvP" when a conflict between PCs escalates beyond the point of no return. The players involved play the conflict out to its conclusion. Afterwards, the PC who "turned traitor", if still alive, **becomes an NPC** controlled by the GM. That player gets a **new PC**, if one is available. Players are strongly recommended to wait until Act 3 [p. 32].
- **PvP (CAM)**: no specific rules; the group resolves it [p. 32].

---

## 13. Gear, signature item, cash [p. 33]

### 13.1 Starting gear

- CIN: the scenario sets the starting gear [p. 33].
- CAM: **choose two of the starting items** from your career's gear list (see §18). Every career list has **4 lines of "A OR B"** [p. 33, 39–55].
  - Example: Roughneck Nat takes a **cutting torch** (line 1) and a **compression suit** (line 3) [p. 33].
  - AMBIGUOUS (A10): the "OR" implies you take **one item from each of two different lines**. The app should enforce two distinct lines with one option each. The book doesn't explicitly forbid both items of one line, but "OR" excludes it.
- **Every weapon comes with two full reloads** [p. 33]. AMBIGUOUS (A11): this only matters for firearms with magazines. It isn't defined for thrown weapons, the stun baton, or tools such as the cutting torch and bolt gun.
- You are also assumed to have a **uniform or civilian clothing** (free, not listed as an item) [p. 33].
- Quantity items such as "D6 doses …" or "D6 flares": roll D6 at creation. "2 G2 Electroshock Grenades" means 2 grenades [p. 39–55].
- **All carried items must be written on the sheet, one item per row** in the Gear section. If it isn't on the sheet, you don't have it with you [p. 33].
- Vehicles and ships belong to the group, not to one character (CIN scenario or CAM start, see Chapter 7) [p. 33].

### 13.2 Signature item

- Every PC has one **in addition to** normal gear. It is small, of little practical use but sentimental, and says something about the character [p. 33].
- It is **usually tiny**, so it does not encumber. Examples: a patch, a cap, a photo [p. 33]. Each career suggests 3, or you can make one up [p. 39–55].
- **Mechanical use**: **once per Act (CIN) / once per game session (CAM)**, interacting with the signature item reduces your **STRESS LEVEL by one step**. This typically requires a **slow action** ("Use signature item", prerequisite: signature item) [p. 88, 104].
- **Androids don't need a signature item**, because they have no stress [p. 29].
- Character sheet: a separate "Signature Item" field [p. 389].

### 13.3 Starting cash

- CAM: roll for cash per career (the formula is in each career entry; table in §18). Cash rules: Chapter 7 [p. 33, 161].
- Note: **Kid gets $D6 (1–6 dollars), not ×100**. Company Agent and Officer get **$2D6×100**. Everyone else gets **$D6×100** [p. 39–55].
- Context: Chapter 7 lists weekly salary ranges per occupation [p. 162]. The campaign frameworks list weekly salaries: Space Truckers $400–960, Colonial Marines $500–760, Frontier Colonists $500–640 [p. 327–329]. These are not part of character creation.

---

## 14. Encumbrance [p. 34–35]

- **Normal capacity: STRENGTH × 2 regular items** "without problems" [p. 34]. Example: STRENGTH 5 → 10 items [p. 35].
- **Regular item** (roughly small-bag size, a few kg at most) counts as **1** [p. 34].
- **Heavy** items count as **2** and typically take 2 rows on the sheet. Some heavy items count as **3 or 4**, as shown in the gear lists [p. 34].
- **Light** items count as **½**; two light items share one row [p. 34].
- Some light items count as **¼**, written "¼" in the gear lists [p. 34].
- **Tiny** items count as **0** (rule of thumb: it can be hidden in a closed fist). They **must still be listed** on the sheet, in the "Tiny Items" box [p. 34, 389].
- Gear tables give weight numerically: 0 (tiny), 1/4, 1/2, 1, 2, … [e.g. p. 127, 131, 133].
- **Over-encumbered**: you can temporarily carry **up to twice the normal limit, i.e. STRENGTH × 4 items** [p. 34].
  - While over-encumbered you must make a **MOBILITY roll whenever you want to run or crawl in a Round of combat**. On failure you must either **drop what you are carrying, or stay put** [p. 34].
  - More than STRENGTH × 4 is not allowed.
- **Pack Mule** talent: carry twice as many objects as normal without being encumbered, i.e. STRENGTH × 4 [p. 79]. AMBIGUOUS (A12): it isn't stated whether the over-encumbered ceiling also doubles (to STRENGTH × 8). The conservative choice is to keep STRENGTH × 4 as the absolute max.
- **Food and water encumbrance** (each consumable separately) [p. 34–35]:

| Supply rating | Counts as |
|---|---|
| 0 | nothing (no longer encumbers) |
| 1–4 | One item |
| 5–8 | Two items |
| Every further 4 | One more item |

  Formula: items = ceil(rating / 4).
- **Air and power** don't usually count toward encumbrance (they are included in suits and other gear). **External air tanks or batteries can count** as items [p. 34].
- The character sheet has **10 numbered gear rows** [p. 389]. That is enough for STRENGTH 5 × 2 (humans max 10). Android STRENGTH up to 8 → 16 items, so the app must not cap the rows at 10.

---

## 15. Consumables [p. 34–35]

- There are four consumables: **Air, Food, Water, Power** [p. 34].
- They are **not tracked at all times**. Inside a functional ship, station or colony you are assumed to have them. **The GM decides** when resources are scarce and tracking starts [p. 34].
- Each consumable has a **Supply rating** on the character sheet (Air, Food, Water, Power fields). Higher is better [p. 34, 389].
- **Supply roll**: roll **Stress Dice equal to the current Supply rating, max 6 dice**. **For every facehugger (1) rolled, the Supply rating drops by 1** [p. 34].
  - Derived: a Supply roll is not a skill roll, so facehuggers on it do **not** trigger a Panic Roll. The panic trigger is "facehugger on Stress Dice *in a skill roll*" [p. 103–104].
- At **Supply 0** you are **out** of that consumable [p. 34]. The effects are cross-referenced below.
- **Group consumables**: usually tracked per character, but they can be tracked for the whole group depending on the situation. The GM has final say [p. 34].
- **When to roll** (guidelines for the GM) [p. 35]:

| Consumable | Supply roll |
|---|---|
| Air | Every Turn, and after every strenuous activity such as combat or a MOBILITY roll |
| Water | Once per day, and after every strenuous activity such as combat or a MOBILITY roll |
| Food | Once per day |
| Power | Situational, depending on the gear used (Chapter 6, e.g. Power Supply roll after each use of a motion tracker) |

- Things that reduce Supply: **only the Supply roll** (facehuggers). Talents reduce the number of dice:
  - **Calm Breather**: Air Supply roll uses 2 dice fewer than the Supply rating, min 1 die [p. 78].
  - **Light Eater**: Food Supply roll uses 2 dice fewer than the Supply rating, min 1 die [p. 78].
  - AMBIGUOUS (A13): the order of the "max 6 dice" cap and the "−2" is not stated. Recommended: dice = max(1, min(rating, 6) − 2). The alternative, min(6, max(1, rating − 2)), differs at rating ≥ 7.
- No starting Supply ratings are given for characters. AMBIGUOUS (A14): Air comes from the suit (e.g. Mk.50 "Air Supply 5", Mk.35 "Air Supply 4", gear spec p. 127–128), Power from the gear item (e.g. "Power Supply 5" when fully charged). Food and Water are set by the GM or scenario.
- Effects of running out (cross-reference only; details in the conditions/hazards spec):
  - Food: **Starving** after a day without sufficient food [p. 106].
  - Water: **Dehydrated** after a day without sufficient water [p. 106].
  - Sleep: **Exhausted** after a day without sufficient sleep. The sleep need is 1 Shift per day, or 1 Shift every two days with Light Sleeper [p. 78, 106].
  - Air: **Suffocation**. STAMINA roll every Turn or after each strenuous activity, cumulative −1 per roll. Failure → 0 Health and a Death Roll every Round [p. 110].
  - Going without sleep, food or water also gives +1 STRESS LEVEL [p. 103].
  - **Androids need no air, food, water or sleep** (§20) [p. 111].

---

## 16. Developing your character — Experience (CAM) [p. 35–36]

### 16.1 Earning XP

- XP is awarded **after the end of each game session**. The group discusses and answers the questions below; each "yes" gives **1 XP** [p. 35]:

| # | Question | XP |
|---|---|---|
| 1 | Did you participate in the game session? (1 XP just for being there) | 1 |
| 2 | Did you risk or sacrifice something to further your Personal Agenda? | 1 |
| 3 | Did you risk your life for your Buddy PC? | 1 |
| 4 | Did you challenge or stand up to your Rival PC? | 1 |
| 5 | Did you make a Panic Roll? | 1 |
| 6 | Did you overcome a dangerous event, violently or non-violently? | 1 |
| 7 | Did you make a significant discovery or revelation? | 1 |
| 8 | Did you perform an extraordinary action of some kind? | 1 |
| 9 | Did you earn any money? | 1 |

- **Max 1 XP per question** → **max 9 XP per session** [p. 35].
- If the answer isn't clear-cut, the group seeks consensus; otherwise **the GM has final say**. Record XP on the sheet [p. 35].
- Derived: androids can never answer "yes" to #5, since they never make Panic Rolls [p. 29, 35].
- Character sheet: 10 XP boxes [p. 389]. AMBIGUOUS (A15): no XP cap is stated. Unspent XP presumably carries over.

### 16.2 Spending XP

- XP can **only be spent between sessions** [p. 36].

| Purchase | Cost | Requirements / limits | Page |
|---|---|---|---|
| Raise a skill by 1 level | **5 XP** | Skill max 5 | [p. 36, 28] |
| Learn a new skill (0 → 1) | **5 XP** | One of: **succeeded with the skill at level 0 this session**, or **≥ 1 Shift of teaching** by someone with skill level ≥ 1 | [p. 36] |
| Learn a talent | **5 XP** ("always") | Requires **a day of practice** and a **successful WITS roll (attribute only)**. **One attempt per day.** With a **teacher who has the talent** the roll **succeeds automatically**. Career talents are only for your own career | [p. 36, 74] |

- AMBIGUOUS (A16): it is unclear whether the "used & succeeded / teacher" prerequisite also applies to raising an existing skill. The text places it after "Learning a new skill…" and the teacher minimum of "skill level 1" only makes sense for new skills. Recommended: it applies to new skills only.
- AMBIGUOUS (A17): talent learning needs a WITS roll after a day of practice, but XP is spent between sessions. The book doesn't say whether the 5 XP is lost on a failed roll, or whether you retry the next day without paying again. Recommended: pay once and retry daily until success, with the GM adjudicating downtime.
- AMBIGUOUS (A18): it isn't stated whether "attribute only" rolls (talent learning, Resilient, the post-session EMPATHY trauma roll) include Stress Dice or can be pushed. The coup de grâce rule explicitly says "attribute only, without Stress Dice" [p. 99]. Resilient is explicitly un-pushable [p. 76].
- Repeatable talents: **Like the Back of Your Hand** (once per vehicle) and **Weapon Specialist** (once per weapon type) [p. 76, 79]. Each additional instance costs 5 XP (derived). All other talents can be taken once.
- **Attributes cannot be raised** (no rule) — see A2.

### 16.3 Other between-session steps

- **Change Buddy and Rival** freely after a session [p. 36].
- **Permanent mental trauma** check: if you rolled **13+ on a Panic Roll during the session**, make an **EMPATHY roll (attribute only)** after the session. DECIDED (A19): if that roll **fails**, you develop a permanent mental trauma; roll D6 on the table on p. 101 [p. 104]. Source: publisher's FAQ and later printings. Note: the 1st printing has trauma on a success. Full rules belong to the panic spec.

---

## 17. Pushing-twice and attribute-swap talents — application notes

- "Push any skill roll based on X twice" (Past the Limit, True Grit: STRENGTH; Cunning, Inquisitive: WITS; Nimble, Reckless: AGILITY; Compassion, Influence: EMPATHY). **Each push adds +1 STRESS LEVEL**, so 2 pushes = +2 [p. 74–77].
- A skill is "based on" its own attribute (§6.1). AMBIGUOUS (A20): it is unclear whether an attribute substitution changes this. Examples: Stoic (STAMINA with WITS) + Cunning, Take Control (MANIPULATION with WITS) + Inquisitive, Menacing (MANIPULATION with STRENGTH) + True Grit, Flyweight (block with AGILITY). Recommended: base it on the attribute actually rolled.
- A second push is still forbidden if a facehugger appeared, since that forces a Panic Roll [p. 60–61]. Androids can't push at all [p. 29].

---

## 18. The nine careers [p. 38–55]

### 18.1 Summary

| Career | Key attribute | Key skills | Career talents | Starting cash | Page |
|---|---|---|---|---|---|
| Colonial Marine | STRENGTH | Close Combat, Stamina, Ranged Combat | Banter, Overkill, Past the Limit | $D6×100 | [p. 39] |
| Colonial Marshal | WITS | Observation, Ranged Combat, Manipulation | Authority, Investigator, Subdue | $D6×100 | [p. 41] |
| Company Agent | WITS | Comtech, Observation, Manipulation | Cunning, Personal Safety, Take Control | **$2D6×100** | [p. 43] |
| Kid | AGILITY | Mobility, Survival, Observation | Beneath Notice, Dodge, Nimble | **$D6** | [p. 45] |
| Medic | EMPATHY | Mobility, Observation, Medical Aid | Calming Presence, Compassion, Field Surgeon | $D6×100 | [p. 47] |
| Officer | EMPATHY | Ranged Combat, Command, Manipulation | Field Commander, Influence, Pull Rank | **$2D6×100** | [p. 49] |
| Pilot | AGILITY | ERRATA: Piloting, Ranged Combat, Comtech (printed: Mobility instead of Piloting) | Full Throttle, Like the Back of Your Hand, Reckless | $D6×100 | [p. 51] |
| Roughneck | STRENGTH | Heavy Machinery, Stamina, Close Combat | Resilient, The Long Haul, True Grit | $D6×100 | [p. 53] |
| Scientist | WITS | Observation, Survival, Comtech | Analysis, Breakthrough, Inquisitive | $D6×100 | [p. 55] |

- ERRATA (p. 51): Pilot key skills are **Piloting, Ranged Combat, Comtech**. Note: the printed page lists Mobility instead of Piloting. A21 resolved [p. 51].

### 18.2 Starting gear lists (choose two items; see A10) [p. 39–55]

**Colonial Marine** [p. 39], cash $D6×100

| Line | Option A | Option B |
|---|---|---|
| 1 | M41A Pulse Rifle | M56A2 Smart Gun |
| 2 | M314 Motion Tracker | 2 G2 Electroshock Grenades |
| 3 | IRC Mk.35 Pressure Suit | M3 Personnel Armor |
| 4 | Signal Flare | Deck of cards |

**Colonial Marshal** [p. 41], cash $D6×100

| Line | Option A | Option B |
|---|---|---|
| 1 | .357 Magnum Revolver | Armat Model 37A2 12 gauge pump-action |
| 2 | Binoculars | Hi-beam flashlight |
| 3 | Personal medkit | Stun baton |
| 4 | D6 doses Neversleep pills | Hand radio |

**Company Agent** [p. 43], cash $2D6×100

| Line | Option A | Option B |
|---|---|---|
| 1 | Leather briefcase | Chrome briefcase |
| 2 | Gold-plated pen | Rolex watch |
| 3 | Data transmitter card with corporate clearance level | M4A3 Service Pistol |
| 4 | D6 doses Neversleep | D6 doses Naproleve |

**Kid** [p. 45], cash $D6

| Line | Option A | Option B |
|---|---|---|
| 1 | Fishing line | Laser pointer |
| 2 | Magnet | Radio-controlled car |
| 3 | Yo-yo | Electronic handheld game |
| 4 | Personal locator beacon | Coloring pens |

**Medic** [p. 47], cash $D6×100

| Line | Option A | Option B |
|---|---|---|
| 1 | Surgical kit | IRC Mk.50 compression suit |
| 2 | D6 doses Naproleve | D6 doses Neversleep |
| 3 | Personal medkit | D6 doses experimental X-Drugs |
| 4 | Samani E-Series watch | Hand radio |

**Officer** [p. 49], cash $2D6×100

| Line | Option A | Option B |
|---|---|---|
| 1 | M4A3 Service Pistol | Rexim RXF-M5 EVA Pistol |
| 2 | Samani E-Series watch | Binoculars |
| 3 | M314 motion tracker | IRC Mk.50 compression suit |
| 4 | Seegson P-DAT | IFF transponder |

**Pilot** [p. 51], cash $D6×100

| Line | Option A | Option B |
|---|---|---|
| 1 | M4A3 service pistol | PR-PUT uplink terminal |
| 2 | Hand radio | D6 flares |
| 3 | Maintenance jack | Seegson P-DAT |
| 4 | Seegson system diagnostic device | IRC Mk.50 compression suit |

**Roughneck** [p. 53], cash $D6×100

| Line | Option A | Option B |
|---|---|---|
| 1 | Cutting torch | Watsumi DV-303 bolt gun |
| 2 | D6 doses Hydr8tion | Maintenance jack |
| 3 | Stash of hard liquor | IRC Mk.50 compression suit |
| 4 | Hi-beam flashlight | Seegson C-Series magnetic tape recorder |

**Scientist** [p. 55], cash $D6×100

| Line | Option A | Option B |
|---|---|---|
| 1 | Digital video camera | Hand radio |
| 2 | Seegson P-DAT | Neuro visor |
| 3 | Seegson System Diagnostic Device | Personal Data Transmitter |
| 4 | M314 motion tracker | Personal medkit |

(Item stats, weights and costs: gear spec, Chapter 6.)

### 18.3 Signature item suggestions (choose one or invent one) [p. 39–55]

| Career | Suggestions |
|---|---|
| Colonial Marine | Bullet that you survived; Lost friend's dog tags; Trophy from a defeated enemy [p. 39] |
| Colonial Marshal | Photo of a loved one; Dented flask with an inscription on the front; News clipping of an unsolved case [p. 41] |
| Company Agent | Letter of corporate authorization; Divorce papers; Employee of the Year award [p. 43] |
| Kid | Lunchbox covered in stickers; Favorite doll or action figure; Bracelet made by older sibling [p. 45] |
| Medic | Framed medical certificate; Letter from son or daughter; Last psych evaluation: "All clear at last." [p. 47] |
| Officer | Ship's cat; Letter of recommendation; ICC Commercial Flight Officer license [p. 49] |
| Pilot | Dashboard dancer; Pilot's logbook; Pilot shades [p. 51] |
| Roughneck | Tool belt; Photo of partner; Crucifix or other religious symbol [p. 53] |
| Scientist | Albert Einstein Award; Unfinished scientific paper; Blackmail letters [p. 55] |

### 18.4 Personal Agenda suggestions (CAM; paraphrased; choose one or invent one) [p. 39–55]

| Career | Agenda 1 | Agenda 2 | Agenda 3 |
|---|---|---|---|
| Colonial Marine [p. 39] | Decorated hero: defend your reputation at all costs | You helped cover up a war crime; no one must ever know | Your buddy's death spooked you; you secretly fear combat and must overcome it |
| Colonial Marshal [p. 41] | Your longtime partner betrayed you and joined a crime syndicate; get even | You dream of turning in the badge and settling down; work towards it | A past bad deed has come back to haunt you; decide what you are made of |
| Company Agent [p. 43] | Greedy for power; never miss a chance to get ahead | The Company is withholding information from you; find out what and why | You're a nice guy blackmailed by the Company into dirty work; get even |
| Kid [p. 45] | Find an adult you can truly trust | No family left; make sure you never end up alone again | Nobody gives you anything to do; explore, try things, make your own fun |
| Medic [p. 47] | Addicted to a strong painkiller; protect your stash and your secret | You hold unusual classified medical reports the Company wants; find out why | You swore never to take a life, and you mean it |
| Officer [p. 49] | From an officer family; you need a promotion or award, soon | You messed up before; avoid taking the blame for any more screw-ups | Mistakes are deadly; nobody under your watch screws up, and they must understand why |
| Pilot [p. 51] | Push the limit; take risks | Stubborn; you don't back down even if friends might get hurt | Loner; happier doing tasks without relying on others |
| Roughneck [p. 53] | Compulsive thrill-seeker; if there's a risk, step up | You sacrificed your family for the job; never let your friends down | Downtime matters: Aspen beer (Souta Dry if desperate) and time alone |
| Scientist [p. 55] | Your last project was stolen; keep many findings secret | You hate authority; be uncooperative when possible | You find it hard to delegate, even if it means extra work |

### 18.5 Typical names (3 male / 3 female) and appearance options [p. 39–55]

| Career | Male names | Female names | Appearance options |
|---|---|---|---|
| Colonial Marine | Marcus Mullaney, Nik Elson, Vic Pasengrau | Kimi Diem, Tara Zanelli, Chrissy Lopez | Crew cut; Arm tattoo; Scar; Cold eyes; Cocky grin; Personalized body armor |
| Colonial Marshal | Jack Kitani, Barrell Klein, Ivan Mankov | Akira Kano, Angela Harris, Lee-Ann Jenkins | Chewing a toothpick; Cigarette in mouth; Impressive mustache; Worn cap; Scar across face; Graying hair; Crew cut; Inquisitive gaze; Old leather jacket |
| Company Agent | Conrad Schmidt, Alexander Balconi, Ryan Middlebrook | Michiko Nogumi, Sheridan Lamara, Mercedes Prince | Icy glare; Disarming smile; Expensive Rolex; Unique signet ring; Tanned; Elaborate hairstyle; Emotionless stare; Monogrammed silk tie |
| Kid | Chip Harrington, Hugo Turner, Jakey Myers | Meggie Wu, Maisie Kelly, Becca David | Grubby and unkempt; Trendy light-up sneakers; Torn denim jeans; Band T-shirt; Cargo shorts; Ponytail; Bored expression; Baseball cap |
| Medic | Cho Hadfield, Ken Ibana, Sullivan Ward | Ana Kasnavik, Juno Blanchard, Katie Aberly | Sympathetic smile; Short, tidy hair; Warm, caring eyes; Dark bags under eyes; Fidgeting hands; Calm and gentle voice; Cold, unsympathetic stare; Spectacles; White coat |
| Officer | Eugene Proctor, Oliver Bryant, Lloyd T. Darrington | Wendy Stern, Julia Kwang, Camille Kirschner | Crew cut or tied-back hair; Serious, intense expression; Spotless uniform; Overworked and "strung-out"; Stiff posture; Relaxed and soothing voice; Jumpsuit with mission patch; Impatiently taps a foot |
| Pilot | Casper Edmonton, Sven Stackman, Kiel Avari | Fiona O'Neill, Constance Navona, Igraine Turner | Arrogant walk; Steely blue eyes; Multi-pocket flight suit; Sunglasses; Previous mission patches; Deadpan expression; Chews gum; Skeptical look |
| Roughneck | Mac Masterton, Kip Tranter, Charlie Stead | Sassy Diaz, Kat Longridge, Jayden Pace | Tattoos; Scar; Broken nose; Mirthless eyes; Smirking face; Loud laugh; Bruised and calloused hands; Safety goggles; Filthy clomping boots; Wild hair |
| Scientist | Viggo Kowalski, Drew Lancaster, Travis Torrence | Elena Sanchez, Louise Mallory, Karima Yusef | Unkempt appearance; Stained lab coat; Nervous manner; Hands in pockets; Tidy, well-trimmed hair; Thoughtful gaze; Clears throat before speaking; Bleary, overworked eyes |

Names and appearance are free text. The lists are suggestions only [p. 30].

---

## 19. Talents — complete list [p. 74–79]

General rules:
- Talents have no rating [p. 74].
- Career talents are only available to that career; general talents are available to anyone [p. 74].
- At start (CAM) you get 1 career talent [p. 29, 74].
- Learning a talent later costs 5 XP plus practice and a WITS roll (§16) [p. 36].

### 19.1 Career talents (27)

| Talent | Career | Exact effect | Usage limit / cost / trigger | Page |
|---|---|---|---|---|
| **Banter** | Colonial Marine | Your STRESS LEVEL, and that of **everyone within SHORT range** of you, drops **2 steps instead of 1** for every Turn spent in a safe place (normal rest recovery, p. 104) | Several Marines with Banter **don't stack** | [p. 74] |
| **Overkill** | Colonial Marine | When you make a Panic Roll with a result of **11 or higher** and **enemies are in sight**, you may replace the panic effect with the **Overkill effect**: you must immediately attack your enemies and won't stop until you or **all enemies in sight are Broken**. **All other PCs within SHORT range must make an immediate Panic Roll** | Optional per qualifying Panic Roll | [p. 74, 104] |
| **Past the Limit** | Colonial Marine | Push any skill roll based on **STRENGTH twice** | Each push +1 STRESS LEVEL | [p. 74] |
| **Authority** | Colonial Marshal | Use **COMMAND instead of MANIPULATION** to get someone to bend to your will | — | [p. 74] |
| **Investigator** | Colonial Marshal | When you **spend a Turn in a room** or similar location, roll **OBSERVATION**. **For each success** ask the GM one of: *What happened here?* / *Is there anything hidden here, and if so, where?* / *Are there any details here that are out of place, out of the ordinary?* The GM must answer truthfully but may be vague or incomplete | **Only one attempt** (per location) | [p. 74] |
| **Subdue** | Colonial Marshal | When attacking a **humanoid** opponent in close combat, declare a subdue attempt: **+2** to the attack. On a hit you deal **no damage**; instead you **hold the opponent in a grapple**. **Extra successes have no effect** | Declared before the attack | [p. 74] |
| **Cunning** | Company Agent | Push any skill roll based on **WITS twice** | Each push +1 STRESS LEVEL | [p. 75] |
| **Personal Safety** | Company Agent | If you are attacked or otherwise in fatal danger, and another PC or friendly NPC is within **SHORT range (same zone)**, roll **MANIPULATION** (straight roll, **not opposed, not an action**). On success the other character suffers the attack or hazard instead of you | **Using it: +1 STRESS LEVEL** (on use, regardless of result) | [p. 75] |
| **Take Control** | Company Agent | Roll **MANIPULATION using WITS instead of EMPATHY** | Always | [p. 75] |
| **Beneath Notice** | Kid | When you roll a critical injury on yourself, **re-roll** the dice and **choose the result you prefer** | Each critical injury roll on yourself | [p. 75] |
| **Dodge** | Kid | When attacked in close combat you can dodge. It works like **blocking** (fast action; declared before the attacker rolls; counts against your actions in the Round, p. 92) but you roll **MOBILITY instead of CLOSE COMBAT**, and successes can **only reduce damage** (remove attacker successes; no counterattack or disarm). It can even dodge a creature's **signature attack** | Same action economy as blocking | [p. 75, 92] |
| **Nimble** | Kid | Push any skill roll based on **AGILITY twice** | Each push +1 STRESS LEVEL | [p. 75] |
| **Calming Presence** | Medic | Reduce the STRESS LEVEL of **another** character within **SHORT range by 1**, in addition to normal stress recovery. **No roll** | **Once per Turn**. You and the beneficiary must be in a **relatively safe place**. **Not on yourself** | [p. 75] |
| **Compassion** | Medic | Push any skill roll based on **EMPATHY twice** | Each push +1 STRESS LEVEL | [p. 75] |
| **Field Surgeon** | Medic | **+2 to MEDICAL AID** when treating someone **about to die from a critical injury** | — | [p. 75] |
| **Field Commander** | Officer | Use COMMAND to **give orders as a fast action instead of a slow action**, so you can give orders **twice in the same Round** | — | [p. 76, 71] |
| **Influence** | Officer | Push any skill roll based on **EMPATHY twice** | Each push +1 STRESS LEVEL | [p. 76] |
| **Pull Rank** | Officer | Use COMMAND to order **non-officer** PCs and NPCs **of the same organization**: roll **COMMAND vs the target's MANIPULATION** (opposed). On success the target **must** follow the order, even if it means harm or danger to themselves. **Each roll covers one specific action.** It **cannot stop actions triggered by Panic Rolls**. The **Hothead** talent gives +2 to resist | **+1 STRESS LEVEL each use** | [p. 76, 78, 71] |
| **Full Throttle** | Pilot | **+2 to PILOTING** for any **Accelerate or Decelerate** actions when piloting a spacecraft | — | [p. 76] |
| **Like the Back of Your Hand** | Pilot | Choose **one specific vehicle or spacecraft** (an individual craft, not a type): **+2 to PILOTING** with it | **Can be taken several times**, once per vehicle | [p. 76] |
| **Reckless** | Pilot | Push any skill roll based on **AGILITY twice** | Each push +1 STRESS LEVEL | [p. 76] |
| **Resilient** | Roughneck | **Any time you suffer damage**, roll **STRENGTH (attribute only, no skill)**. **Each success eliminates 1 point of damage**; if all damage is eliminated you suffer none | **Cannot be pushed**; **not an action**. AMBIGUOUS (A22): order relative to the armor roll | [p. 76] |
| **The Long Haul** | Roughneck | **Ignore all facehuggers from a single roll** | **Once per Act (CIN) / once per game session (CAM)** | [p. 76] |
| **True Grit** | Roughneck | Push any skill roll based on **STRENGTH twice** | Each push +1 STRESS LEVEL | [p. 76] |
| **Analysis** | Scientist | **OBSERVATION** roll on a strange or alien artifact or creature you **study for ≥ 1 Turn**. **Per success**, ask one: human or alien? / dead or alive? / age? / purpose? / how it works? / what problems it could cause? GM answers truthfully, possibly vaguely. **Success**: every other PC within SHORT range **STRESS LEVEL −1**; **failure**: other PCs **STRESS LEVEL +1** (p. 103 also lists this as a stress trigger: a Scientist on your team fails Analysis) | Requires ≥ 1 Turn of study. AMBIGUOUS (A23): failure-stress scope is other PCs [p. 77] vs the Scientist's team [p. 103] | [p. 77, 103] |
| **Breakthrough** | Scientist | **Automatically pass an OBSERVATION roll of your choice** without rolling. The GM has final say on whether it can be used for a particular roll (to avoid spoilers) | **Once per game session** | [p. 77] |
| **Inquisitive** | Scientist | Push any skill roll based on **WITS twice** | Each push +1 STRESS LEVEL | [p. 77] |

Note: Compassion and Influence are mechanically identical, as are Past the Limit and True Grit, Cunning and Inquisitive, and Nimble and Reckless. They are different talents tied to different careers [p. 74–77].

### 19.2 General talents (31)

| Talent | Exact effect | Usage limit / cost / trigger | Page |
|---|---|---|---|
| **Bodyguard** | If someone within **SHORT range** is **hit by an attack**, dive in: roll **MOBILITY** (not an action in combat). With **one or more successes you take the hit instead** | **Can be pushed** | [p. 78] |
| **Calm Breather** | Air Supply roll: roll **2 dice fewer** than your Supply rating, **min 1 die** | See A13 | [p. 78] |
| **Counselor** | Use COMMAND to reduce the STRESS LEVEL of **another** character within SHORT range: **each success** reduces it by **one extra step** in addition to the default one step (p. 104) | **Once per Turn**; **not on yourself**. AMBIGUOUS (A24): whether the "default one step" happens on a failed roll or needs the normal rest conditions | [p. 78] |
| **EVA Specialist** | **+2 to HEAVY MACHINERY and COMTECH** rolls **when spacewalking** | — | [p. 78] |
| **Fast Reflexes** | During the initiative draw, draw **two initiative cards**, keep one, and **shuffle the other back** into the deck **before others draw** | Each initiative draw | [p. 78, 87] |
| **Flyweight** | When you **block** in close combat, use **AGILITY instead of STRENGTH** | — | [p. 78] |
| **Hard Hitter** | **+2 to CLOSE COMBAT** if you **sacrifice your fast action** | Per Round | [p. 78] |
| **Healer** | **Healing time of critical injuries is halved** for you | — | [p. 78, 101] |
| **Hidden Stash** | **At session start** gain **1 extra item you pick**, kept hidden on you or in a safe place. Must be plausible for you; GM has final say. With GM consent, you may name the item type later, at a dramatic moment mid-session | Once per session | [p. 78] |
| **Hothead** | **+2 to opposed MANIPULATION rolls** whenever someone tries to **give you orders**. It can also be used to **resist Pull Rank** | — | [p. 78] |
| **Killer** | When your enemy sustains a critical injury, you may **swap the D66 digits** (ones die ↔ tens die) | **Only on humans** | [p. 78, 99] |
| **Light Eater** | Food Supply roll: roll **2 dice fewer** than your Supply rating, **min 1 die** | See A13 | [p. 78] |
| **Light Sleeper** | You only need to sleep **one Shift every two days** instead of one Shift every day | Affects the Exhausted condition (p. 106) | [p. 78] |
| **Machinegunner** | When firing **full auto**, your STRESS LEVEL **does not increase** | — | [p. 78] |
| **Menacing** | Roll **MANIPULATION using STRENGTH instead of EMPATHY** when **threatening** someone. On success the opponent **cannot demand anything in return**, but **can still choose to attack you** instead of giving in | — | [p. 78, 70] |
| **Merciless** | Perform a **coup de grâce without rolling for EMPATHY**. Also **decrease your STRESS LEVEL one step each time you cause an enemy to be Broken** | AMBIGUOUS (A25): see the Cold-Blooded note below | [p. 78, 99] |
| **Nerves of Steel** | **−2 to all Panic Rolls** | — | [p. 78, 104] |
| **Pack Mule** | Carry **twice as many objects** as normal without being encumbered (STRENGTH × 4) | See A12 | [p. 79] |
| **Quick Draw** | **Drawing a weapon doesn't cost an action** | — | [p. 79] |
| **Rapid Fire** | Fire a **pistol or rifle** as a **fast action instead of a slow action**, at **−2** | — | [p. 79] |
| **Rapid Reload** | **Reload as a fast action** instead of a slow action | — | [p. 79] |
| **Second Wind** | When **Broken**, get back up immediately without first aid: roll **STAMINA**; **each success = +1 Health**, and you can keep fighting. **No effect against critical injuries** | **Once per Turn** | [p. 79, 98] |
| **Spaceship Commander** | As **captain** during space combat, draw **two initiative cards** and choose one | Space combat only | [p. 79] |
| **Spaceship Gunner** | As a **gunner** on a spaceship during space combat: **+2 to RANGED COMBAT** | — | [p. 79] |
| **Spaceship Mechanic** | **+2** when using **HEAVY MACHINERY or COMTECH to repair damage to a spacecraft** | — | [p. 79] |
| **Stealthy** | **+2 to MOBILITY** when moving **undetected** | — | [p. 79, 85] |
| **Stoic** | Roll **STAMINA using WITS instead of STRENGTH** | AMBIGUOUS (A26): whether it applies to Death Rolls (which are STAMINA rolls, p. 99) | [p. 79] |
| **Tough** | **Health +2**: max Health = STRENGTH + 2 | Permanent | [p. 79] |
| **Watchful** | **+2 to OBSERVATION** when trying to **spot a sneak attack** | — | [p. 79] |
| **Weapon Specialist** | Choose a weapon model from the Chapter 6 weapon lists: **+2** when using that weapon. **Unarmed** is a valid choice | **Can be taken several times**, once per weapon type. AMBIGUOUS (A27): the text says "model" in one place and "type" in another | [p. 79] |
| **Zero-G Training** | **+2 to MOBILITY rolls in zero-G** | — | [p. 79] |

### 19.3 Talent cross-reference issues

- **Cold-Blooded talent**: the coup de grâce rule says it lets you kill a defenseless enemy without the +1 STRESS LEVEL [p. 99], but **no such talent exists** (not in the talent chapter or index). AMBIGUOUS (A25): treat it as **Merciless**. Open point: Merciless only waives the EMPATHY roll, not explicitly the +1 Stress. Default: skip the roll, still +1 Stress, unless the GM rules otherwise.
- Coup de grâce is defined for human targets ("If it's a human being…") [p. 99].
- Talents that interact with stress or panic do nothing for androids (no STRESS LEVEL, no Panic Rolls, no pushing). See §20.4.

---

## 20. Androids / synthetics as player characters

Terminology: the book uses "android" and "synthetic" interchangeably [p. 29, 111].

### 20.1 Permission and modes

- Android PCs allowed but **rare**. CAM: book advises **agreeing with the whole group first** before making one. No formal GM-approval mechanic [p. 29].
- Usually human-looking. **Any career** allowed. May be openly synthetic or pass as human in secret [p. 29, 111].
- CIN: scenarios may include pre-generated android PCs. Hope's Last Day's Holroyd is a Company android, and "normal android rules apply, see page 29" [p. 371].
- When **a person nearby is revealed to be an android**, (human) characters gain **+1 STRESS LEVEL** [p. 103]. The app may need a "reveal" event for secret androids.

### 20.2 Creation differences

- Attributes: after assigning the 14 points (normal 2–4 / key 5 limits), add **+3 to two attributes of choice**. Those two may reach a **max of 8** [p. 28, 29]. "Synthetics generally have higher attribute scores than humans" [p. 111].
- Health = STRENGTH, **including** the +3 (derived from NPC android blocks: Covert STRENGTH 8 → HEALTH 8; Curious STRENGTH 4 → HEALTH 4; Refurbished STRENGTH 7 → HEALTH 7) [p. 355].
- NPC android attribute totals are all 20 = 14 + 3 + 3 (Covert 8/8/3/1, Curious 4/4/8/4, Refurbished 7/7/3/3) [p. 355]. This is consistent with the PC rule.
- Skills and talents: same as humans (no stated difference) [p. 29].
- **No signature item needed**, because there is no stress [p. 29].
- No STRESS LEVEL to initialize [p. 29].
- Encumbrance: STRENGTH × 2, so up to 16 items at STRENGTH 8 (derived from p. 34).

### 20.3 Play differences

| Rule | Android behaviour | Page |
|---|---|---|
| Skill rolls | Roll normally (attribute + skill), act like humans in combat | [p. 111] |
| Pushing | **Cannot push skill rolls** | [p. 29, 111] |
| Stress | **Don't suffer stress; have no STRESS LEVEL** → never add Stress Dice | [p. 29, 111] |
| Panic | **Never make Panic Rolls** | [p. 29, 111] |
| Ammo | Ammo runs out only on a facehugger on Stress Dice, so **androids don't risk running out of ammo**. This is intentional | [p. 96] |
| Damage / Broken | When Broken (0 Health), roll on the **Critical Injuries on Synthetics** table (**D6**, not D66). **Until System Shutdown it keeps operating**, suffering only the critical injury effects | [p. 29, 111] |
| Further damage | Each further attack inflicting damage on a Broken android → **another critical injury**. **Rolling the same injury again shifts it down one step** (e.g. 4 → 5) | [p. 111] |
| Death | **No Death Rolls, cannot "die"** | [p. 111] |
| Healing | **Androids don't heal.** Repair: **a Shift of work + a COMTECH roll** restores **all lost Health and all critical injuries**, provided there has been no System Shutdown. **They can repair themselves** | [p. 111] |
| After System Shutdown | Can be **reactivated to communicate**: needs a **power source, a Turn of work, and a COMTECH roll**. A "revived" android can then be repaired (above) to working order but **suffers a permanent mental trauma** | [p. 111] |
| Resources | **Don't need air, food, water or sleep.** **Immune to vacuum, cold and disease.** Typically **unaffected by alien pathogens** | [p. 111] |
| Grapple stunt | The close-combat grapple stunt works only on **humans and synthetics**, so androids can be grappled | [p. 66, 92] |
| Ship service | If the crew is in cryo sleep, a synthetic can do the weekly ship service rolls | [p. 179] |
| Creatures | Feral Neomorphs find androids a curiosity and only attack them if provoked | [p. 315] |

**Critical Injuries on Synthetics (D6)** [p. 111]

| D6 | Crit | Effect |
|---|---|---|
| 1 | Ruptured Fluid Pump | The android loses its next slow action |
| 2 | Destroyed Leg Servos | Both legs are non-functional. The android must stay prone and can only crawl |
| 3 | Destroyed Arm Servos | One arm is non-functional. The android can only use one-handed gear |
| 4 | Head Dislocation | Neck severely fractured, head in an unnatural position. **All fast actions become slow actions** |
| 5 | Severe Chassis Breach | Internal organs spill out; the android is **completely immobilized** |
| 6 | System Shutdown | Torn to shreds or crushed. Core systems severely damaged; a **COMTECH roll is required to communicate** with the android |

- AMBIGUOUS (A28): the book doesn't say what happens when a duplicate result shifts past 6, or when 6 is rolled again on a shut-down android. Recommended: cap at 6 (System Shutdown).
- AMBIGUOUS (A29): it is unclear whether results 1–5 are cumulative (several crits active at once). Recommended: yes, all active crits apply until repaired.
- Electric Sheep (lore with mechanical hints): programming an android for violence against humans is illegal. Modern synthetics have behavioral inhibitors; earlier models don't. Black-market bypass packages exist. Long periods without maintenance can cause mental degradation or psychosis [p. 111]. No numeric rules.

### 20.4 Derived consequences for android PCs (for the app)

- XP question 5 ("Did you make a Panic Roll?") can never be earned [p. 35, 29].
- Talents that are useless or partly void for androids (derived from no push / no stress / no panic / no consumables):
  - Past the Limit, True Grit, Cunning, Inquisitive, Nimble, Reckless, Compassion, Influence (push twice);
  - Banter and Calming Presence for self (no stress), and Counselor received;
  - Overkill, Nerves of Steel, The Long Haul (there are no Stress Dice, so no facehuggers);
  - Machinegunner;
  - Calm Breather, Light Eater, Light Sleeper;
  - Merciless (only the stress part is void).
- The Stress costs of Personal Safety and Pull Rank don't apply to androids. AMBIGUOUS (A30): this is an exploit. The GM may forbid these talents or impose another cost.
- Beneath Notice works on synthetic crits (it re-rolls "a critical injury on yourself") [p. 75]. Killer does **not** work against androids (humans only) [p. 78].
- Signature item: optional. Its stress-relief use is meaningless [p. 29, 104].
- Story Points (CIN) work normally for the player (no exception stated) [p. 61].
- Conditions Starving, Dehydrated, Exhausted and Freezing, plus vacuum and suffocation, don't apply (no need for food, water or sleep; immune to vacuum and cold) [p. 111].
- AMBIGUOUS (A31): radiation, drugs (e.g. Naproleve, X-Drugs, Neversleep) and Power consumables are **not addressed** for androids. Facehugger impregnation is only covered by "typically unaffected by alien pathogens". GM discretion.
- AMBIGUOUS (A32): an android has no Stress Dice and never panics, so a Supply roll (which uses Stress Dice) is irrelevant for Air, Food and Water. Power for androids is never mentioned.
- AMBIGUOUS (A33): MEDICAL AID / first aid on a Broken android isn't addressed. Repair is COMTECH-based, and a Broken android keeps acting anyway. The human "on your own: +1 Health after a Turn" rule presumably does not apply ("androids don't heal").
- AMBIGUOUS (A34): coup de grâce against a Broken android isn't defined. The coup de grâce rule says "if it's a human being" [p. 99]. For androids, each damaging attack just adds a crit [p. 111].

---

## 21. Campaign framework → suggested careers [p. 327–329]

| Framework | Suggested careers | Weekly salary | Suggested ship | Page |
|---|---|---|---|---|
| Space Truckers | Company Agent, Medic, Officer, Pilot, Roughneck | $400–960 | CM-88G Bison M-Class Commercial Freighter | [p. 327] |
| Colonial Marines | Colonial Marine, Medic, Officer, Pilot, Roughneck | $500–760 | Conestoga-Class Frigate | [p. 328] |
| Frontier Colonists | Company Agent, Kid, Medic, Pilot, Roughneck, Scientist | $500–640 | CM-90S Corvus G-Class Deep Space Salvage Vessel | [p. 329] |

- These are suggestions, not restrictions. The Colonial Marshal career appears in none of them; the Colonial Marshal Bureau is a suggested *employer* for Frontier Colonists [p. 329].
- Decide early whether the PCs are employees (steady pay, assigned ship) or freelancers (lease or own a ship) [p. 326–327].

---

## 22. Character sheet fields [p. 21, 389]

The blank sheet is on [p. 389] (PDF 393). Fields:
- Name, Career, Appearance
- Personal Agenda (CAM only; in CIN the Agenda is *not* written on the sheet [p. 31])
- Relationships: Buddy, Rival
- Attributes ×4 and Skills ×12 (each skill drawn next to its attribute)
- Talents (free list)
- Health (10 boxes), Stress Level (10 boxes)
- Radiation (10 boxes; see the hazards spec [p. 110])
- Critical Injuries (free list)
- Conditions: Starving, Dehydrated, Exhausted, Freezing (checkboxes)
- Consumables: Air, Food, Water, Power (one value each)
- Weapons (rows with Bonus, Damage, Range)
- Armor + Rating
- Encumbrance (value)
- Gear (10 numbered rows)
- Signature Item
- Tiny Items
- Experience Points (10 boxes)
- Story Points (3 boxes)

There is no cash field on the printed sheet. The app needs one anyway [p. 33].

---

## Ambiguities / Game Mother discretion

| ID | Issue | Recommended default | Page |
|---|---|---|---|
| A1 | Android +3: must the two attributes be distinct? | Yes, two different attributes | [p. 28, 29] |
| A2 | No rule to raise attributes after creation | Attributes fixed after creation | [p. 36] |
| A3 | Must all 10 skill points be spent? | Yes (validate sum = 10) | [p. 28] |
| A4 | No stated max STRESS LEVEL (sheet has 10 boxes) | No hard cap; UI shows ≥ 10 | [p. 103, 389] |
| A5 | Story Points: multiple per roll? Usable on non-skill rolls? Do they cancel panic? | DECIDED: Cinematic only, max 1 per roll, skill rolls only (old default allowed 1+). Still open, default: never cancel a triggered Panic Roll; GM can override | [p. 61] |
| A6 | Starting Story Points for a new player | 0 plus any carried over, max 3 | [p. 61, 383] |
| A7 | CAM agenda "bonus XP" vs XP question #2 | Same single XP, not cumulative | [p. 31, 35] |
| A8 | Changing a Campaign Personal Agenda | Allowed with GM consent, between sessions | [p. 31] |
| A9 | Buddy/Rival mandatory? Impossible with only 2 PCs | Optional fields; must be different PCs | [p. 31] |
| A10 | Gear choice "two of the starting items" with "A OR B" lines | Pick 2 different lines, 1 option per line | [p. 33, 39–55] |
| A11 | "Two full reloads" for non-firearm weapons | Only for weapons that use reloads | [p. 33] |
| A12 | Pack Mule: does the over-encumbered ceiling double too? | No; absolute max stays STRENGTH × 4 | [p. 34, 79] |
| A13 | Calm Breather / Light Eater vs 6-dice cap: order of operations | dice = max(1, min(rating, 6) − 2) | [p. 34, 78] |
| A14 | Starting Supply ratings (Air/Food/Water/Power) not defined for characters | From the suit/gear (Air, Power) or set by the GM (Food, Water) | [p. 34] |
| A15 | XP cap / carry-over (sheet has 10 boxes) | No cap; unspent XP carries over | [p. 35, 389] |
| A16 | Does the "used & succeeded / teacher" prerequisite also apply to raising existing skills? | New skills only | [p. 36] |
| A17 | Talent learning: is XP lost on a failed WITS roll? Timing vs "between sessions" | Pay once, retry once per in-game day; GM handles downtime | [p. 36] |
| A18 | Attribute-only rolls: Stress Dice? Pushable? | No Stress Dice, not pushable (matches coup de grâce and Resilient wording) | [p. 36, 76, 99] |
| A19 | Mental trauma: does it follow a success or a failure of the EMPATHY roll? (1st printing: success) | DECIDED: trauma on a **failed** roll (publisher's FAQ, later printings); threshold stays 13+ | [p. 104] |
| A20 | "Skill roll based on X" when a talent substitutes the attribute | Use the attribute actually rolled | [p. 74–79] |
| A21 | Pilot key skills omit PILOTING (printed: Mobility, Ranged Combat, Comtech) | ERRATA: Piloting, Ranged Combat, Comtech. Resolved | [p. 51] |
| A22 | Resilient: before or after the armor roll? | After armor (it reduces damage actually suffered) | [p. 76] |
| A23 | Analysis failure stress: "other PCs within SHORT range" [p. 77] vs "in your team" [p. 103] | Other PCs within SHORT range | [p. 77, 103] |
| A24 | Counselor "default one step": is it granted on a failed roll? | Default step only under normal rest conditions; extra steps per success | [p. 78, 104] |
| A25 | "Cold-Blooded" talent referenced but doesn't exist. Merciless vs the coup de grâce +1 Stress | Treat as Merciless; skips the EMPATHY roll; +1 Stress still applies unless the GM rules otherwise | [p. 78, 99] |
| A26 | Stoic and Death Rolls (Death Roll = STAMINA roll) | Stoic applies (WITS) | [p. 79, 99] |
| A27 | Weapon Specialist "model" vs "type" | Per weapon model (e.g. M41A) | [p. 79] |
| A28 | Synthetic crit duplicate shift past 6 | Cap at 6 (System Shutdown) | [p. 111] |
| A29 | Synthetic crits cumulative? | Yes, all active until repaired | [p. 111] |
| A30 | Android using stress-costing talents (Personal Safety, Pull Rank) for free | GM discretion; flag in UI | [p. 75, 76, 29] |
| A31 | Androids vs radiation, drugs, facehugger implantation | GM discretion ("typically unaffected by alien pathogens") | [p. 111] |
| A32 | Androids and Power consumable | GM discretion; no tracking by default | [p. 111] |
| A33 | MEDICAL AID / natural +1 Health on androids | Not applicable; only COMTECH repair | [p. 111] |
| A34 | Coup de grâce on Broken androids | Not defined; further damage = another crit | [p. 99, 111] |

Additional areas the book explicitly leaves to the GM:
- Final say on XP answers [p. 35].
- When consumables are tracked, and whether per character or per group [p. 34].
- Supply-roll intervals ("guidelines") [p. 35].
- Hidden Stash items [p. 78].
- Breakthrough use [p. 77].
- Calling PvP (CIN) [p. 32].
- Which item is dropped (panic, not this spec).

---

## Time-related rules

| Trigger / unit | Rule | Page |
|---|---|---|
| Units | **Round** 5–10 s (combat); **Turn** 5–10 min (stealth); **Shift** 5–10 h (recovery). Usually **4 Shifts/day**: Morning, Day, Evening, Night. GM tracks time | [p. 83] |
| Round | Over-encumbered: MOBILITY roll whenever you want to run or crawl in a Round of combat | [p. 34] |
| Round | Field Commander: give orders twice in the same Round | [p. 76] |
| Round | Hard Hitter: sacrifice the fast action this Round for +2 | [p. 78] |
| Round | Synthetic crit 1: loses next slow action. Crit 4: all fast actions become slow | [p. 111] |
| Turn | **Air Supply roll every Turn** (and after strenuous activity) | [p. 35] |
| Turn | Stress −1 per full Turn resting in a safe area. **Banter: −2 per Turn** for you and everyone within SHORT range | [p. 104, 74] |
| Turn | **Calming Presence**: once per Turn | [p. 75] |
| Turn | **Counselor**: once per Turn | [p. 78] |
| Turn | **Second Wind**: once per Turn | [p. 79] |
| Turn | **Investigator**: spend a Turn in the location (one attempt) | [p. 74] |
| Turn | **Analysis**: ≥ 1 Turn of study | [p. 77] |
| Turn | Android reactivation after System Shutdown: a Turn of work + power source + COMTECH | [p. 111] |
| Turn | Health recovery: 1 per Turn of rest (human). A Broken PC alone gets +1 Health after one Turn | [p. 98] |
| Shift | Learning a new skill via a teacher: at least **one Shift** of instruction | [p. 36] |
| Shift | Android repair: **a Shift of work** + COMTECH → all Health and crits restored | [p. 111] |
| Shift | Sleep need: 1 Shift per day (**Light Sleeper**: 1 Shift every two days) | [p. 78, 106] |
| Day | **Water Supply roll once per day** (plus after strenuous activity). **Food Supply roll once per day** | [p. 35] |
| Day | Talent learning: **a day of practice**, **one WITS attempt per day** | [p. 36] |
| Day | Starving / Dehydrated / Exhausted begin after one day without food / water / sleep | [p. 106] |
| Strenuous activity | Air and Water Supply rolls after combat or a MOBILITY roll | [p. 35] |
| Per use (gear) | Power Supply rolls are situational per item (e.g. after each motion-tracker use) | [p. 35, 86] |
| Act (CIN) | New hidden Agenda handout at the **start of each Act** | [p. 31] |
| Act (CIN) | **End of each Act**: GM evaluates Agendas → +1 Story Point (max 3) | [p. 31, 216] |
| Act (CIN) | **Signature item**: −1 Stress, **once per Act** (slow action) | [p. 104, 88] |
| Act (CIN) | **The Long Haul**: once per Act | [p. 76] |
| Scenario end (CIN) | Hope's Last Day: Story Points awarded in the epilogue. Story Points persist for the **next Cinematic scenario** (belong to the player) | [p. 383] |
| Session start | **Hidden Stash**: extra item at the beginning of each session | [p. 78] |
| Session (CAM) | **Signature item**: once per session. **The Long Haul**: once per session | [p. 104, 76] |
| Session | **Breakthrough**: once per game session (both modes; the text says "game session") | [p. 77] |
| Session end (CAM) | XP award: 9 questions, max 1 XP each | [p. 35] |
| Session end (CAM) | Agenda discussion (the agenda XP) | [p. 31] |
| Between sessions (CAM) | **Spend XP** (only then) | [p. 36] |
| Between sessions (CAM) | **Change Buddy/Rival** | [p. 36] |
| Session end | **Mental trauma check** if any Panic Roll of 13+ occurred this session (EMPATHY roll, attribute only; DECIDED: trauma if it fails, see A19) | [p. 104] |
| Creation | Roll cash ($D6×100 / $2D6×100 / $D6) and D6 quantities of starting gear once | [p. 39–55] |

---

## State the app must track

### Per player (not per PC)
- `storyPoints` (0–3, CIN; persists across Cinematic scenarios and PC deaths) [p. 61, 383]
- Current PC assignment (CIN pre-generated selection; replacement after PvP) [p. 26, 32]

### Per character — identity and creation
- `mode`: Cinematic or Campaign (the rules differ) [p. 19]
- `isPregenerated` (CIN) / created (CAM) [p. 26]
- `isAndroid` (bool), `androidRevealed` (bool, for secret androids; a reveal triggers +1 Stress on nearby humans) [p. 29, 103]
- `controller`: player or GM (NPC after PvP or leftover pregen) [p. 26, 32]
- `career` (one of 9) → key attribute, key skills, career-talent pool, gear list, cash formula [p. 38–55]
- `name`, `appearance` (free text) [p. 30]
- `attributes` {STRENGTH, AGILITY, WITS, EMPATHY}, with `baseAttributes` and `androidBonusTargets` [2] kept separately for validation [p. 28–29]
- `skills` {12 skills: 0–5} [p. 28, 58]
- `talents` [list; repeatable entries carry a parameter: vehicle id for Like the Back of Your Hand, weapon model for Weapon Specialist] [p. 76, 79]
- `personalAgenda`:
  - CAM: text, visible [p. 31]
  - CIN: per-Act hidden handout, GM-controlled, visible only to the owning player and the GM [p. 31, 216]
- `buddyId`, `rivalId` (other PCs; must differ) [p. 31]
- `signatureItem` (not required for androids) [p. 29, 33]
- `cash` [p. 33]

### Per character — derived / running values
- `healthMax` = STRENGTH (+2 if Tough). `healthCurrent` (0..max) [p. 30, 79]
- `broken` (bool, Health = 0) [p. 98]
- `stressLevel` (≥ 0; none for androids) [p. 30, 29]
- `encumbranceCapacity` = STRENGTH × 2 (× 2 again with Pack Mule). `encumbranceMax` = STRENGTH × 4. `currentLoad` = Σ item weights + food/water supply items. `overEncumbered` flag [p. 34, 79]
- `criticalInjuries` [list]; human (D66 table, other spec) or synthetic (D6 table), with healing timers (Healer halves them) [p. 78, 111]
- `systemShutdown` (android), `permanentMentalTrauma` (android revived after shutdown; humans via the panic rule) [p. 111, 104]
- `conditions` {Starving, Dehydrated, Exhausted, Freezing} [p. 106]
- `radiation` (10 boxes, permanent marks; hazards spec) [p. 110, 389]
- `daysWithoutSleep` (for Exhausted modifiers and Light Sleeper's 2-day cycle) [p. 78, 106]
- `armor` (rating), `weapons` (bonus, damage, range, reloads) [p. 389, 33]

### Per character — inventory
- `gear[]`: item, weight class (0 / ¼ / ½ / 1 / 2 / 3 / 4), quantity or doses, reloads per weapon (2 at start) [p. 33–34]
- `tinyItems[]` (weight 0, must be listed) [p. 34]
- `hiddenStashUsedThisSession` / pending choice (Hidden Stash) [p. 78]

### Per character (or per group) — consumables
- `supply` {air, food, water, power}: integer ratings. A `tracked` flag per consumable (GM toggles), and a `scope` (individual or group) [p. 34]
- Power is often per gear item (e.g. motion tracker Power Supply), so it may live on items rather than the character [p. 35]

### Per character — usage limits (reset timers)
- `signatureItemUsed` (reset per Act in CIN / per session in CAM) [p. 104]
- `longHaulUsed` (per Act CIN / per session CAM) [p. 76]
- `breakthroughUsed` (per session) [p. 77]
- `calmingPresenceUsedThisTurn`, `counselorUsedThisTurn`, `secondWindUsedThisTurn` [p. 75, 78, 79]
- `investigatorAttempted` (per location) [p. 74]
- `pushesThisRoll` (max 1, or 2 with the attribute-matching push talent; 0 for androids) [p. 60, 74–77, 29]
- `highestPanicThisSession` (to trigger the ≥ 13 mental-trauma check) [p. 104]

### Per character — development (CAM)
- `xpCurrent` (unspent), plus a session XP checklist (9 booleans) [p. 35]
- `skillsUsedSuccessfullyUntrainedThisSession` [set] and `skillsTaughtThisSession` [set, with teacher id and Shift count] (prerequisites for new skills) [p. 36]
- `talentTraining`: in-progress talent, days practised, teacher id (auto-success if the teacher has the talent) [p. 36]

### Per session / scenario (global)
- `currentAct` (CIN 1–3) [p. 31, 216]
- `sessionNumber`, `sessionPhase` (in play / between sessions; XP spending and Buddy/Rival changes are only allowed between sessions) [p. 36]
- `campaignFramework` (Space Truckers / Colonial Marines / Frontier Colonists) [p. 26, 326]
- Time trackers: Round, Turn, Shift, day (consumable rolls and once-per-Turn talents) [p. 83]
- `pvpCalled` (CIN) [p. 32]
