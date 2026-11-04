<script setup>
import { computed, ref, toRaw, watch } from 'vue'
import { useI18n } from 'vue-i18n'

import ActorEditor from '@/components/ActorEditor.vue'
import CharacterSheet from '@/components/CharacterSheet.vue'
import XButton from '@/ui/XButton.vue'
import XConfirm from '@/ui/XConfirm.vue'
import XField from '@/ui/XField.vue'
import XStatus from '@/ui/XStatus.vue'
import { put } from '@/api'
import { originLabel } from '@/labels'
import { useAuthStore } from '@/stores/auth'
import { useRulesStore } from '@/stores/rules'
import { useTeamStore } from '@/stores/team'

// The sheet of a character in the team, for its owner and the game master.
// The game master can correct values (logged, undoable); the owner chooses
// buddy and rival among the other characters (F-52) and can leave the team
// with the character.
const props = defineProps({ character: { type: Object, required: true } })
const { t } = useI18n()
const rules = useRulesStore()
const team = useTeamStore()

const auth = useAuthStore()
const editing = ref(null)
const note = ref('')
const error = ref('')

const own = computed(() => props.character.owner_id === auth.user?.id)
// In Cinematic play the Agenda is the current Act's (A-13), not the sheet's.
const sheet = computed(() => {
  const base = props.character.sheet ?? {}
  return team.view.mode === 'cinematic' ? { ...base, agenda: props.character.agenda ?? '' } : base
})
const others = computed(() => team.view.characters.filter((c) => c.id !== props.character.id))
const buddy = ref(null)
const rival = ref(null)
// Follows the stored choice, not every refresh of the team: the key is
// plain text, so it changes only with the values.
watch(
  () => JSON.stringify([props.character.sheet?.buddy ?? null, props.character.sheet?.rival ?? null]),
  (stored) => {
    const [b, r] = JSON.parse(stored)
    buddy.value = b
    rival.value = r
  },
  { immediate: true },
)

async function saveRelations() {
  error.value = ''
  try {
    await put(`/characters/${props.character.id}/relations`, { buddy: buddy.value ?? null, rival: rival.value ?? null })
    await team.refresh()
  } catch (e) {
    error.value = e.message
  }
}

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
      <div
        v-if="character.broken || character.actor.panic"
        class="flex flex-wrap items-center gap-2"
      >
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
      <CharacterSheet
        :sheet="sheet"
        :actor="character.actor"
        :health-max="character.health_max"
        :names="team.characterNames"
      />
      <div
        v-if="own && others.length"
        class="flex flex-wrap items-end gap-2"
      >
        <XField :label="t('sheet.buddy')">
          <select
            v-model="buddy"
            data-test="buddy"
          >
            <option :value="null">
              –
            </option>
            <option
              v-for="c in others"
              :key="c.id"
              :value="c.id"
            >
              {{ c.name }}
            </option>
          </select>
        </XField>
        <XField :label="t('sheet.rival')">
          <select
            v-model="rival"
            data-test="rival"
          >
            <option :value="null">
              –
            </option>
            <option
              v-for="c in others"
              :key="c.id"
              :value="c.id"
            >
              {{ c.name }}
            </option>
          </select>
        </XField>
        <XButton
          data-test="save-relations"
          @click="saveRelations"
        >
          {{ t('common.save') }}
        </XButton>
      </div>
      <RouterLink
        v-if="own"
        :to="`/characters/${character.id}`"
      >
        {{ t(team.view.mode === 'cinematic' ? 'sheet.editTexts' : 'sheet.edit') }}
      </RouterLink>
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
