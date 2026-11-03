# Xenoforge

Begleit-App für das ALIEN-Rollenspiel (Free League, Grundregelwerk 1. Auflage). Spielleitung und Spieler verwalten Charaktere, würfeln per Klick, führen Kämpfe, bauen Schiffe, spielen Raumkämpfe und lassen die Spielzeit mitlaufen. Karte und Figuren bleiben auf dem Tisch.

**Stand:** Phase 1, Regel-Engine: Würfe, Push, Stress, Panik und das Effekt-System als getestete Bibliothek. Eine Oberfläche zum Spielen kommt mit Phase 2.

- [Projektplan](docs/PLAN.md)
- [Regel-Spezifikation mit Seitenangaben](docs/rules/)
- [Errata und Festlegungen](docs/rules/00-errata-und-festlegungen.md)
- [Datenformat der Regeldaten](docs/datenformat.md)
- [Regel-Engine](docs/engine.md)

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

Für den Betrieb baut `npm run build` die Oberfläche nach `frontend/dist`; der Server liefert sie mit aus. Einstellungen über Umgebungsvariablen:

| Variable | Bedeutung | Standard |
| --- | --- | --- |
| `XENOFORGE_ADDR` | Adresse und Port | `127.0.0.1:3000` |
| `XENOFORGE_STATIC` | gebaute Oberfläche | `frontend/dist` |
| `RUST_LOG` | Log-Ausgabe | `info` |

```
crates/engine    Regel-Engine und Datentypen, ohne Netz und Datenbank
crates/server    Webserver (axum)
data/            Regeldaten, Texte, erzeugte Schemas
frontend/        Vue 3, Pinia, Vue Router, vue-i18n
tools/           Hilfswerkzeuge, z. B. die Wortlautprüfung
```

## Lizenz

Code und eigene Texte stehen unter der [Apache License 2.0](LICENSE).

## Hinweis

Inoffizielles Fan-Projekt, weder mit Free League Publishing noch mit den Rechteinhabern von ALIEN verbunden. Das Repo enthält keine Texte oder Bilder aus dem Regelwerk, nur Spielwerte mit Seitenverweisen und eigene Kurzbeschreibungen. Zum Spielen braucht ihr das Regelwerk.
