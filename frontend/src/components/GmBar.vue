<script setup>
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

import XButton from '@/ui/XButton.vue'
import XField from '@/ui/XField.vue'
import { useTeamStore } from '@/stores/team'

// The game master's actions for the whole team: a fixed pool such as
// Mother's, the end of an Act or session, and undo. Ending an Act or a
// session asks once more first.
const { t } = useI18n()
const team = useTeamStore()
const pool = ref(8)
const confirming = ref(null) // 'end_act' | 'end_session'
const error = ref('')

const questions = { end_act: 'gm.confirmEndAct', end_session: 'gm.confirmEndSession' }
const labels = { end_act: 'gm.endAct', end_session: 'gm.endSession' }

async function attempt(work) {
  error.value = ''
  try {
    await work()
  } catch (e) {
    error.value = e.message
  }
}

const run = (command) => attempt(() => team.command(command))

async function confirm() {
  const command = confirming.value
  confirming.value = null
  await run({ command })
}
</script>

<template>
  <div class="grid gap-3">
    <div
      v-if="!confirming"
      class="flex flex-wrap items-center gap-2"
    >
      <XField :label="t('gm.pool')">
        <input
          v-model.number="pool"
          type="number"
          min="1"
        >
      </XField>
      <XButton @click="run({ command: 'roll', request: { actor: null, kind: { fixed_pool: { dice: pool } } } })">
        {{ t('gm.rollPool') }}
      </XButton>
      <XButton
        v-if="team.view.mode === 'cinematic'"
        data-test="end-act"
        @click="confirming = 'end_act'"
      >
        {{ t('gm.endAct') }}
      </XButton>
      <XButton
        data-test="end-session"
        @click="confirming = 'end_session'"
      >
        {{ t('gm.endSession') }}
      </XButton>
      <XButton
        variant="danger"
        data-test="undo"
        @click="attempt(() => team.undo())"
      >
        {{ t('gm.undo') }}
      </XButton>
    </div>
    <div
      v-else
      class="flex flex-wrap items-center gap-2"
      role="alertdialog"
    >
      <span>{{ t(questions[confirming]) }}</span>
      <XButton
        variant="primary"
        data-test="confirm"
        @click="confirm"
      >
        {{ t(labels[confirming]) }}
      </XButton>
      <XButton
        variant="quiet"
        @click="confirming = null"
      >
        {{ t('common.cancel') }}
      </XButton>
    </div>
    <p
      v-if="error"
      class="text-red"
      role="alert"
    >
      {{ error }}
    </p>
  </div>
</template>
