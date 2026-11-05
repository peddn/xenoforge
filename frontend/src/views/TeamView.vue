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
import XConfirm from '@/ui/XConfirm.vue'
import XField from '@/ui/XField.vue'
import XModal from '@/ui/XModal.vue'
import XPanel from '@/ui/XPanel.vue'
import XStatus from '@/ui/XStatus.vue'
import { del, post } from '@/api'
import { useAuthStore } from '@/stores/auth'
import { useCharactersStore } from '@/stores/characters'
import { useRulesStore } from '@/stores/rules'
import { useTeamStore } from '@/stores/team'
import { refusal } from '@/labels'

const { t } = useI18n()
const route = useRoute()
const team = useTeamStore()
const characters = useCharactersStore()
const rules = useRulesStore()
const auth = useAuthStore()

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
// Members without a character in the team, e.g. before choosing (A-12).
// They can leave, and the game master can take them out.
const waiting = computed(() =>
  team.view.members.filter((m) => !team.view.characters.some((c) => c.owner_id === m.id)),
)

async function removeMember(member) {
  error.value = ''
  try {
    await del(`/teams/${team.view.id}/members/${member.id}`)
    await team.refresh()
  } catch (e) {
    error.value = refusal(e, t)
  }
}
const freeOwn = computed(() => characters.list.filter((c) => !c.team_id && c.data.status !== 'draft'))
// Looked up live, so the dialog shows current values; it closes when the
// character leaves the team, its values are no longer to be seen (an
// offered character someone else chose) or the decision is made.
const current = computed(() => {
  if (!dialog.value || !team.view) return null
  if (dialog.value.kind === 'decision') return team.view.pending.find((p) => p.id === dialog.value.id)
  const character = team.view.characters.find((c) => c.id === dialog.value.id)
  return character?.actor ? character : null
})
// Gone is gone: the dialog does not come back by itself, e.g. when the
// character is offered again later.
watch(current, (now) => {
  if (dialog.value && !now) dialog.value = null
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
    error.value = refusal(e, t)
  }
}
</script>

<template>
  <!-- Wide screens: team and crew on the left, the log on the right from top
       to bottom; phones: all one below the other. The decision bar is fixed
       at the bottom edge, so the log leaves room for it. -->
  <div
    v-if="team.view"
    class="grid items-start gap-3 lg:grid-cols-2"
    :class="{ 'pb-16': team.decisions.length }"
  >
    <div class="grid min-w-0 content-start gap-3">
      <XPanel :title="team.view.name">
        <template #actions>
          <XStatus
            :tone="team.connected ? 'ok' : 'warn'"
            :text="t(team.connected ? 'team.live' : 'team.offline')"
          />
        </template>
        <p class="text-dim">
          {{ t(`mode.${team.view.mode}`) }}<template v-if="team.view.act">
            · {{ team.view.act > rules.constant('cinematic_acts') ? t('team.scenarioOver') : t('team.act', { act: team.view.act }) }}
          </template> · {{ t('team.gm') }}: {{ team.view.gm.name }}
        </p>
        <div
          v-if="waiting.length"
          class="flex flex-wrap items-center gap-2 text-dim"
          data-test="waiting"
        >
          <span>{{ t('team.waiting') }}</span>
          <span
            v-for="m in waiting"
            :key="m.id"
            class="flex items-center gap-1"
          >
            {{ m.name }}
            <XConfirm
              v-if="team.view.is_gm || m.id === auth.user?.id"
              variant="danger"
              size="small"
              :label="m.id === auth.user?.id ? t('team.leave') : t('common.remove')"
              :question="t('team.removeMemberConfirm', { name: m.name })"
              :data-test="`remove-member-${m.id}`"
              @confirm="removeMember(m)"
            />
          </span>
        </div>
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
      <XPanel :title="t('team.crew')">
        <CrewList
          @roll="open('roll', $event)"
          @sheet="open('sheet', $event)"
          @actions="open('actions', $event)"
        />
      </XPanel>
    </div>
    <XPanel
      :title="t('team.log')"
      class="lg:sticky lg:top-4 lg:grid-rows-[auto_minmax(0,1fr)]"
      :class="team.decisions.length ? 'lg:h-[calc(100dvh-9.5rem)]' : 'lg:h-[calc(100dvh-5.5rem)]'"
    >
      <LogList />
    </XPanel>

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
  </div>
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
