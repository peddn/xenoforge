<script setup>
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

import XButton from '@/ui/XButton.vue'
import XDie from '@/ui/XDie.vue'
import { choiceLabel, decisionText, opsText, refusal } from '@/labels'
import { useRulesStore } from '@/stores/rules'
import { useTeamStore } from '@/stores/team'

// One open decision with everything needed to make it: for a roll the dice
// so far, the successes and the pushes left; for a choice of targets who
// asks, why and what happens to each one chosen. All as the server sends
// them.
const props = defineProps({ pending: { type: Object, required: true } })
const { t } = useI18n()
const rules = useRulesStore()
const team = useTeamStore()
const chosen = ref([])
const error = ref('')

async function decide(choice) {
  error.value = ''
  try {
    await team.command({ command: 'decide', pending: props.pending.id, choice })
  } catch (e) {
    error.value = refusal(e, t)
  }
}

function toggle(id, on) {
  chosen.value = on ? [...chosen.value, id] : chosen.value.filter((c) => c !== id)
}
</script>

<template>
  <div
    class="grid gap-3"
    data-test="decision"
  >
    <p
      v-if="pending.kind === 'targets' && pending.source"
      class="text-strong"
      data-test="decision-cause"
    >
      {{ t('decision.cause', { actor: team.characterNames[pending.by] ?? pending.by, source: rules.name(pending.source) }) }}
    </p>
    <p>{{ decisionText(pending, t, rules) }}</p>
    <p
      v-if="pending.ops?.length"
      class="text-dim"
      data-test="decision-effect"
    >
      {{ t('decision.effect', { effect: opsText(pending.ops, t) }) }}
    </p>
    <div
      v-if="pending.roll"
      class="grid gap-2 border border-line p-3"
    >
      <p>
        {{ rules.name(pending.roll.skill ?? pending.roll.attribute) }} ·
        {{ t('roll.preview', { base: pending.roll.base_dice, stress: pending.roll.stress_dice }) }}
      </p>
      <div class="flex flex-wrap gap-1">
        <XDie
          v-for="(die, i) in pending.roll.dice"
          :key="i"
          :die="die"
        />
      </div>
      <p>
        {{ t('log.successes', pending.roll.successes) }} ·
        <span class="text-dim">{{ t('decision.pushesLeft', pending.roll.pushes_left) }}</span>
      </p>
    </div>
    <div
      v-if="pending.kind === 'targets'"
      class="flex flex-wrap items-center gap-2"
    >
      <label
        v-for="id in pending.candidates"
        :key="id"
        class="flex items-center gap-2"
      >
        <input
          type="checkbox"
          :checked="chosen.includes(id)"
          @change="toggle(id, $event.target.checked)"
        >
        {{ team.characterNames[id] }}
      </label>
      <XButton @click="decide({ targets: chosen })">
        {{ t('common.confirm') }}
      </XButton>
    </div>
    <template v-else>
      <div class="flex flex-wrap items-center gap-2">
        <XButton
          v-for="(option, i) in pending.options"
          :key="i"
          :variant="i === 0 ? 'primary' : 'normal'"
          :data-test="`choice-${typeof option === 'string' ? option : Object.keys(option)[0]}`"
          @click="decide(option)"
        >
          {{ choiceLabel(option, t, rules) }}
        </XButton>
      </div>
      <p
        v-if="pending.options.includes('push')"
        class="text-sm text-dim"
      >
        {{ t('decision.pushHint') }}
      </p>
    </template>
    <p
      v-if="error"
      class="text-red"
      role="alert"
    >
      {{ error }}
    </p>
  </div>
</template>
