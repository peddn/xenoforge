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
</script>

<template>
  <span
    :class="['die', `die-${die.kind}`, { 'die-success': success, 'die-facehugger': facehugger, 'die-locked': die.locked }]"
    :title="label"
    :aria-label="label"
  >{{ success ? '✶' : facehugger ? '☣' : die.face }}</span>
</template>
