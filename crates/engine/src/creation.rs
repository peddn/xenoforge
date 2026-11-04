//! Creating a Campaign player character in ten steps [p. 27–36] (plan
//! section 5.2).
//!
//! The engine checks a draft and builds the actor from it. The server keeps
//! the draft, shows the problems and rolls the starting cash with its dice.
//! Every limit comes from `constants.json`.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::data::{Id, Page};
use crate::dice::Dice;
use crate::engine::Engine;
use crate::state::Actor;

/// A character being created. Attributes are the 14 points before an
/// android's bonus.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Draft {
    pub career: Option<Id>,
    pub android: bool,
    pub attributes: BTreeMap<Id, u8>,
    /// The two attributes an android raises by 3 (F-50).
    pub android_bonus: Vec<Id>,
    pub skills: BTreeMap<Id, u8>,
    pub talent: Option<Id>,
    /// What the talent names, e.g. the vehicle of Like the Back of Your
    /// Hand.
    pub talent_param: Option<String>,
    pub name: String,
    pub appearance: String,
    pub agenda: String,
    pub signature_item: String,
}

/// The step of the book's order a problem belongs to [p. 27].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CreationStep {
    Career,
    Attributes,
    Skills,
    Talent,
    Name,
    Appearance,
    Agenda,
    SignatureItem,
}

/// Something that keeps a draft from being finished. `code` names the
/// rule; `id`, `value` and `limit` fill in the message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreationProblem {
    pub step: CreationStep,
    pub code: String,
    pub page: Page,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i16>,
}

fn problem(step: CreationStep, code: &str, page: Page) -> CreationProblem {
    CreationProblem {
        step,
        code: code.to_owned(),
        page,
        id: None,
        value: None,
        limit: None,
    }
}

impl CreationProblem {
    fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_owned());
        self
    }

    fn value(mut self, value: i16, limit: i16) -> Self {
        self.value = Some(value);
        self.limit = Some(limit);
        self
    }
}

/// Starting cash as rolled.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cash {
    pub dice: Vec<u8>,
    pub amount: u32,
}

impl Engine {
    /// Everything that keeps the draft from being finished, in the book's
    /// order of steps; empty when it is done.
    pub fn check_creation(&self, draft: &Draft) -> Vec<CreationProblem> {
        use CreationStep as S;
        let mut out = Vec::new();
        let rules = self.rules();
        let career = draft
            .career
            .as_deref()
            .and_then(|c| rules.careers.iter().find(|e| e.id == c));
        match (&draft.career, career) {
            (None, _) => out.push(problem(S::Career, "career_missing", 27)),
            (Some(id), None) => out.push(problem(S::Career, "unknown", 27).id(id)),
            _ => {}
        }

        // Attributes: 14 points, each 2 to 4, the key attribute up to 5
        // [p. 28]; all points spent (F-51).
        let min = self.constant("creation_attribute_min");
        let mut sum = 0;
        for (id, value) in &draft.attributes {
            if !rules.attributes.iter().any(|a| a.id == *id) {
                out.push(problem(S::Attributes, "unknown", 28).id(id));
            }
            sum += i16::from(*value);
        }
        for attribute in &rules.attributes {
            let value = i16::from(draft.attributes.get(&attribute.id).copied().unwrap_or(0));
            let key = career.is_some_and(|c| c.key_attribute == attribute.id);
            let max = self.constant(if key {
                "creation_key_attribute_max"
            } else {
                "creation_attribute_max"
            });
            if value < min || value > max {
                out.push(
                    problem(S::Attributes, "attribute_range", 28)
                        .id(&attribute.id)
                        .value(value, max),
                );
            }
        }
        let points = self.constant("creation_attribute_points");
        if sum != points {
            out.push(problem(S::Attributes, "attribute_points", 28).value(sum, points));
        }

        // Androids: +3 on two different attributes (F-50) [p. 29].
        if draft.android {
            let wanted = self.constant("android_bonus_attributes");
            let count = i16::try_from(draft.android_bonus.len()).unwrap_or(i16::MAX);
            let mut distinct = draft.android_bonus.clone();
            distinct.sort();
            distinct.dedup();
            if count != wanted || distinct.len() != draft.android_bonus.len() {
                out.push(problem(S::Attributes, "android_bonus", 29).value(count, wanted));
            }
            for id in &draft.android_bonus {
                if !rules.attributes.iter().any(|a| a.id == *id) {
                    out.push(problem(S::Attributes, "unknown", 29).id(id));
                }
            }
        }

        // Skills: 10 points, key skills up to 3, the others up to 1
        // [p. 28]; all points spent (F-51).
        let mut sum = 0;
        for (id, level) in &draft.skills {
            if self.skill(id).is_none() {
                out.push(problem(S::Skills, "unknown", 28).id(id));
                continue;
            }
            let key = career.is_some_and(|c| c.key_skills.contains(id));
            let max = self.constant(if key {
                "creation_key_skill_max"
            } else {
                "creation_other_skill_max"
            });
            let level = i16::from(*level);
            if level > max {
                out.push(
                    problem(S::Skills, "skill_range", 28)
                        .id(id)
                        .value(level, max),
                );
            }
            sum += level;
        }
        let points = self.constant("creation_skill_points");
        if sum != points {
            out.push(problem(S::Skills, "skill_points", 28).value(sum, points));
        }

        // One of the three career talents [p. 29, 74].
        match draft.talent.as_deref() {
            None => out.push(problem(S::Talent, "talent_missing", 29)),
            Some(id) => match self.talent(id) {
                None => out.push(problem(S::Talent, "unknown", 29).id(id)),
                Some(t) => {
                    if career.is_some_and(|c| !c.talents.contains(&t.id)) {
                        out.push(problem(S::Talent, "talent_not_career", 74).id(id));
                    }
                    let named = draft
                        .talent_param
                        .as_deref()
                        .is_some_and(|p| !p.trim().is_empty());
                    if t.parameter.is_some() && !named {
                        out.push(problem(S::Talent, "talent_parameter", t.page).id(id));
                    }
                }
            },
        }

        // Name, appearance, agenda, signature item [p. 30, 31, 33]. An
        // android needs no signature item [p. 29].
        let empty = |text: &str| text.trim().is_empty();
        if empty(&draft.name) {
            out.push(problem(S::Name, "name_missing", 30));
        }
        if empty(&draft.appearance) {
            out.push(problem(S::Appearance, "appearance_missing", 30));
        }
        if empty(&draft.agenda) {
            out.push(problem(S::Agenda, "agenda_missing", 31));
        }
        if !draft.android && empty(&draft.signature_item) {
            out.push(problem(S::SignatureItem, "signature_item_missing", 33));
        }
        out
    }

    /// The actor a draft describes: attributes with an android's bonus,
    /// skills, the talent, full Health and STRESS LEVEL 0 [p. 28–30].
    /// Values the check rejects are taken as they are; finishing needs a
    /// clean check.
    pub fn creation_actor(&self, draft: &Draft) -> Actor {
        let mut actor = Actor::pc();
        actor.android = draft.android;
        let bonus = u8::try_from(self.constant("android_attribute_bonus")).unwrap_or(0);
        for attribute in &self.rules().attributes {
            let mut value = draft.attributes.get(&attribute.id).copied().unwrap_or(0);
            if draft.android && draft.android_bonus.contains(&attribute.id) {
                value = value.saturating_add(bonus);
            }
            actor.attributes.insert(attribute.id.clone(), value);
        }
        for (id, level) in &draft.skills {
            if *level > 0 && self.skill(id).is_some() {
                actor.skills.insert(id.clone(), *level);
            }
        }
        if let Some(talent) = &draft.talent {
            actor.talents.insert(talent.clone());
            let named = self.talent(talent).is_some_and(|t| t.parameter.is_some());
            if let Some(param) = draft.talent_param.as_deref().map(str::trim) {
                if named && !param.is_empty() {
                    actor
                        .talent_params
                        .insert(talent.clone(), vec![param.to_owned()]);
                }
            }
        }
        actor.health = self.full_health(&actor);
        actor
    }

    /// Rolls the starting cash of a career [p. 39–55].
    pub fn roll_cash(&self, career: &str, dice: &mut dyn Dice) -> Option<Cash> {
        let career = self.rules().careers.iter().find(|c| c.id == career)?;
        let faces: Vec<u8> = (0..career.cash.dice).map(|_| dice.d6()).collect();
        let sum: u32 = faces.iter().map(|f| u32::from(*f)).sum();
        Some(Cash {
            dice: faces,
            amount: sum.saturating_mul(career.cash.times),
        })
    }
}
