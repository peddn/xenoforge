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
import { originLabel, parseMoney, refusal } from '@/labels'
import { useAuthStore } from '@/stores/auth'
import { useRulesStore } from '@/stores/rules'
import { useTeamStore } from '@/stores/team'

// The sheet of a character in the team, for its owner and the game master.
// The game master can correct values (logged, undoable) and gives items; the
// owner moves, drops and hands over the character's items, chooses buddy and
// rival among the other characters (F-52) and can leave the team with the
// character.
const props = defineProps({ character: { type: Object, required: true } })
const { t, locale } = useI18n()
const rules = useRulesStore()
const team = useTeamStore()

const auth = useAuthStore()
const editing = ref(null)
// The values the correction started from; the server refuses the
// correction if the character changed since.
const base = ref(null)
const stale = ref(false)
const note = ref('')
const error = ref('')

const own = computed(() => props.character.owner_id === auth.user?.id)
// Buddy and rival: the owner chooses in Campaign, the scenario (so the game
// master) sets them in Cinematic play [p. 31].
const setsRelations = computed(() => (team.view.mode === 'cinematic' ? team.view.is_gm : own.value))
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

// Once per Act or session the owner turns to their signature item:
// stress −1 [p. 104]. Androids have no stress level.
async function useSignatureItem() {
  error.value = ''
  try {
    await team.command({ command: 'use_signature_item', actor: String(props.character.id) })
  } catch (e) {
    error.value = refusal(e, t)
  }
}

async function saveRelations() {
  error.value = ''
  try {
    await put(`/characters/${props.character.id}/relations`, { buddy: buddy.value ?? null, rival: rival.value ?? null })
    await team.refresh()
  } catch (e) {
    error.value = refusal(e, t)
  }
}

function startCorrection() {
  // A copy to edit; structuredClone refuses Vue's reactive proxies.
  editing.value = structuredClone(toRaw(props.character.actor))
  base.value = structuredClone(toRaw(props.character.actor))
  stale.value = false
  error.value = ''
}

async function save() {
  error.value = ''
  try {
    await team.correct(props.character.id, editing.value, note.value, base.value)
    editing.value = null
    note.value = ''
  } catch (e) {
    stale.value = e.status === 409
    error.value = stale.value ? t('gm.correctionStale') : refusal(e, t)
  }
}

// Item commands go to the team for this character; true when they went
// through.
async function item(command) {
  error.value = ''
  try {
    await team.command({ ...command, actor: String(props.character.id) })
    await team.refresh()
    return true
  } catch (e) {
    error.value = refusal(e, t)
    return false
  }
}

// Supply rolls are the game master's: Food and Water while there is any,
// Air on a worn suit that holds some (F-75) [p. 34]; synthetics need
// none [p. 111].
const supplyRolls = computed(() => {
  const actor = props.character.actor ?? {}
  if (actor.android) return []
  const list = ['food', 'water'].filter((s) => (actor.supplies?.[s] ?? 0) > 0)
  const suit = (actor.items ?? []).find(
    (p) => p.place === 'worn' && rules.entry('gear', p.item)?.supply?.kind === 'air',
  )
  if (suit && (suit.supply ?? 1) > 0) list.push('air')
  return list
})
async function supplyRoll(supply) {
  error.value = ''
  try {
    await team.command({ command: 'roll', request: { actor: String(props.character.id), kind: { supply: { supply } } } })
  } catch (e) {
    error.value = refusal(e, t)
  }
}

// A booking by the game master: wages, a reward, an expense, a loan (F-76).
const amount = ref('')
const reason = ref('')
const cents = computed(() => parseMoney(amount.value, locale.value))
const amountBad = computed(() => cents.value !== null && !Number.isFinite(cents.value))
const canBook = computed(() => !!cents.value && !amountBad.value && !!reason.value.trim())
async function book() {
  error.value = ''
  if (!canBook.value) return
  try {
    await team.command({ command: 'book', actor: String(props.character.id), amount: cents.value, note: reason.value })
    amount.value = ''
    reason.value = ''
  } catch (e) {
    error.value = refusal(e, t)
  }
}

async function leave() {
  error.value = ''
  try {
    await team.removeCharacter(props.character.id)
  } catch (e) {
    error.value = refusal(e, t)
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
        :encumbrance="character.encumbrance"
        :gear-move="own || team.view.is_gm"
        :gear-add="team.view.is_gm"
        :gear-use="own || team.view.is_gm"
        :gear-refill="team.view.is_gm"
        :gear-buy="team.view.is_gm"
        :money="own || team.view.is_gm"
        :others="others"
        :send-item="item"
      />
      <div
        v-if="own && !team.view.is_gm && !character.actor.android"
        class="flex flex-wrap items-center gap-2"
      >
        <XButton
          :disabled="character.actor.signature_item_used"
          data-test="use-signature-item"
          @click="useSignatureItem"
        >
          {{ t('sheet.useSignatureItem') }}
        </XButton>
        <span
          v-if="character.actor.signature_item_used"
          class="text-sm text-dim"
        >{{ t('sheet.signatureItemUsed') }}</span>
      </div>
      <div
        v-if="setsRelations && others.length"
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
          v-if="stale"
          data-test="reload-correction"
          @click="startCorrection"
        >
          {{ t('gm.reloadCorrection') }}
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
      <span
        v-if="!editing"
        class="flex flex-wrap items-end gap-2"
        data-test="booking"
      >
        <XField :label="t('money.amount')">
          <input
            v-model="amount"
            class="w-28"
            :aria-invalid="amountBad"
            data-test="booking-amount"
          >
        </XField>
        <XField :label="t('money.reason')">
          <input
            v-model="reason"
            maxlength="100"
            data-test="booking-reason"
          >
        </XField>
        <XButton
          :disabled="!canBook"
          data-test="booking-go"
          @click="book"
        >
          {{ t('money.book') }}
        </XButton>
        <XStatus
          v-if="amountBad"
          tone="warn"
          :text="t('money.invalid')"
          data-test="booking-invalid"
        />
      </span>
      <span
        v-if="!editing && supplyRolls.length"
        class="flex flex-wrap items-center gap-2"
      >
        <span class="text-sm text-dim uppercase">{{ t('sheet.supplyRoll') }}</span>
        <XButton
          v-for="supply in supplyRolls"
          :key="supply"
          size="small"
          :data-test="`supply-roll-${supply}`"
          @click="supplyRoll(supply)"
        >
          {{ t(`supply.${supply}`) }}
        </XButton>
      </span>
    </div>
    <div
      v-else-if="character.owner_id === auth.user?.id"
      class="flex flex-wrap items-center gap-2"
    >
      <XConfirm
        variant="danger"
        :label="t('team.leave')"
        :question="t(character.returns ? 'team.leaveReturns' : 'team.leaveConfirm', { name: character.name })"
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
