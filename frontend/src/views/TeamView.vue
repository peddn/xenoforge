<script setup>
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute } from 'vue-router'

import CharacterActions from '@/components/CharacterActions.vue'
import CrewList from '@/components/CrewList.vue'
import DecisionList from '@/components/DecisionList.vue'
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

// The open dialog: { kind: 'roll' | 'sheet' | 'actions', id }.
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
// character leaves the team.
const current = computed(() => team.view?.characters.find((c) => c.id === dialog.value?.id))
const titles = { roll: 'roll.title', sheet: 'sheet.title', actions: 'gm.actionsTitle' }

const open = (kind, id) => (dialog.value = { kind, id })

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
      <p class="muted">
        {{ t(`mode.${team.view.mode}`) }} · {{ t('team.gm') }}: {{ team.view.gm.name }}
      </p>
      <template v-if="team.view.is_gm">
        <div class="row">
          <XField :label="t('team.invite')">
            <input
              class="wide"
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
        class="error"
        role="alert"
      >
        {{ error }}
      </p>
    </XPanel>

    <DecisionList />

    <div class="grid-2">
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
      :title="t(titles[dialog.kind], { name: current.name })"
      @close="dialog = null"
    >
      <RollPanel
        v-if="dialog.kind === 'roll'"
        :character="current"
        @done="dialog = null"
      />
      <SheetDialog
        v-else-if="dialog.kind === 'sheet'"
        :character="current"
        @close="dialog = null"
      />
      <CharacterActions
        v-else
        :character="current"
      />
    </XModal>
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
