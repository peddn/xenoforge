<script setup>
import { onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute, useRouter } from 'vue-router'

import ActorEditor from '@/components/ActorEditor.vue'
import XButton from '@/ui/XButton.vue'
import XField from '@/ui/XField.vue'
import XPanel from '@/ui/XPanel.vue'
import { get } from '@/api'
import { useCharactersStore } from '@/stores/characters'

const { t } = useI18n()
const route = useRoute()
const router = useRouter()
const store = useCharactersStore()

const character = ref(null)
const message = ref('')
const error = ref('')

onMounted(async () => {
  character.value = await get(`/characters/${route.params.id}`)
})

async function save() {
  error.value = ''
  try {
    character.value = await store.save(character.value)
    message.value = t('common.saved')
  } catch (e) {
    error.value = e.message
  }
}

async function remove() {
  await store.remove(character.value.id)
  router.push('/')
}
</script>

<template>
  <XPanel
    v-if="character"
    :title="character.name"
  >
    <XField :label="t('sheet.name')">
      <input
        v-model="character.name"
        data-test="name"
      >
    </XField>
    <XField :label="t('sheet.appearance')">
      <input v-model="character.data.sheet.appearance">
    </XField>
    <XField :label="t('sheet.notes')">
      <textarea
        v-model="character.data.sheet.notes"
        rows="3"
      />
    </XField>
    <p
      v-if="character.team_id"
      class="muted"
    >
      {{ t('sheet.inTeam') }}
    </p>
    <ActorEditor
      v-model="character.data.actor"
      :disabled="!!character.team_id"
    />
    <div class="row">
      <XButton
        variant="primary"
        data-test="save"
        @click="save"
      >
        {{ t('common.save') }}
      </XButton>
      <XButton
        v-if="!character.team_id"
        variant="danger"
        @click="remove"
      >
        {{ t('common.delete') }}
      </XButton>
      <span
        v-if="message"
        class="muted"
        role="status"
      >{{ message }}</span>
      <span
        v-if="error"
        class="error"
        role="alert"
      >{{ error }}</span>
    </div>
  </XPanel>
</template>
