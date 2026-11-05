import { vi } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'

import { createAppI18n } from '@/i18n'
import { useRulesStore } from '@/stores/rules'
import { useTeamStore } from '@/stores/team'

/** Answers fetch calls by "METHOD /path" and records them; `null` answers
 * 404. The team view reloaded after a command is the one in the store,
 * unless a test gives its own. */
export function mockApi(routes) {
  const calls = []
  vi.stubGlobal(
    'fetch',
    vi.fn(async (url, options = {}) => {
      const method = options.method ?? 'GET'
      const path = url.replace(/^\/api/, '')
      const body = options.body ? JSON.parse(options.body) : undefined
      calls.push({ method, path, body })
      let handler = routes[`${method} ${path}`]
      const team = useTeamStore().view
      if (handler === undefined && method === 'GET' && team && path === `/teams/${team.id}`) handler = () => team
      if (!handler) return { ok: false, status: 404, statusText: 'not found', text: async () => '' }
      const value = typeof handler === 'function' ? handler(body) : handler
      return { ok: true, status: 200, text: async () => JSON.stringify(value) }
    }),
  )
  return calls
}

export const rulesData = {
  attributes: [{ id: 'wits', page: 27 }],
  skills: [{ id: 'observation', page: 58, attribute: 'wits' }],
  careers: [
    {
      id: 'officer',
      page: 49,
      key_attribute: 'wits',
      key_skills: ['observation'],
      talents: ['pull_rank'],
      cash: { dice: 2, times: 100 },
      names: ['Gorman'],
      looks: ['cap'],
      items: ['pen'],
      agendas: ['order'],
    },
  ],
  talents: [
    { id: 'stoic', page: 79, effects: [] },
    { id: 'watchful', page: 80, effects: [{ kind: 'modifier', applies_to: { skills: ['observation'] }, change: { add: 2 }, when: 'watchful.when' }] },
    { id: 'pull_rank', page: 76, career: 'officer', effects: [{ kind: 'capability', grants: 'pull_rank_order' }] },
  ],
  actions: [{ id: 'pull_rank_order', page: 76, kind: 'slow', target: true }],
  traumas: [],
  conditions: [],
  'critical-injuries': [],
  panic: [],
  constants: [
    { id: 'difficulty_average', page: 63, value: 0 },
    { id: 'creation_attribute_min', page: 28, value: 2 },
    { id: 'creation_attribute_points', page: 28, value: 14 },
    { id: 'cinematic_acts', page: 216, value: 3 },
    { id: 'story_points_max', page: 61, value: 3 },
  ],
}

/** Pinia, i18n and rule data ready to mount components. */
export function setup() {
  const pinia = createPinia()
  setActivePinia(pinia)
  const rules = useRulesStore()
  rules.data = rulesData
  rules.loaded = true
  return { pinia, i18n: createAppI18n('de') }
}
