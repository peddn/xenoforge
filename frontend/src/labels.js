// Turns engine data (choices, origins, events) into text. Rule names come
// from the rule data texts, interface words from vue-i18n.

export function choiceLabel(choice, t, rules) {
  if (typeof choice === 'string') return t(`choice.${choice}`)
  if ('use_talent' in choice) return t('choice.use_talent', { talent: rules.name(choice.use_talent) })
  if ('answer' in choice) return t(choice.answer ? 'choice.yes' : 'choice.no')
  return t('choice.targets')
}

/** Entry id behind an origin such as {"injury":"broken_ribs"}. */
export function originEntry(origin) {
  const [kind, value] = Object.entries(origin)[0]
  if (kind === 'condition') return value.id
  if (kind === 'block') return value.by
  if (kind === 'part') return originEntry(value.of)
  return value
}

export function originLabel(origin, t, rules) {
  const kind = Object.keys(origin)[0]
  const name = rules.name(originEntry(origin))
  if (kind === 'block') return t('origin.block', { name, blocks: blockLabel(origin.block.blocks, t, rules) })
  if (kind === 'condition' && origin.condition.cold) return `${name} (${t(`cold.${origin.condition.cold}`)})`
  return name
}

export function blockLabel(blocks, t, rules) {
  if (typeof blocks === 'string') return t(`blocks.${blocks}`)
  return rules.name(blocks.action)
}

// Sources: effect labels such as "talent/stoic", ids of difficulty levels,
// and "gear" or "help" from the roll request.
const requestSources = { gear: 'roll.gear', help: 'roll.helpers' }

export function modsText(mods, t, rules) {
  return mods
    .map((m) => {
      const id = m.source.split('/').pop()
      const source = requestSources[m.source] ? t(requestSources[m.source]) : rules.name(id)
      return `${source} ${m.value > 0 ? '+' : ''}${m.value}`
    })
    .join(', ')
}
