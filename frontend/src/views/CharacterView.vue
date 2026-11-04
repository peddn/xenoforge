<script setup>
import { computed, onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute, useRouter } from 'vue-router'

import ActorEditor from '@/components/ActorEditor.vue'
import CharacterSheet from '@/components/CharacterSheet.vue'
import CreationWizard from '@/components/CreationWizard.vue'
import XButton from '@/ui/XButton.vue'
import XConfirm from '@/ui/XConfirm.vue'
import XField from '@/ui/XField.vue'
import XPanel from '@/ui/XPanel.vue'
import { get } from '@/api'
import { useAuthStore } from '@/stores/auth'
import { useCharactersStore } from '@/stores/characters'
import { useRulesStore } from '@/stores/rules'

// One's own character: a draft in the wizard, otherwise the sheet. Texts and
// the agenda can always be edited (F-54). Values only on a free character of
// a game master outside a team (A-09).
const { t } = useI18n()
const route = useRoute()
const router = useRouter()
const auth = useAuthStore()
const rules = useRulesStore()
const store = useCharactersStore()

const character = ref(null)
const message = ref('')
const error = ref('')

const free = computed(() => character.value.data.status === 'free' && auth.user?.role === 'gm')

onMounted(async () => {
  try {
    character.value = await get(`/characters/${route.params.id}`)
  } catch (e) {
    error.value = e.message
  }
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
  error.value = ''
  try {
    await store.remove(character.value.id)
    router.push('/')
  } catch (e) {
    error.value = e.message
  }
}
</script>

<template>
  <XPanel
    v-if="character?.data.status === 'draft'"
    :title="character.name || t('wizard.title')"
  >
    <CreationWizard
      :character="character"
      @finished="character = $event"
    />
    <div class="flex flex-wrap items-center gap-2 border-t border-dashed border-line pt-3">
      <XConfirm
        variant="danger"
        :label="t('common.delete')"
        :question="t('wizard.deleteConfirm')"
        data-test="delete"
        @confirm="remove"
      />
    </div>
  </XPanel>
  <XPanel
    v-else-if="character"
    :title="character.name"
  >
    <div class="grid cols-auto items-start gap-3">
      <XField :label="t('sheet.name')">
        <input
          v-model="character.name"
          data-test="name"
        >
      </XField>
      <XField
        v-if="free"
        :label="t('sheet.career')"
      >
        <select v-model="character.data.sheet.career">
          <option :value="null">
            –
          </option>
          <option
            v-for="career in rules.list('careers')"
            :key="career.id"
            :value="career.id"
          >
            {{ rules.name(career.id) }}
          </option>
        </select>
      </XField>
      <XField :label="t('sheet.appearance')">
        <input v-model="character.data.sheet.appearance">
      </XField>
      <XField :label="t('sheet.signatureItem')">
        <input v-model="character.data.sheet.signature_item">
      </XField>
    </div>
    <XField :label="t('sheet.agenda')">
      <textarea
        v-model="character.data.sheet.agenda"
        rows="2"
        data-test="agenda"
      />
    </XField>
    <XField :label="t('sheet.notes')">
      <textarea
        v-model="character.data.sheet.notes"
        rows="3"
      />
    </XField>
    <p
      v-if="character.team_id"
      class="text-dim"
    >
      {{ t('sheet.inTeam') }}
    </p>
    <ActorEditor
      v-if="free"
      v-model="character.data.actor"
      :disabled="!!character.team_id"
    />
    <CharacterSheet
      v-else
      :sheet="character.data.sheet"
      :actor="character.data.actor"
      :start-cash="character.data.start_cash"
      :texts="false"
    />
    <div class="flex flex-wrap items-center gap-2">
      <XButton
        variant="primary"
        data-test="save"
        @click="save"
      >
        {{ t('common.save') }}
      </XButton>
      <XConfirm
        v-if="!character.team_id"
        variant="danger"
        :label="t('common.delete')"
        :question="t('sheet.deleteConfirm', { name: character.name })"
        data-test="delete"
        @confirm="remove"
      />
      <span
        v-if="message"
        class="text-dim"
        role="status"
      >{{ message }}</span>
      <span
        v-if="error"
        class="text-red"
        role="alert"
      >{{ error }}</span>
    </div>
  </XPanel>
  <p
    v-else-if="error"
    class="text-red"
    role="alert"
  >
    {{ error }}
  </p>
</template>
