<script setup>
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute, useRouter } from 'vue-router'

import XButton from '@/ui/XButton.vue'
import XField from '@/ui/XField.vue'
import XPanel from '@/ui/XPanel.vue'
import { useAuthStore } from '@/stores/auth'
import { refusal } from '@/labels'

const { t } = useI18n()
const auth = useAuthStore()
const route = useRoute()
const router = useRouter()

const mode = ref('login')
const name = ref('')
const password = ref('')
const role = ref('player')
const error = ref('')

async function submit() {
  error.value = ''
  try {
    if (mode.value === 'login') await auth.login(name.value, password.value)
    else await auth.register(name.value, password.value, role.value)
    router.push(route.query.next ?? '/')
  } catch (e) {
    error.value = refusal(e, t)
  }
}
</script>

<template>
  <XPanel :title="t(mode === 'login' ? 'auth.login' : 'auth.register')">
    <div class="flex flex-wrap items-center gap-2">
      <XButton
        :aria-pressed="mode === 'login'"
        data-test="mode-login"
        @click="mode = 'login'"
      >
        {{ t('auth.login') }}
      </XButton>
      <XButton
        :aria-pressed="mode === 'register'"
        data-test="mode-register"
        @click="mode = 'register'"
      >
        {{ t('auth.register') }}
      </XButton>
    </div>
    <form
      class="grid content-start gap-3 border border-line p-3"
      @submit.prevent="submit"
    >
      <XField :label="t('auth.name')">
        <input
          v-model="name"
          autocomplete="username"
          data-test="name"
          required
        >
      </XField>
      <XField :label="t('auth.password')">
        <input
          v-model="password"
          type="password"
          :autocomplete="mode === 'login' ? 'current-password' : 'new-password'"
          data-test="password"
          required
        >
      </XField>
      <fieldset
        v-if="mode === 'register'"
        class="flex flex-wrap items-center gap-2"
      >
        <legend class="text-sm text-dim uppercase">
          {{ t('auth.role') }}
        </legend>
        <label class="flex flex-wrap items-center gap-2"><input
          v-model="role"
          type="radio"
          value="player"
        > {{ t('auth.player') }}</label>
        <label class="flex flex-wrap items-center gap-2"><input
          v-model="role"
          type="radio"
          value="gm"
          data-test="role-gm"
        > {{ t('auth.gm') }}</label>
      </fieldset>
      <p
        v-if="mode === 'register'"
        class="text-dim"
      >
        {{ t('auth.roleHint') }}
      </p>
      <p
        v-if="error"
        class="text-red"
        role="alert"
      >
        {{ error }}
      </p>
      <XButton
        type="submit"
        variant="primary"
        data-test="submit"
      >
        {{ t(mode === 'login' ? 'auth.login' : 'auth.register') }}
      </XButton>
    </form>
  </XPanel>
</template>
