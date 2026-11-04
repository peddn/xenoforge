<script setup>
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

import XButton from '@/ui/XButton.vue'
import XField from '@/ui/XField.vue'
import { useTeamStore } from '@/stores/team'

// The game master's actions for the whole team: a fixed pool such as
// Mother's, the end of an Act or session, and undo.
const { t } = useI18n()
const team = useTeamStore()
const pool = ref(8)
const error = ref('')

async function attempt(work) {
  error.value = ''
  try {
    await work()
  } catch (e) {
    error.value = e.message
  }
}

const run = (command) => attempt(() => team.command(command))
</script>

<template>
  <div class="stack">
    <div class="row">
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
        @click="run({ command: 'end_act' })"
      >
        {{ t('gm.endAct') }}
      </XButton>
      <XButton @click="run({ command: 'end_session' })">
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
    <p
      v-if="error"
      class="error"
      role="alert"
    >
      {{ error }}
    </p>
  </div>
</template>
