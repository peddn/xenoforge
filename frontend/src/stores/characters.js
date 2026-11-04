import { ref } from 'vue'
import { defineStore } from 'pinia'

import { del, get, post, put } from '@/api'

// The person's own characters (plan 5.0).
export const useCharactersStore = defineStore('characters', () => {
  const list = ref([])

  async function load() {
    list.value = await get('/characters')
  }

  async function create(name) {
    const character = await post('/characters', { name })
    list.value.push(character)
    return character
  }

  async function save(character) {
    const saved = await put(`/characters/${character.id}`, { name: character.name, data: character.data })
    const index = list.value.findIndex((c) => c.id === saved.id)
    if (index >= 0) list.value[index] = saved
    return saved
  }

  async function remove(id) {
    await del(`/characters/${id}`)
    list.value = list.value.filter((c) => c.id !== id)
  }

  return { list, load, create, save, remove }
})
