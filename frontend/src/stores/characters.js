import { ref } from 'vue'
import { defineStore } from 'pinia'

import { del, get, post, put } from '@/api'

// The person's own characters (plan 5.0).
export const useCharactersStore = defineStore('characters', () => {
  const list = ref([])

  async function load() {
    list.value = await get('/characters')
  }

  function remember(character) {
    const index = list.value.findIndex((c) => c.id === character.id)
    if (index >= 0) list.value[index] = character
    else list.value.push(character)
    return character
  }

  // A draft for the wizard, or with a name a character set up freely by a
  // game master (A-09).
  async function create(name) {
    return remember(await post('/characters', name ? { free: true, name } : {}))
  }

  async function save(character) {
    return remember(await put(`/characters/${character.id}`, { name: character.name, data: character.data }))
  }

  // The answer carries the engine's problems with the draft.
  async function saveDraft(id, draft) {
    return remember(await put(`/characters/${id}`, { data: { draft } }))
  }

  // The server rolls the starting cash.
  async function finish(id) {
    return remember(await post(`/characters/${id}/finish`))
  }

  async function remove(id) {
    await del(`/characters/${id}`)
    list.value = list.value.filter((c) => c.id !== id)
  }

  return { list, load, create, save, saveDraft, finish, remove }
})
