import { computed } from 'vue'
import { useI18n } from 'vue-i18n'

import { bookingReason, changeText, choiceLabel, destinationLabel, itemName, modsText, moneyText, originLabel } from '@/labels'
import { useRulesStore } from '@/stores/rules'
import { useTeamStore } from '@/stores/team'

// How log entries read: one line, maybe dice, a status and a detail per
// entry (plan 4.2). The log shows them; the game master's undo names the
// step it takes back with them.
export function useLogText() {
  const { t, locale } = useI18n()
  const rules = useRulesStore()
  const team = useTeamStore()

  const name = (id) => team.characterNames[id] ?? (id ? `#${id}` : t('auth.gm'))

  // The start of each roll: whose it is and what is rolled.
  const rollStarts = computed(() => {
    const map = {}
    for (const entry of team.log) {
      const event = entry.entry.event
      if (event?.event === 'roll_started') map[event.roll] = event
    }
    return map
  })

  function what(event) {
    if (event.death) return t('roll.death')
    if (event.skill) return rules.name(event.skill)
    if (event.attribute) return rules.name(event.attribute)
    return t('log.pool')
  }

  function describe(entry) {
    const e = entry.entry
    if (e.type === 'changed') {
      return { text: t('log.changed', { name: e.name, changes: e.changes.map((c) => changeText(c, t, rules)).join(', ') }) }
    }
    if (e.type !== 'engine') {
      const key = e.type === 'character_offered' && !e.offered ? 'character_unoffered' : e.type
      return { text: t(`log.${key}`, { ...e, by: entry.by ?? '', name: e.name ?? '' }) }
    }
    const ev = e.event
    const actor = name(ev.actor ?? rollStarts.value[ev.roll]?.actor)
    switch (ev.event) {
      case 'roll_started':
        return {
          text: t('log.roll_started', { actor, what: what(ev), base: ev.base_dice, stress: ev.stress_dice }),
          detail: modsText(ev.modifications, t, rules),
        }
      case 'dice_thrown':
        return { dice: ev.dice }
      case 'roll_finished': {
        // After a decision the block is the player's: name whose roll it was.
        const start = rollStarts.value[ev.roll]
        const counted = ev.stunts ? `${t('log.successes', ev.successes)}, ${t('log.stunts', ev.stunts)}` : t('log.successes', ev.successes)
        return {
          status: { tone: ev.outcome === 'success' ? 'ok' : 'warn', text: t(`outcome.${ev.outcome}`) },
          // Skills without a stunt list count every success [p. 71].
          text: start ? t('log.roll_result', { actor, what: what(start), result: counted }) : counted,
          detail: ev.against != null ? t('log.against', { against: t('log.successes', ev.against) }) : '',
        }
      }
      case 'decision':
        return { text: t('log.decision', { actor, options: ev.options.map((o) => choiceLabel(o, t, rules)).join(' / ') }) }
      case 'question':
        return { text: t('log.question', { actor, question: rules.text(ev.question) }) }
      case 'choose_targets':
        // Older entries do not name the source.
        if (!ev.source) return { text: t('log.choose_targets', { who: t(`who.${ev.who}`) }) }
        return { text: t('log.choose_targets_from', { source: rules.name(ev.source), actor, who: t(`who.${ev.who}`) }) }
      case 'targets_chosen':
        return {
          text: t('log.targets_chosen', {
            source: rules.name(ev.source),
            actor,
            targets: ev.targets.length ? ev.targets.map(name).join(', ') : t('log.nobody'),
          }),
        }
      case 'stress_changed':
        return { text: t('log.stress_changed', { actor, from: ev.from, to: ev.to }) }
      case 'damaged':
        return { text: t('log.damaged', { actor, amount: ev.amount, health: ev.health }) }
      case 'broken':
        return { status: { tone: 'alert', text: t('crew.broken') }, text: actor }
      case 'panic_rolled':
        return {
          text: t('log.panic_rolled', { actor, d6: ev.d6, stress: ev.stress, value: ev.value }),
          detail: [modsText(ev.modifications, t, rules), ev.raised_from != null ? t('log.raised', { from: ev.raised_from }) : '']
            .filter(Boolean)
            .join(' · '),
        }
      case 'panic_effect':
        return { status: { tone: 'warn', text: rules.name(ev.effect) }, text: actor }
      case 'panic_stopped':
        return { text: t('log.panic_stopped', { actor }) }
      case 'source_applied':
        return { text: t('log.source_applied', { actor, source: originLabel(ev.origin, t, rules) }) }
      case 'source_ended':
        return { text: t('log.source_ended', { actor, source: originLabel(ev.origin, t, rules) }) }
      case 'hint':
        return { text: `${actor}: ${rules.text(ev.text)}` }
      case 'talent_used':
        return { text: t('log.talent_used', { actor, talent: rules.name(ev.talent) }) }
      case 'story_point_awarded':
        return { text: t('log.story_point_awarded', { player: team.personNames[ev.player] ?? `#${ev.player}` }) }
      case 'story_points_changed':
        return { text: t('log.story_points_changed', { player: team.personNames[ev.player] ?? `#${ev.player}`, from: ev.from, to: ev.to }) }
      case 'act_ended':
        return { text: ev.act ? t('log.act_ended_n', { act: ev.act }) : t('log.act_ended') }
      case 'action_performed':
        return { text: t('log.action_performed', { actor, action: rules.name(ev.action) }) }
      case 'item_added':
        return { text: t('log.item_added', { actor, item: itemName(ev, rules), count: ev.count }) }
      case 'item_moved':
        return {
          text: t('log.item_moved', {
            actor,
            item: itemName(ev, rules),
            count: ev.count,
            to: destinationLabel(ev.to, t, team.characterNames),
          }),
        }
      case 'item_used':
        return { text: t('log.item_used', { actor, item: rules.name(ev.item) }) }
      case 'item_refilled':
        return { text: t('log.item_refilled', { actor, item: rules.name(ev.item), from: ev.from, to: ev.to }) }
      case 'supply_changed':
        return { text: t('log.supply_changed', { actor, supply: t(`supply.${ev.supply}`), from: ev.from, to: ev.to }) }
      case 'supply_rolled': {
        const params = { actor, supply: t(`supply.${ev.supply}`), item: ev.item ? rules.name(ev.item) : '', from: ev.from, to: ev.to }
        return { dice: ev.dice, text: t(ev.item ? 'log.supply_rolled_item' : 'log.supply_rolled', params) }
      }
      case 'booked':
        return {
          text: t('log.booked', {
            actor,
            amount: `${ev.booking.amount > 0 ? '+' : ''}${moneyText(ev.booking.amount, locale.value)}`,
            reason: bookingReason(ev.booking.reason, t, rules),
            balance: moneyText(ev.balance, locale.value),
          }),
        }
      case 'reloads_added':
        return { text: t('log.reloads_added', { actor, item: rules.name(ev.item), count: ev.count }) }
      case 'item_used_up':
        return { text: t('log.item_used_up', { actor, item: itemName(ev, rules) }) }
      case 'trauma_check':
        return {
          dice: ev.dice,
          text: ev.trauma ? t('log.trauma', { actor, trauma: rules.name(ev.trauma), d6: ev.d6 ?? '–' }) : t('log.no_trauma', { actor }),
        }
      default:
        return { text: t(`log.${ev.event}`, { ...ev, actor }) }
    }
  }

  return { describe, name, what, time: (at) => new Date(at * 1000).toLocaleTimeString(locale.value, { hour: '2-digit', minute: '2-digit' }) }
}
