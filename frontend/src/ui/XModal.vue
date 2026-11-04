<script setup>
import { onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'

// A dialog over the page, built on the native <dialog> element: Escape
// closes it and focus stays inside. The parent shows it with v-if and
// removes it on `close`.
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
    class="modal"
    @cancel.prevent="emit('close')"
    @click="backdrop"
  >
    <div class="modal-box">
      <header class="panel-title">
        <h2>{{ title }}</h2>
        <button
          type="button"
          class="btn btn-quiet"
          :aria-label="t('common.close')"
          data-test="modal-close"
          @click="emit('close')"
        >
          ✕
        </button>
      </header>
      <slot />
    </div>
  </dialog>
</template>
