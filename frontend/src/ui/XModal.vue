<script setup>
import { onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'

import XButton from '@/ui/XButton.vue'
import XPanel from '@/ui/XPanel.vue'

// A dialog over the page, built on the native <dialog> element: Escape
// closes it and focus stays inside. The parent shows it with v-if and
// removes it on `close`. On phones it takes the whole screen.
defineProps({ title: { type: String, required: true } })
const emit = defineEmits(['close'])
const { t } = useI18n()
const dialog = ref(null)

onMounted(() => {
  if (dialog.value.showModal) dialog.value.showModal()
  else dialog.value.setAttribute('open', '') // test environments
})

// A click on the dark area around the box closes it as well.
function backdrop(event) {
  if (event.target === dialog.value) emit('close')
}
</script>

<template>
  <dialog
    ref="dialog"
    class="m-auto max-h-[calc(100dvh-2rem)] w-[min(44rem,calc(100vw-2rem))] bg-raised text-fg backdrop:bg-black/75 max-sm:m-0 max-sm:h-dvh max-sm:max-h-none max-sm:w-screen max-sm:max-w-none"
    @cancel.prevent="emit('close')"
    @click="backdrop"
  >
    <XPanel
      :title="title"
      class="max-sm:min-h-full"
    >
      <template #actions>
        <XButton
          variant="quiet"
          :aria-label="t('common.close')"
          data-test="modal-close"
          @click="emit('close')"
        >
          ✕
        </XButton>
      </template>
      <slot />
    </XPanel>
  </dialog>
</template>
