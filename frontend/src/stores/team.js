import { computed, ref } from 'vue'
import { defineStore } from 'pinia'

import { del, get, post, put } from '@/api'
import { useAuthStore } from '@/stores/auth'

// One team at the table: its view, the shared log (Server-Sent Events) and
// commands to the engine.
export const useTeamStore = defineStore('team', () => {
  const view = ref(null)
  const log = ref([])
  const connected = ref(false)
  // Set when the team is no longer visible, e.g. after one's character was
  // taken out of it.
  const gone = ref(false)
  let source = null
  let refreshTimer = null

  const characterNames = computed(() =>
    Object.fromEntries((view.value?.characters ?? []).map((c) => [String(c.id), c.name])),
  )

  const ownerOf = (actor) => view.value?.characters.find((c) => String(c.id) === actor)?.owner_id

  // Whether the logged-in person makes this decision: the owner of the
  // character, or the game master when no character decides (targets).
  function decidesMyself(pending) {
    const me = useAuthStore().user?.id
    return pending.actor ? ownerOf(pending.actor) === me : !!view.value?.is_gm
  }

  // Open decisions shown: players see their own, the game master all.
  const decisions = computed(() =>
    (view.value?.pending ?? []).filter((p) => view.value.is_gm || decidesMyself(p)),
  )

  async function refresh() {
    if (!view.value) return
    try {
      view.value = await get(`/teams/${view.value.id}`)
    } catch (e) {
      if (e.status !== 404) throw e
      close()
      view.value = null
      gone.value = true
    }
  }

  // Many events come in bursts; refresh the view once per burst.
  function scheduleRefresh() {
    clearTimeout(refreshTimer)
    refreshTimer = setTimeout(() => refresh().catch(() => {}), 150)
  }

  async function open(id) {
    close()
    log.value = []
    gone.value = false
    try {
      view.value = await get(`/teams/${id}`)
    } catch (e) {
      if (e.status !== 404) throw e
      gone.value = true
      return
    }
    source = new EventSource(`/api/teams/${id}/events`)
    source.onopen = () => (connected.value = true)
    source.onerror = () => {
      connected.value = false
      // The browser gives up for good on 401 or 404; find out which.
      if (source?.readyState === 2) refresh().catch(() => {})
    }
    source.onmessage = (message) => {
      const entry = JSON.parse(message.data)
      if (!log.value.some((e) => e.id === entry.id)) log.value.push(entry)
      scheduleRefresh()
    }
  }

  function close() {
    clearTimeout(refreshTimer)
    source?.close()
    source = null
    connected.value = false
  }

  // `note`: why, in the person's words; shown with the command in the log.
  async function command(command, note) {
    return post(`/teams/${view.value.id}/commands`, { command, note: note || undefined })
  }

  async function preview(request) {
    return post(`/teams/${view.value.id}/preview`, request)
  }

  async function undo() {
    return post(`/teams/${view.value.id}/undo`)
  }

  async function correct(characterId, actor, note) {
    return put(`/teams/${view.value.id}/characters/${characterId}`, { actor, note: note || undefined })
  }

  // The game master takes a character out, or a player leaves with theirs.
  async function removeCharacter(characterId) {
    await del(`/teams/${view.value.id}/characters/${characterId}`)
    await refresh()
  }

  // Choosing and handing over characters (A-12), Agendas per Act (A-13).
  const characterPath = (id, what) => `/teams/${view.value.id}/characters/${id}/${what}`
  const offer = (id, offered) => put(characterPath(id, 'offer'), { offered })
  const claim = (id) => post(characterPath(id, 'claim'))
  const handOver = (id, owner) => put(characterPath(id, 'owner'), { owner })
  const setAgendas = (id, agendas) => put(characterPath(id, 'agendas'), { agendas })

  async function newInvite() {
    await post(`/teams/${view.value.id}/invite`)
    await refresh()
  }

  // Names of the people in the team by id: members and the game master.
  const personNames = computed(() =>
    Object.fromEntries(
      [view.value?.gm, ...(view.value?.members ?? [])].filter(Boolean).map((p) => [String(p.id), p.name]),
    ),
  )

  return {
    view,
    log,
    connected,
    gone,
    characterNames,
    personNames,
    decisions,
    decidesMyself,
    open,
    close,
    refresh,
    command,
    preview,
    undo,
    correct,
    removeCharacter,
    offer,
    claim,
    handOver,
    setAgendas,
    newInvite,
  }
})
