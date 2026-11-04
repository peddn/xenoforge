<script setup>
defineProps({ title: { type: String, default: '' } })
</script>

<template>
  <section class="panel">
    <header
      v-if="title || $slots.actions"
      class="panel-title"
    >
      <h2>{{ title }}</h2>
      <slot name="actions" />
    </header>
    <slot />
  </section>
</template>
