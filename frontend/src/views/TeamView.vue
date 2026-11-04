<script setup>
import { computed, onMounted, onUnmounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute } from 'vue-router'

import CharacterActions from '@/components/CharacterActions.vue'
import CrewList from '@/components/CrewList.vue'
import DecisionBar from '@/components/DecisionBar.vue'
import DecisionDialog from '@/components/DecisionDialog.vue'
import GmBar from '@/components/GmBar.vue'
import LogList from '@/components/LogList.vue'
import RollPanel from '@/components/RollPanel.vue'
import SheetDialog from '@/components/SheetDialog.vue'
import XButton from '@/ui/XButton.vue'
import XField from '@/ui/XField.vue'
import XModal from '@/ui/XModal.vue'
import XPanel from '@/ui/XPanel.vue'
import XStatus from '@/ui/XStatus.vue'
import { post } from '@/api'
import { useCharactersStore } from '@/stores/characters'
import { useTeamStore } from '@/stores/team'

const { t } = useI18n()
const route = useRoute()
const team = useTeamStore()
const characters = useCharactersStore()

// The open dialog: { kind: 'roll' | 'sheet' | 'actions', id } with a
// character id, or { kind: 'decision', id } with a pending decision's id.
const dialog = ref(null)
const ownChoice = ref(null)
const error = ref('')

onMounted(async () => {
  await team.open(route.params.id)
  if (team.view?.is_gm) await characters.load()
})
onUnmounted(() => team.close())

const inviteLink = computed(() =>
  team.view?.invite_token ? `${location.origin}/join/${team.view.invite_token}` : '',
)
const freeOwn = computed(() => characters.list.filter((c) => !c.team_id))
// Looked up live, so the dialog shows current values; it closes when the
// character leaves the team or the decision is made.
const current = computed(() => {
  if (!dialog.value || !team.view) return null
  if (dialog.value.kind === 'decision') return team.view.pending.find((p) => p.id === dialog.value.id)
  return team.view.characters.find((c) => c.id === dialog.value.id)
})
const title = computed(() => {
  const { kind } = dialog.value
  if (kind === 'decision') {
    const name = current.value.actor ? team.characterNames[current.value.actor] : t('auth.gm')
    return t('decision.titleFor', { name })
  }
  const titles = { roll: 'roll.title', sheet: 'sheet.title', actions: 'gm.actionsTitle' }
  return t(titles[kind], { name: current.value.name })
})

const open = (kind, id) => (dialog.value = { kind, id })

// A decision the logged-in person has to make opens by itself, once, as
// soon as no other dialog is open; the bar keeps it in view after that.
const shown = new Set()
watch(
  [() => team.decisions, dialog],
  () => {
    if (dialog.value && current.value) return
    const next = team.decisions.find((p) => !shown.has(p.id) && team.decidesMyself(p))
    if (next) {
      shown.add(next.id)
      open('decision', next.id)
    }
  },
  { immediate: true },
)

async function addOwn() {
  error.value = ''
  try {
    await post(`/teams/${team.view.id}/characters`, { character: ownChoice.value })
    await characters.load()
    await team.refresh()
  } catch (e) {
    error.value = e.message
  }
}
</script>

<template>
  <template v-if="team.view">
    <XPanel :title="team.view.name">
      <template #actions>
        <XStatus
          :tone="team.connected ? 'ok' : 'warn'"
          :text="t(team.connected ? 'team.live' : 'team.offline')"
        />
      </template>
      <p class="text-dim">
        {{ t(`mode.${team.view.mode}`) }} · {{ t('team.gm') }}: {{ team.view.gm.name }}
      </p>
      <template v-if="team.view.is_gm">
        <div class="flex flex-wrap items-center gap-2">
          <XField :label="t('team.invite')">
            <input
              class="w-[40ch] max-w-full"
              readonly
              :value="inviteLink"
              data-test="invite"
              @focus="$event.target.select()"
            >
          </XField>
          <XButton
            variant="quiet"
            @click="team.newInvite()"
          >
            {{ t('team.newInvite') }}
          </XButton>
          <template v-if="freeOwn.length">
            <XField :label="t('team.addOwn')">
              <select v-model="ownChoice">
                <option
                  v-for="c in freeOwn"
                  :key="c.id"
                  :value="c.id"
                >
                  {{ c.name }}
                </option>
              </select>
            </XField>
            <XButton
              :disabled="!ownChoice"
              @click="addOwn"
            >
              {{ t('common.add') }}
            </XButton>
          </template>
        </div>
        <GmBar />
      </template>
      <p
        v-if="error"
        class="text-red"
        role="alert"
      >
        {{ error }}
      </p>
    </XPanel>

    <div class="grid cols-auto items-start gap-3">
      <XPanel :title="t('team.crew')">
        <CrewList
          @roll="open('roll', $event)"
          @sheet="open('sheet', $event)"
          @actions="open('actions', $event)"
        />
      </XPanel>
      <XPanel :title="t('team.log')">
        <LogList />
      </XPanel>
    </div>

    <XModal
      v-if="dialog && current"
      :key="`${dialog.kind}-${dialog.id}`"
      :title="title"
      @close="dialog = null"
    >
      <DecisionDialog
        v-if="dialog.kind === 'decision'"
        :pending="current"
      />
      <RollPanel
        v-else-if="dialog.kind === 'roll'"
        :character="current"
        @done="dialog = null"
      />
      <SheetDialog
        v-else-if="dialog.kind === 'sheet'"
        :character="current"
      />
      <CharacterActions
        v-else
        :character="current"
      />
    </XModal>

    <DecisionBar @open="open('decision', $event)" />
  </template>
  <XPanel
    v-else-if="team.gone"
    :title="t('team.goneTitle')"
  >
    <p>{{ t('team.gone') }}</p>
    <RouterLink to="/">
      {{ t('team.back') }}
    </RouterLink>
  </XPanel>
</template>
