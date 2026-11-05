import { afterEach, describe, expect, it, vi } from 'vitest'
import { flushPromises, mount } from '@vue/test-utils'
import { createMemoryHistory, createRouter } from 'vue-router'

import App from '@/App.vue'

import DecisionBar from '@/components/DecisionBar.vue'
import DecisionDialog from '@/components/DecisionDialog.vue'
import GmBar from '@/components/GmBar.vue'
import LogList from '@/components/LogList.vue'
import CharacterActions from '@/components/CharacterActions.vue'
import CrewList from '@/components/CrewList.vue'
import CharacterSheet from '@/components/CharacterSheet.vue'
import CreationWizard from '@/components/CreationWizard.vue'
import GearPanel from '@/components/GearPanel.vue'
import RollPanel from '@/components/RollPanel.vue'
import SheetDialog from '@/components/SheetDialog.vue'
import { cashFormula, choiceLabel, modsText, originLabel, problemText, quarters, weightText } from '@/labels'
import { guard, routes } from '@/router'
import { useAuthStore } from '@/stores/auth'
import { useRulesStore } from '@/stores/rules'
import { useTeamStore } from '@/stores/team'
import LoginView from '@/views/LoginView.vue'
import JoinView from '@/views/JoinView.vue'
import TeamView from '@/views/TeamView.vue'

import { mockApi, setup } from './helpers'
import { createAppI18n } from '@/i18n'

afterEach(() => {
  // No reload of a team left over for the next test.
  useTeamStore().close()
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
      'POST /teams/5/preview': {
        base_dice: 6,
        stress_dice: 2,
        modifications: [],
        talents: ['stoic'],
        items: [{ uid: 0, item: 'binoculars', value: 2, when: 'binoculars.when' }],
        max_pushes: 1,
      },
      'POST /teams/5/commands': [],
    })
    useTeamStore().view = { id: 5, characters: [] }
    const wrapper = mount(RollPanel, {
      props: { character: { id: 9, name: 'Charlize', actor: { items: [{ uid: 0, item: 'binoculars', count: 1 }] } } },
      global: { plugins: [pinia, i18n] },
    })
    await vi.advanceTimersByTimeAsync(200)
    await flushPromises()
    expect(wrapper.get('[data-test="preview"]').text()).toContain('6 Grundwürfel, 2 Stresswürfel')
    expect(calls[0].body).toMatchObject({ actor: '9', kind: { skill: { skill: 'observation' } }, modifiers: [] })
    // Players set no difficulty and no free gear bonus (A-08, F-59); they
    // tick the items the server offers. The preview is asked for once.
    expect(wrapper.text()).not.toContain('Schwierigkeit:')
    expect(wrapper.text()).toContain('Spielleitung')
    expect(wrapper.find('[data-test="gear"]').exists()).toBe(false)
    expect(wrapper.get('[data-test="items"]').text()).toContain('binoculars +2')
    await wrapper.get('[data-test="roll-item-0"]').setValue(true)
    await vi.advanceTimersByTimeAsync(1000)
    await flushPromises()
    const previews = calls.filter((c) => c.path === '/teams/5/preview')
    expect(previews).toHaveLength(2)
    expect(previews[1].body.items).toEqual([0])
    expect(previews[1].body.modifiers).toEqual([])
    await wrapper.get('[data-test="roll-go"]').trigger('click')
    await flushPromises()
    await wrapper.get('[data-test="note"]').setValue('Tür zum Labor')
    await wrapper.get('[data-test="roll-go"]').trigger('click')
    await flushPromises()
    const rolls = calls.filter((c) => c.path === '/teams/5/commands')
    expect(rolls[0].body.command.command).toBe('roll')
    expect(rolls[0].body.note).toBeUndefined()
    expect(rolls[1].body.note).toBe('Tür zum Labor')
  })

  it('keeps the items to untick when a preview fails, and clears them for another skill', async () => {
    vi.useFakeTimers()
    const { pinia, i18n } = setup()
    const offer = { base_dice: 4, stress_dice: 0, modifications: [], talents: [], max_pushes: 1 }
    const weapons = [
      { uid: 0, item: 'm4a3_service_pistol', value: 2, weapon: true },
      { uid: 1, item: 'knife', value: 0, weapon: true },
    ]
    const calls = []
    vi.stubGlobal(
      'fetch',
      vi.fn(async (url, options = {}) => {
        const body = JSON.parse(options.body)
        calls.push(body)
        if (body.items?.length > 1) {
          return { ok: false, status: 422, statusText: 'rule', text: async () => JSON.stringify({ message: 'one weapon per roll (F-59)' }) }
        }
        return { ok: true, status: 200, text: async () => JSON.stringify({ ...offer, items: weapons }) }
      }),
    )
    useTeamStore().view = { id: 5, characters: [] }
    const wrapper = mount(RollPanel, {
      props: { character: { id: 9, name: 'Charlize', actor: { items: [] } } },
      global: { plugins: [pinia, i18n] },
    })
    await vi.advanceTimersByTimeAsync(200)
    await flushPromises()
    await wrapper.get('[data-test="roll-item-0"]').setValue(true)
    await wrapper.get('[data-test="roll-item-1"]').setValue(true)
    await vi.advanceTimersByTimeAsync(200)
    await flushPromises()
    expect(wrapper.text()).toContain('one weapon per roll')
    await wrapper.get('[data-test="roll-item-1"]').setValue(false)
    await vi.advanceTimersByTimeAsync(200)
    await flushPromises()
    expect(wrapper.find('[role="alert"]').exists()).toBe(false)
    await wrapper.get('[data-test="roll-what"]').setValue('attribute:wits')
    await vi.advanceTimersByTimeAsync(200)
    await flushPromises()
    expect(calls.at(-1).items).toEqual([])
  })

  const pushDecision = {
    id: 4,
    kind: 'roll',
    actor: '9',
    options: ['push', 'accept'],
    candidates: [],
    roll: {
      skill: 'observation',
      base_dice: 2,
      stress_dice: 1,
      dice: [
        { kind: 'base', face: 6, locked: false },
        { kind: 'base', face: 3, locked: false },
        { kind: 'stress', face: 2, locked: false },
      ],
      successes: 1,
      pushes_left: 1,
    },
  }

  it('shows the roll in the decision and sends the choice', async () => {
    const { pinia, i18n } = setup()
    const calls = mockApi({ 'POST /teams/5/commands': [] })
    useTeamStore().view = { id: 5, characters: [{ id: 9, name: 'Charlize', owner_id: 2 }], pending: [pushDecision] }
    const wrapper = mount(DecisionDialog, { props: { pending: pushDecision }, global: { plugins: [pinia, i18n] } })
    expect(wrapper.findAll('[aria-label^="Grundwürfel"], [aria-label^="Stresswürfel"]')).toHaveLength(3)
    expect(wrapper.text()).toContain('1 Erfolg')
    expect(wrapper.text()).toContain('1 Push übrig')
    await wrapper.get('[data-test="choice-push"]').trigger('click')
    await flushPromises()
    expect(calls[0].body).toEqual({ command: { command: 'decide', pending: 4, choice: 'push' } })
  })

  it('names who asks the game master, why and what follows, and closes once decided (F-39)', async () => {
    vi.useFakeTimers()
    const { pinia, i18n } = setup()
    const targets = {
      id: 3,
      kind: 'targets',
      actor: null,
      options: [],
      who: 'friendly_pcs_short_range',
      candidates: ['1', '4'],
      by: '2',
      source: 'analysis',
      ops: [{ op: 'stress', delta: -1 }],
    }
    const team = useTeamStore()
    team.view = {
      id: 5,
      is_gm: true,
      characters: [
        { id: 1, name: 'Nat' },
        { id: 2, name: 'Ilse Varga' },
        { id: 4, name: 'David Daniels' },
      ],
      pending: [targets],
    }
    // Stress −1 at 0 changes nothing: the answer brings no log entry.
    const calls = mockApi({
      'POST /teams/5/commands': [],
      'GET /teams/5': { ...team.view, pending: [] },
    })
    const wrapper = mount(DecisionDialog, { props: { pending: targets }, global: { plugins: [pinia, i18n] } })
    expect(wrapper.get('[data-test="decision-cause"]').text()).toBe('Ilse Varga · analysis')
    expect(wrapper.get('[data-test="decision-effect"]').text()).toBe('Folge für jeden Gewählten: Stress −1')
    expect(wrapper.text()).toContain('David Daniels')
    await wrapper.get('input[type="checkbox"]').setValue(true)
    await wrapper.get('button').trigger('click')
    await flushPromises()
    expect(calls[0].body).toEqual({ command: { command: 'decide', pending: 3, choice: { targets: ['1'] } } })
    await vi.advanceTimersByTimeAsync(200)
    await flushPromises()
    expect(calls.some((c) => c.method === 'GET' && c.path === '/teams/5')).toBe(true)
    expect(team.view.pending).toEqual([])
  })

  it('logs who the game master chose, also no one', () => {
    const { pinia, i18n } = setup()
    const team = useTeamStore()
    team.view = { id: 5, characters: [{ id: 1, name: 'Nat' }, { id: 2, name: 'Ilse Varga' }] }
    const entry = (id, event) => ({ id, batch: id, by: 'Mother', at: 0, entry: { type: 'engine', event } })
    team.log = [
      entry(1, { event: 'choose_targets', pending: 3, actor: '2', source: 'analysis', who: 'friendly_pcs_short_range', candidates: ['1'] }),
      entry(2, { event: 'targets_chosen', pending: 3, actor: '2', source: 'analysis', targets: ['1'] }),
      entry(3, { event: 'targets_chosen', pending: 4, actor: '2', source: 'analysis', targets: [] }),
      entry(4, { event: 'choose_targets', pending: 1, actor: '2', who: 'friendly_pcs_short_range', candidates: ['1'] }),
    ]
    const text = mount(LogList, { global: { plugins: [pinia, i18n] } }).text()
    expect(text).toContain('analysis (Ilse Varga): Die Spielleitung wählt, wer betroffen ist: befreundete SC auf kurzer Distanz')
    expect(text).toContain('analysis (Ilse Varga) betrifft: Nat')
    expect(text).toContain('analysis (Ilse Varga) betrifft: niemanden')
    expect(text).toContain('Die Spielleitung wählt, wer betroffen ist: befreundete SC')
  })

  it('shows the bar only for one\'s own decisions, the game master sees all', () => {
    const { pinia, i18n } = setup()
    mockApi({})
    const auth = useAuthStore()
    const team = useTeamStore()
    auth.user = { id: 3, name: 'Burke', role: 'player' }
    team.view = { id: 5, is_gm: false, characters: [{ id: 9, name: 'Charlize', owner_id: 2 }], pending: [pushDecision] }
    const bar = () => mount(DecisionBar, { global: { plugins: [pinia, i18n] } })
    expect(bar().find('[data-test="decision-bar"]').exists()).toBe(false)

    auth.user = { id: 2, name: 'Gina', role: 'player' }
    expect(bar().get('[data-test="decision-4"]').text()).toContain('Charlize · Wurf')

    auth.user = { id: 1, name: 'Mother', role: 'gm' }
    team.view = { ...team.view, is_gm: true }
    expect(bar().get('[data-test="decision-4"]').text()).toContain('Wartet auf Charlize')
  })

  it('opens one\'s own decision by itself, not the players\' ones for the game master', async () => {
    const { pinia, i18n } = setup()
    vi.stubGlobal(
      'EventSource',
      class {
        close() {}
      },
    )
    const view = (isGm) => ({
      id: 5,
      name: 'Hope',
      mode: 'cinematic',
      gm: { id: 1, name: 'Mother' },
      is_gm: isGm,
      members: [],
      characters: [{ id: 9, name: 'Charlize', owner_id: 2, owner_name: 'Gina', actor: null, hints: [] }],
      pending: [pushDecision],
      story_points: {},
      rulings: {},
    })
    const auth = useAuthStore()
    auth.checked = true
    const mountAs = async (user, isGm) => {
      auth.user = user
      mockApi({ 'GET /teams/5': view(isGm), 'GET /characters': [] })
      const r = router()
      r.push('/teams/5')
      await r.isReady()
      const wrapper = mount(TeamView, { global: { plugins: [pinia, i18n, r] } })
      await flushPromises()
      return wrapper
    }
    const player = await mountAs({ id: 2, name: 'Gina', role: 'player' }, false)
    expect(player.find('[data-test="decision"]').exists()).toBe(true)
    player.unmount()

    const gm = await mountAs({ id: 1, name: 'Mother', role: 'gm' }, true)
    expect(gm.find('[data-test="decision"]').exists()).toBe(false)
    await gm.get('[data-test="decision-4"]').trigger('click')
    expect(gm.find('[data-test="decision"]').exists()).toBe(true)
  })

  it('lets the game master correct values', async () => {
    const { pinia, i18n } = setup()
    vi.stubGlobal(
      'EventSource',
      class {
        close() {}
      },
    )
    const actor = { attributes: { wits: 4 }, skills: {}, talents: [], health: 3, stress: 1, sources: [] }
    const calls = mockApi({
      'GET /teams/5': {
        id: 5,
        name: 'Hope',
        mode: 'cinematic',
        gm: { id: 1, name: 'Mother' },
        is_gm: true,
        members: [],
        characters: [{ id: 9, name: 'Charlize', owner_id: 2, owner_name: 'Gina', actor, hints: [], broken: false, health_max: 3 }],
        pending: [],
        story_points: { 2: 1 },
        rulings: {},
      },
      'GET /characters': [],
      'PUT /teams/5/characters/9': [],
      'POST /teams/5/commands': [],
      'POST /teams/5/preview': { base_dice: 4, stress_dice: 1, modifications: [], talents: [], max_pushes: 1 },
    })
    const auth = useAuthStore()
    auth.user = { id: 1, name: 'Mother', role: 'gm' }
    auth.checked = true
    const r = router()
    r.push('/teams/5')
    await r.isReady()
    const wrapper = mount(TeamView, { global: { plugins: [pinia, i18n, r] } })
    await flushPromises()
    // The sheet opens as a dialog; the game master corrects from there.
    await wrapper.get('[data-test="sheet-9"]').trigger('click')
    expect(wrapper.find('dialog').exists()).toBe(true)
    await wrapper.get('[data-test="correct"]').trigger('click')
    await wrapper.get('[data-test="attr-wits"]').setValue(5)
    await wrapper.get('[data-test="save-correction"]').trigger('click')
    await flushPromises()
    const put = calls.find((c) => c.method === 'PUT')
    expect(put.body.actor.attributes.wits).toBe(5)

    // Closing, then the actions for the same character.
    await wrapper.get('[data-test="modal-close"]').trigger('click')
    expect(wrapper.find('dialog').exists()).toBe(false)
    await wrapper.get('[data-test="actions-9"]').trigger('click')
    await wrapper.get('[data-test="stress-up"]').trigger('click')
    await flushPromises()
    const command = calls.find((c) => c.path === '/teams/5/commands')
    expect(command.body.command).toEqual({ command: 'change_stress', actors: ['9'], delta: 1 })

    // Story Points in Cinematic play, full Health from the server.
    expect(wrapper.get('[data-test="crew-9"]').text()).toContain('3/3')
  })

  it('reports an expired session', async () => {
    const { onUnauthorized, get } = await import('@/api')
    const seen = vi.fn()
    onUnauthorized(seen)
    vi.stubGlobal(
      'fetch',
      vi.fn(async () => ({ ok: false, status: 401, statusText: 'Unauthorized', text: async () => '' })),
    )
    await expect(get('/teams')).rejects.toThrow()
    expect(seen).toHaveBeenCalledOnce()
    onUnauthorized(() => {})
  })

  it('lets go of a team that is no longer visible', async () => {
    setup()
    mockApi({ 'GET /teams/5': null })
    const team = useTeamStore()
    await team.open(5)
    expect(team.gone).toBe(true)
    team.view = { id: 5 }
    team.gone = false
    await team.refresh()
    expect(team.view).toBe(null)
    expect(team.gone).toBe(true)
  })
})

describe('log', () => {
  it('shows each command as one block', () => {
    const { pinia, i18n } = setup()
    const team = useTeamStore()
    team.view = { id: 5, characters: [{ id: 9, name: 'Charlize' }] }
    const joined = (id, batch) => ({ id, batch, by: 'Gina', at: 0, entry: { type: 'character_joined', name: 'Charlize' } })
    const stress = (id, batch) => ({
      id,
      batch,
      by: 'Mother',
      at: 60,
      entry: { type: 'engine', event: { event: 'stress_changed', actor: '9', from: id - 2, to: id - 1 } },
    })
    // The first entry is older and has no batch.
    team.log = [{ ...joined(1), batch: undefined }, stress(2, 2), stress(3, 2), joined(4, 4)]
    const wrapper = mount(LogList, { global: { plugins: [pinia, i18n] } })
    const blocks = wrapper.findAll('[data-test="log-block"]')
    expect(blocks).toHaveLength(3)
    expect(blocks[1].findAll('[data-test="log-entry"]')).toHaveLength(2)
    expect(blocks[1].find('header').text()).toContain('Mother')
  })
})

describe('app', () => {
  it('shows the fan project note behind the info symbol', async () => {
    const { pinia, i18n } = setup()
    mockApi({})
    const r = router()
    r.push('/login')
    await r.isReady()
    const wrapper = mount(App, { global: { plugins: [pinia, i18n, r] } })
    expect(wrapper.text()).not.toContain('Inoffizielles Fan-Projekt')
    await wrapper.get('[data-test="about"]').trigger('click')
    expect(wrapper.get('dialog').text()).toContain('Inoffizielles Fan-Projekt')
  })
})

describe('team members', () => {
  const charlize = {
    id: 9,
    name: 'Charlize',
    owner_id: 2,
    owner_name: 'Gina',
    actor: { attributes: {}, skills: {}, talents: [], health: 3, stress: 0, sources: [] },
    health_max: 3,
    hints: [],
  }

  it('takes a character out after asking, with a note for actions', async () => {
    const { pinia, i18n } = setup()
    const calls = mockApi({ 'POST /teams/5/commands': [], 'DELETE /teams/5/characters/9': [], 'GET /teams/5': {} })
    useTeamStore().view = { id: 5, is_gm: true, mode: 'campaign', gm: { id: 1, name: 'Mother' }, members: [], characters: [charlize] }
    const wrapper = mount(CharacterActions, { props: { character: charlize }, global: { plugins: [pinia, i18n] } })
    await wrapper.get('[data-test="action-note"]').setValue('Säureblut')
    await wrapper.findAll('button').find((b) => b.text() === 'Schaden').trigger('click')
    await flushPromises()
    expect(calls[0].body).toEqual({ command: { command: 'damage', actor: '9', amount: 1 }, note: 'Säureblut' })
    expect(wrapper.get('[data-test="action-note"]').element.value).toBe('')

    await wrapper.get('[data-test="remove"]').trigger('click')
    expect(calls.some((c) => c.method === 'DELETE')).toBe(false)
    await wrapper.get('[data-test="confirm"]').trigger('click')
    await flushPromises()
    expect(calls.some((c) => c.method === 'DELETE' && c.path === '/teams/5/characters/9')).toBe(true)
  })

  it('lets the owner leave, not the game master', () => {
    const { pinia, i18n } = setup()
    mockApi({})
    const auth = useAuthStore()
    const team = useTeamStore()
    const sheet = () => mount(SheetDialog, { props: { character: charlize }, global: { plugins: [pinia, i18n] } })
    auth.user = { id: 2, name: 'Gina', role: 'player' }
    team.view = { id: 5, is_gm: false, characters: [charlize] }
    expect(sheet().find('[data-test="leave"]').exists()).toBe(true)
    auth.user = { id: 1, name: 'Mother', role: 'gm' }
    team.view = { id: 5, is_gm: true, characters: [charlize] }
    const gm = sheet()
    expect(gm.find('[data-test="leave"]').exists()).toBe(false)
    expect(gm.find('[data-test="correct"]').exists()).toBe(true)
  })

  it('shows the note of a command in the log', () => {
    const { pinia, i18n } = setup()
    const team = useTeamStore()
    team.view = { id: 5, characters: [] }
    team.log = [
      { id: 1, batch: 1, by: 'Mother', at: 0, note: 'Mother prüft die Schleuse', entry: { type: 'engine', event: { event: 'session_ended' } } },
    ]
    const wrapper = mount(LogList, { global: { plugins: [pinia, i18n] } })
    expect(wrapper.get('[data-test="log-note"]').text()).toBe('„Mother prüft die Schleuse“')
  })
})

describe('game master', () => {
  it('asks before ending the session', async () => {
    const { pinia, i18n } = setup()
    const calls = mockApi({ 'POST /teams/5/commands': [] })
    useTeamStore().view = { id: 5, mode: 'campaign', characters: [] }
    const wrapper = mount(GmBar, { global: { plugins: [pinia, i18n] } })
    expect(wrapper.find('[data-test="end-act"]').exists()).toBe(false)
    await wrapper.get('[data-test="end-session"]').trigger('click')
    expect(calls).toHaveLength(0)
    expect(wrapper.text()).toContain('Sitzung wirklich beenden?')
    await wrapper.get('[data-test="confirm"]').trigger('click')
    await flushPromises()
    expect(calls[0].body).toEqual({ command: { command: 'end_session' } })
  })

  it('shows the end of a session in the log', () => {
    const { pinia, i18n } = setup()
    const team = useTeamStore()
    team.view = { id: 5, characters: [] }
    team.log = [{ id: 1, batch: 1, by: 'Mother', at: 0, entry: { type: 'engine', event: { event: 'session_ended' } } }]
    const wrapper = mount(LogList, { global: { plugins: [pinia, i18n] } })
    expect(wrapper.text()).toContain('Sitzung beendet')
  })
})

describe('labels', () => {
  it('names choices and sources', () => {
    const { i18n } = setup()
    const t = i18n.global.t
    const rules = { name: (id) => `<${id}>` }
    expect(choiceLabel('push', t, rules)).toBe('Pushen')
    expect([0, 1, 3].map((n) => t('log.stunts', n))).toEqual(['kein Stunt', '1 Stunt', '3 Stunts'])
    const mods = [
      { source: 'gear', value: 2 },
      { source: 'help', value: 1 },
      { source: 'difficulty_hard', value: -2 },
      { source: 'talent/stoic', value: 1 },
    ]
    expect(modsText(mods, t, rules)).toBe('Ausrüstung +2, Helfer +1, <difficulty_hard> -2, <stoic> +1')
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

describe('creation wizard', () => {
  const draft = (problems) => ({
    id: 7,
    name: '',
    owner_id: 2,
    team_id: null,
    data: { status: 'draft', draft: {}, sheet: {}, actor: { attributes: {}, health: 0 } },
    problems,
  })
  const missing = (step, code, page) => ({ step, code, page })

  it('saves each change and opens the next step when the engine is content (plan 5.2)', async () => {
    vi.useFakeTimers()
    const { pinia, i18n } = setup()
    const calls = mockApi({
      'PUT /characters/7': (body) => ({
        ...draft(body.data.draft.career ? [{ ...missing('attributes', 'attribute_points', 28), value: 2, limit: 14 }] : []),
        data: { status: 'draft', draft: body.data.draft, sheet: {}, actor: { attributes: { wits: 2 }, health: 2 } },
      }),
    })
    const wrapper = mount(CreationWizard, {
      props: { character: draft([missing('career', 'career_missing', 27)]) },
      global: { plugins: [pinia, i18n] },
    })
    expect(wrapper.text()).toContain('Wähle eine Karriere. (S. 27)')
    expect(wrapper.get('[data-test="next"]').attributes('disabled')).toBeDefined()
    expect(wrapper.get('[data-test="step-skills"]').attributes('disabled')).toBeDefined()

    await wrapper.get('[data-test="career-officer"]').setValue(true)
    await vi.advanceTimersByTimeAsync(400)
    await flushPromises()
    expect(calls[0].body.data.draft.career).toBe('officer')
    await wrapper.get('[data-test="next"]').trigger('click')
    // Attributes start at the lowest value; the engine counts the points.
    expect(wrapper.get('[data-test="attr-wits"]').element.value).toBe('2')
    await vi.advanceTimersByTimeAsync(400)
    await flushPromises()
    expect(calls.at(-1).body.data.draft.attributes).toEqual({ wits: 2 })
    expect(wrapper.get('[data-test="step-problems"]').text()).toContain('von 14 Attributpunkten')
    expect(wrapper.get('[data-test="next"]').attributes('disabled')).toBeDefined()
  })

  it('labels what each career sets [p. 38–55]', () => {
    const { pinia, i18n } = setup()
    mockApi({})
    const wrapper = mount(CreationWizard, {
      props: { character: draft([missing('career', 'career_missing', 27)]) },
      global: { plugins: [pinia, i18n] },
    })
    const facts = wrapper.get('[data-test="career-facts-officer"]')
    expect(facts.text()).toContain('Schlüsselattributwits')
    expect(facts.text()).toContain('Schlüsselfertigkeitenobservation')
    expect(facts.text()).toContain('Talentepull_rank')
    expect(facts.text()).toContain('Startgeld2W6 × 100 $')
    expect(facts.get('[title="pull_rank.desc"]').exists()).toBe(true)
  })

  it('finishes a complete draft; the server rolls the cash', async () => {
    const { pinia, i18n } = setup()
    const finished = { ...draft([]), data: { status: 'finished', sheet: {}, actor: {}, start_cash: { dice: [3, 4], amount: 700 } } }
    const calls = mockApi({ 'POST /characters/7/finish': finished })
    const wrapper = mount(CreationWizard, { props: { character: draft([]) }, global: { plugins: [pinia, i18n] } })
    expect(wrapper.text()).toContain('Alle Schritte sind erfüllt.')
    await wrapper.get('[data-test="finish"]').trigger('click')
    await flushPromises()
    expect(calls[0]).toMatchObject({ method: 'POST', path: '/characters/7/finish' })
    expect(wrapper.emitted('finished')[0][0].data.start_cash.amount).toBe(700)
  })

  it('names problems and cash as the book does', () => {
    const { i18n } = setup()
    const rules = useRulesStore()
    const t = i18n.global.t
    expect(problemText({ step: 'skills', code: 'skill_range', page: 28, id: 'observation', value: 2, limit: 1 }, t, rules)).toBe(
      'observation höchstens 1. (S. 28)',
    )
    expect(cashFormula({ dice: 2, times: 100 }, t)).toBe('2W6 × 100 $')
    const en = createAppI18n('en').global.t
    expect(cashFormula({ dice: 1, times: 1 }, en)).toBe('$D6')
    expect(en('sheet.cashRolled', { amount: 400, dice: '4' })).toBe('$400 (rolled: 4)')
  })
})

describe('gear', () => {
  it('chooses two starting items from two lines (F-60)', async () => {
    vi.useFakeTimers()
    const { pinia, i18n } = setup()
    const calls = mockApi({
      'PUT /characters/7': (body) => ({
        id: 7,
        name: '',
        data: { status: 'draft', draft: body.data.draft, sheet: {}, actor: {} },
        problems: [],
      }),
    })
    const wrapper = mount(CreationWizard, {
      props: {
        character: {
          id: 7,
          name: '',
          data: { status: 'draft', draft: { career: 'officer' }, sheet: {}, actor: {} },
          problems: [{ step: 'gear', code: 'gear_count', page: 39, value: 0, limit: 2 }],
        },
      },
      global: { plugins: [pinia, i18n] },
    })
    const gear = wrapper.get('[data-test="start-gear"]')
    expect(gear.text()).toContain('W6 × signal_flare')
    expect(gear.text()).toContain('2 × deck_of_cards')
    expect(wrapper.text()).toContain('0 von 2 Startgegenständen gewählt. (S. 39)')
    await wrapper.get('[data-test="gear-0-0"]').trigger('click')
    await wrapper.get('[data-test="gear-1-0"]').trigger('click')
    expect(wrapper.get('[data-test="gear-2-0"]').attributes('disabled')).toBeDefined()
    await wrapper.get('[data-test="gear-1-1"]').trigger('click')
    expect(wrapper.get('[data-test="gear-1-1"]').attributes('aria-pressed')).toBe('true')
    await vi.advanceTimersByTimeAsync(400)
    await flushPromises()
    expect(calls.at(-1).body.data.draft.gear).toEqual([
      { line: 0, option: 0 },
      { line: 1, option: 1 },
    ])
    await wrapper.get('[data-test="gear-0-0"]').trigger('click')
    expect(wrapper.get('[data-test="gear-2-0"]').attributes('disabled')).toBeUndefined()
  })

  const actor = {
    attributes: { strength: 1 },
    items: [
      { uid: 0, item: 'm4a3_service_pistol', count: 1, place: 'carried', reloads: 2 },
      { uid: 1, item: 'irc_mk50_compression_suit', count: 1, place: 'worn', supply: 4 },
      { uid: 2, item: 'signal_flare', count: 3, place: 'stowed' },
      { uid: 3, name: 'Tagebuch', weight: '1/4', count: 1 },
    ],
    next_item: 4,
  }

  it('shows items and the load from the server, and moves them', async () => {
    const { pinia, i18n } = setup()
    const send = vi.fn(async () => true)
    const wrapper = mount(GearPanel, {
      props: {
        actor: {
          ...actor,
          items: [...actor.items, { uid: 4, item: 'irc_mk50_compression_suit', count: 2, place: 'carried' }],
        },
        encumbrance: { carried: 7, normal: 4, max: 8 },
        canMove: true,
        others: [{ id: 9, name: 'Charlize' }],
        send,
      },
      global: { plugins: [pinia, i18n] },
    })
    expect(wrapper.get('[data-test="encumbrance"]').text()).toBe('Traglast 1¾ von 1, überladen bis 2')
    expect(wrapper.text()).toContain('Überladen')
    const suit = wrapper.get('[data-test="item-1"]').text()
    expect(suit).toContain('angezogen')
    expect(suit).toContain('Luft 4/5')
    expect(wrapper.get('[data-test="item-0"]').text()).toContain('Bonus +2, Schaden 1, mittel · 2 Magazine')
    expect(wrapper.get('[data-test="item-3"]').text()).toContain('Tagebuch')
    // Only a suit is put on.
    const pistolTargets = wrapper.get('[data-test="move-0"]').findAll('option').map((o) => o.element.value)
    expect(pistolTargets).toEqual(['', 'stowed', 'character:9'])
    expect(wrapper.get('[data-test="move-2"]').findAll('option').map((o) => o.element.value)).not.toContain('worn')

    await wrapper.get('[data-test="move-0"]').setValue('stowed')
    await wrapper.get('[data-test="item-2"] input[type="number"]').setValue(2)
    await wrapper.get('[data-test="move-2"]').setValue('character:9')
    await wrapper.get('[data-test="drop-3"]').trigger('click')
    await wrapper.get('[data-test="item-3"] [data-test="confirm"]').trigger('click')
    // One suit of a stack is put on [p. 98].
    await wrapper.get('[data-test="move-4"]').setValue('worn')
    expect(send.mock.calls.map((c) => c[0])).toEqual([
      { command: 'move_item', uid: 0, to: 'stowed' },
      { command: 'move_item', uid: 2, to: { character: '9' }, count: 2 },
      { command: 'move_item', uid: 3, to: 'dropped' },
      { command: 'move_item', uid: 4, to: 'worn', count: 1 },
    ])
  })

  it('lets the game master add from the catalog or a free item', async () => {
    const { pinia, i18n } = setup()
    const answers = [false, true, true]
    const send = vi.fn(async () => answers.shift())
    const wrapper = mount(GearPanel, {
      props: { actor: { items: [] }, canAdd: true, send },
      global: { plugins: [pinia, i18n] },
    })
    expect(wrapper.find('[data-test="move-0"]').exists()).toBe(false)
    await wrapper.get('[data-test="gear-search"]').setValue('auto')
    expect(wrapper.find('[data-test="pick-autodoc"]').exists()).toBe(false)
    await wrapper.get('[data-test="gear-search"]').setValue('m4a3')
    await wrapper.get('[data-test="pick-m4a3_service_pistol"]').trigger('click')
    // Refused: the choice stays for another try.
    await wrapper.get('[data-test="add-go"]').trigger('click')
    await flushPromises()
    expect(wrapper.get('[data-test="pick-m4a3_service_pistol"]').attributes('aria-pressed')).toBe('true')
    await wrapper.get('[data-test="add-go"]').trigger('click')
    await flushPromises()
    await wrapper.get('[data-test="add-free"]').setValue(true)
    expect(wrapper.get('[data-test="add-go"]').attributes('disabled')).toBeDefined()
    await wrapper.get('[data-test="free-name"]').setValue(' Kiste ')
    await wrapper.get('[data-test="add-go"]').trigger('click')
    await flushPromises()
    expect(send.mock.calls.map((c) => c[0])).toEqual([
      { command: 'add_item', item: { item: 'm4a3_service_pistol' } },
      { command: 'add_item', item: { item: 'm4a3_service_pistol' } },
      { command: 'add_item', item: { name: 'Kiste', weight: '1' } },
    ])
    expect(wrapper.get('[data-test="free-name"]').element.value).toBe('')
  })

  it('sends the item commands of the sheet to the team for the character', async () => {
    const { pinia, i18n } = setup()
    const calls = mockApi({ 'POST /teams/5/commands': [], 'GET /teams/5': { id: 5, characters: [] } })
    useAuthStore().user = { id: 2, role: 'player' }
    const character = { id: 3, name: 'Gorman', owner_id: 2, actor, sheet: {}, hints: [], encumbrance: { carried: 7, normal: 4, max: 8 } }
    useTeamStore().view = { id: 5, mode: 'campaign', is_gm: false, characters: [character] }
    const wrapper = mount(SheetDialog, { props: { character }, global: { plugins: [pinia, i18n] } })
    expect(wrapper.find('[data-test="add-item"]').exists()).toBe(false)
    await wrapper.get('[data-test="move-0"]').setValue('stowed')
    await flushPromises()
    expect(calls[0].body).toEqual({ command: { command: 'move_item', uid: 0, to: 'stowed', actor: '3' } })
  })

  it('names items in the log and weights as the book writes them', () => {
    const { pinia, i18n } = setup()
    const team = useTeamStore()
    team.view = { id: 5, characters: [{ id: 3, name: 'Gorman' }, { id: 9, name: 'Charlize' }] }
    const event = (ev) => ({ id: ev.uid + 1, batch: ev.uid + 1, by: 'Mother', at: 0, entry: { type: 'engine', event: ev } })
    team.log = [
      event({ event: 'item_added', actor: '3', uid: 0, item: 'knife', count: 1, place: 'carried' }),
      event({ event: 'item_moved', actor: '3', uid: 1, name: 'Tagebuch', to: { character: '9' }, count: 1 }),
      event({ event: 'item_used_up', actor: '9', uid: 2, item: 'personal_medkit' }),
    ]
    const text = mount(LogList, { global: { plugins: [pinia, i18n] } }).text()
    expect(text).toContain('Gorman bekommt knife × 1')
    expect(text).toContain('Gorman: Tagebuch × 1 – an Charlize gegeben')
    expect(text).toContain('Charlize: personal_medkit verbraucht')
    expect([0, 1, 2, 4, 14].map(quarters)).toEqual(['0', '¼', '½', '1', '3½'])
    expect(weightText('1/4')).toBe('¼')
  })
})

describe('sheet and talents', () => {
  const gorman = {
    id: 3,
    name: 'Gorman',
    owner_id: 2,
    owner_name: 'Gina',
    actor: { attributes: { wits: 4 }, skills: { observation: 2 }, talents: ['pull_rank', 'watchful'], health: 4, stress: 1 },
    sheet: { career: 'officer', agenda: 'Bring them home.', buddy: null, rival: 9 },
    health_max: 4,
    hints: [],
  }
  const charlize = { id: 9, name: 'Charlize', owner_id: 5, owner_name: 'Ada', actor: null, hints: [] }

  it('shows the sheet with buddy and rival by name', () => {
    const { pinia, i18n } = setup()
    const wrapper = mount(CharacterSheet, {
      props: { sheet: gorman.sheet, actor: gorman.actor, healthMax: 4, names: { 9: 'Charlize' }, startCash: { dice: [5], amount: 500 } },
      global: { plugins: [pinia, i18n] },
    })
    const text = wrapper.text()
    expect(text).toContain('Bring them home.')
    expect(text).toContain('RivalCharlize')
    expect(text).toContain('4/4')
    expect(text).toContain('500 $ (gewürfelt: 5)')
    expect(wrapper.get('[data-test="sheet-wits"]').text()).toContain('observation2')
  })

  it('lets the owner choose buddy and rival in the team (F-52)', async () => {
    const { pinia, i18n } = setup()
    const calls = mockApi({ 'PUT /characters/3/relations': {}, 'GET /teams/5': { id: 5, characters: [] } })
    useAuthStore().user = { id: 2, name: 'Gina', role: 'player' }
    useTeamStore().view = { id: 5, is_gm: false, characters: [gorman, charlize] }
    const wrapper = mount(SheetDialog, { props: { character: gorman }, global: { plugins: [pinia, i18n] } })
    await wrapper.get('[data-test="buddy"]').setValue(9)
    await wrapper.get('[data-test="rival"]').findAll('option')[0].setSelected()
    await wrapper.get('[data-test="save-relations"]').trigger('click')
    await flushPromises()
    expect(calls[0]).toEqual({ method: 'PUT', path: '/characters/3/relations', body: { buddy: 9, rival: null } })
  })

  it('offers situational talents with their situation and talent actions with a target (A-10)', async () => {
    vi.useFakeTimers()
    const { pinia, i18n } = setup()
    const calls = mockApi({
      'POST /teams/5/preview': { base_dice: 6, stress_dice: 1, modifications: [], talents: ['watchful'], max_pushes: 1 },
      'POST /teams/5/commands': [],
    })
    useTeamStore().view = { id: 5, is_gm: false, characters: [gorman, charlize] }
    const wrapper = mount(RollPanel, { props: { character: gorman }, global: { plugins: [pinia, i18n] } })
    await vi.advanceTimersByTimeAsync(200)
    await flushPromises()
    expect(wrapper.get('[data-test="talent-watchful"]').element.closest('label').textContent).toContain('watchful.when')
    expect(wrapper.get('[data-test="action-pull_rank_order"]').attributes('disabled')).toBeDefined()
    await wrapper.get('[data-test="target"]').setValue(9)
    await wrapper.get('[data-test="action-pull_rank_order"]').trigger('click')
    await flushPromises()
    const command = calls.find((c) => c.path === '/teams/5/commands')
    expect(command.body.command).toEqual({ command: 'perform_action', actor: '3', action: 'pull_rank_order', target: '9' })
  })

  it('names the character and the action in the log', () => {
    const { pinia, i18n } = setup()
    const team = useTeamStore()
    team.view = { id: 5, characters: [gorman] }
    team.log = [
      { id: 1, batch: 1, by: 'Mother', at: 0, entry: { type: 'engine', event: { event: 'action_performed', actor: '3', action: 'pull_rank_order' } } },
      { id: 2, batch: 2, by: 'Mother', at: 0, entry: { type: 'engine', event: { event: 'signature_item_used', actor: '3' } } },
    ]
    const text = mount(LogList, { global: { plugins: [pinia, i18n] } }).text()
    expect(text).toContain('Gorman: pull_rank_order')
    expect(text).toContain('Gorman nutzt das Signature Item')
  })
})

describe('choosing and Acts (A-12, A-13)', () => {
  const gm = { id: 1, name: 'Mother' }
  const nat = {
    id: 9,
    name: 'Nat',
    owner_id: 1,
    owner_name: 'Mother',
    offered: true,
    actor: { attributes: {}, skills: {}, talents: [], health: 3, stress: 0, sources: [] },
    sheet: {},
    agendas: ['Survive.'],
    health_max: 3,
    hints: [],
  }
  const ripley = { ...nat, id: 7, name: 'Ripley', owner_id: 2, owner_name: 'Ann', offered: false, agenda: 'Save the girl.' }
  const view = (isGm) => ({
    id: 5,
    is_gm: isGm,
    mode: 'cinematic',
    act: 2,
    gm,
    members: [{ id: 2, name: 'Ann' }],
    characters: [nat, ripley],
    story_points: { 2: 1 },
    pending: [],
  })

  it('joins without a character', async () => {
    const { pinia, i18n } = setup()
    const calls = mockApi({
      'GET /join/abc': { team: 'Hope', gm: 'Mother', mode: 'cinematic' },
      'GET /characters': [],
      'POST /join/abc': { team: 5 },
    })
    const r = router()
    useAuthStore().user = { id: 2, name: 'Ann', role: 'player' }
    useAuthStore().checked = true
    await r.push('/join/abc')
    const wrapper = mount(JoinView, { global: { plugins: [pinia, i18n, r] } })
    await flushPromises()
    expect(wrapper.get('[data-test="join-character"]').element.value).toBe('none')
    await wrapper.findAll('button').find((b) => b.text() === 'Beitreten').trigger('click')
    await flushPromises()
    expect(calls.find((c) => c.method === 'POST').body).toEqual({ character: null })
  })

  it('lets a player take an offered character', async () => {
    const { pinia, i18n } = setup()
    const calls = mockApi({ 'POST /teams/5/characters/9/claim': [], 'GET /teams/5': view(false) })
    useAuthStore().user = { id: 2, name: 'Ann', role: 'player' }
    useTeamStore().view = view(false)
    const wrapper = mount(CrewList, { global: { plugins: [pinia, i18n] } })
    expect(wrapper.get('[data-test="crew-9"]').text()).toContain('Zur Wahl')
    await wrapper.get('[data-test="claim-9"]').trigger('click')
    await flushPromises()
    expect(calls[0]).toMatchObject({ method: 'POST', path: '/teams/5/characters/9/claim' })
  })

  it('ends an Act with Story Points for those who furthered their Agenda', async () => {
    const { pinia, i18n } = setup()
    const calls = mockApi({ 'POST /teams/5/commands': [] })
    useTeamStore().view = view(true)
    const wrapper = mount(GmBar, { global: { plugins: [pinia, i18n] } })
    await wrapper.get('[data-test="end-act"]').trigger('click')
    expect(wrapper.get('[data-test="end-act-form"]').text()).toContain('Akt 2 beenden')
    expect(wrapper.find('[data-test="agenda-9"]').exists()).toBe(false)
    await wrapper.get('[data-test="agenda-7"]').setValue(true)
    await wrapper.get('[data-test="end-act-confirm"]').trigger('click')
    await flushPromises()
    expect(calls[0].body).toEqual({ command: { command: 'end_act', agenda: ['7'] } })
  })

  it('lets the game master offer, hand over, set Agendas and Story Points', async () => {
    const { pinia, i18n } = setup()
    const calls = mockApi({
      'PUT /teams/5/characters/9/offer': [],
      'PUT /teams/5/characters/9/owner': [],
      'PUT /teams/5/characters/9/agendas': [],
      'POST /teams/5/commands': [],
    })
    useTeamStore().view = view(true)
    const wrapper = mount(CharacterActions, { props: { character: nat }, global: { plugins: [pinia, i18n] } })
    await wrapper.get('[data-test="offer"]').trigger('click')
    await wrapper.get('[data-test="new-owner"]').setValue(2)
    await wrapper.get('[data-test="hand-over"]').trigger('click')
    await wrapper.get('[data-test="agenda-act-2"]').setValue('Escape.')
    await wrapper.get('[data-test="save-agendas"]').trigger('click')
    await flushPromises()
    expect(calls.map((c) => [c.path, c.body])).toEqual([
      ['/teams/5/characters/9/offer', { offered: false }],
      ['/teams/5/characters/9/owner', { owner: 2 }],
      ['/teams/5/characters/9/agendas', { agendas: ['Survive.', 'Escape.', ''] }],
    ])
    expect(wrapper.find('[data-test="story-points"]').exists()).toBe(false, 'the GM has no Story Points')

    const other = mount(CharacterActions, { props: { character: ripley }, global: { plugins: [pinia, i18n] } })
    expect(other.find('[data-test="offer"]').exists()).toBe(false)
    expect(other.get('[data-test="story-points"]').element.value).toBe('1')
    await other.get('[data-test="story-points"]').setValue(3)
    await other.get('[data-test="set-story-points"]').trigger('click')
    await flushPromises()
    expect(calls.at(-1).body.command).toEqual({ command: 'set_story_points', player: '2', points: 3 })
  })

  it('lets a player without a character leave, and only the GM sets buddy and rival in Cinematic', async () => {
    const { pinia, i18n } = setup()
    vi.stubGlobal(
      'EventSource',
      class {
        close() {}
      },
    )
    const calls = mockApi({ 'GET /teams/5': { ...view(false), members: [{ id: 2, name: 'Ann' }, { id: 3, name: 'Bob' }] }, 'DELETE /teams/5/members/3': [] })
    useAuthStore().user = { id: 3, name: 'Bob', role: 'player' }
    useAuthStore().checked = true
    const r = router()
    r.push('/teams/5')
    await r.isReady()
    const wrapper = mount(TeamView, { global: { plugins: [pinia, i18n, r] } })
    await flushPromises()
    expect(wrapper.get('[data-test="waiting"]').text()).toContain('Bob')
    expect(wrapper.find('[data-test="remove-member-2"]').exists()).toBe(false)
    await wrapper.get('[data-test="remove-member-3"]').trigger('click')
    await wrapper.get('[data-test="confirm"]').trigger('click')
    await flushPromises()
    expect(calls.some((c) => c.method === 'DELETE' && c.path === '/teams/5/members/3')).toBe(true)

    useAuthStore().user = { id: 2, name: 'Ann', role: 'player' }
    useTeamStore().view = view(false)
    const sheet = mount(SheetDialog, { props: { character: ripley }, global: { plugins: [pinia, i18n] } })
    expect(sheet.find('[data-test="buddy"]').exists()).toBe(false)
    useTeamStore().view = view(true)
    const gmSheet = mount(SheetDialog, { props: { character: ripley }, global: { plugins: [pinia, i18n] } })
    expect(gmSheet.find('[data-test="buddy"]').exists()).toBe(true)
  })

  it('shows the Agenda of the current Act on the sheet', () => {
    const { pinia, i18n } = setup()
    mockApi({})
    useAuthStore().user = { id: 2, name: 'Ann', role: 'player' }
    useTeamStore().view = view(false)
    const wrapper = mount(SheetDialog, { props: { character: ripley }, global: { plugins: [pinia, i18n] } })
    expect(wrapper.text()).toContain('Save the girl.')
  })

  it('names who chose, who got Story Points and which Act ended', () => {
    const { pinia, i18n } = setup()
    const team = useTeamStore()
    team.view = view(true)
    team.log = [
      { id: 1, batch: 1, by: 'Ann', at: 0, entry: { type: 'character_claimed', character: 9, name: 'Nat' } },
      { id: 2, batch: 2, by: 'Mother', at: 0, entry: { type: 'engine', event: { event: 'act_ended', act: 2 } } },
      { id: 3, batch: 2, by: 'Mother', at: 0, entry: { type: 'engine', event: { event: 'story_point_awarded', player: '2' } } },
      { id: 4, batch: 4, by: 'Mother', at: 0, entry: { type: 'engine', event: { event: 'story_points_changed', player: '2', from: 2, to: 3 } } },
    ]
    const text = mount(LogList, { global: { plugins: [pinia, i18n] } }).text()
    expect(text).toContain('Ann spielt jetzt Nat')
    expect(text).toContain('Akt 2 beendet')
    expect(text).toContain('Ann bekommt einen Story Point')
    expect(text).toContain('Ann: Story Points 2 → 3')
  })
})

describe('covert android (A-14)', () => {
  it('chooses covert in the wizard, reveals by the game master and shows it on the sheet', async () => {
    vi.useFakeTimers()
    const { pinia, i18n } = setup()
    const calls = mockApi({
      'PUT /characters/7': (body) => ({ id: 7, name: '', data: { status: 'draft', draft: body.data.draft, sheet: {}, actor: {} }, problems: [] }),
      'POST /teams/5/commands': [],
    })
    const wizard = mount(CreationWizard, {
      props: { character: { id: 7, name: '', data: { status: 'draft', draft: {}, sheet: {}, actor: {} }, problems: [{ step: 'career', code: 'career_missing', page: 27 }] } },
      global: { plugins: [pinia, i18n] },
    })
    expect(wizard.find('[data-test="android-covert"]').exists()).toBe(false)
    await wizard.get('[data-test="android"]').setValue(true)
    await wizard.get('[data-test="android-covert"]').setValue(true)
    await vi.advanceTimersByTimeAsync(400)
    await flushPromises()
    expect(calls.at(-1).body.data.draft).toMatchObject({ android: true, android_covert: true })

    const holroyd = { id: 4, name: 'Holroyd', owner_id: 1, owner_name: 'Mother', actor: { android: true, covert: true, sources: [] } }
    useTeamStore().view = { id: 5, is_gm: true, mode: 'cinematic', act: 1, gm: { id: 1, name: 'Mother' }, members: [], characters: [holroyd], story_points: {} }
    const actions = mount(CharacterActions, { props: { character: holroyd }, global: { plugins: [pinia, i18n] } })
    await actions.get('[data-test="reveal-android"]').trigger('click')
    await actions.get('[data-test="confirm"]').trigger('click')
    await flushPromises()
    expect(calls.at(-1).body.command).toEqual({ command: 'reveal_android', actor: '4' })

    const sheet = mount(CharacterSheet, { props: { sheet: {}, actor: holroyd.actor }, global: { plugins: [pinia, i18n] } })
    expect(sheet.text()).toContain('Androidverdeckt')
  })
})

