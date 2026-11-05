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
  if (kind === 'used') return value.item
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

/** What an open decision is about, e.g. "Roll: push or accept?". */
export function decisionText(pending, t, rules) {
  if (['roll', 'start_roll', 'overkill'].includes(pending.kind)) return t(`decision.kind.${pending.kind}`)
  if (pending.question) return rules.text(pending.question)
  if (pending.who) return t('decision.targets', { who: t(`who.${pending.who}`) })
  return t('decision.title')
}

/** What happens to each one chosen, e.g. "Stress −1" or "Panic Roll". */
export function opsText(ops, t) {
  return ops
    .map((op) => {
      if (op.op === 'stress') return t('op.stress', { delta: `${op.delta > 0 ? '+' : '−'}${Math.abs(op.delta)}` })
      if (op.op === 'damage') return t('op.damage', { amount: op.amount })
      return t(`op.${op.op}`)
    })
    .join(', ')
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

/** A problem of a draft as the engine names it, with the book page. */
export function problemText(problem, t, rules) {
  const params = {
    name: problem.id ? rules.name(problem.id) : '',
    value: problem.value,
    limit: problem.limit,
    min: rules.constant('creation_attribute_min'),
  }
  if (problem.code === 'talent_parameter') {
    params.kind = t(`wizard.parameter.${rules.entry('talents', problem.id)?.parameter ?? 'vehicle'}`)
  }
  return `${t(`wizard.problem.${problem.code}`, params)} (${t('wizard.page', { page: problem.page })})`
}

/** Starting cash of a career as in the book, e.g. "$2D6 × 100". */
export function cashFormula(cash, t) {
  const dice = `${cash.dice > 1 ? cash.dice : ''}${t('wizard.die')}`
  return t('wizard.cashFormula', { dice, times: cash.times > 1 ? ` × ${cash.times}` : '' })
}

const FRACTIONS = ['', '¼', '½', '¾']

/** Quarters of a regular item as the book writes weights, e.g. 14 → "3½". */
export function quarters(q) {
  const whole = Math.floor(q / 4)
  const part = FRACTIONS[q % 4]
  if (!whole) return part || '0'
  return `${whole}${part}`
}

/** A weight from the gear catalog, e.g. "1/2" → "½" [p. 34]. */
export function weightText(weight) {
  return { '1/4': '¼', '1/2': '½' }[weight] ?? weight ?? '–'
}

/** Name of an item: from the catalog, or the name of a free item. */
export function itemName(possession, rules) {
  return possession.item ? rules.name(possession.item) : (possession.name ?? '')
}

/** Where an item went, for the log, e.g. "worn" or {"character":"5"}. */
export function destinationLabel(to, t, names) {
  if (typeof to === 'string') return t(`log.to.${to}`)
  return t('log.toCharacter', { name: names[to.character] ?? `#${to.character}` })
}

/** An amount in cents as dollars, e.g. 123400 → "$1,234" or "1.234 $". */
export function moneyText(cents, locale) {
  const whole = cents % 100 === 0
  return new Intl.NumberFormat(locale, {
    style: 'currency',
    currency: 'USD',
    minimumFractionDigits: whole ? 0 : 2,
    maximumFractionDigits: whole ? 0 : 2,
  }).format(cents / 100)
}

/**
 * A sum of money as typed, in cents: "1.234,50" in German, "1,234.50" in
 * English, and "12.5" or "12,5" either way; "$", spaces and "−" are fine.
 * Null when empty, NaN when it is no sum.
 */
export function parseMoney(text, locale) {
  let s = String(text ?? '')
    .replace(/[\s$]/g, '')
    .replace(/\u2212/g, '-')
  if (!s) return null
  const sign = s[0] === '-' ? -1 : 1
  if (s[0] === '-' || s[0] === '+') s = s.slice(1)
  const decimal = String(locale).startsWith('de') ? ',' : '.'
  const group = decimal === ',' ? '.' : ','
  let whole = s
  let fraction = ''
  const at = s.lastIndexOf(decimal)
  if (at >= 0) {
    whole = s.slice(0, at)
    fraction = s.slice(at + 1)
  } else {
    // Only the other mark: with one or two digits after it, the decimals
    // as typed in the other language; otherwise thousands.
    const parts = s.split(group)
    if (parts.length === 2 && /^\d{1,2}$/.test(parts[1])) [whole, fraction] = parts
  }
  const grouped = new RegExp(`^(\\d*|\\d{1,3}(\\${group}\\d{3})+)$`)
  if (!grouped.test(whole) || !/^\d{0,2}$/.test(fraction) || !(whole || fraction)) return NaN
  const cents = Number(whole.replaceAll(group, '') || 0) * 100 + Number(fraction.padEnd(2, '0'))
  if (!Number.isSafeInteger(cents)) return NaN
  return cents === 0 ? 0 : sign * cents
}

/** A value a correction changed, e.g. "Stress 1 → 3" (plan 10.2). */
export function changeText(change, t, rules) {
  const [group, key] = change.path.split('.')
  const named = { health: 'sheet.health', stress: 'sheet.stress', talents: 'sheet.talents', android: 'sheet.android' }
  let label = change.path
  if ((group === 'attributes' || group === 'skills') && key) label = rules.name(key)
  else if (group === 'supplies' && key) label = t(`supply.${key}`)
  else if (named[group] && !key) label = t(named[group])
  const value = (v) => {
    if (v == null) return '–'
    if (typeof v === 'boolean') return t(v ? 'choice.yes' : 'choice.no')
    if (Array.isArray(v)) return v.length ? v.map((x) => (typeof x === 'string' ? rules.name(x) : JSON.stringify(x))).join(', ') : '–'
    if (typeof v === 'object') return JSON.stringify(v)
    return String(v)
  }
  return `${label} ${value(change.from)} → ${value(change.to)}`
}

/** The balance: the sum of the bookings, in cents. */
export function balance(ledger) {
  return (ledger ?? []).reduce((sum, b) => sum + b.amount, 0)
}

/** Why money came in or went out. */
export function bookingReason(reason, t, rules) {
  if (reason.kind === 'start') return t('money.start')
  if (reason.kind === 'note') return reason.text
  if (reason.kind === 'reloads') return t('money.reloads', { item: rules.name(reason.item), count: reason.count })
  const item = reason.item ? rules.name(reason.item) : (reason.name ?? '')
  if (reason.reloads) return t('money.purchaseReloads', { item, count: reason.count, reloads: reason.reloads })
  return t('money.purchase', { item, count: reason.count })
}
