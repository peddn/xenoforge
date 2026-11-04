<script setup>
import { onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'

import XButton from '@/ui/XButton.vue'
import XField from '@/ui/XField.vue'
import XPanel from '@/ui/XPanel.vue'
import { get, post } from '@/api'
import { useAuthStore } from '@/stores/auth'
import { useCharactersStore } from '@/stores/characters'

const { t } = useI18n()
const auth = useAuthStore()
const characters = useCharactersStore()
const router = useRouter()

const teams = ref([])
const newCharacter = ref('')
const newTeam = ref('')
const newMode = ref('cinematic')
const error = ref('')

onMounted(async () => {
  await characters.load()
  teams.value = await get('/teams')
})

const teamName = (id) => teams.value.find((team) => team.id === id)?.name ?? `#${id}`

async function createCharacter() {
  error.value = ''
  try {
    const character = await characters.create(newCharacter.value)
    newCharacter.value = ''
    router.push(`/characters/${character.id}`)
  } catch (e) {
    error.value = e.message
  }
}

async function createTeam() {
  error.value = ''
  try {
    const team = await post('/teams', { name: newTeam.value, mode: newMode.value })
    router.push(`/teams/${team.id}`)
  } catch (e) {
    error.value = e.message
  }
}
</script>

<template>
  <div class="grid-2">
    <XPanel :title="t('home.characters')">
      <ul class="list">
        <li
          v-for="character in characters.list"
          :key="character.id"
        >
          <RouterLink :to="`/characters/${character.id}`">
            {{ character.name }}
          </RouterLink>
          <span
            v-if="character.team_id"
            class="muted"
          > · {{ teamName(character.team_id) }}</span>
        </li>
        <li
          v-if="!characters.list.length"
          class="muted"
        >
          {{ t('home.noCharacters') }}
        </li>
      </ul>
      <form
        class="row"
        @submit.prevent="createCharacter"
      >
        <XField :label="t('home.newCharacter')">
          <input
            v-model="newCharacter"
            required
            data-test="new-character"
          >
        </XField>
        <XButton type="submit">
          {{ t('common.create') }}
        </XButton>
      </form>
    </XPanel>

    <XPanel :title="t('home.teams')">
      <ul class="list">
        <li
          v-for="team in teams"
          :key="team.id"
        >
          <RouterLink :to="`/teams/${team.id}`">
            {{ team.name }}
          </RouterLink>
          <span class="muted"> · {{ t(`mode.${team.mode}`) }} · {{ t(team.is_gm ? 'auth.gm' : 'auth.player') }}</span>
        </li>
        <li
          v-if="!teams.length"
          class="muted"
        >
          {{ t('home.noTeams') }}
        </li>
      </ul>
      <form
        v-if="auth.user?.role === 'gm'"
        class="row"
        @submit.prevent="createTeam"
      >
        <XField :label="t('home.newTeam')">
          <input
            v-model="newTeam"
            required
            data-test="new-team"
          >
        </XField>
        <XField :label="t('home.mode')">
          <select v-model="newMode">
            <option value="cinematic">
              {{ t('mode.cinematic') }}
            </option>
            <option value="campaign">
              {{ t('mode.campaign') }}
            </option>
          </select>
        </XField>
        <XButton type="submit">
          {{ t('common.create') }}
        </XButton>
      </form>
      <p
        v-else
        class="muted"
      >
        {{ t('home.teamsByGm') }}
      </p>
    </XPanel>
    <p
      v-if="error"
      class="error"
      role="alert"
    >
      {{ error }}
    </p>
  </div>
</template>
