<script setup>
import { computed, onUnmounted, ref, toRaw, watch } from 'vue'
import { useI18n } from 'vue-i18n'

import XButton from '@/ui/XButton.vue'
import XFacts from '@/ui/XFacts.vue'
import XField from '@/ui/XField.vue'
import { cashFormula, problemText } from '@/labels'
import { useCharactersStore } from '@/stores/characters'
import { useRulesStore } from '@/stores/rules'

// The wizard for Campaign characters in the book's ten steps (plan 5.2,
// p. 27–36). Every change is saved as a draft; the engine checks it and
// names the problems. The next step opens when this one has none. Limits
// shown come from constants.json; the interface computes no rules.
const props = defineProps({ character: { type: Object, required: true } })
const emit = defineEmits(['finished'])
const { t } = useI18n()
const rules = useRulesStore()
const characters = useCharactersStore()

// The wizard's steps, the engine's steps whose problems each shows, and the
// page of the rule.
const STEPS = [
  { id: 'career', checks: ['career'], page: 27 },
  { id: 'attributes', checks: ['attributes'], page: 28 },
  { id: 'skills', checks: ['skills'], page: 28 },
  { id: 'talent', checks: ['talent'], page: 29 },
  { id: 'name', checks: ['name', 'appearance'], page: 30 },
  { id: 'agenda', checks: ['agenda'], page: 31 },
  { id: 'relations', checks: [], page: 31 },
  { id: 'gear', checks: ['signature_item'], page: 33 },
  { id: 'cash', checks: [], page: 39 },
  { id: 'finish', checks: [], page: 30 },
]

const EMPTY = {
  career: null,
  android: false,
  attributes: {},
  android_bonus: [],
  skills: {},
  talent: null,
  talent_param: null,
  name: '',
  appearance: '',
  agenda: '',
  signature_item: '',
}
// A copy to edit; structuredClone refuses Vue's reactive proxies.
const draft = ref({ ...EMPTY, ...structuredClone(toRaw(props.character.data.draft ?? {})) })
const problems = ref(props.character.problems ?? [])
const actor = ref(props.character.data.actor)
const saving = ref(false)
const error = ref('')

const stepProblems = (step) => problems.value.filter((p) => step.checks.includes(p.step))
// A step opens when all steps before it are fine.
const reachable = (index) => STEPS.slice(0, index).every((s) => !stepProblems(s).length)
const firstOpen = STEPS.findIndex((s) => stepProblems(s).length)
const index = ref(firstOpen < 0 ? STEPS.length - 1 : firstOpen)
const step = computed(() => STEPS[index.value])

const career = computed(() => rules.entry('careers', draft.value.career))
const attributes = computed(() => rules.list('attributes'))
const skillsOf = (attribute) => rules.list('skills').filter((s) => s.attribute === attribute)
const talent = computed(() => rules.entry('talents', draft.value.talent))
const c = (id) => rules.constant(id)
const sum = (values) => Object.values(values).reduce((a, b) => a + b, 0)
const isKeySkill = (id) => career.value?.key_skills.includes(id)

const ruleParams = computed(() => ({
  points: c(step.value.id === 'skills' ? 'creation_skill_points' : 'creation_attribute_points'),
  min: c('creation_attribute_min'),
  max: c('creation_attribute_max'),
  keyMax: c(step.value.id === 'skills' ? 'creation_key_skill_max' : 'creation_key_attribute_max'),
  otherMax: c('creation_other_skill_max'),
  key: career.value ? rules.name(career.value.key_attribute) : '',
  keys: (career.value?.key_skills ?? []).map(rules.name).join(', '),
  formula: career.value ? cashFormula(career.value.cash, t) : '',
}))

// Attributes start at the lowest value, so the points left are plain.
watch(
  index,
  () => {
    if (step.value.id !== 'attributes') return
    for (const a of attributes.value) draft.value.attributes[a.id] ??= c('creation_attribute_min')
  },
  { immediate: true },
)

function setNumber(group, id, value) {
  draft.value[group][id] = Math.max(0, Math.trunc(Number(value) || 0))
}

// What a career sets, labelled [p. 38–55]; talents with their description
// as tooltip.
const careerFacts = (entry) => [
  [t('wizard.keyAttribute'), rules.name(entry.key_attribute)],
  [t('wizard.keySkills'), entry.key_skills.map(rules.name).join(', ')],
  [t('sheet.talents'), entry.talents.map((id) => ({ text: rules.name(id), title: rules.text(`${id}.desc`) }))],
  [t('sheet.startCash'), cashFormula(entry.cash, t)],
]

// The whole box chooses the career; the radio button stays for the keyboard.
function chooseCareer(id) {
  if (draft.value.career === id) return
  draft.value.career = id
  careerChanged()
}

function careerChanged() {
  if (!career.value?.talents.includes(draft.value.talent)) {
    draft.value.talent = null
    draft.value.talent_param = null
  }
}

function addLook(text) {
  const now = draft.value.appearance.trim()
  draft.value.appearance = now ? `${now}, ${text}` : text
}

// One save at a time, so the server keeps the last draft; a change during
// a save is sent right after it.
let timer = null
let inFlight = false
let again = false
watch(
  draft,
  () => {
    clearTimeout(timer)
    saving.value = true
    timer = setTimeout(save, 300)
  },
  { deep: true },
)
onUnmounted(() => {
  // Leaving the page saves what is still pending.
  if (timer) {
    clearTimeout(timer)
    save()
  }
})

async function save() {
  timer = null
  if (inFlight) {
    again = true
    return
  }
  inFlight = true
  try {
    const saved = await characters.saveDraft(props.character.id, draft.value)
    if (!again) {
      problems.value = saved.problems ?? []
      actor.value = saved.data.actor
    }
    error.value = ''
  } catch (e) {
    error.value = e.message
  } finally {
    inFlight = false
    if (again) {
      again = false
      save()
    } else if (!timer) {
      saving.value = false
    }
  }
}

async function finish() {
  error.value = ''
  try {
    emit('finished', await characters.finish(props.character.id))
  } catch (e) {
    error.value = e.message
  }
}
</script>

<template>
  <div class="grid gap-3">
    <ol class="flex flex-wrap gap-1">
      <li
        v-for="(s, i) in STEPS"
        :key="s.id"
      >
        <XButton
          size="small"
          :variant="stepProblems(s).length && reachable(i) ? 'notice' : 'normal'"
          :aria-pressed="i === index"
          :disabled="!reachable(i)"
          :data-test="`step-${s.id}`"
          @click="index = i"
        >
          {{ i + 1 }} {{ t(`wizard.step.${s.id}`) }}
        </XButton>
      </li>
    </ol>

    <h3>{{ index + 1 }}. {{ t(`wizard.step.${step.id}`) }}</h3>
    <p>
      {{ t(`wizard.rule.${step.id}`, ruleParams) }}
      <span class="text-dim">({{ t('wizard.page', { page: step.page }) }})</span>
    </p>

    <template v-if="step.id === 'career'">
      <div class="grid cols-auto gap-2">
        <div
          v-for="entry in rules.list('careers')"
          :key="entry.id"
          class="grid cursor-pointer content-start gap-2 border p-2"
          :class="draft.career === entry.id ? 'border-fg' : 'border-line'"
          @click="chooseCareer(entry.id)"
        >
          <span class="flex items-center justify-between gap-2">
            <label class="flex cursor-pointer items-center gap-2 text-strong uppercase">
              <input
                v-model="draft.career"
                type="radio"
                :value="entry.id"
                :data-test="`career-${entry.id}`"
                @change="careerChanged"
              >
              {{ rules.name(entry.id) }}
            </label>
            <span class="text-sm text-dim">{{ t('wizard.page', { page: entry.page }) }}</span>
          </span>
          <XFacts
            stacked
            :items="careerFacts(entry)"
            :data-test="`career-facts-${entry.id}`"
          />
        </div>
      </div>
      <label class="flex flex-wrap items-center gap-2">
        <input
          v-model="draft.android"
          type="checkbox"
          data-test="android"
        >
        {{ t('wizard.android') }}
      </label>
    </template>

    <template v-else-if="step.id === 'attributes'">
      <p>{{ t('wizard.spent', { spent: sum(draft.attributes), points: c('creation_attribute_points') }) }}</p>
      <div class="flex flex-wrap gap-3">
        <XField
          v-for="a in attributes"
          :key="a.id"
          :label="`${rules.name(a.id)}${career?.key_attribute === a.id ? ' ★' : ''}`"
        >
          <input
            type="number"
            :min="c('creation_attribute_min')"
            :max="c(career?.key_attribute === a.id ? 'creation_key_attribute_max' : 'creation_attribute_max')"
            :value="draft.attributes[a.id] ?? 0"
            :data-test="`attr-${a.id}`"
            @input="setNumber('attributes', a.id, $event.target.value)"
          >
        </XField>
      </div>
      <fieldset
        v-if="draft.android"
        class="grid gap-2 border border-line p-3"
      >
        <legend class="text-sm text-dim uppercase">
          {{ t('wizard.androidBonus', { bonus: c('android_attribute_bonus'), count: c('android_bonus_attributes') }) }}
        </legend>
        <label
          v-for="a in attributes"
          :key="a.id"
          class="flex flex-wrap items-center gap-2"
        >
          <input
            v-model="draft.android_bonus"
            type="checkbox"
            :value="a.id"
            :disabled="!draft.android_bonus.includes(a.id) && draft.android_bonus.length >= c('android_bonus_attributes')"
            :data-test="`bonus-${a.id}`"
          >
          {{ rules.name(a.id) }} → {{ actor.attributes?.[a.id] ?? 0 }}
        </label>
      </fieldset>
    </template>

    <template v-else-if="step.id === 'skills'">
      <p>{{ t('wizard.spent', { spent: sum(draft.skills), points: c('creation_skill_points') }) }}</p>
      <div class="grid cols-auto items-start gap-3">
        <div
          v-for="a in attributes"
          :key="a.id"
          class="grid content-start gap-2 border border-line p-3"
        >
          <span class="text-sm text-dim uppercase">{{ rules.name(a.id) }}</span>
          <XField
            v-for="skill in skillsOf(a.id)"
            :key="skill.id"
            :label="`${rules.name(skill.id)}${isKeySkill(skill.id) ? ' ★' : ''}`"
          >
            <input
              type="number"
              min="0"
              :max="c(isKeySkill(skill.id) ? 'creation_key_skill_max' : 'creation_other_skill_max')"
              :value="draft.skills[skill.id] ?? 0"
              :data-test="`skill-${skill.id}`"
              @input="setNumber('skills', skill.id, $event.target.value)"
            >
          </XField>
        </div>
      </div>
    </template>

    <template v-else-if="step.id === 'talent'">
      <label
        v-for="id in career?.talents ?? []"
        :key="id"
        class="flex items-start gap-2 border border-line p-2"
      >
        <input
          v-model="draft.talent"
          type="radio"
          :value="id"
          :data-test="`talent-${id}`"
          @change="draft.talent_param = null"
        >
        <span class="grid gap-0.5">
          <span class="text-strong uppercase">{{ rules.name(id) }}</span>
          <span>{{ rules.text(`${id}.desc`) }}</span>
          <span
            v-if="rules.entry('talents', id)?.later"
            class="text-sm text-dim"
          >{{ t('wizard.later', { part: t(`wizard.part.${rules.entry('talents', id).later}`) }) }}</span>
        </span>
      </label>
      <XField
        v-if="talent?.parameter"
        :label="t(`wizard.parameter.${talent.parameter}`)"
      >
        <input
          v-model="draft.talent_param"
          maxlength="60"
          data-test="talent-param"
        >
      </XField>
    </template>

    <template v-else-if="step.id === 'name'">
      <XField :label="t('sheet.name')">
        <input
          v-model="draft.name"
          maxlength="60"
          data-test="draft-name"
        >
      </XField>
      <div class="flex flex-wrap items-center gap-1">
        <span class="text-sm text-dim uppercase">{{ t('wizard.suggestions') }}</span>
        <XButton
          v-for="name in career?.names ?? []"
          :key="name"
          variant="quiet"
          size="small"
          @click="draft.name = name"
        >
          {{ name }}
        </XButton>
      </div>
      <XField :label="t('sheet.appearance')">
        <input
          v-model="draft.appearance"
          data-test="draft-appearance"
        >
      </XField>
      <div class="flex flex-wrap items-center gap-1">
        <span class="text-sm text-dim uppercase">{{ t('wizard.suggestions') }}</span>
        <XButton
          v-for="look in career?.looks ?? []"
          :key="look"
          variant="quiet"
          size="small"
          @click="addLook(rules.text(`${career.id}.look.${look}`))"
        >
          {{ rules.text(`${career.id}.look.${look}`) }}
        </XButton>
      </div>
    </template>

    <template v-else-if="step.id === 'agenda'">
      <XField :label="t('sheet.agenda')">
        <textarea
          v-model="draft.agenda"
          rows="3"
          data-test="draft-agenda"
        />
      </XField>
      <div class="grid gap-1">
        <span class="text-sm text-dim uppercase">{{ t('wizard.suggestions') }}</span>
        <XButton
          v-for="agenda in career?.agendas ?? []"
          :key="agenda"
          variant="quiet"
          size="small"
          class="text-left normal-case"
          @click="draft.agenda = rules.text(`${career.id}.agenda.${agenda}`)"
        >
          {{ rules.text(`${career.id}.agenda.${agenda}`) }}
        </XButton>
      </div>
    </template>

    <template v-else-if="step.id === 'gear'">
      <p
        v-if="draft.android"
        class="text-dim"
      >
        {{ t('wizard.androidItem') }}
      </p>
      <XField :label="t('sheet.signatureItem')">
        <input
          v-model="draft.signature_item"
          data-test="draft-item"
        >
      </XField>
      <div class="flex flex-wrap items-center gap-1">
        <span class="text-sm text-dim uppercase">{{ t('wizard.suggestions') }}</span>
        <XButton
          v-for="item in career?.items ?? []"
          :key="item"
          variant="quiet"
          size="small"
          @click="draft.signature_item = rules.text(`${career.id}.item.${item}`)"
        >
          {{ rules.text(`${career.id}.item.${item}`) }}
        </XButton>
      </div>
    </template>

    <template v-else-if="step.id === 'finish'">
      <p>
        {{ t('sheet.health') }}: {{ actor.health }} · {{ t('sheet.stress') }}: {{ actor.stress ?? 0 }}
      </p>
      <p v-if="!problems.length">
        {{ t('wizard.none') }}
      </p>
      <ul
        v-else
        class="grid gap-1.5"
        data-test="problems"
      >
        <li
          v-for="(problem, i) in problems"
          :key="i"
        >
          ▲ {{ problemText(problem, t, rules) }}
        </li>
      </ul>
    </template>

    <ul
      v-if="step.id !== 'finish' && stepProblems(step).length"
      class="grid gap-1.5 text-amber"
      data-test="step-problems"
    >
      <li
        v-for="(problem, i) in stepProblems(step)"
        :key="i"
      >
        ▲ {{ problemText(problem, t, rules) }}
      </li>
    </ul>

    <div class="flex flex-wrap items-center gap-2">
      <XButton
        v-if="index > 0"
        variant="quiet"
        @click="index--"
      >
        {{ t('wizard.back') }}
      </XButton>
      <XButton
        v-if="index < STEPS.length - 1"
        variant="primary"
        :disabled="saving || stepProblems(step).length > 0"
        data-test="next"
        @click="index++"
      >
        {{ t('wizard.next') }}
      </XButton>
      <XButton
        v-else
        variant="primary"
        :disabled="saving || problems.length > 0"
        data-test="finish"
        @click="finish"
      >
        {{ t('wizard.finish') }}
      </XButton>
      <span
        class="text-dim"
        role="status"
      >{{ t(saving ? 'wizard.saving' : 'wizard.saved') }}</span>
      <span
        v-if="error"
        class="text-red"
        role="alert"
      >{{ error }}</span>
    </div>
  </div>
</template>
