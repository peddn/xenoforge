# Datenformat der Regeldaten

Alle Regelwerte stehen in `data/`, nicht im Code. Dieses Dokument beschreibt das Format. Maßgeblich sind die Rust-Typen in `crates/engine/src/data/` (`model.rs` für die Einträge, `effects.rs` für Effekte und Operationen). Die JSON-Schemas in `data/schema/` werden daraus erzeugt und nie von Hand bearbeitet.

```
data/
├─ rules/    eine JSON-Datei pro Regeltabelle
├─ i18n/     Namen und Texte, de.json und en.json
└─ schema/   erzeugte JSON-Schemas (für Editoren und zur Kontrolle)
```

Die Daten wachsen mit den Phasen. Phase 0 legte einen **Probesatz** an, genug, um jedes Feld und jede Effektart einmal zu benutzen; Phase 1 ergänzte die Talente, die in Würfe und Panik eingreifen. Die übrigen Einträge kommen mit den Phasen, die sie brauchen.

## Dateien

Jede Datei in `data/rules/` hat diese Form:

```json
{
  "$schema": "../schema/skills.schema.json",
  "entries": [
    { "id": "mobility", "page": 58, "attribute": "agility" }
  ]
}
```

Der Eintrag `$schema` ist Pflicht und muss auf das eigene Schema zeigen. Editoren wie VS Code prüfen und vervollständigen damit beim Tippen. Unbekannte Felder sind überall ein Fehler.

| Datei | Inhalt | Wichtige Felder |
| --- | --- | --- |
| `constants.json` | Einzelne Regelwerte, die die Engine braucht: höchstens drei Helfer, sechs Würfel beim Vorratswurf, ein Push, Panikeffekt ab 7, vier Schichten pro Tag | `value` |
| `attributes.json` | Attribute | – |
| `careers.json` | Karrieren | `key_attribute`, `key_skills` (drei), `talents` (die drei Karrieretalente), `cash` (`dice` W6 mal `times` Dollar), `names` (Namensvorschläge wie im Buch), `looks`, `items`, `agendas` (Vorschläge für Aussehen, Signature Item und Agenda; Texte unter `<karriere>.look.<id>`, `.item.<id>`, `.agenda.<id>`, A-11), `gear` (vier Zeilen „A oder B“ mit `item` aus `gear.json` und wahlweise `count` oder `dice` W6, F-60) |
| `skills.json` | Fertigkeiten | `attribute` |
| `actions.json` | Aktionen, auf die Effekte verweisen | `kind`: `slow`, `fast`, `turn` (dauert einen Turn oder länger, außerhalb der Runden), `space_sensor`, `space_pilot`, `space_gunner`, `space_engineer`; `target`, wenn die Aktion auf einen anderen Charakter zielt (Pull Rank) |
| `panic.json` | Paniktabelle und Panikeffekte außerhalb der Tabelle (Overkill) | `roll` (fehlt außerhalb der Tabelle), `cancels_action` (10+), `trauma_check` (13+), `on_apply`, `effects`, `lasts` |
| `traumas.json` | Dauerhafte Traumata | `d6`, `effects`, `ruling` |
| `critical-injuries.json` | Kritische Verletzungen | `d66`, `fatal`, `instant_death`, `healing`, `on_apply`, `effects` |
| `conditions.json` | Zustände wie Freezing | `effects`, `ends` |
| `gear.json` | Ausrüstung aus Kapitel 6 und Karrieregegenstände ohne Werte (F-64) | `category`; `weight` als `"0"`, `"1/4"`, `"1/2"`, `"1"` bis `"4"`, fehlt bei nicht Tragbarem (F-63); `cost` in Cent mit `up_to_cents` für Spannen; `weapon` (`skill`, `bonus`, `damage`, `blast`, `stun`, `fire`, `range`, `features`, `reloads` nach F-61), `armor` (`rating`, `vacuum`), `supply` (`air` oder `power` mit `max`), `skills` (eigene Fertigkeitsstufen), `vehicle`, `effects` (nur Modifikatoren: bei Anzügen wirken sie getragen und brauchen keine Lage, sonst werden sie im Würfeldialog angeboten, F-59), `single_use` (nach dem Einsatz in einer Probe verbraucht, F-68) |
| `talents.json` | Talente | `career` (fehlt bei allgemeinen Talenten), `effects`; `parameter` (`vehicle` oder `weapon`) und `repeatable`, wenn das Talent etwas Eigenes benennt und mehrfach genommen werden kann; `later` (`gear`, `combat`, `time`, `space_combat`), solange der Teil der App fehlt, der das Talent wirken lässt. Ein Talent hat entweder `effects` oder `later` |
| `ship-systems.json` | Kernkomponenten, Module, sonstige Systeme | `category`: `core`, `module`, `other`; `effects` |
| `ship-damage.json` | Komponentenschäden | `table`: `minor`, `major`; `roll`, `system`, `repair`, `ruling`, `on_apply`, `effects` |
| `rulings.json` | Festlegungen F-NN aus dem Katalog | `pages`, `options`, `default` |

### Ids und Seiten

- `id` ist über **alle** Dateien eindeutig, kleingeschrieben mit Unterstrichen (`sprained_ankle`). Festlegungen heißen wie im Katalog (`F-02`).
- Jeder Eintrag nennt die Buchseite (`page`, bei Festlegungen `pages`), gedruckte Seitenzahl der 1. Auflage.

### Würfelbereiche

`roll` ist `{ "from": 16, "to": 16 }`. Ohne `to` ist der Bereich nach oben offen (Panik 15+). Auf D66-Tabellen sind beide Enden D66-Ergebnisse. Die Prüfung verlangt bei der Paniktabelle Lückenlosigkeit ab 1 und ein offenes Ende, bei Schadenstabellen gültige Würfe ohne Überschneidung. Ein Panikwurf unter 1 (möglich mit Nerves of Steel) zählt zum ersten Bereich, denn weniger als „keine Wirkung“ gibt es nicht. Einträge ohne `roll` stehen außerhalb der Tabelle und sind nur über eine Ersetzung erreichbar; die Prüfung verlangt, dass eine sie benutzt.

### Heilung

`healing` bei kritischen Verletzungen: `"none"`, `{ "days_d6": 2 }` (2D6 Tage), `"until_medical_aid"` oder `"permanent"`. Daraus ergibt sich die Dauer der Effekte der Verletzung.

## Texte

Texte stehen in `data/i18n/de.json` und `en.json` als flache Schlüssel:

- `<id>.name`: Name, Pflicht für jeden Eintrag.
- `<id>.desc`: Kurzbeschreibung in eigenen Worten, freiwillig.
- Weitere Schlüssel für Hinweise und Fragen beginnen mit der Id des Eintrags (`phobia.hint`, `alcoholism.question`).
- Festlegungen: `F-NN.name` und ein Schlüssel je Option (`F-02.double`).

Beide Sprachen haben genau dieselben Schlüssel. Ein Schlüssel, den kein Eintrag benutzt, ist ein Fehler. Keine Sätze aus dem Buch übernehmen, siehe [Werkzeug](#wortlaut-prüfen).

## Effekte

Ein **Effekt** wirkt, solange seine Quelle aktiv ist. Eine **Operation** (`op`) passiert einmal: beim Anwenden (`on_apply`) oder wenn ein Trigger oder ein periodischer Effekt auslöst. Die Grundidee steht im Plan, Abschnitt 4.4: Grundwerte werden nie verändert; Effekte liegen darüber und werden bei jeder Berechnung ausgewertet.

Gemeinsame Felder aller Effekte:

| Feld | Bedeutung | Standard |
| --- | --- | --- |
| `kind` | Effektart, siehe unten | Pflicht |
| `target` | `bearer` (Träger der Quelle), `ship`, `aboard` (alle an Bord) | `bearer` |
| `lasts` | Dauer, siehe unten | ergibt sich aus der Quelle |
| `gm_only` | nur für die Spielleitung sichtbar | `false` |

Die acht Effektarten:

| `kind` | Wirkung | Felder | Beispiel |
| --- | --- | --- | --- |
| `modifier` | ändert Würfelpool, Push-Anzahl, einen Wert des Charakters oder Schiffswert | `applies_to` (`skills`, `based_on`, `rolls`, `stats`, `ship_stats`), `modifies` (`dice` oder `pushes`), `change` (`add`, `multiply`, `divide`), `when` | Tremble: −2 auf alles unter AGILITY. Past the Limit: ein Push mehr bei STRENGTH. Tough: volle Health +2 |
| `restriction` | verbietet etwas | `blocks`: `{ "action": "run" }`, `"stress_relief"`, `"health_recovery"` | Sprained ankle: kein Rennen |
| `trigger` | Operationen bei einem Ereignis | `on`: `{ "skill_roll": [...] }`, `{ "action": "run" }`, `"sleep"`; `then` | Crotch hit: 1 Schaden bei jeder MOBILITY-Probe |
| `periodic` | Operationen bei jedem Zeitschritt | `every`: `{ "step": "shift" }` oder `{ "by_cold": {...} }`; `then` | Alcoholism: Frage jede Schicht |
| `replacement` | ersetzt etwas („stattdessen“) | `replaces`, `with`, `uses`, `optional` | The Long Haul, Overkill, Breakthrough, Stoic, Authority |
| `condition` | legt einen Zustand an | `condition`, `cold`, `unless_wearing` | Life support offline: Freezing |
| `capability` | ermöglicht eine Aktion | `grants` | Docking Umbilical: Andocken |
| `hint` | Hinweis für die Spielleitung | `text`, `tier`: `hint` (handeln) oder `info` (ohne Spielfolge) | Intercoms disabled |

**Modifikatoren im Einzelnen.** `skills` und `based_on` gelten für Proben mit einer Fertigkeit, auch den Death Roll (eine STAMINA-Probe). Maßgeblich sind Fertigkeit und Attribut, mit denen tatsächlich gewürfelt wird (F-35). `rolls` nennt andere Wurfarten: `panic` (verändert das Ergebnis), `air_supply`, `food_supply`, `water_supply`, `power_supply` (verändern die Zahl der Stresswürfel, mindestens einer, Reihenfolge nach F-09). Multiplizieren und Teilen gibt es nur für Schiffswerte und die Traglast, und die Traglast wird nur multipliziert (Pack Mule); `pushes` nur für Proben mit Fertigkeit. `at_least` setzt eine Untergrenze für den veränderten Wert, etwa „mindestens ein Würfel“ bei Calm Breather. `stats` nennt Werte des Charakters: `health_max` (volle Health, sonst STRENGTH) und `carry_capacity` (normale Traglast, sonst STRENGTH × 2, höchstens STRENGTH × 4, F-62). `when` ist der Textschlüssel einer Lage, die die App nicht sieht, etwa „bewegt sich unbemerkt“ bei Stealthy: Der Modifikator gilt nur, wenn die Person sie im Würfeldialog ankreuzt (A-10).

**Ersetzungen im Einzelnen.** Zu jeder Art von `replaces` passt genau eine Art von `with`:

| `replaces` | `with` | Bedeutung | Beispiel |
| --- | --- | --- | --- |
| `"facehuggers_in_roll"` | `"ignore"` | Facehugger des Wurfs zählen als leer | The Long Haul |
| `{ "panic_result": { "from": 11 } }` | `{ "panic_effect": "<id>" }` | statt des Paniktabellen-Effekts ab diesem Wert ein Eintrag außerhalb der Tabelle | Overkill |
| `{ "roll": "<fertigkeit>" }` | `"success"` | die Probe gilt ohne Würfeln als bestanden | Breakthrough |
| `{ "attribute_for": "<fertigkeit>" }` | `{ "attribute": "<attribut>" }` | anderes Attribut für diese Fertigkeit | Take Control, Menacing, Stoic |
| `{ "skill": "<fertigkeit>" }` | `{ "skill": "<fertigkeit>" }` | andere Fertigkeit | Authority |

Mit `optional` entscheidet die Person, ob sie die Ersetzung nutzt. Dazu gehören Bedingungen, die die App nicht kennt, etwa „Feinde in Sicht“ bei Overkill oder „beim Drohen“ bei Menacing; `uses` begrenzt das pro Akt oder Sitzung (`act_or_session`: pro Akt im Cinematic-Spiel, pro Sitzung in der Kampagne).

Die Stufen AUTO, HINWEIS und INFO aus dem Plan (Abschnitt 7.4) ergeben sich daraus: Alle Effektarten außer `hint` wendet die App selbst an (AUTO), `hint` erscheint je nach `tier` als HINWEIS oder INFO.

### Operationen

| `op` | Wirkung | Felder |
| --- | --- | --- |
| `stress` | STRESS LEVEL ändern | `delta`, `who` |
| `damage` | Schaden ohne Rüstung | `amount`, `who` |
| `roll` | Probe würfeln und verzweigen | `roll`: `{ "skill": ... }` oder `{ "attribute": ... }`; `on_success`, `on_fail` |
| `panic_roll` | Panikwurf | `who` |
| `lose_next_action` | nächste Aktion entfällt | `action`: `slow` |
| `drop_item` | Gegenstand fallen lassen | `chosen_by`: `gm` |
| `block` | etwas für eine Weile verbieten | `blocks`, `lasts` |
| `ask` | Ja/Nein-Frage an Spieler oder Spielleitung | `question`, `on_no` |
| `hint` | Hinweis ins Protokoll | `text` |
| `special` | benannte Funktion der Engine für Einzelfälle | `handler` |

`who` ist `bearer` (Standard), `friendly_pcs_short_range`, `friendly_who_hear`, `friendly_who_see`, `friendly_who_witness` oder `aboard`. Die App kennt keine Positionen; Gruppen, die von Entfernung, Hören oder Sehen abhängen, schlägt sie vor und die Spielleitung bestätigt.

`special` ist für Regeln, die sich mit dem Vokabular nicht sauber ausdrücken lassen. Jeder Name muss in der Engine existieren (`SPECIAL_HANDLERS`). Bisher gibt es `pull_rank`: der vergleichende Wurf von Pull Rank, COMMAND gegen die MANIPULATION des Ziels (S. 76); das Ziel nennt der Befehl, der die Aktion auslöst.

### Dauer

`lasts` ist eines von:

- `"instant"`: nur die einmaligen Operationen.
- `{ "steps": { "unit": "round", "count": 1 } }`: Schritte einer Zeiteinheit (`round`, `turn`, `shift`, `day`).
- `{ "until": "panic_stops" }`: bis zu einem Ereignis (`panic_stops`, `healed`, `medical_aid`, `repaired`, `warm`, `day_change`).
- `"permanent"` oder `"while_source"`.

Fehlt `lasts` an einem Effekt, gilt die Dauer der Quelle:

| Quelle | Dauer |
| --- | --- |
| Panikergebnis | `lasts` des Ergebnisses |
| Kritische Verletzung | aus `healing` |
| Zustand | bis `ends` |
| Schiffsschaden | bis repariert |
| Talent, Trauma, Schiffssystem | solange die Quelle besteht |

## Festlegungen

Einträge mit einer umschaltbaren Festlegung tragen `ruling`:

```json
"effects": [ { "kind": "modifier", "applies_to": { "ship_stats": ["ftl_rating"] }, "change": { "multiply": 2 } } ],
"ruling": {
  "id": "F-02",
  "alternatives": {
    "halve": [ { "kind": "modifier", "applies_to": { "ship_stats": ["ftl_rating"] }, "change": { "divide": 2 } } ]
  }
}
```

Die eigenen `effects` gelten für die Standardoption der Festlegung, `alternatives` enthält die Effekte für jede andere Option. Die Prüfung verlangt genau diese Optionen.

`rulings.json` und die Tabelle in `docs/rules/00-errata-und-festlegungen.md` müssen übereinstimmen: gleiche Ids in gleicher Reihenfolge, gleiche Seiten, eine Option ohne Alternative, zwei mit.

## Prüfungen

```
cargo test -p xenoforge-engine
```

prüft neben dem Schema:

- eindeutige Ids im richtigen Muster, Seiten im Bereich des Buchs;
- alle Verweise (Fertigkeiten, Attribute, Aktionen, Zustände, Schiffssysteme, Karrieren, Festlegungen);
- Texte: jeder Name und jeder benutzte Schlüssel in beiden Sprachen, keine unbenutzten Schlüssel, keine leeren Texte;
- Würfelbereiche der Tabellen;
- jede Effektart kommt in den Daten mindestens einmal vor;
- Festlegungen gegen den Katalog;
- die Schemas in `data/schema/` sind aktuell.

Nach einer Änderung an den Rust-Typen die Schemas neu erzeugen:

```
XENOFORGE_UPDATE_SCHEMAS=1 cargo test -p xenoforge-engine schemas
```

## Wortlaut prüfen

`tools/verbatim-check.mjs` sucht Stellen, die mit dem Buch wörtlich übereinstimmen. Es braucht ein eigenes Exemplar als Text, das nie ins Repo gehört:

```
pdftotext alien-rpg-core-rulebook.pdf ~/alien-book.txt 2>/dev/null
node tools/verbatim-check.mjs --book ~/alien-book.txt
```

Ohne weitere Angaben prüft es `data/i18n`, `docs` und `README.md` und meldet jede Zeile mit 12 oder mehr aufeinanderfolgenden Wörtern aus dem Buch. Tabellenzeilen mit Namen und Zahlen können dabei auffallen, ohne Fließtext zu enthalten; diese bitte von Hand ansehen.
