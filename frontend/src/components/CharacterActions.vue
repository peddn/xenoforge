<script setup>
import { computed, ref } from 'vue'
import { useI18n } from 'vue-i18n'

import XButton from '@/ui/XButton.vue'
import XField from '@/ui/XField.vue'
import { originLabel } from '@/labels'
import { useRulesStore } from '@/stores/rules'
import { useTeamStore } from '@/stores/team'

// The game master's actions for one character (plan section 3): engine
// commands by hand.
const props = defineProps({ character: { type: Object, required: true } })
const { t } = useI18n()
const rules = useRulesStore()
const team = useTeamStore()

const amount = ref(1)
const sourceKind = ref('trauma')
const sourceId = ref('')
const cold = ref('sub_zero')
const error = ref('')

const actor = computed(() => String(props.character.id))
const entries = computed(() =>
  rules.list({ trauma: 'traumas', condition: 'conditions', injury: 'critical-injuries' }[sourceKind.value]),
)
// Whether the chosen condition asks for a cold level is read from its rule
// data; the engine applies it.
const needsCold = computed(() => {
  if (sourceKind.value !== 'condition') return false
  const condition = entries.value.find((c) => c.id === sourceId.value)
  return (condition?.effects ?? []).some((e) => e.kind === 'periodic' && e.every?.by_cold)
})

async function run(command) {
  error.value = ''
  try {
    await team.command(command)
  } catch (e) {
    error.value = e.message
  }
}

function apply() {
  const id = sourceId.value || entries.value[0]?.id
  const source =
    sourceKind.value === 'condition'
      ? { condition: { id, cold: needsCold.value ? cold.value : null } }
      : { [sourceKind.value]: id }
  run({ command: 'apply', actor: actor.value, source })
}
</script>

<template>
  <div class="stack">
    <div class="row">
      <XField :label="t('gm.amount')">
        <input
          v-model.number="amount"
          type="number"
          min="1"
        >
      </XField>
      <XButton
        data-test="stress-up"
        @click="run({ command: 'change_stress', actors: [actor], delta: amount })"
      >
        {{ t('gm.stressUp') }}
      </XButton>
      <XButton @click="run({ command: 'change_stress', actors: [actor], delta: -amount })">
        {{ t('gm.stressDown') }}
      </XButton>
      <XButton @click="run({ command: 'damage', actor, amount })">
        {{ t('gm.damage') }}
      </XButton>
    </div>
    <div class="row">
      <XButton @click="run({ command: 'panic_roll', actor })">
        {{ t('gm.panicRoll') }}
      </XButton>
      <XButton @click="run({ command: 'stop_panic', actor })">
        {{ t('gm.stopPanic') }}
      </XButton>
      <XButton @click="run({ command: 'use_signature_item', actor })">
        {{ t('gm.signatureItem') }}
      </XButton>
    </div>
    <div class="row">
      <XField :label="t('gm.source')">
        <select
          v-model="sourceKind"
          @change="sourceId = ''"
        >
          <option value="trauma">
            {{ t('gm.trauma') }}
          </option>
          <option value="condition">
            {{ t('gm.condition') }}
          </option>
          <option value="injury">
            {{ t('gm.injury') }}
          </option>
        </select>
      </XField>
      <XField :label="t('gm.entry')">
        <select v-model="sourceId">
          <option
            v-for="entry in entries"
            :key="entry.id"
            :value="entry.id"
          >
            {{ rules.name(entry.id) }}
          </option>
        </select>
      </XField>
      <XField
        v-if="needsCold"
        :label="t('gm.cold')"
      >
        <select v-model="cold">
          <option
            v-for="level in ['above_freezing', 'sub_zero', 'deep_space']"
            :key="level"
            :value="level"
          >
            {{ t(`cold.${level}`) }}
          </option>
        </select>
      </XField>
      <XButton @click="apply">
        {{ t('gm.apply') }}
      </XButton>
    </div>
    <ul
      v-if="character.actor?.sources?.length"
      class="list"
    >
      <li
        v-for="(source, i) in character.actor.sources"
        :key="i"
        class="row"
      >
        {{ originLabel(source.origin, t, rules) }}
        <XButton
          variant="quiet"
          @click="run({ command: 'remove', actor, origin: source.origin })"
        >
          {{ t('common.remove') }}
        </XButton>
      </li>
    </ul>
    <p
      v-if="error"
      class="error"
      role="alert"
    >
      {{ error }}
    </p>
  </div>
</template>
