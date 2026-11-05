<script setup>
defineProps({
  title: { type: String, default: '' },
  // The header stays in view while the content scrolls, e.g. in a dialog.
  sticky: { type: Boolean, default: false },
})
</script>

<template>
  <!-- One column that may shrink, so long content wraps or scrolls inside instead of widening the page. -->
  <section class="grid min-w-0 grid-cols-1 content-start gap-3 border border-line bg-raised px-4 py-3">
    <header
      v-if="title || $slots.actions"
      :class="[
        'flex flex-wrap items-center justify-between gap-2 border-b border-dashed border-line pb-1.5',
        sticky && 'sticky top-0 z-10 -mt-3 bg-raised pt-3',
      ]"
    >
      <h2>{{ title }}</h2>
      <slot name="actions" />
    </header>
    <slot />
  </section>
</template>
