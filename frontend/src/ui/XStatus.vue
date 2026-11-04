<script setup>
// A status code in capitals. The symbol carries the meaning as well as the
// colour (plan 11.5).
const SYMBOLS = { ok: '●', info: '·', warn: '▲', alert: '▲▲' }
defineProps({
  tone: { type: String, default: 'info' },
  text: { type: String, required: true },
})
</script>

<template>
  <span :class="['status', `status-${tone}`]">
    <span aria-hidden="true">{{ SYMBOLS[tone] }}</span>
    {{ text }}
  </span>
</template>
