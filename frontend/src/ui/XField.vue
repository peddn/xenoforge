<script setup>
defineProps({ label: { type: String, required: true } })
</script>

<template>
  <label class="grid gap-1">
    <span class="text-sm text-dim uppercase">{{ label }}</span>
    <slot />
  </label>
</template>
