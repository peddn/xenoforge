import { ref } from 'vue'
import { defineStore } from 'pinia'

import { get, post } from '@/api'

// The logged-in person (A-07: role player or gm).
export const useAuthStore = defineStore('auth', () => {
  const user = ref(null)
  const checked = ref(false)

  async function load() {
    try {
      user.value = await get('/auth/me')
    } catch {
      user.value = null
    }
    checked.value = true
  }

  async function login(name, password) {
    user.value = await post('/auth/login', { name, password })
  }

  async function register(name, password, role) {
    user.value = await post('/auth/register', { name, password, role })
  }

  async function logout() {
    await post('/auth/logout')
    user.value = null
  }

  return { user, checked, load, login, register, logout }
})
