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
// On phones a little taller, so a finger hits them.
const SIZES = { normal: 'px-3 py-1 max-sm:min-h-10', small: 'px-2 py-0.5 text-sm max-sm:min-h-9' }
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
