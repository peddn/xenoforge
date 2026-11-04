import { afterEach, describe, expect, it, vi } from 'vitest'
import { flushPromises, mount } from '@vue/test-utils'
import { createMemoryHistory, createRouter } from 'vue-router'

import DecisionList from '@/components/DecisionList.vue'
import RollPanel from '@/components/RollPanel.vue'
import { choiceLabel, originLabel } from '@/labels'
import { guard, routes } from '@/router'
import { useAuthStore } from '@/stores/auth'
import { useTeamStore } from '@/stores/team'
import LoginView from '@/views/LoginView.vue'

import { mockApi, setup } from './helpers'

afterEach(() => {
  vi.unstubAllGlobals()
  vi.useRealTimers()
})

function router() {
  const r = createRouter({ history: createMemoryHistory(), routes })
  r.beforeEach(guard)
  return r
}

describe('accounts', () => {
  it('registers with the chosen role (A-07)', async () => {
    const { pinia, i18n } = setup()
    const calls = mockApi({ 'POST /auth/register': { id: 1, name: 'Mother', role: 'gm' } })
    const r = router()
    const wrapper = mount(LoginView, { global: { plugins: [pinia, i18n, r] } })
    await wrapper.get('[data-test="mode-register"]').trigger('click')
    await wrapper.get('[data-test="name"]').setValue('Mother')
    await wrapper.get('[data-test="password"]').setValue('secret-password')
    await wrapper.get('[data-test="role-gm"]').setValue(true)
    await wrapper.get('form').trigger('submit')
    await flushPromises()
    expect(calls.find((c) => c.method === 'POST')).toEqual({
      method: 'POST',
      path: '/auth/register',
      body: { name: 'Mother', password: 'secret-password', role: 'gm' },
    })
    expect(useAuthStore().user.role).toBe('gm')
  })

  it('sends people without a session to the login page', async () => {
    setup()
    mockApi({})
    const r = router()
    await r.push('/teams/3')
    expect(r.currentRoute.value.name).toBe('login')
    expect(r.currentRoute.value.query.next).toBe('/teams/3')
  })
})

describe('rolling', () => {
  it('asks the server for the dice and sends the roll', async () => {
    vi.useFakeTimers()
    const { pinia, i18n } = setup()
    const calls = mockApi({
      'POST /teams/5/preview': { base_dice: 6, stress_dice: 2, modifications: [], talents: ['stoic'], max_pushes: 1 },
      'POST /teams/5/commands': [],
    })
    useTeamStore().view = { id: 5, characters: [] }
    const wrapper = mount(RollPanel, {
      props: { character: { id: 9, name: 'Charlize' } },
      global: { plugins: [pinia, i18n] },
    })
    await vi.advanceTimersByTimeAsync(200)
    await flushPromises()
    expect(wrapper.get('[data-test="preview"]').text()).toContain('6 Grundwürfel, 2 Stresswürfel')
    expect(calls[0].body).toMatchObject({ actor: '9', kind: { skill: { skill: 'observation' } } })
    await wrapper.get('[data-test="roll-go"]').trigger('click')
    await flushPromises()
    const roll = calls.find((c) => c.path === '/teams/5/commands')
    expect(roll.body.command.command).toBe('roll')
  })

  it('offers the decision to the owner and sends the choice', async () => {
    const { pinia, i18n } = setup()
    const calls = mockApi({ 'POST /teams/5/commands': [] })
    useAuthStore().user = { id: 2, name: 'Gina', role: 'player' }
    useTeamStore().view = {
      id: 5,
      is_gm: false,
      characters: [{ id: 9, name: 'Charlize', owner_id: 2 }],
      pending: [{ id: 4, kind: 'roll', actor: '9', options: ['push', 'accept'], candidates: [] }],
    }
    const wrapper = mount(DecisionList, { global: { plugins: [pinia, i18n] } })
    expect(wrapper.text()).toContain('Pushen')
    await wrapper.get('[data-test="choice-push"]').trigger('click')
    await flushPromises()
    expect(calls[0].body).toEqual({ command: { command: 'decide', pending: 4, choice: 'push' } })
  })

  it('hides other people\'s decisions', () => {
    const { pinia, i18n } = setup()
    mockApi({})
    useAuthStore().user = { id: 3, name: 'Burke', role: 'player' }
    useTeamStore().view = {
      id: 5,
      is_gm: false,
      characters: [{ id: 9, name: 'Charlize', owner_id: 2 }],
      pending: [{ id: 4, kind: 'roll', actor: '9', options: ['push', 'accept'], candidates: [] }],
    }
    const wrapper = mount(DecisionList, { global: { plugins: [pinia, i18n] } })
    expect(wrapper.find('[data-test="decisions"]').exists()).toBe(false)
  })
})

describe('labels', () => {
  it('names choices and sources', () => {
    const { i18n } = setup()
    const t = i18n.global.t
    const rules = { name: (id) => `<${id}>` }
    expect(choiceLabel('push', t, rules)).toBe('Pushen')
    expect(choiceLabel({ use_talent: 'stoic' }, t, rules)).toBe('Mit <stoic>')
    expect(choiceLabel({ answer: false }, t, rules)).toBe('Nein')
    expect(originLabel({ condition: { id: 'freezing', cold: 'sub_zero' } }, t, rules)).toBe('<freezing> (unter null)')
    expect(originLabel({ block: { blocks: 'stress_relief', by: 'depression' } }, t, rules)).toBe(
      '<depression>: Stressabbau gesperrt',
    )
  })
})

describe('translations', () => {
  it('have the same keys in German and English', async () => {
    const { default: de } = await import('@/i18n/de.json')
    const { default: en } = await import('@/i18n/en.json')
    const keys = (obj, prefix = '') =>
      Object.entries(obj).flatMap(([k, v]) => (typeof v === 'object' ? keys(v, `${prefix}${k}.`) : [`${prefix}${k}`]))
    expect(keys(en).sort()).toEqual(keys(de).sort())
  })
})
