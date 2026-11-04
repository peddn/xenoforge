<script setup>
import { watch, watchEffect } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'

import XButton from '@/ui/XButton.vue'
import { LOCALES, saveLocale } from '@/i18n'
import { useAuthStore } from '@/stores/auth'
import { useRulesStore } from '@/stores/rules'

const { t, locale } = useI18n()
const auth = useAuthStore()
const rules = useRulesStore()
const router = useRouter()

watchEffect(() => {
  document.documentElement.lang = locale.value
})
// Rule texts follow the interface language.
watch(locale, (value) => rules.load(value).catch(() => {}), { immediate: true })

function setLocale(code) {
  locale.value = code
  saveLocale(code)
}

async function logout() {
  await auth.logout()
  router.push('/login')
}
</script>

<template>
  <header class="bar">
    <RouterLink
      to="/"
      class="brand"
    >
      XENOFORGE
    </RouterLink>
    <span class="muted">{{ t('app.tagline') }}</span>
    <div class="bar-right">
      <template v-if="auth.user">
        <span data-test="user">{{ auth.user.name }} · {{ t(auth.user.role === 'gm' ? 'auth.gm' : 'auth.player') }}</span>
        <XButton
          variant="quiet"
          @click="logout"
        >
          {{ t('auth.logout') }}
        </XButton>
      </template>
      <nav
        class="row"
        :aria-label="t('language.label')"
      >
        <XButton
          v-for="code in LOCALES"
          :key="code"
          :aria-pressed="locale === code"
          :data-test="`lang-${code}`"
          @click="setLocale(code)"
        >
          {{ t(`language.${code}`) }}
        </XButton>
      </nav>
    </div>
  </header>
  <main>
    <RouterView v-if="rules.loaded" />
  </main>
  <footer class="disclaimer">
    {{ t('app.disclaimer') }}
  </footer>
</template>
