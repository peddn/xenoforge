import { computed, ref } from 'vue'
import { defineStore } from 'pinia'

import { get, post, put } from '@/api'

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
    source.onerror = () => (connected.value = false)
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

  async function command(command) {
    return post(`/teams/${view.value.id}/commands`, { command })
  }

  async function preview(request) {
    return post(`/teams/${view.value.id}/preview`, request)
  }

  async function undo() {
    return post(`/teams/${view.value.id}/undo`)
  }

  async function correct(characterId, actor) {
    return put(`/teams/${view.value.id}/characters/${characterId}`, { actor })
  }

  async function newInvite() {
    await post(`/teams/${view.value.id}/invite`)
    await refresh()
  }

  return { view, log, connected, gone, characterNames, open, close, refresh, command, preview, undo, correct, newInvite }
})
