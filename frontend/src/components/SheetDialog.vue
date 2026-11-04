<script setup>
import { ref, toRaw } from 'vue'
import { useI18n } from 'vue-i18n'

import ActorEditor from '@/components/ActorEditor.vue'
import XButton from '@/ui/XButton.vue'
import XConfirm from '@/ui/XConfirm.vue'
import XField from '@/ui/XField.vue'
import XStatus from '@/ui/XStatus.vue'
import { originLabel } from '@/labels'
import { useAuthStore } from '@/stores/auth'
import { useRulesStore } from '@/stores/rules'
import { useTeamStore } from '@/stores/team'

// The sheet of a character in the team, for its owner and the game master.
// The game master can correct values (logged, undoable); the owner can leave
// the team with the character. The full sheet follows with 2b.
const props = defineProps({ character: { type: Object, required: true } })
const { t } = useI18n()
const rules = useRulesStore()
const team = useTeamStore()

const auth = useAuthStore()
const editing = ref(null)
const note = ref('')
const error = ref('')

function startCorrection() {
  // A copy to edit; structuredClone refuses Vue's reactive proxies.
  editing.value = structuredClone(toRaw(props.character.actor))
}

async function save() {
  error.value = ''
  try {
    await team.correct(props.character.id, editing.value, note.value)
    editing.value = null
    note.value = ''
  } catch (e) {
    error.value = e.message
  }
}

async function leave() {
  error.value = ''
  try {
    await team.removeCharacter(props.character.id)
  } catch (e) {
    error.value = e.message
  }
}
</script>

<template>
  <div class="grid gap-3">
    <p class="text-dim">
      {{ character.owner_name }}
    </p>
    <div
      v-if="!editing"
      class="grid gap-3"
    >
      <div class="flex flex-wrap items-center gap-2">
        <span>{{ t('sheet.health') }}: {{ character.actor.health }}/{{ character.health_max }}</span>
        <span v-if="!character.actor.android">{{ t('sheet.stress') }}: {{ character.actor.stress }}</span>
        <XStatus
          v-if="character.broken"
          tone="alert"
          :text="t('crew.broken')"
        />
        <XStatus
          v-if="character.actor.panic"
          tone="warn"
          :text="`${t('crew.panic')} ${character.actor.panic.value} · ${rules.name(character.actor.panic.effect)}`"
        />
      </div>
      <ul
        v-if="character.actor.sources?.length"
        class="grid gap-1.5"
      >
        <li
          v-for="(source, i) in character.actor.sources"
          :key="i"
        >
          {{ originLabel(source.origin, t, rules) }}
        </li>
      </ul>
      <ActorEditor
        :model-value="character.actor"
        disabled
      />
    </div>
    <template v-else>
      <ActorEditor v-model="editing" />
      <XField :label="t('common.note')">
        <input
          v-model="note"
          maxlength="200"
          data-test="correction-note"
        >
      </XField>
    </template>
    <div
      v-if="team.view.is_gm"
      class="flex flex-wrap items-center gap-2"
    >
      <template v-if="editing">
        <XButton
          variant="primary"
          data-test="save-correction"
          @click="save"
        >
          {{ t('common.save') }}
        </XButton>
        <XButton
          variant="quiet"
          @click="editing = null"
        >
          {{ t('common.cancel') }}
        </XButton>
      </template>
      <XButton
        v-else
        data-test="correct"
        @click="startCorrection"
      >
        {{ t('gm.correct') }}
      </XButton>
    </div>
    <div
      v-else-if="character.owner_id === auth.user?.id"
      class="flex flex-wrap items-center gap-2"
    >
      <XConfirm
        variant="danger"
        :label="t('team.leave')"
        :question="t('team.leaveConfirm', { name: character.name })"
        data-test="leave"
        @confirm="leave"
      />
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
