<script setup>
defineProps({ label: { type: String, required: true } })
</script>

<template>
  <label class="grid min-w-0 max-w-full grid-cols-1 gap-1">
    <span class="text-sm text-dim uppercase">{{ label }}</span>
    <slot />
  </label>
</template>
