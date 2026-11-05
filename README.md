# Xenoforge

Begleit-App für das ALIEN-Rollenspiel (Free League, Grundregelwerk 1. Auflage). Spielleitung und Spieler verwalten Charaktere, würfeln per Klick, führen Kämpfe, bauen Schiffe, spielen Raumkämpfe und lassen die Spielzeit mitlaufen. Karte und Figuren bleiben auf dem Tisch.

**Stand:** Phase 2, Spieltisch: Konten mit Rolle, Teams mit Einladungslink, Würfe per Klick mit Push und Entscheidungen, Werkzeuge der Spielleitung und ein gemeinsames Protokoll in Echtzeit; Charaktererschaffung mit Assistent, vorgefertigte Charaktere für Cinematic, Ausrüstung mit Traglast, Boni im Wurf, Benutzen von Medikamenten, Essen und Geräten, Geld mit Buchungen und Kauf. Die Regel-Engine (Phase 1) rechnet Würfe, Stress, Panik und Effekte. Als Nächstes: Testsitzung zum Abschluss von Phase 2, dann Kampf und Gegner (Phase 3).

- [Projektplan](docs/PLAN.md)
- [Regel-Spezifikation mit Seitenangaben](docs/rules/)
- [Errata und Festlegungen](docs/rules/00-errata-und-festlegungen.md)
- [Datenformat der Regeldaten](docs/datenformat.md)
- [Regel-Engine](docs/engine.md)
- [Server: Rechte, Sichtbarkeit, Speicher, Schnittstelle](docs/server.md)

**Technik:** Rust-Server (axum, sqlx, SQLite) und Vue 3 mit Pinia und Vue Router in JavaScript.

## Entwicklung

Voraussetzungen: Rust 1.85 oder neuer, Node 22.12 oder neuer.

```
# Server und Prüfungen
cargo test
cargo run -p xenoforge-server          # http://127.0.0.1:3000

# Oberfläche, zweites Terminal
cd frontend
npm install
npm run dev                            # http://localhost:5173, leitet /api an den Server weiter
```

### Testdaten

Zum schnellen Ausprobieren füllt `--seed` eine leere Datenbank mit Beispieldaten und startet dann den Server:

```
XENOFORGE_DB=test.db cargo run -p xenoforge-server -- --seed
```

Angelegt werden die Konten Mother (Spielleitung), Ann, Bob und Cid, alle mit dem Passwort `xenoforge-test`, dazu zwei Teams:

- **Testlauf Kampagne:** Ann spielt Nat (Buchbeispiel S. 28, mit Schneidbrenner und Kompressionsanzug wie S. 33, dazu Bier und eine Fertigmahlzeit), Bob eine Wissenschaftlerin, beide über den Assistenten erschaffen, mit Startausrüstung, Buddy und Rival. Ann hat außerdem einen halb fertigen Entwurf.
- **Testlauf Szenario** (Cinematic): drei Charaktere der Spielleitung zur Wahl, einer davon ein verdeckter Android, jeder mit Agenden für drei Akte und Ausrüstung aus dem Szenario (darunter Bewegungsmelder und Naproleve). Ann und Cid sind ohne Charakter beigetreten.

Eine Datenbank mit Konten lehnt `--seed` ab, damit nie Beispieldaten in echte Daten geraten. Zum Zurücksetzen `test.db` löschen und neu starten. Mehrere Personen gleichzeitig testet man in getrennten Browsern oder privaten Fenstern.

## Betrieb

Ein Programm, eine Datenbankdatei, kein Docker:

```
cd frontend && npm ci && npm run build && cd ..
cargo build --release -p xenoforge-server
XENOFORGE_DB=/var/lib/xenoforge/xenoforge.db ./target/release/xenoforge-server
```

Der Server liefert die gebaute Oberfläche aus `frontend/dist` und die Regeldaten aus `data/` mit aus und legt die Datenbank beim ersten Start an. Hinter einem Reverse Proxy mit HTTPS `XENOFORGE_SECURE_COOKIES=true` setzen; der Proxy darf Server-Sent Events nicht puffern (bei nginx `proxy_buffering off;` für `/api/teams/`).

| Variable | Bedeutung | Standard |
| --- | --- | --- |
| `XENOFORGE_ADDR` | Adresse und Port | `127.0.0.1:3000` |
| `XENOFORGE_DB` | Datenbankdatei | `xenoforge.db` |
| `XENOFORGE_STATIC` | gebaute Oberfläche | `frontend/dist` |
| `XENOFORGE_DATA` | Regeldaten | `data` |
| `XENOFORGE_SECURE_COOKIES` | `true` hinter HTTPS | `false` |
| `RUST_LOG` | Log-Ausgabe | `info` |

Gesichert wird die Datenbankdatei, am besten mit `sqlite3 xenoforge.db ".backup sicherung.db"` im laufenden Betrieb.

## Aufbau

```
crates/engine    Regel-Engine und Datentypen, ohne Netz und Datenbank
crates/server    Webserver: Konten, Teams, Befehle, Protokoll (axum, sqlx)
data/            Regeldaten, Texte, erzeugte Schemas
frontend/        Vue 3, Pinia, Vue Router, vue-i18n
tools/           Hilfswerkzeuge, z. B. die Wortlautprüfung
```

## Lizenz

Code und eigene Texte stehen unter der [Apache License 2.0](LICENSE).

## Hinweis

Inoffizielles Fan-Projekt, weder mit Free League Publishing noch mit den Rechteinhabern von ALIEN verbunden. Das Repo enthält keine Texte oder Bilder aus dem Regelwerk, nur Spielwerte mit Seitenverweisen und eigene Kurzbeschreibungen. Zum Spielen braucht ihr das Regelwerk.
