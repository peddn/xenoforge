<script setup>
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'

import XFacts from '@/ui/XFacts.vue'
import { useRulesStore } from '@/stores/rules'

// The character sheet to read, in the order of the printed one [p. 389]:
// texts, attributes with their skills, talents. Gear follows with 2c.
const props = defineProps({
  sheet: { type: Object, required: true },
  actor: { type: Object, required: true },
  // Full Health from the server, if known.
  healthMax: { type: Number, default: null },
  startCash: { type: Object, default: null },
  // Names of the team's characters; without them buddy and rival are left out.
  names: { type: Object, default: null },
  // Without the texts, where they are edited next to the sheet.
  texts: { type: Boolean, default: true },
})
const { t } = useI18n()
const rules = useRulesStore()

const facts = computed(() => {
  const list = [[t('sheet.career'), props.sheet.career ? rules.name(props.sheet.career) : '']]
  if (props.texts) {
    list.push([t('sheet.appearance'), props.sheet.appearance], [t('sheet.agenda'), props.sheet.agenda])
  }
  if (props.names) {
    list.push([t('sheet.buddy'), props.names[props.sheet.buddy] ?? ''])
    list.push([t('sheet.rival'), props.names[props.sheet.rival] ?? ''])
  }
  if (props.texts) list.push([t('sheet.signatureItem'), props.sheet.signature_item])
  const health = props.healthMax == null ? props.actor.health : `${props.actor.health}/${props.healthMax}`
  list.push([t('sheet.health'), String(health ?? 0)])
  if (!props.actor.android) list.push([t('sheet.stress'), String(props.actor.stress ?? 0)])
  if (props.startCash) {
    list.push([
      t('sheet.startCash'),
      t('sheet.cashRolled', { amount: props.startCash.amount, dice: props.startCash.dice.join(', ') }),
    ])
  }
  return list
})

const skillsOf = (attribute) => rules.list('skills').filter((s) => s.attribute === attribute)
const talentText = (id) => {
  const param = props.actor.talent_params?.[id]
  return param?.length ? `${rules.name(id)} (${param.join(', ')})` : rules.name(id)
}
</script>

<template>
  <div class="grid gap-3">
    <XFacts :items="facts" />
    <div class="grid cols-auto items-start gap-3">
      <div
        v-for="attribute in rules.list('attributes')"
        :key="attribute.id"
        class="grid content-start gap-1 border border-line p-3"
        :data-test="`sheet-${attribute.id}`"
      >
        <div class="flex justify-between gap-2 uppercase">
          <span>{{ rules.name(attribute.id) }}</span>
          <span class="text-strong">{{ actor.attributes?.[attribute.id] ?? 0 }}</span>
        </div>
        <div
          v-for="skill in skillsOf(attribute.id)"
          :key="skill.id"
          class="flex justify-between gap-2 text-sm"
        >
          <span>{{ rules.name(skill.id) }}</span>
          <span :class="actor.skills?.[skill.id] ? 'text-strong' : 'text-dim'">{{ actor.skills?.[skill.id] ?? 0 }}</span>
        </div>
      </div>
    </div>
    <div class="grid gap-1">
      <span class="text-sm text-dim uppercase">{{ t('sheet.talents') }}</span>
      <ul class="grid gap-1">
        <li
          v-for="id in actor.talents ?? []"
          :key="id"
          :title="rules.text(`${id}.desc`)"
        >
          {{ talentText(id) }}
        </li>
        <li
          v-if="!actor.talents?.length"
          class="text-dim"
        >
          –
        </li>
      </ul>
    </div>
  </div>
</template>
