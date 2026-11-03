<script setup>
import { onMounted } from 'vue'
import { useI18n } from 'vue-i18n'

import { useStatusStore } from '@/stores/status'

const { t } = useI18n()
const status = useStatusStore()

// Colour never carries the meaning alone (plan section 11.5).
const SYMBOLS = { checking: '…', online: '●', offline: '✕' }

onMounted(() => status.check())
</script>

<template>
  <section class="panel">
    <h1>{{ t('status.heading') }}</h1>
    <dl>
      <dt>{{ t('status.server') }}</dt>
      <dd
        :class="['state', status.state]"
        role="status"
        data-test="server-state"
      >
        <span aria-hidden="true">{{ SYMBOLS[status.state] }}</span>
        {{ t(`status.${status.state}`) }}
      </dd>
      <template v-if="status.version">
        <dt>{{ t('status.version') }}</dt>
        <dd data-test="version">
          {{ status.version }}
        </dd>
      </template>
    </dl>
    <button
      type="button"
      :disabled="status.state === 'checking'"
      @click="status.check()"
    >
      {{ t('status.retry') }}
    </button>
  </section>
</template>
