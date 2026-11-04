<script setup>
defineProps({ title: { type: String, default: '' } })
</script>

<template>
  <section class="grid min-w-0 content-start gap-3 border border-line bg-raised px-4 py-3">
    <header
      v-if="title || $slots.actions"
      class="flex flex-wrap items-center justify-between gap-2 border-b border-dashed border-line pb-1.5"
    >
      <h2>{{ title }}</h2>
      <slot name="actions" />
    </header>
    <slot />
  </section>
</template>
