<script setup>
import { computed, nextTick, onMounted, onUnmounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'

import XDie from '@/ui/XDie.vue'
import XStatus from '@/ui/XStatus.vue'
import { choiceLabel, modsText, originLabel } from '@/labels'
import { useRulesStore } from '@/stores/rules'
import { useTeamStore } from '@/stores/team'

// The shared log: every roll with each die and each modification with its
// source (plan 4.2). Everything one command caused forms one block.
const { t, locale } = useI18n()
const rules = useRulesStore()
const team = useTeamStore()
const box = ref(null)

const name = (id) => team.characterNames[id] ?? (id ? `#${id}` : t('auth.gm'))

// Which character a roll belongs to, from its start event.
const rollActors = computed(() => {
  const map = {}
  for (const entry of team.log) {
    const event = entry.entry.event
    if (event?.event === 'roll_started') map[event.roll] = event.actor
  }
  return map
})

function what(event) {
  if (event.skill) return rules.name(event.skill)
  if (event.attribute) return rules.name(event.attribute)
  return t('log.pool')
}

function describe(entry) {
  const e = entry.entry
  if (e.type !== 'engine') return { text: t(`log.${e.type}`, { name: e.name ?? '' }) }
  const ev = e.event
  const actor = name(ev.actor ?? rollActors.value[ev.roll])
  switch (ev.event) {
    case 'roll_started':
      return {
        text: t('log.roll_started', { actor, what: what(ev), base: ev.base_dice, stress: ev.stress_dice }),
        detail: modsText(ev.modifications, t, rules),
      }
    case 'dice_thrown':
      return { dice: ev.dice }
    case 'roll_finished':
      return {
        status: { tone: ev.outcome === 'success' ? 'ok' : 'warn', text: t(`outcome.${ev.outcome}`) },
        text: `${t('log.successes', ev.successes)}, ${t('log.stunts', ev.stunts)}`,
        detail: ev.against != null ? t('log.against', { against: t('log.successes', ev.against) }) : '',
      }
    case 'decision':
      return { text: t('log.decision', { actor, options: ev.options.map((o) => choiceLabel(o, t, rules)).join(' / ') }) }
    case 'question':
      return { text: t('log.question', { actor, question: rules.text(ev.question) }) }
    case 'choose_targets':
      return { text: t('log.choose_targets', { who: t(`who.${ev.who}`) }) }
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
    case 'action_performed':
      return { text: t('log.action_performed', { actor, action: rules.name(ev.action) }) }
    case 'trauma_check':
      return { dice: ev.dice, text: ev.trauma ? t('log.trauma', { actor, trauma: rules.name(ev.trauma) }) : t('log.no_trauma', { actor }) }
    default:
      return { text: t(`log.${ev.event}`, { ...ev, actor }) }
  }
}

const time = (at) => new Date(at * 1000).toLocaleTimeString(locale.value, { hour: '2-digit', minute: '2-digit' })

// Entries of one command are consecutive; older ones without a batch stand
// alone.
const blocks = computed(() => {
  const list = []
  for (const entry of team.log) {
    const batch = entry.batch ?? entry.id
    let block = list.at(-1)
    if (!block || block.batch !== batch) {
      block = { batch, by: entry.by, at: time(entry.at), note: entry.note, rows: [] }
      list.push(block)
    }
    block.rows.push({ id: entry.id, ...describe(entry) })
  }
  return list
})

// The log keeps to its newest entry, also when its height changes (the
// decision bar comes and goes), unless someone scrolled up to read.
let atBottom = true

function onScroll() {
  const b = box.value
  atBottom = b.scrollHeight - b.scrollTop - b.clientHeight < 24
}

function keepAtBottom() {
  if (box.value && atBottom) box.value.scrollTop = box.value.scrollHeight
}

watch(
  () => team.log.length,
  async () => {
    await nextTick()
    keepAtBottom()
  },
)

let resize = null
onMounted(() => {
  if (typeof ResizeObserver === 'undefined') return // test environments
  resize = new ResizeObserver(keepAtBottom)
  resize.observe(box.value)
})
onUnmounted(() => resize?.disconnect())
</script>

<template>
  <div
    ref="box"
    class="grid max-h-[60vh] min-h-0 content-start gap-2 overflow-y-auto lg:max-h-none"
    aria-live="polite"
    data-test="log"
    @scroll="onScroll"
  >
    <section
      v-for="block in blocks"
      :key="block.batch"
      class="grid gap-1.5 border border-l-[3px] border-line border-l-dim bg-bg px-2.5 py-1.5"
      data-test="log-block"
    >
      <header class="text-xs text-dim">
        {{ block.by ?? '' }} · {{ block.at }}
      </header>
      <p
        v-if="block.note"
        class="text-strong"
        data-test="log-note"
      >
        {{ t('log.note', { note: block.note }) }}
      </p>
      <div
        v-for="row in block.rows"
        :key="row.id"
        class="grid gap-1"
        data-test="log-entry"
      >
        <div v-if="row.status || row.text">
          <XStatus
            v-if="row.status"
            :tone="row.status.tone"
            :text="row.status.text"
          />
          {{ row.text }}
        </div>
        <div
          v-if="row.dice"
          class="flex flex-wrap gap-1"
        >
          <XDie
            v-for="(die, i) in row.dice"
            :key="i"
            :die="die"
          />
        </div>
        <div
          v-if="row.detail"
          class="text-xs text-dim"
        >
          {{ row.detail }}
        </div>
      </div>
    </section>
  </div>
</template>
