//! Dice [p. 22].
//!
//! The engine never creates random numbers itself. It asks a [`Dice`]
//! source, so the server can use a real random source and tests can use
//! fixed results ([`ScriptedDice`]).

use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

/// A source of six-sided dice.
pub trait Dice {
    /// One D6, 1–6.
    fn d6(&mut self) -> u8;
}

/// Plays back fixed results, in order. Running out is a bug in the caller
/// and panics.
#[derive(Debug, Clone, Default)]
pub struct ScriptedDice {
    faces: VecDeque<u8>,
}

impl ScriptedDice {
    pub fn new(faces: impl IntoIterator<Item = u8>) -> Self {
        let faces: VecDeque<u8> = faces.into_iter().collect();
        assert!(
            faces.iter().all(|f| (1..=6).contains(f)),
            "die faces are 1–6: {faces:?}"
        );
        Self { faces }
    }

    /// Results not used yet.
    pub fn remaining(&self) -> usize {
        self.faces.len()
    }
}

impl Dice for ScriptedDice {
    fn d6(&mut self) -> u8 {
        self.faces
            .pop_front()
            .expect("the scripted dice ran out of results")
    }
}

/// Base Die or Stress Die [p. 22, 58].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DieKind {
    Base,
    Stress,
}

/// One die of a pool.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Die {
    pub kind: DieKind,
    pub face: u8,
    /// Kept from an earlier throw when pushing [p. 60].
    pub locked: bool,
}

impl Die {
    /// A 6 on any die [p. 58].
    pub fn success(&self) -> bool {
        self.face == 6
    }

    /// A 1 on a Stress Die [p. 58].
    pub fn facehugger(&self) -> bool {
        self.kind == DieKind::Stress && self.face == 1
    }
}

/// Throws `base` Base Dice and `stress` Stress Dice, Base Dice first.
pub fn throw(dice: &mut dyn Dice, base: u8, stress: u8) -> Vec<Die> {
    let kinds = std::iter::repeat_n(DieKind::Base, base.into())
        .chain(std::iter::repeat_n(DieKind::Stress, stress.into()));
    kinds
        .map(|kind| Die {
            kind,
            face: dice.d6(),
            locked: false,
        })
        .collect()
}

/// D3: a D6 halved, rounded up [p. 22].
pub fn d3(dice: &mut dyn Dice) -> u8 {
    dice.d6().div_ceil(2)
}

/// D66: first die tens, second die ones [p. 22].
pub fn d66(dice: &mut dyn Dice) -> u8 {
    let tens = dice.d6();
    tens * 10 + dice.d6()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d3_and_d66() {
        // p. 22: a 5 on the D6 is a 3 on the D3; 3 then 6 is 36.
        assert_eq!(d3(&mut ScriptedDice::new([5])), 3);
        assert_eq!(d3(&mut ScriptedDice::new([1])), 1);
        assert_eq!(d66(&mut ScriptedDice::new([3, 6])), 36);
    }

    #[test]
    fn only_ones_on_stress_dice_are_facehuggers() {
        let dice = throw(&mut ScriptedDice::new([1, 6, 1, 6]), 2, 2);
        let faces: Vec<_> = dice.iter().map(|d| (d.facehugger(), d.success())).collect();
        assert_eq!(
            faces,
            [(false, false), (false, true), (true, false), (false, true)]
        );
    }
}
