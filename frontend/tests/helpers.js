import { vi } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'

import { createAppI18n } from '@/i18n'
import { useRulesStore } from '@/stores/rules'

/** Answers fetch calls by "METHOD /path" and records them. */
export function mockApi(routes) {
  const calls = []
  vi.stubGlobal(
    'fetch',
    vi.fn(async (url, options = {}) => {
      const method = options.method ?? 'GET'
      const path = url.replace(/^\/api/, '')
      const body = options.body ? JSON.parse(options.body) : undefined
      calls.push({ method, path, body })
      const handler = routes[`${method} ${path}`]
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
  talents: [{ id: 'stoic', page: 79, effects: [] }],
  traumas: [],
  conditions: [],
  'critical-injuries': [],
  panic: [],
  constants: [{ id: 'difficulty_average', page: 63, value: 0 }],
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
