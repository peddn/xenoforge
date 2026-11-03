import { ref } from 'vue'
import { defineStore } from 'pinia'

// State of the connection to the server (GET /api/health).
export const useStatusStore = defineStore('status', () => {
  const state = ref('checking') // 'checking' | 'online' | 'offline'
  const version = ref(null)

  async function check() {
    state.value = 'checking'
    try {
      const response = await fetch('/api/health')
      if (!response.ok) throw new Error(`HTTP ${response.status}`)
      const health = await response.json()
      version.value = health.version
      state.value = health.status === 'ok' ? 'online' : 'offline'
    } catch {
      version.value = null
      state.value = 'offline'
    }
  }

  return { state, version, check }
})
