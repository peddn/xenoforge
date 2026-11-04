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
- **Spieler treten über einen Einladungslink bei** und bringen dabei einen ihrer freien Charaktere mit. Ein Charakter ist in höchstens einem Team. Die Spielleitung kann den Link erneuern; der alte gilt dann nicht mehr.
- Die Spielleitung kann Charaktere aus dem Team nehmen. Hat ein Spieler danach keinen Charakter mehr im Team, sieht er es nicht mehr; ein offener Ereignisstrom endet und muss sich neu anmelden.
- Wer weder Spielleitung noch Mitglied ist, bekommt für das Team `404`, nicht `403`: Es soll nicht einmal erkennbar sein, dass es das Team gibt.

### Wer darf welchen Befehl

| Befehl | Spieler | Spielleitung |
| --- | --- | --- |
| `roll` für eigenen Charakter | ja | ja |
| `roll` für fremde Charaktere oder ohne Charakter | nein | ja |
| `decide` | wenn die Entscheidung beim eigenen Charakter liegt | ja |
| `decide` bei der Wahl der Betroffenen (`targets`) | nein | ja |
| `use_signature_item`, `perform_action` für eigenen Charakter | ja | ja |
| alle übrigen (Stress, Schaden, Panik, Quellen, Akt- und Sitzungsende …) | nein | ja |
| Vorschau eines Wurfs | eigener Charakter | alle |
| Werte korrigieren, Rückgängig | nein | ja |

## Sichtbarkeit (A-01)

- Werte eines Charakters (Attribute, Fertigkeiten, Stress, Zustände) sehen die Spielleitung und der Besitzer. Die anderen sehen Name und Spieler.
- Story Points sieht jeder Spieler für sich, die Spielleitung für alle.
- Jeder Protokolleintrag ist entweder für **alle** im Team oder nur für die **Spielleitung** sichtbar. Nur für die Spielleitung sind Hinweise der Engine, die als solche markiert sind. Gefiltert wird an einer Stelle (`log.rs`), für das Nachladen und für den Live-Strom gleich.
- Die Wahl der Betroffenen eines Effekts ist eine offene Entscheidung nur für die Spielleitung; Spieler sehen sie nicht.

## Spielstand

Der Spielstand der Engine (`GameState`) wird für jeden Befehl zusammengesetzt:

- aus dem **Team-Dokument** (Modus, offene Entscheidungen, Story Points, Festlegungen, Zeit …), gespeichert als JSON in `teams.state`,
- und aus den **Charakteren** des Teams, jeweils als JSON in `characters.data`. Jeder Akteur bekommt dabei den Besitzer als Spieler gesetzt, denn Story Points gehören dem Spieler.

Nach dem Befehl teilt der Server den neuen Stand wieder auf und speichert Team, Charaktere und Protokolleinträge in **einer Transaktion**. Lehnt die Engine ab, ändert sich nichts, und die Antwort ist `422` mit dem Grund samt Buchseite.

Außerhalb eines Teams bearbeitet der Besitzer seinen Charakter frei. Im Team ändern sich die Werte nur noch über Befehle oder über eine Korrektur der Spielleitung; Name, Aussehen und Notizen bleiben frei. Volle Health folgt STRENGTH (S. 30), solange Health nicht selbst geändert wurde.

### Rückgängig

Vor jedem Befehl und jeder Korrektur legt der Server einen Schnappschuss von Team und Charakteren an; die letzten 20 bleiben erhalten. **Rückgängig** stellt den letzten wieder her und schreibt das ins Protokoll; die alten Einträge bleiben stehen. Wird ein Charakter aus dem Team genommen, endet die Kette, weil ältere Stände ihn noch enthalten.

## Speicher (SQLite)

Die Datenbank ist eine Datei (`XENOFORGE_DB`). Die Migration ist im Programm eingebaut und läuft beim Start.

| Tabelle | Inhalt |
| --- | --- |
| `users` | Name, Passwort-Hash, Rolle |
| `sessions` | SHA-256 des Sitzungs-Tokens, Konto, Zeitpunkt |
| `teams` | Spielleitung, Name, Einladungs-Token, Team-Dokument (JSON) |
| `members` | wer als Spieler in welchem Team ist |
| `characters` | Besitzer, Team (oder keines), Name, Bogen und Werte (JSON) |
| `events` | Protokoll: Team, Auslöser, Sichtbarkeit, Eintrag (JSON) |
| `snapshots` | Stände für Rückgängig |

Beim Start prüft der Server die Regeldaten in `XENOFORGE_DATA` und startet nicht, wenn sie Fehler haben.

## Schnittstelle

Alle Pfade beginnen mit `/api`. Antworten sind JSON; Fehler haben die Form `{ "error": "<code>", "message": "<text>" }` mit den Codes `bad_request` (400), `unauthorized` (401), `forbidden` (403), `not_found` (404), `conflict` (409), `rule` (422) und `internal` (500).

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
| `POST /characters` | `{ name }` | neuer Charakter |
| `GET /characters/{id}` | – | Charakter |
| `PUT /characters/{id}` | `{ name, data: { sheet, actor } }` | Charakter; Werte nur außerhalb eines Teams |
| `DELETE /characters/{id}` | – | nur außerhalb eines Teams, sonst 409 |

### Teams

| Methode und Pfad | Wer | Inhalt | Antwort |
| --- | --- | --- | --- |
| `GET /teams` | alle | – | eigene Teams |
| `POST /teams` | Spielleitung | `{ name, mode: "cinematic" \| "campaign" }` | neues Team |
| `GET /teams/{id}` | Team | – | Ansicht des Teams, gefiltert nach Rolle |
| `POST /teams/{id}/invite` | Spielleitung | – | neues Einladungs-Token |
| `POST /teams/{id}/characters` | Spielleitung | `{ character }` | eigenen Charakter hinzufügen |
| `PUT /teams/{id}/characters/{c}` | Spielleitung | `{ actor }` | Werte korrigieren |
| `DELETE /teams/{id}/characters/{c}` | Spielleitung | – | Charakter aus dem Team nehmen |
| `GET /join/{token}` | angemeldet | – | Team, Spielleitung, Modus |
| `POST /join/{token}` | angemeldet | `{ character }` | Beitritt mit freiem Charakter |

### Spiel

| Methode und Pfad | Inhalt | Antwort |
| --- | --- | --- |
| `POST /teams/{id}/commands` | `{ command }`, ein Engine-Befehl | neue Protokolleinträge, soweit sichtbar |
| `POST /teams/{id}/preview` | Wurfanfrage wie in `roll` | Würfelzahl und Modifikatoren, ohne zu würfeln |
| `POST /teams/{id}/undo` | – | Eintrag `undone` |
| `GET /teams/{id}/events` | `Last-Event-ID` oder `?after=<id>` | Server-Sent Events, je Eintrag `{ id, visible_to, by, at, entry }` |

Einträge im Protokoll (`entry.type`): `engine` (mit dem Ereignis der Engine), `character_joined`, `character_removed`, `corrected`, `undone`.

### Regeldaten

`GET /data/rules/*.json` und `GET /data/i18n/{de,en}.json` liefern die Regeldaten und ihre Texte aus `data/`, damit die Oberfläche Namen und Seitenangaben zeigen kann.

## Einstellungen

| Variable | Bedeutung | Standard |
| --- | --- | --- |
| `XENOFORGE_ADDR` | Adresse und Port | `127.0.0.1:3000` |
| `XENOFORGE_DB` | Datenbankdatei | `xenoforge.db` |
| `XENOFORGE_STATIC` | gebaute Oberfläche | `frontend/dist` |
| `XENOFORGE_DATA` | Regeldaten | `data` |
| `XENOFORGE_SECURE_COOKIES` | `true` hinter HTTPS | `false` |
| `RUST_LOG` | Log-Ausgabe | `info` |
