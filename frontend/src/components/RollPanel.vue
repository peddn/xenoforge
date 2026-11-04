<script setup>
import { computed, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'

import XButton from '@/ui/XButton.vue'
import XField from '@/ui/XField.vue'
import { modsText } from '@/labels'
import { useRulesStore } from '@/stores/rules'
import { useTeamStore } from '@/stores/team'

// The content of the roll dialog (plan 6.1). The dice count comes from the server's
// preview; this component computes no rules. The difficulty is the game
// master's (A-08). Talents with a situational bonus are ticked by the player
// (A-10); talents that grant an action, such as Analysis, are used from here
// too [p. 74–77].
const props = defineProps({ character: { type: Object, required: true } })
const emit = defineEmits(['done'])
const { t } = useI18n()
const rules = useRulesStore()
const team = useTeamStore()

const what = ref('skill:observation')
const difficulty = ref('difficulty_average')
const gear = ref(0)
const helpers = ref(0)
const talents = ref([])
const note = ref('')
const preview = ref(null)
const error = ref('')
const target = ref(null)
let timer = null

// The situation in which a talent's bonus counts, from its rule data.
const situation = (id) => rules.entry('talents', id)?.effects?.find((e) => e.when)?.when
const param = (id) => props.character.actor?.talent_params?.[id]?.join(', ')

// Actions granted by the character's talents; `target` in the action data
// says whether it aims at someone else.
const talentActions = computed(() =>
  (props.character.actor?.talents ?? []).flatMap((talent) =>
    (rules.entry('talents', talent)?.effects ?? [])
      .filter((e) => e.kind === 'capability' && rules.entry('actions', e.grants))
      .map((e) => ({ talent, action: rules.entry('actions', e.grants) })),
  ),
)
const others = computed(() => (team.view.characters ?? []).filter((c) => c.id !== props.character.id))

const request = computed(() => {
  const [kind, id] = what.value.split(':')
  const modifiers = []
  const level = team.view.is_gm && rules.difficulties().find((d) => d.id === difficulty.value)
  if (level && level.value !== 0) modifiers.push({ source: level.id, value: level.value })
  if (Number(gear.value)) modifiers.push({ source: 'gear', value: Number(gear.value) })
  const kinds = { skill: { skill: { skill: id } }, attribute: { attribute: { attribute: id } }, death: 'death' }
  return {
    actor: String(props.character.id),
    kind: kinds[kind],
    modifiers,
    helpers: Number(helpers.value) || 0,
    talents: talents.value,
  }
})

watch(
  request,
  () => {
    clearTimeout(timer)
    timer = setTimeout(async () => {
      try {
        preview.value = await team.preview(request.value)
        // Assign only on a change: a new array would start the next preview.
        const offered = talents.value.filter((id) => preview.value.talents.includes(id))
        if (offered.length !== talents.value.length) talents.value = offered
        error.value = ''
      } catch (e) {
        preview.value = null
        error.value = e.message
      }
    }, 150)
  },
  { immediate: true, deep: true },
)

async function send(command) {
  try {
    await team.command(command, note.value)
    emit('done')
  } catch (e) {
    error.value = e.message
  }
}

const roll = () => send({ command: 'roll', request: request.value })

function perform(action) {
  const command = { command: 'perform_action', actor: String(props.character.id), action: action.id }
  if (action.target) command.target = target.value == null ? null : String(target.value)
  send(command)
}
</script>

<template>
  <div class="grid gap-3">
    <div class="flex flex-wrap items-center gap-2">
      <XField :label="t('roll.what')">
        <select
          v-model="what"
          data-test="roll-what"
        >
          <optgroup
            v-for="attribute in rules.list('attributes')"
            :key="attribute.id"
            :label="rules.name(attribute.id)"
          >
            <option
              v-for="skill in rules.list('skills').filter((s) => s.attribute === attribute.id)"
              :key="skill.id"
              :value="`skill:${skill.id}`"
            >
              {{ rules.name(skill.id) }}
            </option>
            <option :value="`attribute:${attribute.id}`">
              {{ t('roll.attributeOnly', { name: rules.name(attribute.id) }) }}
            </option>
          </optgroup>
          <option value="death:">
            {{ t('roll.death') }}
          </option>
        </select>
      </XField>
      <XField
        v-if="team.view.is_gm"
        :label="t('roll.difficulty')"
      >
        <select v-model="difficulty">
          <option
            v-for="level in rules.difficulties()"
            :key="level.id"
            :value="level.id"
          >
            {{ rules.name(level.id) }} ({{ level.value > 0 ? '+' : '' }}{{ level.value }})
          </option>
        </select>
      </XField>
      <XField :label="t('roll.gear')">
        <input
          v-model="gear"
          type="number"
          min="0"
          :max="rules.constant('gear_bonus_max')"
          data-test="gear"
        >
      </XField>
      <XField :label="t('roll.helpers')">
        <input
          v-model="helpers"
          type="number"
          min="0"
        >
      </XField>
    </div>
    <XField :label="t('common.note')">
      <input
        v-model="note"
        maxlength="200"
        data-test="note"
      >
    </XField>
    <p
      v-if="!team.view.is_gm"
      class="text-dim"
    >
      {{ t('roll.difficultyByGm') }}
    </p>
    <div
      v-if="preview?.talents.length"
      class="flex flex-wrap items-center gap-2"
    >
      <label
        v-for="id in preview.talents"
        :key="id"
        class="flex flex-wrap items-center gap-2"
      >
        <input
          v-model="talents"
          type="checkbox"
          :value="id"
          :data-test="`talent-${id}`"
        >
        {{ rules.name(id) }}<template v-if="param(id)"> ({{ param(id) }})</template>
        <span
          v-if="situation(id)"
          class="text-dim"
        >– {{ rules.text(situation(id)) }}</span>
      </label>
    </div>
    <p
      v-if="preview"
      data-test="preview"
    >
      {{ t('roll.preview', { base: preview.base_dice, stress: preview.stress_dice }) }}
      <span
        v-if="preview.modifications.length"
        class="text-dim"
      > · {{ modsText(preview.modifications, t, rules) }}</span>
    </p>
    <div
      v-if="talentActions.length"
      class="flex flex-wrap items-end gap-2 border-t border-dashed border-line pt-3"
    >
      <span class="w-full text-sm text-dim uppercase">{{ t('roll.talentActions') }}</span>
      <XField
        v-if="talentActions.some((a) => a.action.target)"
        :label="t('roll.target')"
      >
        <select
          v-model="target"
          data-test="target"
        >
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
        v-for="{ talent, action } in talentActions"
        :key="action.id"
        :disabled="action.target && target == null"
        :data-test="`action-${action.id}`"
        @click="perform(action)"
      >
        {{ rules.name(talent) }}: {{ rules.name(action.id) }}
      </XButton>
    </div>
    <p
      v-if="error"
      class="text-red"
      role="alert"
    >
      {{ error }}
    </p>
    <div class="flex flex-wrap items-center gap-2">
      <XButton
        variant="primary"
        data-test="roll-go"
        @click="roll"
      >
        {{ t('roll.roll') }}
      </XButton>
      <XButton
        variant="quiet"
        @click="emit('done')"
      >
        {{ t('common.cancel') }}
      </XButton>
    </div>
  </div>
</template>
