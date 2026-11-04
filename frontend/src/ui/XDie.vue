<script setup>
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'

// One die: a 6 is a success, a 1 on a Stress Die a facehugger [p. 22].
const props = defineProps({ die: { type: Object, required: true } })
const { t } = useI18n()
const success = computed(() => props.die.face === 6)
const facehugger = computed(() => props.die.kind === 'stress' && props.die.face === 1)
const label = computed(() => {
  const kind = t(props.die.kind === 'stress' ? 'dice.stress' : 'dice.base')
  const mark = success.value ? t('dice.success') : facehugger.value ? t('dice.facehugger') : ''
  return `${kind} ${props.die.face}${mark ? `, ${mark}` : ''}`
})
const look = computed(() => {
  const stress = props.die.kind === 'stress'
  if (facehugger.value) return 'border-red bg-red text-bg'
  if (success.value) return `${stress ? 'border-amber' : 'border-dim'} bg-fg text-bg`
  return stress ? 'border-dashed border-amber text-amber' : 'border-dim'
})
</script>

<template>
  <span
    :class="['inline-grid size-7 place-items-center border', look, { 'outline-1 outline-offset-2 outline-strong outline-dotted': die.locked }]"
    :title="label"
    :aria-label="label"
  >{{ success ? '✶' : facehugger ? '☣' : die.face }}</span>
</template>
