# Xenoforge – Hinweise für die Arbeit am Repo

Begleit-App für das ALIEN RPG (Free League, Grundregelwerk 1. Auflage 2019). Der Plan steht in `docs/PLAN.md`, die Regeln in `docs/rules/`.

## Arbeitsweise

- Vor jeder Umsetzung den Plan und die Vorschläge zeigen und auf Zustimmung warten.
- Bei jedem Vorschlag, auch bei Wünschen des Nutzers, vorher abschätzen: Wie viel Komplexität kommt dazu? Entstehen Parallelentwicklungen oder doppelte Mechanismen? Wächst die Codebasis unnötig? Die Abschätzung gehört mit in den Vorschlag, dazu Alternativen und eine Empfehlung.

## Regeltreue

- Die Regeln müssen exakt dem Buch folgen. Quelle ist `docs/rules/`; `docs/rules/00-errata-und-festlegungen.md` geht vor.
- Jede Regel im Code und jeder Datensatz in `data/` nennt die Buchseite.
- Regelwerte stehen in `data/`, nicht verstreut im Code.
- Unklare Stellen nicht still entscheiden: als Eintrag im Katalog der Festlegungen ergänzen und nachfragen.

## Öffentliches Repo

- Keine wörtlichen Passagen, Bilder oder Flufftexte aus dem Regelwerk. Zahlenwerte ja, Beschreibungen nur in eigenen Worten.

## Technik

- Server: Rust (Cargo-Workspace). `crates/engine` ist eine reine Bibliothek ohne Netz und Datenbank; `crates/server` nutzt axum, sqlx und SQLite.
- Oberfläche: Vue 3 mit Pinia und Vue Router, **JavaScript, kein TypeScript**, Build mit Vite, Texte über vue-i18n (Deutsch und Englisch).
- Der Server würfelt; die Oberfläche rechnet keine Regeln selbst.
- Format der Regeldaten: `docs/datenformat.md`. Die Rust-Typen sind die Quelle, die Schemas in `data/schema/` werden erzeugt.

## Befehle

Vor jedem Commit müssen alle grün sein (wie in der CI):

```
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
cargo test
cd frontend && npm run lint && npm test && npm run build
```

- Schemas nach Änderung der Typen neu erzeugen: `XENOFORGE_UPDATE_SCHEMAS=1 cargo test -p xenoforge-engine schemas`
- Wortlaut prüfen (eigenes Buch als Text nötig): `node tools/verbatim-check.mjs --book <buch.txt> data/i18n`
- Git: ein Branch pro Phase, Abschluss per Pull Request.

## Sprache

- Dokumentation und Oberflächentexte zuerst auf Deutsch, Englisch als zweite Sprache.
- Code, Bezeichner und Regelbegriffe auf Englisch wie im Buch.
