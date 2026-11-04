<script setup>
defineProps({
  // normal | primary | danger | quiet; amber ones for what waits on someone:
  // highlight (filled) and notice (outlined)
  variant: { type: String, default: 'normal' },
  size: { type: String, default: 'normal' }, // normal | small
  type: { type: String, default: 'button' },
})

const VARIANTS = {
  normal: 'border-dim text-fg',
  primary: 'border-fg bg-fg text-bg',
  danger: 'border-red text-red',
  quiet: 'border-transparent text-dim',
  highlight: 'border-amber bg-amber text-bg',
  notice: 'border-amber text-amber',
}
const SIZES = { normal: 'px-3 py-1', small: 'px-2 py-0.5 text-sm' }
</script>

<template>
  <button
    :type="type"
    :class="[
      'cursor-pointer border tracking-wide uppercase disabled:cursor-default disabled:opacity-45 aria-pressed:bg-fg aria-pressed:text-bg',
      VARIANTS[variant],
      SIZES[size],
    ]"
  >
    <slot />
  </button>
</template>
