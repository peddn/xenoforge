<script setup>
import { ref, watch, watchEffect } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'

import XButton from '@/ui/XButton.vue'
import XModal from '@/ui/XModal.vue'
import { LOCALES, saveLocale } from '@/i18n'
import { useAuthStore } from '@/stores/auth'
import { useRulesStore } from '@/stores/rules'

const { t, locale } = useI18n()
const auth = useAuthStore()
const rules = useRulesStore()
const router = useRouter()
// The note that this is an unofficial fan project, behind the ⓘ.
const about = ref(false)

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
  <header class="flex flex-wrap items-center gap-x-4 gap-y-2 border-b border-line px-4 py-2.5">
    <RouterLink
      to="/"
      class="font-bold tracking-[0.3em] no-underline"
    >
      XENOFORGE
    </RouterLink>
    <span class="text-dim">{{ t('app.tagline') }}</span>
    <div class="ml-auto flex flex-wrap items-center gap-2">
      <template v-if="auth.user">
        <span data-test="user">{{ auth.user.name }} · {{ t(auth.user.role === 'gm' ? 'auth.gm' : 'auth.player') }}</span>
        <XButton
          variant="quiet"
          @click="logout"
        >
          {{ t('auth.logout') }}
        </XButton>
      </template>
      <XButton
        variant="quiet"
        :aria-label="t('about.title')"
        :title="t('about.title')"
        data-test="about"
        @click="about = true"
      >
        ⓘ
      </XButton>
      <nav
        class="flex flex-wrap items-center gap-2"
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
  <main class="mx-auto grid w-full max-w-6xl flex-1 content-start gap-3 p-4">
    <RouterView v-if="rules.loaded" />
  </main>
  <XModal
    v-if="about"
    :title="t('about.title')"
    @close="about = false"
  >
    <p>{{ t('app.disclaimer') }}</p>
    <p class="text-dim">
      {{ t('about.license') }}
    </p>
    <a href="https://github.com/peddn/xenoforge">github.com/peddn/xenoforge</a>
  </XModal>
</template>
