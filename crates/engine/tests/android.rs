//! A covert android and its reveal (A-14) [p. 29, 103].

mod common;

use common::*;
use xenoforge_engine::state::Mode;
use xenoforge_engine::{Choice, Command, Event, RuleError};

fn reveal(actor: &str) -> Command {
    Command::RevealAndroid {
        actor: actor.into(),
    }
}

/// The reveal makes the android open; the game master picks the humans
/// nearby, who gain stress +1 [p. 103].
#[test]
fn reveal_gives_stress_nearby() {
    let mut holroyd = pc().android();
    holroyd.covert = true;
    let mut t = Table::new(Mode::Cinematic)
        .with("holroyd", holroyd)
        .with("davis", pc().stress(1))
        .with("ann", pc())
        .with("bishop", pc().android());
    let ev = t.run(reveal("holroyd"), &[]);
    assert_eq!(
        ev[0],
        Event::AndroidRevealed {
            actor: "holroyd".into()
        }
    );
    assert!(!t.actor("holroyd").covert);
    let candidates = ev
        .iter()
        .find_map(|e| match e {
            Event::ChooseTargets { candidates, .. } => Some(candidates.clone()),
            _ => None,
        })
        .unwrap();
    assert_eq!(candidates, ["ann", "davis"], "humans only (F-39)");
    t.decide(&ev, Choice::Targets(vec!["davis".into()]), &[]);
    assert_eq!((t.actor("davis").stress, t.actor("ann").stress), (2, 0));
}

/// Only a covert android can be revealed, and only once.
#[test]
fn only_covert_androids() {
    let mut t = Table::new(Mode::Campaign)
        .with("bishop", pc().android())
        .with("ann", pc());
    for actor in ["bishop", "ann"] {
        assert!(matches!(
            t.try_run(reveal(actor), &[]),
            Err(RuleError::NotAllowed(_))
        ));
    }
}
