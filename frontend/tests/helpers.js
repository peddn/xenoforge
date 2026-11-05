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
      gear: [
        [{ item: 'm4a3_service_pistol' }, { item: 'binoculars' }],
        [{ item: 'signal_flare', dice: 1 }, { item: 'irc_mk50_compression_suit' }],
        [{ item: 'knife' }, { item: 'deck_of_cards', count: 2 }],
      ],
    },
  ],
  gear: [
    {
      id: 'm4a3_service_pistol',
      page: 119,
      category: 'pistol',
      weight: '1/2',
      weapon: { skill: 'ranged_combat', bonus: 2, damage: 1, range: 'medium', reloads: true },
    },
    { id: 'binoculars', page: 133, category: 'vision', weight: '1/2' },
    {
      id: 'irc_mk50_compression_suit',
      page: 127,
      category: 'armor',
      weight: '1',
      armor: { rating: 2, vacuum: true },
      supply: { kind: 'air', max: 5 },
    },
    { id: 'signal_flare', page: 55, category: 'other', weight: '1/4' },
    { id: 'knife', page: 126, category: 'close_combat', weight: '1/2', weapon: { skill: 'close_combat', damage: 2, range: 'engaged' } },
    { id: 'deck_of_cards', page: 39, category: 'other', weight: '0' },
    { id: 'autodoc', page: 135, category: 'medical' },
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
    { id: 'start_gear_items', page: 39, value: 2 },
    { id: 'gear_bonus_max', page: 128, value: 3 },
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
