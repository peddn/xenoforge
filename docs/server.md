# Server

Der Server (`crates/server`) verbindet die Regel-Engine mit Konten, Teams, Speicher und dem gemeinsamen Protokoll. Er rechnet keine Regeln selbst: Jeder Spielzug geht als Befehl an die Engine (`docs/engine.md`), und was die Engine meldet, landet im Protokoll.

## Überblick

```
Browser ── HTTP (JSON) ──▶ axum ──▶ Engine (rein)
   ▲                         │
   └──── Server-Sent Events ◀┴── Protokoll (SQLite)
```

- **Befehle** kommen per HTTP. Ein globales Schloss sorgt dafür, dass Befehle nacheinander laufen; zwei gleichzeitige Klicks können sich nicht überholen.
- **Ereignisse** gehen per Server-Sent Events an alle im Team. Nach einem Verbindungsabbruch schickt der Browser die letzte gesehene Nummer (`Last-Event-ID`) und bekommt nachgeliefert, was er verpasst hat.
- **Gewürfelt wird auf dem Server** (Betriebssystem-Zufall, ohne Modulo-Verzerrung). Die Oberfläche zeigt nur an.

## Konten und Rechte (A-07, A-04)

- Bei der Registrierung wählt man die Rolle: **Spieler** oder **Spielleitung**. Name 2 bis 40 Zeichen, eindeutig ohne Rücksicht auf Groß- und Kleinschreibung; Passwort mindestens 8 Zeichen, gespeichert als Argon2-Hash.
- Anmeldung über ein Cookie `xf_session` (HttpOnly, SameSite=Lax, auf Wunsch Secure). Im Cookie steht ein zufälliges Token; die Datenbank kennt nur dessen SHA-256. Eine Sitzung gilt 30 Tage.
- **Teams legt nur die Spielleitung an.** Sie kann ein Team ohne Spieler vorbereiten und eigene Charaktere hineinnehmen. Andere Spielleitungen sehen das Team nicht.
- **Spieler treten über einen Einladungslink bei** und bringen dabei einen ihrer freien Charaktere mit; ein Entwurf muss erst fertiggestellt sein (A-09). Sie können auch ohne Charakter beitreten und einen wählen, den die Spielleitung anbietet (A-12). Ein Charakter ist in höchstens einem Team. Die Spielleitung kann den Link erneuern; der alte gilt dann nicht mehr.
- Die Spielleitung kann Charaktere aus dem Team nehmen, und ein Spieler kann mit seinem eigenen Charakter austreten. Hat ein Spieler danach keinen Charakter mehr im Team, sieht er es nicht mehr; ein offener Ereignisstrom endet und muss sich neu anmelden.
- Wer weder Spielleitung noch Mitglied ist, bekommt für das Team `404`, nicht `403`: Es soll nicht einmal erkennbar sein, dass es das Team gibt.

### Wer darf welchen Befehl

| Befehl | Spieler | Spielleitung |
| --- | --- | --- |
| `roll` für eigenen Charakter: Fertigkeit, Attribut, Death Roll; Helfer und Ausrüstungsbonus | ja | ja |
| `roll` mit Schwierigkeit, fester Pool, Vorrat, einzelne Stresswürfel (A-08) | nein | ja |
| `roll` für fremde Charaktere oder ohne Charakter | nein | ja |
| `decide` | wenn die Entscheidung beim eigenen Charakter liegt | ja |
| `decide` bei der Wahl der Betroffenen (`targets`) | nein | ja |
| `use_signature_item`, `perform_action` für eigenen Charakter, als Ziel jeder im Team (Pull Rank) | ja | ja |
| alle übrigen (Stress, Schaden, Panik, Quellen, Akt- und Sitzungsende, Story Points setzen …) | nein | ja |
| Vorschau eines Wurfs | eigener Charakter, Regeln wie bei `roll` | alle |
| Werte korrigieren, Rückgängig | nein | ja |

### Modifikationen eines Wurfs (A-08)

Ein Wurf bringt höchstens eine Schwierigkeit und einen Ausrüstungsbonus mit, als Kennung und Wert:

- `{"source": "difficulty_hard", "value": -2}`: nur die Spielleitung, nur mit dem Wert aus der Tabelle S. 63 (`data/rules/constants.json`).
- `{"source": "gear", "value": 2}`: 0 bis `gear_bonus_max`. Mit dem Ausrüstungskatalog (2c) kommt der Bonus vom Gegenstand.

Helfer stehen in `helpers`; es zählen höchstens drei (S. 63). Alles andere lehnt der Server ab.

## Sichtbarkeit (A-01)

- Die Werte im Bogen (Attribute, Fertigkeiten, Talente, Stress, Zustände) sehen die Spielleitung und der Besitzer. Die anderen sehen Name und Spieler. Solange die Spielleitung einen Charakter zur Wahl anbietet, sehen alle im Team seinen Bogen, aber weder Agenda noch Notizen; ein verdeckter Android erscheint dabei als Mensch (A-12, A-14, S. 29).
- Agenden pro Akt (Cinematic) sieht die Spielleitung alle, der Besitzer nur die des laufenden Akts (A-13). Sie liegen beim Team, nicht im Charakter, damit sie über die Charakter-Schnittstelle nie herausgehen.
- Was im Spiel geschieht, steht nach A-01 im gemeinsamen Protokoll, so wie es am Tisch angesagt würde: Würfe, Stress und Schaden, Panik, eingesetzte Talente und die Entscheidungen, auf die gewartet wird.
- Story Points sieht jeder Spieler für sich, die Spielleitung für alle. Dass jemand beim Aktende einen bekommt, steht für alle im Protokoll (S. 216); was die Spielleitung von Hand setzt, nur für sie.
- Jeder Protokolleintrag ist entweder für **alle** im Team oder nur für die **Spielleitung** sichtbar. Nur für die Spielleitung sind Hinweise der Engine, die als solche markiert sind. Gefiltert wird an einer Stelle (`log.rs`), für das Nachladen und für den Live-Strom gleich.
- Wen ein Effekt trifft, entscheidet nur die Spielleitung; die offene Entscheidung sehen Spieler nicht, im Protokoll steht nur, dass gewählt wird.

## Spielstand

Der Spielstand der Engine (`GameState`) wird für jeden Befehl zusammengesetzt:

- aus dem **Team-Dokument** (Modus, offene Entscheidungen, Story Points, Festlegungen, Zeit …), gespeichert als JSON in `teams.state`,
- und aus den **Charakteren** des Teams, jeweils als JSON in `characters.data`. Jeder Akteur bekommt dabei den Besitzer als Spieler gesetzt, denn Story Points gehören dem Spieler.

Nach dem Befehl teilt der Server den neuen Stand wieder auf und speichert Team, Charaktere und Protokolleinträge in **einer Transaktion**. Alles, was Team oder Charaktere schreibt, läuft unter demselben Schloss; so überschreibt kein Speichern einen Spielzug mit älteren Werten. Lehnt die Engine ab, ändert sich nichts, und die Antwort ist `422` mit dem Grund samt Buchseite.

### Erschaffung und Bogen (A-09, Plan 5.2)

Ein Charakter hat einen Status:

- **Entwurf:** Spieler erschaffen mit dem Assistenten. Der Server speichert den Entwurf, lässt die Engine die Figur bauen und gibt ihre Probleme mit Schritt und Buchseite zurück. Ein Entwurf kann keinem Team beitreten. **Fertigstellen** geht nur ohne Probleme; dann würfelt der Server das Startgeld, und der Entwurf wird zum Bogen.
- **Fertig:** Die Werte ändern sich nur im Spiel. Name, Aussehen, Agenda (F-54), Signature Item und Notizen bleiben frei.
- **Frei:** Nur Spielleitungskonten legen frei an, für vorgefertigte Charaktere und Szenarien; nur sie bearbeiten außerhalb eines Teams auch die Werte und die Karriere. Charaktere von vor dem Assistenten gelten als frei.

Im Team ändern sich die Werte nur noch über Befehle oder über eine Korrektur der Spielleitung. Volle Health folgt STRENGTH (S. 30, Tough +2), solange Health nicht selbst geändert wurde. Buddy und Rival wählt in Campaign der Besitzer, in Cinematic die Spielleitung, weil das Szenario sie vorgibt (S. 31). Sie haben einen eigenen Pfad, damit Speichern der Texte sie nie berührt: andere Spielercharaktere desselben Teams, nicht derselbe, beide freiwillig (F-52). Texte haben höchstens 4000 Zeichen. Verlässt einer das Team, fallen die Verweise weg. Agenda, Buddy und Rival sieht wie alle Bogenwerte nur der Besitzer und die Spielleitung.

### Rückgängig

Vor jedem Befehl und jeder Korrektur legt der Server einen Schnappschuss des Team-Dokuments und der Werte aller Charaktere an; die letzten 20 bleiben erhalten. **Rückgängig** stellt den letzten wieder her und schreibt das ins Protokoll; die alten Einträge bleiben stehen. Name, Aussehen und Notizen bleiben, wie der Spieler sie zuletzt gespeichert hat, und Nummern von Würfen und Entscheidungen werden nie zweimal vergeben. Wird ein Charakter aus dem Team genommen, endet die Kette, weil ältere Stände ihn noch enthalten.

## Speicher (SQLite)

Die Datenbank ist eine Datei (`XENOFORGE_DB`). Die Migration ist im Programm eingebaut und läuft beim Start.

| Tabelle | Inhalt |
| --- | --- |
| `users` | Name, Passwort-Hash, Rolle |
| `sessions` | SHA-256 des Sitzungs-Tokens, Konto, Zeitpunkt |
| `teams` | Spielleitung, Name, Einladungs-Token, Team-Dokument (JSON), Agenden pro Akt (JSON) |
| `members` | wer als Spieler in welchem Team ist |
| `characters` | Besitzer, Team (oder keines), Name, Bogen und Werte (JSON) |
| `events` | Protokoll: Team, Auslöser, Sichtbarkeit, Eintrag (JSON) |
| `snapshots` | Stände für Rückgängig |

Beim Start prüft der Server die Regeldaten in `XENOFORGE_DATA` und startet nicht, wenn sie Fehler haben.

## Schnittstelle

Alle Pfade beginnen mit `/api`. Antworten sind JSON; Fehler haben die Form `{ "error": "<code>", "message": "<text>" }` mit den Codes `bad_request` (400), `unauthorized` (401), `forbidden` (403), `not_found` (404), `conflict` (409), `rule` (422) und `internal` (500). Bei `internal` steht der Grund nur im Log des Servers.

### Konto

| Methode und Pfad | Inhalt | Antwort |
| --- | --- | --- |
| `POST /auth/register` | `{ name, password, role: "player" \| "gm" }` | Konto, setzt das Cookie |
| `POST /auth/login` | `{ name, password }` | Konto, setzt das Cookie |
| `POST /auth/logout` | – | löscht Sitzung und Cookie |
| `GET /auth/me` | – | `{ id, name, role }` oder 401 |

### Charaktere

| Methode und Pfad | Inhalt | Antwort |
| --- | --- | --- |
| `GET /characters` | – | eigene Charaktere |
| `POST /characters` | `{}` für einen Entwurf, `{ free: true, name }` nur Spielleitung | neuer Charakter, ein Entwurf mit `problems` |
| `GET /characters/{id}` | – | Charakter |
| `PUT /characters/{id}` | Entwurf: `{ data: { draft } }`; sonst `{ name, data: { sheet, actor } }` | Charakter, beim Entwurf mit `problems`; Werte nur bei freien Charakteren der Spielleitung außerhalb eines Teams |
| `POST /characters/{id}/finish` | – | fertiger Charakter mit `start_cash`; 400, solange es Probleme gibt |
| `PUT /characters/{id}/relations` | `{ buddy, rival }`, Ids oder `null` | Charakter; nur im Team (F-52); in Cinematic nur die Spielleitung |
| `DELETE /characters/{id}` | – | nur außerhalb eines Teams, sonst 409 |

### Teams

| Methode und Pfad | Wer | Inhalt | Antwort |
| --- | --- | --- | --- |
| `GET /teams` | alle | – | eigene Teams |
| `POST /teams` | Spielleitung | `{ name, mode: "cinematic" \| "campaign" }` | neues Team |
| `GET /teams/{id}` | Team | – | Ansicht des Teams, gefiltert nach Rolle |
| `POST /teams/{id}/invite` | Spielleitung | – | neues Einladungs-Token |
| `POST /teams/{id}/characters` | Spielleitung | `{ character }` | eigenen Charakter hinzufügen |
| `PUT /teams/{id}/characters/{c}` | Spielleitung | `{ actor, note? }` | Werte korrigieren |
| `DELETE /teams/{id}/characters/{c}` | Spielleitung oder Besitzer | – | Charakter aus dem Team nehmen bzw. austreten |
| `GET /join/{token}` | angemeldet | – | Team, Spielleitung, Modus |
| `POST /join/{token}` | angemeldet | `{ character }`, Id oder `null` | Beitritt mit freiem Charakter oder ohne (A-12) |
| `PUT /teams/{id}/characters/{c}/offer` | Spielleitung | `{ offered }` | eigenen Charakter zur Wahl anbieten oder zurückziehen |
| `POST /teams/{id}/characters/{c}/claim` | Spieler | – | angebotenen Charakter wählen; wer zuerst kommt, wird Besitzer, sonst 409; nur wer noch keinen Charakter im Team hat |
| `PUT /teams/{id}/characters/{c}/owner` | Spielleitung | `{ owner }` | eigenen Charakter an ein Mitglied übergeben; einen vorgefertigten zurücknehmen nur in Cinematic (S. 32); beendet die Rückgängig-Kette |
| `DELETE /teams/{id}/members/{user}` | Spielleitung oder der Spieler selbst | – | Spieler ohne Charakter im Team austreten lassen |
| `PUT /teams/{id}/characters/{c}/agendas` | Spielleitung | `{ agendas }`, höchstens drei | Agenden pro Akt (Cinematic, A-13) |

### Spiel

| Methode und Pfad | Inhalt | Antwort |
| --- | --- | --- |
| `POST /teams/{id}/commands` | `{ command, note? }`, ein Engine-Befehl und auf Wunsch eine Notiz | neue Protokolleinträge, soweit sichtbar |
| `POST /teams/{id}/preview` | Wurfanfrage wie in `roll` | Würfelzahl und Modifikatoren, ohne zu würfeln |
| `POST /teams/{id}/undo` | – | Eintrag `undone` |
| `GET /teams/{id}/events` | `Last-Event-ID` oder `?after=<id>` | Server-Sent Events, je Eintrag `{ id, batch, visible_to, by, at, entry }` |

Offene Entscheidungen (`pending` in der Ansicht des Teams) bringen bei einem Wurf den bisherigen Stand mit (`roll`): Fertigkeit oder Attribut, Würfel, Erfolge und übrige Pushes. Die Erfolge zählt die Engine. Bei der Wahl der Betroffenen, die nur die Spielleitung sieht, kommen dazu, wessen Aktion oder Panik fragt (`by`), welches Talent, welcher Panikeffekt oder welche Regel dahintersteht (`source`) und was jedem Gewählten geschieht (`ops`, z. B. Stress −1). Die Wahl selbst steht als `targets_chosen` im Protokoll, auch wenn sie nichts ändert. Die Oberfläche lädt die Ansicht nach jedem Befehl neu, nicht nur bei neuen Einträgen.

Einträge im Protokoll (`entry.type`): `engine` (mit dem Ereignis der Engine), `character_joined`, `character_removed` (durch die Spielleitung), `character_left` (der Spieler tritt aus), `corrected`, `undone`. Alle Einträge eines Befehls tragen in `batch` die Nummer des ersten; die Oberfläche zeigt sie als einen Block. Eine Notiz zum Befehl (`note`, höchstens 200 Zeichen) steht am ersten Eintrag; sie ist für alle sichtbar wie der Befehl selbst.

### Regeldaten

`GET /data/rules/*.json` und `GET /data/i18n/{de,en}.json` liefern die Regeldaten und ihre Texte aus `data/`, damit die Oberfläche Namen und Seitenangaben zeigen kann.

## Einstellungen

Aufruf: `xenoforge-server [--seed]`. Mit `--seed` füllt der Server eine leere Datenbank mit Beispieldaten und startet dann (`seed.rs`). Die Daten entstehen über die eigene Schnittstelle im Prozess (`local.rs`), also mit denselben Prüfungen wie im Spiel; dieselbe Schnittstelle nutzen die Tests. Eine Datenbank mit Konten lehnt `--seed` ab.

| Variable | Bedeutung | Standard |
| --- | --- | --- |
| `XENOFORGE_ADDR` | Adresse und Port | `127.0.0.1:3000` |
| `XENOFORGE_DB` | Datenbankdatei | `xenoforge.db` |
| `XENOFORGE_STATIC` | gebaute Oberfläche | `frontend/dist` |
| `XENOFORGE_DATA` | Regeldaten | `data` |
| `XENOFORGE_SECURE_COOKIES` | `true` hinter HTTPS | `false` |
| `RUST_LOG` | Log-Ausgabe | `info` |
