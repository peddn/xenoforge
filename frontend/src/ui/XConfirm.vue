<script setup>
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

import XButton from '@/ui/XButton.vue'

// A button that asks once more before it acts: the first click shows the
// question, the second confirms.
defineProps({
  label: { type: String, required: true },
  question: { type: String, required: true },
  variant: { type: String, default: 'normal' },
})
const emit = defineEmits(['confirm'])
defineOptions({ inheritAttrs: false })
const { t } = useI18n()
const asking = ref(false)

function confirm() {
  asking.value = false
  emit('confirm')
}
</script>

<template>
  <XButton
    v-if="!asking"
    :variant="variant"
    v-bind="$attrs"
    @click="asking = true"
  >
    {{ label }}
  </XButton>
  <span
    v-else
    class="flex flex-wrap items-center gap-2"
    role="alertdialog"
  >
    <span>{{ question }}</span>
    <XButton
      :variant="variant === 'normal' ? 'primary' : variant"
      data-test="confirm"
      @click="confirm"
    >
      {{ label }}
    </XButton>
    <XButton
      variant="quiet"
      @click="asking = false"
    >
      {{ t('common.cancel') }}
    </XButton>
  </span>
</template>
