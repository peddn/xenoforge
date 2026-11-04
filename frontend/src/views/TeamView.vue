<script setup>
import { computed, onMounted, onUnmounted, ref, toRaw } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute } from 'vue-router'

import ActorEditor from '@/components/ActorEditor.vue'
import CrewList from '@/components/CrewList.vue'
import DecisionList from '@/components/DecisionList.vue'
import GmTools from '@/components/GmTools.vue'
import LogList from '@/components/LogList.vue'
import RollPanel from '@/components/RollPanel.vue'
import XButton from '@/ui/XButton.vue'
import XField from '@/ui/XField.vue'
import XPanel from '@/ui/XPanel.vue'
import XStatus from '@/ui/XStatus.vue'
import { del, post } from '@/api'
import { useCharactersStore } from '@/stores/characters'
import { useTeamStore } from '@/stores/team'

const { t } = useI18n()
const route = useRoute()
const team = useTeamStore()
const characters = useCharactersStore()

const rolling = ref(null)
const correcting = ref(null)
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

function startCorrection(character) {
  // A copy to edit; structuredClone refuses Vue's reactive proxies.
  correcting.value = { id: character.id, name: character.name, actor: structuredClone(toRaw(character.actor)) }
}

async function saveCorrection() {
  try {
    await team.correct(correcting.value.id, correcting.value.actor)
    correcting.value = null
  } catch (e) {
    error.value = e.message
  }
}

async function addOwn() {
  try {
    await post(`/teams/${team.view.id}/characters`, { character: ownChoice.value })
    await characters.load()
    await team.refresh()
  } catch (e) {
    error.value = e.message
  }
}

async function removeCharacter(character) {
  try {
    await del(`/teams/${team.view.id}/characters/${character.id}`)
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
      <div
        v-if="team.view.is_gm"
        class="row"
      >
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
    </XPanel>

    <DecisionList />

    <div class="grid-2">
      <XPanel :title="t('team.crew')">
        <CrewList
          @roll="rolling = $event"
          @correct="startCorrection"
        />
        <RollPanel
          v-if="rolling"
          :key="rolling.id"
          :character="rolling"
          @done="rolling = null"
        />
        <div
          v-if="correcting"
          class="panel"
        >
          <h3>{{ t('gm.correctTitle', { name: correcting.name }) }}</h3>
          <ActorEditor v-model="correcting.actor" />
          <div class="row">
            <XButton
              variant="primary"
              @click="saveCorrection"
            >
              {{ t('common.save') }}
            </XButton>
            <XButton
              variant="danger"
              @click="removeCharacter(correcting); correcting = null"
            >
              {{ t('team.removeCharacter') }}
            </XButton>
            <XButton
              variant="quiet"
              @click="correcting = null"
            >
              {{ t('common.cancel') }}
            </XButton>
          </div>
        </div>
        <GmTools v-if="team.view.is_gm && team.view.characters.length" />
      </XPanel>
      <XPanel :title="t('team.log')">
        <LogList />
      </XPanel>
    </div>
    <p
      v-if="error"
      class="error"
      role="alert"
    >
      {{ error }}
    </p>
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
