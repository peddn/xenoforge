//! Rule engine for Xenoforge, a companion app for the ALIEN RPG
//! (Free League, core rulebook 1st edition 2019).
//!
//! The engine is a pure library: no network, no database. It holds the
//! rule data format (`data`, see `docs/datenformat.md`) and the rules that
//! work on a game state (`engine`, see `docs/engine.md`).

mod active;
pub mod creation;
pub mod data;
pub mod dice;
pub mod engine;
pub mod inventory;
pub mod panic;
pub mod roll;
pub mod state;

pub use active::{ActiveEffect, Modification};
pub use engine::{Choice, Command, Engine, Event, NewSource, Preview, RuleError, Step};
