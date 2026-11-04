<script setup>
import { ref, toRaw } from 'vue'
import { useI18n } from 'vue-i18n'

import ActorEditor from '@/components/ActorEditor.vue'
import XButton from '@/ui/XButton.vue'
import XStatus from '@/ui/XStatus.vue'
import { del } from '@/api'
import { originLabel } from '@/labels'
import { useRulesStore } from '@/stores/rules'
import { useTeamStore } from '@/stores/team'

// The sheet of a character in the team, for its owner and the game master.
// The game master can correct values (logged, undoable) and take the
// character out of the team. The full sheet follows with 2b.
const props = defineProps({ character: { type: Object, required: true } })
const emit = defineEmits(['close'])
const { t } = useI18n()
const rules = useRulesStore()
const team = useTeamStore()

const editing = ref(null)
const removing = ref(false)
const error = ref('')

function startCorrection() {
  // A copy to edit; structuredClone refuses Vue's reactive proxies.
  editing.value = structuredClone(toRaw(props.character.actor))
}

async function save() {
  error.value = ''
  try {
    await team.correct(props.character.id, editing.value)
    editing.value = null
  } catch (e) {
    error.value = e.message
  }
}

async function remove() {
  error.value = ''
  try {
    await del(`/teams/${team.view.id}/characters/${props.character.id}`)
    await team.refresh()
    emit('close')
  } catch (e) {
    error.value = e.message
  }
}
</script>

<template>
  <div class="stack">
    <p class="muted">
      {{ character.owner_name }}
    </p>
    <div
      v-if="!editing"
      class="stack"
    >
      <div class="row">
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
        class="list"
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
    <ActorEditor
      v-else
      v-model="editing"
    />
    <div
      v-if="team.view.is_gm"
      class="row"
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
      <template v-else>
        <XButton
          data-test="correct"
          @click="startCorrection"
        >
          {{ t('gm.correct') }}
        </XButton>
        <XButton
          v-if="!removing"
          variant="danger"
          @click="removing = true"
        >
          {{ t('team.removeCharacter') }}
        </XButton>
        <template v-else>
          <span>{{ t('team.removeConfirm', { name: character.name }) }}</span>
          <XButton
            variant="danger"
            data-test="remove-confirm"
            @click="remove"
          >
            {{ t('team.removeCharacter') }}
          </XButton>
          <XButton
            variant="quiet"
            @click="removing = false"
          >
            {{ t('common.cancel') }}
          </XButton>
        </template>
      </template>
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
