<script setup>
import { computed, ref } from 'vue'
import { useI18n } from 'vue-i18n'

import XButton from '@/ui/XButton.vue'
import XConfirm from '@/ui/XConfirm.vue'
import XField from '@/ui/XField.vue'
import { useRulesStore } from '@/stores/rules'
import { useTeamStore } from '@/stores/team'

// The game master's actions for the whole team: a fixed pool such as
// Mother's, the end of an Act or session, and undo. Ending an Act or a
// session asks once more first.
const { t } = useI18n()
const team = useTeamStore()
const rules = useRulesStore()
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

// Ending an Act: who furthered their Agenda gets a Story Point (A-13,
// p. 216). Characters of the game master have no player to reward.
const endingAct = ref(false)
const agenda = ref([])
const players = computed(() => (team.view.characters ?? []).filter((c) => c.owner_id !== team.view.gm?.id))

function endAct() {
  attempt(async () => {
    await team.command({ command: 'end_act', agenda: agenda.value.map(String) })
    endingAct.value = false
    agenda.value = []
  })
}

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
      <XButton
        v-if="team.view.mode === 'cinematic' && team.view.act <= rules.constant('cinematic_acts') && !endingAct"
        data-test="end-act"
        @click="endingAct = true"
      >
        {{ t('gm.endAct') }}
      </XButton>
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
    <fieldset
      v-if="endingAct"
      class="grid gap-2 border border-amber p-3"
      data-test="end-act-form"
    >
      <legend class="text-sm text-amber uppercase">
        {{ t('gm.endActTitle', { act: team.view.act }) }}
      </legend>
      <p class="text-dim">
        {{ t('gm.agendaQuestion') }}
      </p>
      <label
        v-for="c in players"
        :key="c.id"
        class="flex flex-wrap items-center gap-2"
      >
        <input
          v-model="agenda"
          type="checkbox"
          :value="c.id"
          :data-test="`agenda-${c.id}`"
        >
        {{ c.name }} ({{ c.owner_name }})
      </label>
      <div class="flex flex-wrap items-center gap-2">
        <XButton
          variant="primary"
          data-test="end-act-confirm"
          @click="endAct"
        >
          {{ t('gm.endAct') }}
        </XButton>
        <XButton
          variant="quiet"
          @click="endingAct = false"
        >
          {{ t('common.cancel') }}
        </XButton>
      </div>
    </fieldset>
    <p
      v-if="error"
      class="text-red"
      role="alert"
    >
      {{ error }}
    </p>
  </div>
</template>
