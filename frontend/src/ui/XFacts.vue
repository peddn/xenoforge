<script setup>
// Labels with their values, e.g. on the sheet. A value is text or a list of
// parts with their own tooltip, such as talents with their description.
// Empty values show a dash. `stacked` puts each label above its value, for
// narrow boxes.
defineProps({
  items: { type: Array, required: true },
  stacked: { type: Boolean, default: false },
})
</script>

<template>
  <dl :class="['grid', stacked ? 'gap-0.5' : 'grid-cols-[auto_1fr] gap-x-3 gap-y-1']">
    <template
      v-for="[label, value] in items"
      :key="label"
    >
      <dt class="text-sm text-dim uppercase">
        {{ label }}
      </dt>
      <dd :class="['min-w-0 break-words whitespace-pre-line text-strong', { 'mb-1.5': stacked }]">
        <template v-if="Array.isArray(value)">
          <span
            v-for="(part, i) in value"
            :key="i"
            :title="part.title"
          >{{ part.text }}{{ i < value.length - 1 ? ', ' : '' }}</span>
        </template>
        <template v-else>
          {{ value || '–' }}
        </template>
      </dd>
    </template>
  </dl>
</template>
