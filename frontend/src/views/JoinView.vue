<script setup>
import { computed, onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute, useRouter } from 'vue-router'

import XButton from '@/ui/XButton.vue'
import XField from '@/ui/XField.vue'
import XPanel from '@/ui/XPanel.vue'
import { get, post } from '@/api'
import { useCharactersStore } from '@/stores/characters'
import { refusal } from '@/labels'

// Joining a team: the player chooses one of their free characters (A-04).
const { t } = useI18n()
const route = useRoute()
const router = useRouter()
const characters = useCharactersStore()

const info = ref(null)
const chosen = ref(null)
const error = ref('')
const free = computed(() => characters.list.filter((c) => !c.team_id && c.data.status !== 'draft'))

onMounted(async () => {
  try {
    info.value = await get(`/join/${route.params.token}`)
    await characters.load()
    chosen.value = free.value[0]?.id ?? 'none'
  } catch (e) {
    error.value = refusal(e, t)
  }
})

async function join() {
  try {
    // Without a character of one's own, to choose one in the team (A-12).
    const character = chosen.value === 'none' ? null : chosen.value
    const result = await post(`/join/${route.params.token}`, { character })
    router.push(`/teams/${result.team}`)
  } catch (e) {
    error.value = refusal(e, t)
  }
}
</script>

<template>
  <XPanel :title="t('join.title')">
    <template v-if="info">
      <p>{{ t('join.invited', { team: info.team, gm: info.gm }) }} · {{ t(`mode.${info.mode}`) }}</p>
      <XField :label="t('join.character')">
        <select
          v-model="chosen"
          data-test="join-character"
        >
          <option
            v-for="character in free"
            :key="character.id"
            :value="character.id"
          >
            {{ character.name }}
          </option>
          <option value="none">
            {{ t('join.none') }}
          </option>
        </select>
      </XField>
      <p
        v-if="!free.length"
        class="text-dim"
      >
        {{ t('join.noFree') }} <RouterLink to="/">
          {{ t('home.characters') }}
        </RouterLink>
      </p>
      <div class="flex flex-wrap items-center gap-2">
        <XButton
          variant="primary"
          :disabled="!chosen"
          @click="join"
        >
          {{ t('join.join') }}
        </XButton>
        <XButton
          variant="quiet"
          @click="router.push('/')"
        >
          {{ t('join.decline') }}
        </XButton>
      </div>
    </template>
    <p
      v-if="error"
      class="text-red"
      role="alert"
    >
      {{ error }}
    </p>
  </XPanel>
</template>
