import { afterEach, describe, expect, it, vi } from 'vitest'
import { flushPromises, mount } from '@vue/test-utils'
import { createPinia } from 'pinia'
import { createMemoryHistory, createRouter } from 'vue-router'

import App from '@/App.vue'
import { createAppI18n } from '@/i18n'
import { routes } from '@/router'

async function mountApp(fetchImpl) {
  vi.stubGlobal('fetch', vi.fn(fetchImpl))
  const router = createRouter({ history: createMemoryHistory(), routes })
  router.push('/')
  await router.isReady()
  const wrapper = mount(App, {
    global: { plugins: [createPinia(), router, createAppI18n('de')] },
  })
  await flushPromises()
  return wrapper
}

const healthy = async () => ({
  ok: true,
  json: async () => ({ status: 'ok', version: '0.1.0' }),
})

afterEach(() => {
  vi.unstubAllGlobals()
})

describe('status page', () => {
  it('shows the server as online with its version', async () => {
    const wrapper = await mountApp(healthy)
    expect(fetch).toHaveBeenCalledWith('/api/health')
    expect(wrapper.get('[data-test="server-state"]').text()).toContain('ONLINE')
    expect(wrapper.get('[data-test="version"]').text()).toBe('0.1.0')
  })

  it('shows a missing connection', async () => {
    const wrapper = await mountApp(async () => {
      throw new TypeError('network down')
    })
    expect(wrapper.get('[data-test="server-state"]').text()).toContain('KEINE VERBINDUNG')
    expect(wrapper.find('[data-test="version"]').exists()).toBe(false)
  })

  it('switches between German and English', async () => {
    const wrapper = await mountApp(healthy)
    expect(wrapper.get('h1').text()).toBe('Systemstatus')
    await wrapper.get('[data-test="lang-en"]').trigger('click')
    expect(wrapper.get('h1').text()).toBe('System status')
    expect(document.documentElement.lang).toBe('en')
  })
})

describe('translations', () => {
  it('have the same keys in German and English', async () => {
    const { default: de } = await import('@/i18n/de.json')
    const { default: en } = await import('@/i18n/en.json')
    const keys = (obj, prefix = '') =>
      Object.entries(obj).flatMap(([k, v]) =>
        typeof v === 'object' ? keys(v, `${prefix}${k}.`) : [`${prefix}${k}`],
      )
    expect(keys(en).sort()).toEqual(keys(de).sort())
  })
})
