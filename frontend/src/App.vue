<script setup>
import { watchEffect } from 'vue'
import { useI18n } from 'vue-i18n'

import { LOCALES, saveLocale } from '@/i18n'

const { t, locale } = useI18n()

watchEffect(() => {
  document.documentElement.lang = locale.value
})

function setLocale(code) {
  locale.value = code
  saveLocale(code)
}
</script>

<template>
  <header class="bar">
    <span class="brand">XENOFORGE</span>
    <span class="tagline">{{ t('app.tagline') }}</span>
    <nav
      class="lang"
      :aria-label="t('language.label')"
    >
      <button
        v-for="code in LOCALES"
        :key="code"
        type="button"
        :aria-pressed="locale === code"
        :data-test="`lang-${code}`"
        @click="setLocale(code)"
      >
        {{ t(`language.${code}`) }}
      </button>
    </nav>
  </header>
  <main>
    <RouterView />
  </main>
  <footer class="disclaimer">
    {{ t('app.disclaimer') }}
  </footer>
</template>
