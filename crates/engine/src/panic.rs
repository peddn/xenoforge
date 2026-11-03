//! Panic Roll, More Panic, Overkill and the end of panic [p. 104–105].

use serde::{Deserialize, Serialize};

use crate::active::{self, RollMatch};
use crate::data::Id;
use crate::data::effects::{EndEvent, Lasts, ReplaceWith, Replaceable, RollKind};
use crate::engine::{Ctx, Event, OpFlags, Result, RuleError, Task};
use crate::state::{ActivePanic, ActorId, Ending, Pending};

/// What made the actor roll for panic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PanicCause {
    /// Facehuggers in a skill roll; `slow_action` if that roll was a slow
    /// action (F-28).
    SkillRoll { slow_action: bool },
    /// Anything else: witnesses, critical injury, the game master.
    Other,
}

/// Why a panic ended [p. 104].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StopReason {
    /// A successful COMMAND roll by someone else (F-36).
    Command {
        by: ActorId,
    },
    Broken,
    /// A Turn passed, or a Shift (plan section 9.1).
    TurnPassed,
    /// By hand, e.g. Berserk when the target is Broken (F-29).
    GameMaster,
}

/// When a panic entry's own effects end. "Until panic stops" means as
/// long as the panic runs.
fn effects_end(lasts: Lasts) -> Option<Ending> {
    match lasts {
        Lasts::Until(EndEvent::PanicStops) => Some(Ending::Never),
        other => crate::engine::ending(other),
    }
}

impl Ctx<'_> {
    /// D6 + STRESS LEVEL + modifications, raised by More Panic. Returns
    /// whether the result cancels the action the roll came from.
    pub(crate) fn panic_roll(&mut self, actor: &str, cause: PanicCause) -> Result<bool> {
        let a = self.actor(actor)?;
        if !a.human_pc() {
            return Err(RuleError::NotAllowed(
                "only human player characters make Panic Rolls (F-43)".into(),
            ));
        }
        let stress = a.stress;
        let previous = a.panic.as_ref().map(|p| p.value);
        let effects = self.effects(actor)?;
        let modifications = active::roll_modifiers(
            &effects,
            RollMatch {
                skill: None,
                attribute: None,
                kind: Some(RollKind::Panic),
            },
        );
        let d6 = self.dice.d6();
        let raw = i16::from(d6)
            + i16::from(stress)
            + modifications
                .iter()
                .map(|m| i16::from(m.value))
                .sum::<i16>();
        // Below 1 counts as the first row (docs/datenformat.md).
        let mut value = u8::try_from(raw.max(1)).unwrap_or(u8::MAX);
        let mut raised_from = None;
        // More Panic: a lower result becomes one step more severe; an equal
        // one applies again (F-23); beyond 15+ it stays Catatonic (F-24).
        if let Some(previous) = previous {
            if value < previous {
                raised_from = Some(value);
                value = previous.saturating_add(1);
            }
        }
        let row = self
            .engine
            .panic_row(value)
            .ok_or_else(|| RuleError::UnknownEntry(format!("panic row {value}")))?;
        let (row_id, cancels, trauma) = (row.id.clone(), row.cancels_action, row.trauma_check);
        if trauma {
            // The final value counts (F-26).
            self.actor_mut(actor)?.panic_13_plus = true;
        }
        self.emit(Event::PanicRolled {
            actor: actor.to_owned(),
            d6,
            stress,
            modifications,
            value,
            raised_from,
        });
        let cancelled_slow =
            matches!(cause, PanicCause::SkillRoll { slow_action: true }) && cancels;

        // Overkill may replace the effect [p. 104; F-30].
        let overkill =
            active::replacements(&effects)
                .into_iter()
                .find_map(|u| match (u.replaces, u.with) {
                    (Replaceable::PanicResult { from }, ReplaceWith::PanicEffect(_))
                        if value >= *from =>
                    {
                        Some(u.owner)
                    }
                    _ => None,
                });
        if let Some(talent) = overkill {
            self.actor_mut(actor)?.panic = Some(ActivePanic {
                effect: row_id.clone(),
                value,
                effects_end: None,
            });
            let pending = self.next_pending();
            self.state.pending.insert(
                pending,
                Pending::Overkill {
                    actor: actor.to_owned(),
                    value,
                    talent,
                    effect: row_id,
                    cancelled_slow,
                },
            );
            self.emit(Event::Decision {
                pending,
                actor: Some(actor.to_owned()),
                options: vec![
                    crate::engine::Choice::Overkill,
                    crate::engine::Choice::Accept,
                ],
            });
            return Ok(cancels);
        }
        self.start_panic_effect(actor, &row_id, value, cancelled_slow)?;
        Ok(cancels)
    }

    /// Puts a panic effect in place and runs its one-shot operations.
    /// From 7 on the panic runs until it stops (F-25).
    pub(crate) fn start_panic_effect(
        &mut self,
        actor: &str,
        effect: &Id,
        value: u8,
        cancelled_slow: bool,
    ) -> Result<()> {
        let entry = self
            .engine
            .panic_entry(effect)
            .ok_or_else(|| RuleError::UnknownEntry(effect.clone()))?;
        let (lasts, on_apply) = (entry.lasts, entry.on_apply.clone());
        let a = self.actor_mut(actor)?;
        a.panic = (value >= 7).then(|| ActivePanic {
            effect: effect.clone(),
            value,
            effects_end: effects_end(lasts),
        });
        self.emit(Event::PanicEffect {
            actor: actor.to_owned(),
            effect: effect.clone(),
            value,
        });
        if !on_apply.is_empty() {
            self.queue.push_back(Task::Ops {
                actor: actor.to_owned(),
                owner: effect.clone(),
                ops: on_apply,
                flags: OpFlags {
                    from_condition: false,
                    cancelled_slow,
                },
            });
        }
        Ok(())
    }

    /// Ends a running panic and everything that lasts until then.
    pub(crate) fn stop_panic(&mut self, actor: &str, reason: StopReason) -> Result<()> {
        if self.actor_mut(actor)?.panic.take().is_none() {
            if reason == StopReason::GameMaster {
                return Err(RuleError::NotAllowed(format!(
                    "\"{actor}\" is not panicking"
                )));
            }
            return Ok(());
        }
        self.emit(Event::PanicStopped {
            actor: actor.to_owned(),
            reason,
        });
        self.end_event(actor, EndEvent::PanicStops)
    }
}
