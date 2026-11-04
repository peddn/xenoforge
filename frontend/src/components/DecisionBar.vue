<script setup>
import { useI18n } from 'vue-i18n'

import XButton from '@/ui/XButton.vue'
import { decisionText } from '@/labels'
import { useRulesStore } from '@/stores/rules'
import { useTeamStore } from '@/stores/team'

// Open decisions, always in view at the bottom of the screen. Each one
// blinks briefly when it appears and then stays highlighted until it is
// decided; a click opens it.
const emit = defineEmits(['open'])
const { t } = useI18n()
const rules = useRulesStore()
const team = useTeamStore()

function label(p) {
  const name = p.actor ? team.characterNames[p.actor] : t('auth.gm')
  const what = decisionText(p, t, rules)
  return team.decidesMyself(p) ? `${name} · ${what}` : t('decision.waiting', { name, what })
}
</script>

<template>
  <div
    v-if="team.decisions.length"
    class="sticky bottom-2 z-10 flex flex-wrap justify-center gap-2 border border-amber bg-bg px-3 py-2"
    data-test="decision-bar"
  >
    <XButton
      v-for="p in team.decisions"
      :key="p.id"
      :variant="team.decidesMyself(p) ? 'highlight' : 'notice'"
      class="animate-blink motion-reduce:animate-none"
      :data-test="`decision-${p.id}`"
      @click="emit('open', p.id)"
    >
      ▲ {{ label(p) }}
    </XButton>
  </div>
</template>
