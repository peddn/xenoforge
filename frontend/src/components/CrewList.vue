<script setup>
import { computed, ref } from 'vue'
import { useI18n } from 'vue-i18n'

import XButton from '@/ui/XButton.vue'
import XStatus from '@/ui/XStatus.vue'
import { originLabel } from '@/labels'
import { useAuthStore } from '@/stores/auth'
import { useRulesStore } from '@/stores/rules'
import { useTeamStore } from '@/stores/team'

// The crew part of the overview (plan 3.1). Players see the values of
// their own characters only (A-01). Rolling, the sheet and the game
// master's actions open as dialogs.
const emit = defineEmits(['roll', 'sheet', 'actions'])
const { t } = useI18n()
const auth = useAuthStore()
const rules = useRulesStore()
const team = useTeamStore()

const mine = (c) => c.owner_id === auth.user?.id
const error = ref('')

// A player takes an offered character; whoever comes first gets it (A-12).
async function claim(c) {
  error.value = ''
  try {
    await team.claim(c.id)
    await team.refresh()
  } catch (e) {
    error.value = e.message
  }
}
// Story Points exist in Cinematic play only (F-06) and belong to the player.
const cinematic = computed(() => team.view.mode === 'cinematic')
const storyPoints = (c) => team.view.story_points[String(c.owner_id)] ?? 0
</script>

<template>
  <div class="overflow-x-auto">
    <table>
      <thead>
        <tr>
          <th>{{ t('crew.character') }}</th>
          <th>{{ t('sheet.health') }}</th>
          <th>{{ t('sheet.stress') }}</th>
          <th v-if="cinematic">
            {{ t('crew.storyPoints') }}
          </th>
          <th>{{ t('crew.state') }}</th>
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="c in team.view.characters"
          :key="c.id"
          :data-test="`crew-${c.id}`"
        >
          <td>
            {{ c.name }}<br><span class="text-dim">{{ c.owner_name }}</span>
            <XStatus
              v-if="c.offered"
              class="ml-2"
              tone="warn"
              :text="t('crew.offered')"
            />
            <div
              v-if="c.actor"
              class="mt-1 flex flex-wrap items-center gap-1"
            >
              <XButton
                v-if="c.offered && !team.view.is_gm"
                variant="highlight"
                size="small"
                :data-test="`claim-${c.id}`"
                @click="claim(c)"
              >
                {{ t('crew.claim') }}
              </XButton>
              <XButton
                v-if="mine(c) || team.view.is_gm"
                size="small"
                :data-test="`roll-${c.id}`"
                @click="emit('roll', c.id)"
              >
                {{ t('roll.roll') }}
              </XButton>
              <XButton
                variant="quiet"
                size="small"
                :data-test="`sheet-${c.id}`"
                @click="emit('sheet', c.id)"
              >
                {{ t('crew.sheet') }}
              </XButton>
              <XButton
                v-if="team.view.is_gm"
                variant="quiet"
                size="small"
                :data-test="`actions-${c.id}`"
                @click="emit('actions', c.id)"
              >
                {{ t('crew.actions') }}
              </XButton>
            </div>
          </td>
          <template v-if="c.actor">
            <td>
              <XStatus
                v-if="c.broken"
                tone="alert"
                :text="t('crew.broken')"
              />
              <span v-else>{{ c.actor.health }}/{{ c.health_max }}</span>
            </td>
            <td>{{ c.actor.android ? '–' : c.actor.stress }}</td>
            <td v-if="cinematic">
              {{ storyPoints(c) }}
            </td>
            <td>
              <div v-if="c.actor.panic">
                <XStatus
                  tone="warn"
                  :text="`${t('crew.panic')} ${c.actor.panic.value} · ${rules.name(c.actor.panic.effect)}`"
                />
              </div>
              <div
                v-for="(source, i) in c.actor.sources"
                :key="i"
              >
                {{ originLabel(source.origin, t, rules) }}
              </div>
              <div
                v-for="([label, text], i) in c.hints"
                :key="`h${i}`"
                class="text-dim"
                :title="label"
              >
                ▲ {{ rules.text(text) }}
              </div>
            </td>
          </template>
          <td
            v-else
            :colspan="cinematic ? 4 : 3"
            class="text-dim"
          >
            {{ t('crew.hidden') }}
          </td>
        </tr>
      </tbody>
    </table>
    <p
      v-if="error"
      class="text-red"
      role="alert"
    >
      {{ error }}
    </p>
  </div>
</template>
