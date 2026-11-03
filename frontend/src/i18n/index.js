import { createI18n } from 'vue-i18n'

import de from './de.json'
import en from './en.json'

export const LOCALES = ['de', 'en']
const STORAGE_KEY = 'xenoforge.locale'

// Browser storage can be missing or blocked; the app works without it.
function storedLocale() {
  try {
    const locale = localStorage.getItem(STORAGE_KEY)
    return LOCALES.includes(locale) ? locale : null
  } catch {
    return null
  }
}

export function saveLocale(locale) {
  try {
    localStorage.setItem(STORAGE_KEY, locale)
  } catch {
    // ignore
  }
}

export function createAppI18n(locale = storedLocale() ?? 'de') {
  return createI18n({
    legacy: false,
    locale,
    fallbackLocale: 'de',
    messages: { de, en },
  })
}
