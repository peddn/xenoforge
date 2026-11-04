<script setup>
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'

import XField from '@/ui/XField.vue'
import { useRulesStore } from '@/stores/rules'

// Edits the values that count for the rules. In phase 2a they are entered
// freely; the creation wizard with rule checks follows in 2b.
const actor = defineModel({ type: Object, required: true })
defineProps({ disabled: { type: Boolean, default: false } })

const { t } = useI18n()
const rules = useRulesStore()
const attributes = computed(() => rules.list('attributes'))
const skillsOf = (attribute) => rules.list('skills').filter((s) => s.attribute === attribute)

function setNumber(group, id, value) {
  const number = Math.max(0, Math.trunc(Number(value) || 0))
  actor.value = { ...actor.value, [group]: { ...actor.value[group], [id]: number } }
}

function toggleTalent(id, on) {
  const talents = new Set(actor.value.talents ?? [])
  if (on) talents.add(id)
  else talents.delete(id)
  actor.value = { ...actor.value, talents: [...talents].sort() }
}
</script>

<template>
  <div class="grid cols-auto items-start gap-3">
    <div
      v-for="attribute in attributes"
      :key="attribute.id"
      class="grid content-start gap-3 border border-line p-3"
    >
      <XField :label="rules.name(attribute.id)">
        <input
          type="number"
          min="0"
          :disabled="disabled"
          :value="actor.attributes?.[attribute.id] ?? 0"
          :data-test="`attr-${attribute.id}`"
          @input="setNumber('attributes', attribute.id, $event.target.value)"
        >
      </XField>
      <XField
        v-for="skill in skillsOf(attribute.id)"
        :key="skill.id"
        :label="rules.name(skill.id)"
      >
        <input
          type="number"
          min="0"
          :disabled="disabled"
          :value="actor.skills?.[skill.id] ?? 0"
          :data-test="`skill-${skill.id}`"
          @input="setNumber('skills', skill.id, $event.target.value)"
        >
      </XField>
    </div>
  </div>
  <div class="flex flex-wrap items-center gap-2">
    <XField :label="t('sheet.health')">
      <input
        type="number"
        min="0"
        :disabled="disabled"
        :value="actor.health ?? 0"
        @input="actor = { ...actor, health: Math.max(0, Math.trunc(Number($event.target.value) || 0)) }"
      >
    </XField>
    <XField :label="t('sheet.stress')">
      <input
        type="number"
        min="0"
        :disabled="disabled"
        :value="actor.stress ?? 0"
        @input="actor = { ...actor, stress: Math.max(0, Math.trunc(Number($event.target.value) || 0)) }"
      >
    </XField>
    <label class="flex flex-wrap items-center gap-2">
      <input
        type="checkbox"
        :disabled="disabled"
        :checked="actor.android"
        @change="actor = { ...actor, android: $event.target.checked }"
      >
      {{ t('sheet.android') }}
    </label>
  </div>
  <fieldset class="grid content-start gap-3 border border-line p-3">
    <legend class="text-sm text-dim uppercase">
      {{ t('sheet.talents') }}
    </legend>
    <div class="flex flex-wrap items-center gap-2">
      <label
        v-for="talent in rules.list('talents')"
        :key="talent.id"
        class="flex flex-wrap items-center gap-2"
        :title="rules.text(`${talent.id}.desc`)"
      >
        <input
          type="checkbox"
          :disabled="disabled"
          :checked="(actor.talents ?? []).includes(talent.id)"
          @change="toggleTalent(talent.id, $event.target.checked)"
        >
        {{ rules.name(talent.id) }}
      </label>
    </div>
  </fieldset>
</template>
