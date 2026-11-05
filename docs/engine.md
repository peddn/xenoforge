# Regel-Engine

Die Regel-Engine (`crates/engine`) ist eine reine Bibliothek: kein Netz, keine Datenbank, keine Oberfläche. Sie bekommt einen Spielzustand, einen Befehl und eine Würfelquelle und liefert Ereignisse zurück. Gewürfelt wird nur über die Würfelquelle; der Server setzt in Phase 2 eine echte Zufallsquelle ein, Tests nutzen feste Würfe (`ScriptedDice`). Der Plan dazu steht in `docs/PLAN.md`, Abschnitt 4.

```rust
let engine = Engine::new(RuleData::load(Path::new("data/rules"))?);
let events = engine.apply(&mut state, Command::Roll { request }, &mut dice)?;
```

- **Ein Befehl, eine Antwort.** `apply` ändert den Zustand und gibt die Ereignisse zurück. Scheitert der Befehl, bleibt der Zustand unverändert.
- **Entscheidungen.** Wo das Buch eine Person entscheiden lässt, legt die Engine eine offene Entscheidung (`Pending`) an und meldet sie als Ereignis. Weiter geht es mit `Command::Decide`.
- **Grundwerte bleiben.** Attribute, Fertigkeiten und Talente ändern sich nie durch Effekte. Jede Frage („welche Modifikationen gelten für diesen Wurf?“) wird jedes Mal neu aus den aktiven Quellen beantwortet, mit Angabe der Quelle (`injury/broken_ribs`).
- **Speicherbar.** Der ganze Zustand, auch ein halb fertiger Wurf, lässt sich als JSON speichern und laden.

## Befehle

| Befehl | Wirkung | Seite |
| --- | --- | --- |
| `roll` | Probe: Fertigkeit, nur Attribut, Death Roll, fester Pool, Vorratswurf (Food, Water, Air des Anzugs), Vorrat eines Gegenstands, einzelne Stresswürfel | 34, 58–64, 99, 110 |
| `opposed` | Vergleichender Wurf; nur der Angreifer darf pushen | 64, F-22 |
| `decide` | Antwort auf eine offene Entscheidung | – |
| `panic_roll` | Panikwurf auf Ansage der Spielleitung, etwa nach kritischer Verletzung | 104 |
| `change_stress` | Stress ±N für ausgewählte Charaktere | 211 |
| `damage` | Erlittener Schaden nach Rüstung; Stress +1; Health 0 heißt Broken | 98, 103, F-33 |
| `apply` | Trauma, Zustand oder kritische Verletzung anlegen | 100–101, 106–107 |
| `end` | Ereignis, das Quellen beendet: MEDICAL AID, warm, repariert, geheilt | – |
| `remove` | Quelle von Hand entfernen (Korrektur) | – |
| `stop_panic` | Panik von Hand beenden, etwa Berserk, wenn das Ziel Broken ist | F-29 |
| `step` | Zeitschritt: Runde, Turn oder Schicht, ggf. mit neuem Tag | Plan 9.1 |
| `sleep` | Diese Charaktere schlafen (für Nightmares) | 101 |
| `perform_action` | Aktion ausführen, wahlweise mit Ziel (Pull Rank): prüft Sperren und Fähigkeiten, löst Auslöser aus | 74–77 |
| `use_signature_item` | Stress −1, einmal pro Akt bzw. Sitzung, gesperrt wie anderer Stressabbau | 104, F-31 |
| `end_act` | Akt beenden (nur Cinematic, höchstens drei Akte): Akte zählen, Zähler pro Akt zurücksetzen, ein Story Point für jeden Spieler, dessen Charakter seiner Agenda gefolgt ist, höchstens drei | 61, 104, 216, A-13 |
| `reveal_android` | Verdeckten Android enthüllen: offen; die Spielleitung wählt die Menschen in der Nähe, sie bekommen Stress +1 | 103, A-14 |
| `set_story_points` | Story Points eines Spielers setzen (nur Cinematic), etwa Mitgebrachte | 61, 383, F-53 |
| `end_session` | Zähler zurücksetzen; danach die Trauma-Probe | 104 |
| `add_item` | Gegenstand geben: aus dem Katalog (tragbar, F-63) oder frei mit Name und Gewicht; mehr als die Höchstlast geht nicht | 33–34, 118 |
| `move_item` | Gegenstand mitnehmen, anziehen (nur Anzüge, einer auf einmal), verstauen, ablegen oder einem anderen Charakter geben, wahlweise nur einen Teil des Stapels | 33–34, 98 |
| `use_item` | Gegenstand zur Hand benutzen: Dosis, Essen, Getränk oder Gerät mit Energie | 34, 134–139 |
| `refill_item` | Gerät aufladen oder Luft im Anzug auffüllen, voll oder auf einen Wert | F-75 |
| `book` | Geld buchen: Lohn, Belohnung, Ausgabe, Kredit, in Cent mit Grund; nie unter null | F-76 |
| `buy_item` | Gegenstand verkaufen: Katalogpreis je Stück (bei Spannen der untere Wert) oder eigener Preis, dann wie `add_item` | 118, F-76 |
| `buy_reloads` | Magazine für eine Schusswaffe verkaufen, je etwa 5 % der Waffe oder eigener Preis | 124, F-61 |

## Ablauf einer Fertigkeitsprobe

1. **Pool bauen:** Wer Broken ist, kann keine Fertigkeitsprobe machen, der Death Roll bleibt erlaubt [S. 98–99]. Attribut + Fertigkeit + Modifikationen (Schwierigkeit und freier Ausrüstungsbonus vom Aufrufer, Hilfe höchstens +3, Effekte mit Quelle, gewählte Gegenstände). Stresswürfel = Stresslevel, nur bei menschlichen Spielercharakteren. Ein negativer Überhang nimmt Stresswürfel weg [S. 62]. Vorab gewählte Talente tauschen Attribut oder Fertigkeit, bestehen die Probe ohne Wurf (Breakthrough) oder geben ihren Lage-Bonus (A-10): Talente wie Stealthy wirken nur, wenn sie für den Wurf angekreuzt sind, und nur bei passender Fertigkeit. Die Vorschau nennt alle passenden; verlangt ein Effekt die Probe, bietet die Engine sie wie Stoic vorher an.
2. **Würfeln.** Grundwürfel zuerst, dann Stresswürfel.
3. **Facehugger** auf Stresswürfeln: Mit The Long Haul fragt die Engine, ob es eingesetzt wird. Sonst gibt es keinen Push mehr, und es folgt sofort der Panikwurf; ab 10 ist die Aktion abgebrochen (F-44 für 1–9).
4. **Push oder Story Point anbieten**, solange erlaubt. Ein Push kostet zuerst Stress +1, dann bleiben die 6er liegen, ein neuer Stresswürfel kommt dazu, und der Rest wird neu geworfen (F-20).
5. **Ersatzfertigkeit:** Mit Authority wird COMMAND gewürfelt, samt den Modifikatoren für COMMAND; die Aktion bleibt die angefragte Fertigkeit (MANIPULATION) mit deren Gegenwurf, Folgen und Stunts (F-49). Die Stunt-Listen selbst kommen mit Phase 2.
6. **Ergebnis:** ab einer 6 Erfolg, jede weitere 6 ein Stunt; beim vergleichenden Wurf zählt die Differenz, Gleichstand verliert der Angreifer [S. 64, 70]. Danach laufen Auslöser wie Crotch hit, und eine COMMAND-Probe zum Beenden einer Panik wirkt (F-36).

## Ausrüstung

Jeder Charakter hat eine Liste von Gegenständen (`Possession`) mit fester Nummer, Anzahl, Ort (dabei, angezogen, verstaut), Magazinen und Vorrat [S. 33]. Gegenstände kommen aus `gear.json` oder sind frei mit Name und Gewicht.

- **Traglast** (`Engine::encumbrance`) in Vierteln eines normalen Gegenstands: was dabei oder angezogen ist, dazu Nahrung und Wasser je angefangene 4 Vorrat (F-69); Verstautes und Magazine zählen nicht (F-70). Normal sind STRENGTH × 2, Pack Mule verdoppelt; mehr als STRENGTH × 4 geht nie (F-62) [S. 34, 79]. Wer über der normalen Last liegt, ist überladen; die MOBILITY-Probe dafür kommt mit dem Kampf. Wer, etwa nach weniger STRENGTH, über der Höchstlast liegt, kann noch ablegen und verstauen, aber nichts dazunehmen.
- **Anziehen:** nur Anzüge, immer einer; aus einem Stapel wird einer abgeteilt. Reservemagazine gehören zum ganzen Stapel und bleiben beim Rest, wenn ein Teil abgeht.
- **Stapel:** Gleiche Gegenstände am selben Ort bilden einen Stapel: derselbe Katalogeintrag, bei freien Gegenständen gleicher Name und gleiches Gewicht. Was dazukommt, übergeben oder zurückgelegt wird, landet im vorhandenen Stapel; Magazine zählen zusammen. Dinge mit eigenem Vorrat (Luft, Energie) und der angezogene Anzug bleiben einzeln. Dasselbe Ziel noch einmal (etwa „dabei“ für etwas, das schon dabei ist) lehnt die Engine ab.
- **Angezogener Anzug** wirkt von selbst: seine Modifikatoren zählen (Mk.50 −1 auf AGILITY-Proben, F-66), und ein luftdichter schützt vor Zuständen wie Life support offline (F-18).
- **Gegenstände im Wurf** (F-59): Die Vorschau bietet alles an, was dabei ist und zur Probe passt, mit Wert und Lage. Gewählte Gegenstände (`items` in der Anfrage) zählen je einmal, gleiche Gegenstände zusammen nur einmal (F-71); eine Waffe nur mit ihrer Fertigkeit und höchstens eine. Nur bei Fertigkeitsproben. Das persönliche Medkit ist danach verbraucht (F-68).
- **Talente:** Pack Mule über die Traglast, Weapon Specialist als Lage-Bonus mit der gewählten Waffe (A-10), Hidden Stash als Hinweis an die Spielleitung.
- `Engine::check_items` prüft von Hand gesetzte Gegenstände nach denselben Regeln; `check_overload` lehnt neue Werte ab, die das Überladen verschlimmern, etwa weniger STRENGTH. Wer nach dem Essen über der Höchstlast liegt, bleibt so korrigierbar.
- Geräte und Anzüge mit eigenem Vorrat kommen einzeln, jedes mit seiner Energie oder Luft.
- **Benutzen** (`use_item`, nur was dabei oder angezogen ist): Bei Medikamenten, Essen und Trinken ist eins vom Stapel weg, außer der Schnapsflasche (F-73). Dann laufen die Operationen aus `on_use` (Stress, Food oder Water, Sperren, Hinweise), und die Effekte werden zu Quellen mit Dauer (`used`), die sich addieren; Naproleve zählt die Dosen einer Schicht (F-74). Der Zähler beginnt mit jedem Schicht-Schritt neu.
- **Energie und Luft** (F-75): Ein Gerät mit Energie verlangt nach jedem Benutzen einen Power-Wurf, ebenso nach einer Probe, in der es angekreuzt war (nicht bei abgebrochener Probe). Mit 0 gibt es keinen Bonus und kein Benutzen, bis `refill_item` es auflädt; wurde ein Gerät geleert, während seine Probe noch auf den Push wartete, entfällt der Wurf danach. Der Air-Wurf (`supply: air`) geht auf den angezogenen Anzug; Power am Charakter gibt es nicht. Am Charakter bleiben Food und Water. Androiden würfeln keine Vorräte, denn sie brauchen weder Luft noch Nahrung noch Wasser [S. 111].
- **Broken** (Health 0): kein Benutzen und keine Talentaktion, nur kriechen und murmeln [S. 98].

## Geld

Jeder Charakter hat eine Buchungsliste in Cent (`ledger`); der Kontostand ist ihre Summe (`money::balance`). Die erste Buchung ist das gewürfelte Startgeld (`money::open_account`) [S. 33]. Buchungen, Käufe und Magazine ziehen nie unter null; einen Kredit bucht die Spielleitung als eigene Buchung (F-76). Abgelehnte Käufe buchen nichts. Preise kommen aus `gear.json` (`Engine::price`, `reload_price` mit `reload_cost_percent` aus `constants.json`) [S. 118, 124].

## Talente als Aktion

- **Investigator** und **Analysis** würfeln OBSERVATION; bei Erfolg geht ein Hinweis mit den Fragen ins Protokoll [S. 74, 77]. Analysis senkt bei Erfolg den Stress der anderen Spielercharaktere auf kurzer Distanz um eins und hebt ihn bei Misserfolg um eins; wer nah genug ist, wählt die Spielleitung (F-39).
- **Pull Rank** braucht ein Ziel außer sich selbst: COMMAND gegen dessen MANIPULATION, mit Hothead +2 für das Ziel [S. 76, 78]. Der Stress +1 kommt nach dem Wurf (F-56). Androiden zahlen keinen Stress, die Spielleitung bekommt einen Hinweis (F-55).
- **Tough** hebt die volle Health um 2 [S. 79]; `Engine::full_health` rechnet STRENGTH und solche Boni zusammen.

## Erschaffung

`Engine::check_creation` prüft einen Entwurf (`creation::Draft`) und nennt jedes Problem mit Schritt, Code und Buchseite, in der Reihenfolge des Buchs [S. 27–33]: Karriere; 14 Attributpunkte, je 2 bis 4, das Schlüsselattribut bis 5; 10 Fertigkeitspunkte, Schlüsselfertigkeiten bis 3, andere bis 1; alle Punkte verteilt (F-51); ein Talent der Karriere, ggf. mit Fahrzeug oder Waffe; Name, Aussehen, Agenda; zwei Startgegenstände aus zwei verschiedenen Zeilen der Karriere (F-60); das Signature Item, das Androiden nicht brauchen. Androiden bekommen +3 auf zwei verschiedene Attribute (F-50). `creation_actor` baut daraus die Figur mit voller Health, Stresslevel 0 und der Startausrüstung, Schusswaffen mit zwei Magazinen (F-61); `roll_cash` würfelt das Startgeld der Karriere und `roll_start_gear` Mengen wie „W6 Dosen“. Alle Grenzen stehen in `constants.json`.

## Panik

- **Wert:** W6 + Stresslevel + Modifikationen (Nerves of Steel −2). Unter 1 zählt als 1–6.
- **Mehr Panik:** Läuft eine Panik, wird ein niedrigerer Wert auf eine Stufe über dem laufenden angehoben; gleich bleibt gleich (F-23), über 15+ bleibt es Catatonic (F-24). Eine Panik läuft ab 7, bis sie endet, auch wenn ihr Effekt schon vorbei ist (F-25).
- **Ende:** ein Turn oder eine Schicht vergeht, Broken, eine gelungene COMMAND-Probe eines anderen, die Spielleitung [S. 104].
- **Ketten:** Wer etwas hört, sieht oder auf kurzer Distanz ist, kennt die App nicht. Sie schlägt alle anderen menschlichen Spielercharaktere vor (F-39), die Spielleitung wählt aus. Die Frage nennt Auslöser und Quelle (`choose_targets`), die Wahl ist ein eigenes Ereignis (`targets_chosen`), auch wenn niemand gewählt wird oder sich nichts ändert.
- **Overkill** bietet die Engine ab 11 an; der Wert bleibt für Mehr Panik und die Trauma-Schwelle maßgeblich (F-26, F-30).
- **Broken:** Eine Panik endet, wenn man Broken wird. Beginnt sie, während man schon Broken ist, endet sie sofort; die sofortigen Folgen gelten (F-48).
- **Trauma:** Wer in der Sitzung 13+ hatte, würfelt bei `end_session` einmal EMPATHY ohne Stresswürfel und ohne Push (F-27, F-42); misslingt sie, entsteht ein Trauma (F-01).

## Effekte und Zeit

- Aktive Quellen sind Traumata, Zustände, Verletzungen und Sperren aus `block`-Operationen. Talente und der angezogene Anzug wirken immer. Ein Zustand innerhalb eines Effekts (`condition`) bringt dessen Effekte mit, außer die Figur trägt einen passenden Anzug.
- Dauern laufen nur mit den Schritten ihrer Einheit ab (Plan 9.1). Ein Tag zählt als vier Schicht-Schritte; „bis zum nächsten Tag“ endet beim Tageswechsel.
- Periodische Effekte feuern beim passenden Zeitschritt, Freezing nach Kälte: über null täglich, unter null jede Schicht, im All jeden Turn.
- Verlangt ein Effekt eine Probe, etwa Freezing eine STAMINA-Probe, bietet die Engine passende freiwillige Talente vorher an (Stoic). Wer Broken ist, würfelt nicht; das Ereignis `roll_skipped` und der Hinweis des Zustands gehen an die Spielleitung.
- Schaden aus einem Zustand gibt keinen zusätzlichen Stress, denn der Zustand bringt seinen eigenen mit (F-32, Hayes S. 108).

## Festlegungen

Die Engine liest die gewählte Option jeder Festlegung aus dem Zustand (`rulings`), sonst gilt der Standard aus `data/rules/rulings.json`. Umschaltbar sind in diesem Bereich F-09 (Vorratswurf mit Talent), F-17 (Nightmares), F-47 (The Long Haul und Push) und F-48 (Panik bei Broken). Regelwerte wie die Höchstzahl der Helfer stehen in `data/rules/constants.json`. Zu jeder umschaltbaren Festlegung gibt es Tests für Standard und Alternative.

## Was noch fehlt

| Was | Phase |
| --- | --- |
| Konten, Teams, Speichern, echte Zufallsquelle, Würfeln in der Oberfläche | 2 |
| Verbrauch von Vorräten und Mitteln (Bier, Schnaps, Naproleve, Energie nach dem Einsatz), Geld und Kauf | 2c.2 |
| Initiative, Aktionen pro Runde, Waffenwerte im Kampf (Schaden, Reichweite, Granatwerfer der M41A, Zielen der XM99A), Überladung beim Laufen, Personal Safety, leeres Magazin nach Facehugger (S. 61, 96; F-38), Death Rolls mit Fristen | 3 |
| Ruhe pro Turn und Schicht (F-16), Banter, Calming Presence, Counselor, Zeitmodell mit Schichtmarker | 4 |
| Effekte auf Schiffe und alle an Bord, Fähigkeiten von Modulen; Fahrzeuge, Power Loader (HEAVY MACHINERY 2), Raketen mit Zielbeschränkung | 5 |
| Raumkampf-Panik | 6 |
