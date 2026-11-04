<script setup>
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

import XButton from '@/ui/XButton.vue'
import XConfirm from '@/ui/XConfirm.vue'
import XField from '@/ui/XField.vue'
import { useTeamStore } from '@/stores/team'

// The game master's actions for the whole team: a fixed pool such as
// Mother's, the end of an Act or session, and undo. Ending an Act or a
// session asks once more first.
const { t } = useI18n()
const team = useTeamStore()
const pool = ref(8)
const note = ref('')
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

function rollPool() {
  attempt(async () => {
    await team.command({ command: 'roll', request: { actor: null, kind: { fixed_pool: { dice: pool.value } } } }, note.value)
    note.value = ''
  })
}
</script>

<template>
  <div class="grid gap-3">
    <div class="flex flex-wrap items-end gap-2">
      <XField :label="t('gm.pool')">
        <input
          v-model.number="pool"
          type="number"
          min="1"
        >
      </XField>
      <XField :label="t('common.note')">
        <input
          v-model="note"
          maxlength="200"
          data-test="pool-note"
        >
      </XField>
      <XButton @click="rollPool">
        {{ t('gm.rollPool') }}
      </XButton>
    </div>
    <div class="flex flex-wrap items-center gap-2">
      <XConfirm
        v-if="team.view.mode === 'cinematic'"
        :label="t('gm.endAct')"
        :question="t('gm.confirmEndAct')"
        data-test="end-act"
        @confirm="run({ command: 'end_act' })"
      />
      <XConfirm
        :label="t('gm.endSession')"
        :question="t('gm.confirmEndSession')"
        data-test="end-session"
        @confirm="run({ command: 'end_session' })"
      />
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
      class="text-red"
      role="alert"
    >
      {{ error }}
    </p>
  </div>
</template>
