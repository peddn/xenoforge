<script setup>
import { computed, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'

import XButton from '@/ui/XButton.vue'
import XConfirm from '@/ui/XConfirm.vue'
import XField from '@/ui/XField.vue'
import { originLabel } from '@/labels'
import { useRulesStore } from '@/stores/rules'
import { useTeamStore } from '@/stores/team'

// The game master's actions for one character (plan section 3): engine
// commands by hand, each with an optional note, choosing and handing over
// (A-12), Agendas per Act and Story Points (A-13), and taking the character
// out of the team.
const props = defineProps({ character: { type: Object, required: true } })
const { t } = useI18n()
const rules = useRulesStore()
const team = useTeamStore()

const amount = ref(1)
const sourceKind = ref('trauma')
const sourceId = ref('')
const cold = ref('sub_zero')
const note = ref('')
const error = ref('')

const actor = computed(() => String(props.character.id))
const entries = computed(() =>
  rules.list({ trauma: 'traumas', condition: 'conditions', injury: 'critical-injuries' }[sourceKind.value]),
)
// Whether the chosen condition asks for a cold level is read from its rule
// data; the engine applies it.
const needsCold = computed(() => {
  if (sourceKind.value !== 'condition') return false
  const condition = entries.value.find((c) => c.id === sourceId.value)
  return (condition?.effects ?? []).some((e) => e.kind === 'periodic' && e.every?.by_cold)
})

const cinematic = computed(() => team.view.mode === 'cinematic')
const ownedByGm = computed(() => props.character.owner_id === team.view.gm?.id)
// Taking a character away from a player is for Cinematic play only (p. 32).
const canHandOver = computed(() => ownedByGm.value || cinematic.value)
const people = computed(() =>
  [team.view.gm, ...(team.view.members ?? [])].filter((p) => p && p.id !== props.character.owner_id),
)
const newOwner = ref(null)
const acts = computed(() => rules.constant('cinematic_acts'))
const agendas = ref([])
watch(
  () => JSON.stringify(props.character.agendas ?? []),
  (stored) => {
    const list = JSON.parse(stored)
    agendas.value = Array.from({ length: acts.value }, (_, i) => list[i] ?? '')
  },
  { immediate: true },
)
// Follows the owner's stored points, e.g. after a handover or an Act.
const storyPoints = ref(0)
watch(
  () => team.view.story_points?.[String(props.character.owner_id)] ?? 0,
  (points) => (storyPoints.value = points),
  { immediate: true },
)

function handOver() {
  attempt(async () => {
    await team.handOver(props.character.id, newOwner.value)
    newOwner.value = null
  })
}

async function attempt(work) {
  error.value = ''
  try {
    await work()
  } catch (e) {
    error.value = e.message
  }
}

// The note belongs to one command and is cleared after it.
const run = (command) =>
  attempt(async () => {
    await team.command(command, note.value)
    note.value = ''
  })

function apply() {
  const id = sourceId.value || entries.value[0]?.id
  const source =
    sourceKind.value === 'condition'
      ? { condition: { id, cold: needsCold.value ? cold.value : null } }
      : { [sourceKind.value]: id }
  run({ command: 'apply', actor: actor.value, source })
}
</script>

<template>
  <div class="grid gap-3">
    <XField :label="t('common.note')">
      <input
        v-model="note"
        maxlength="200"
        data-test="action-note"
      >
    </XField>
    <div class="flex flex-wrap items-center gap-2">
      <XField :label="t('gm.amount')">
        <input
          v-model.number="amount"
          type="number"
          min="1"
        >
      </XField>
      <XButton
        data-test="stress-up"
        @click="run({ command: 'change_stress', actors: [actor], delta: amount })"
      >
        {{ t('gm.stressUp') }}
      </XButton>
      <XButton @click="run({ command: 'change_stress', actors: [actor], delta: -amount })">
        {{ t('gm.stressDown') }}
      </XButton>
      <XButton @click="run({ command: 'damage', actor, amount })">
        {{ t('gm.damage') }}
      </XButton>
    </div>
    <div class="flex flex-wrap items-center gap-2">
      <XButton @click="run({ command: 'panic_roll', actor })">
        {{ t('gm.panicRoll') }}
      </XButton>
      <XButton @click="run({ command: 'stop_panic', actor })">
        {{ t('gm.stopPanic') }}
      </XButton>
      <XButton @click="run({ command: 'use_signature_item', actor })">
        {{ t('gm.signatureItem') }}
      </XButton>
      <!-- A covert android revealed: humans nearby gain stress (A-14, p. 103). -->
      <XConfirm
        v-if="character.actor?.android && character.actor?.covert"
        :label="t('gm.revealAndroid')"
        :question="t('gm.revealAndroidConfirm', { name: character.name })"
        data-test="reveal-android"
        @confirm="run({ command: 'reveal_android', actor })"
      />
    </div>
    <div class="flex flex-wrap items-center gap-2">
      <XField :label="t('gm.source')">
        <select
          v-model="sourceKind"
          @change="sourceId = ''"
        >
          <option value="trauma">
            {{ t('gm.trauma') }}
          </option>
          <option value="condition">
            {{ t('gm.condition') }}
          </option>
          <option value="injury">
            {{ t('gm.injury') }}
          </option>
        </select>
      </XField>
      <XField :label="t('gm.entry')">
        <select v-model="sourceId">
          <option
            v-for="entry in entries"
            :key="entry.id"
            :value="entry.id"
          >
            {{ rules.name(entry.id) }}
          </option>
        </select>
      </XField>
      <XField
        v-if="needsCold"
        :label="t('gm.cold')"
      >
        <select v-model="cold">
          <option
            v-for="level in ['above_freezing', 'sub_zero', 'deep_space']"
            :key="level"
            :value="level"
          >
            {{ t(`cold.${level}`) }}
          </option>
        </select>
      </XField>
      <XButton @click="apply">
        {{ t('gm.apply') }}
      </XButton>
    </div>
    <ul
      v-if="character.actor?.sources?.length"
      class="grid gap-1.5"
    >
      <li
        v-for="(source, i) in character.actor.sources"
        :key="i"
        class="flex flex-wrap items-center gap-2"
      >
        {{ originLabel(source.origin, t, rules) }}
        <XButton
          variant="quiet"
          @click="run({ command: 'remove', actor, origin: source.origin })"
        >
          {{ t('common.remove') }}
        </XButton>
      </li>
    </ul>
    <div
      v-if="ownedByGm || canHandOver"
      class="flex flex-wrap items-end gap-2 border-t border-dashed border-line pt-3"
    >
      <XButton
        v-if="ownedByGm"
        data-test="offer"
        @click="attempt(() => team.offer(character.id, !character.offered))"
      >
        {{ t(character.offered ? 'cast.withdraw' : 'cast.offer') }}
      </XButton>
      <template v-if="canHandOver && people.length">
        <XField :label="t('cast.to')">
          <select
            v-model="newOwner"
            data-test="new-owner"
          >
            <option
              v-for="p in people"
              :key="p.id"
              :value="p.id"
            >
              {{ p.id === team.view.gm?.id ? t('cast.gm', { name: p.name }) : p.name }}
            </option>
          </select>
        </XField>
        <XButton
          :disabled="newOwner == null"
          data-test="hand-over"
          @click="handOver"
        >
          {{ t('cast.handOver') }}
        </XButton>
      </template>
    </div>
    <div
      v-if="cinematic"
      class="grid gap-2 border-t border-dashed border-line pt-3"
    >
      <XField
        v-for="(_, i) in agendas"
        :key="i"
        :label="t('cast.agendaAct', { act: i + 1 })"
      >
        <textarea
          v-model="agendas[i]"
          rows="2"
          :data-test="`agenda-act-${i + 1}`"
        />
      </XField>
      <div class="flex flex-wrap items-end gap-2">
        <XButton
          data-test="save-agendas"
          @click="attempt(() => team.setAgendas(character.id, agendas))"
        >
          {{ t('cast.saveAgendas') }}
        </XButton>
        <template v-if="!ownedByGm">
          <XField :label="t('cast.storyPoints', { name: character.owner_name })">
            <input
              v-model.number="storyPoints"
              type="number"
              min="0"
              :max="rules.constant('story_points_max')"
              data-test="story-points"
            >
          </XField>
          <XButton
            data-test="set-story-points"
            @click="run({ command: 'set_story_points', player: String(character.owner_id), points: storyPoints })"
          >
            {{ t('cast.setStoryPoints') }}
          </XButton>
        </template>
      </div>
    </div>
    <div class="flex flex-wrap items-center gap-2 border-t border-dashed border-line pt-3">
      <XConfirm
        variant="danger"
        :label="t('team.removeCharacter')"
        :question="t('team.removeConfirm', { name: character.name })"
        data-test="remove"
        @confirm="attempt(() => team.removeCharacter(character.id))"
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
