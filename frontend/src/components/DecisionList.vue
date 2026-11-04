<script setup>
import { computed, ref } from 'vue'
import { useI18n } from 'vue-i18n'

import XButton from '@/ui/XButton.vue'
import { choiceLabel } from '@/labels'
import { useAuthStore } from '@/stores/auth'
import { useRulesStore } from '@/stores/rules'
import { useTeamStore } from '@/stores/team'

// Open decisions for the person who has to make them: their own
// characters' rolls, or for the game master all of them.
const { t } = useI18n()
const auth = useAuthStore()
const rules = useRulesStore()
const team = useTeamStore()
const chosen = ref({})
const error = ref('')

const owner = (actor) => team.view.characters.find((c) => String(c.id) === actor)?.owner_id
const mine = computed(() =>
  team.view.pending.filter((p) => team.view.is_gm || (p.actor && owner(p.actor) === auth.user?.id)),
)

function toggle(pending, id, on) {
  const list = new Set(chosen.value[pending] ?? [])
  if (on) list.add(id)
  else list.delete(id)
  chosen.value = { ...chosen.value, [pending]: [...list] }
}

async function decide(pending, choice) {
  error.value = ''
  try {
    await team.command({ command: 'decide', pending, choice })
  } catch (e) {
    error.value = e.message
  }
}
</script>

<template>
  <div
    v-if="mine.length"
    class="panel"
    data-test="decisions"
  >
    <h3>{{ t('decision.title') }}</h3>
    <div
      v-for="p in mine"
      :key="p.id"
      class="panel"
    >
      <p>
        <strong>{{ p.actor ? team.characterNames[p.actor] : t('auth.gm') }}</strong>
        <span v-if="['roll', 'start_roll', 'overkill'].includes(p.kind)"> · {{ t(`decision.kind.${p.kind}`) }}</span>
        <span v-if="p.question"> · {{ rules.text(p.question) }}</span>
        <span v-if="p.who"> · {{ t('decision.targets', { who: t(`who.${p.who}`) }) }}</span>
      </p>
      <div
        v-if="p.kind === 'targets'"
        class="row"
      >
        <label
          v-for="id in p.candidates"
          :key="id"
          class="row"
        >
          <input
            type="checkbox"
            :checked="(chosen[p.id] ?? []).includes(id)"
            @change="toggle(p.id, id, $event.target.checked)"
          >
          {{ team.characterNames[id] }}
        </label>
        <XButton @click="decide(p.id, { targets: chosen[p.id] ?? [] })">
          {{ t('common.confirm') }}
        </XButton>
      </div>
      <div
        v-else
        class="row"
      >
        <XButton
          v-for="(option, i) in p.options"
          :key="i"
          :variant="i === 0 ? 'primary' : 'normal'"
          :data-test="`choice-${typeof option === 'string' ? option : Object.keys(option)[0]}`"
          @click="decide(p.id, option)"
        >
          {{ choiceLabel(option, t, rules) }}
        </XButton>
      </div>
    </div>
    <p
      v-if="error"
      class="error"
      role="alert"
    >
      {{ error }}
    </p>
  </div>
</template>
