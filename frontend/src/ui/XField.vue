<script setup>
defineProps({ label: { type: String, required: true } })
</script>

<template>
  <label class="field">
    <span class="field-label">{{ label }}</span>
    <slot />
  </label>
</template>
