import { ref } from 'vue'
import { defineStore } from 'pinia'

import { get } from '@/api'

const FILES = ['attributes', 'skills', 'talents', 'traumas', 'conditions', 'critical-injuries', 'panic', 'constants']

// Rule data and its texts, served from data/ (the interface computes no
// rules; it only shows names and lists).
export const useRulesStore = defineStore('rules', () => {
  const data = ref({})
  const texts = ref({})
  const loaded = ref(false)
  const textLocale = ref(null)

  async function load(locale) {
    if (!loaded.value) {
      const entries = await Promise.all(FILES.map((f) => get(`/data/rules/${f}.json`)))
      data.value = Object.fromEntries(FILES.map((f, i) => [f, entries[i].entries]))
      loaded.value = true
    }
    if (textLocale.value !== locale) {
      texts.value = await get(`/data/i18n/${locale}.json`)
      textLocale.value = locale
    }
  }

  /** Name of a rule entry, or the id if unknown. */
  function name(id) {
    return texts.value[`${id}.name`] ?? id
  }

  /** Any text key of the rule data. */
  function text(key) {
    return texts.value[key] ?? key
  }

  function list(file) {
    return data.value[file] ?? []
  }

  /** Difficulty levels [p. 63] from constants.json. */
  function difficulties() {
    return list('constants').filter((c) => c.id.startsWith('difficulty_'))
  }

  return { data, loaded, load, name, text, list, difficulties }
})
