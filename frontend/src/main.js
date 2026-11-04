import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import { onUnauthorized } from './api'
import { createAppI18n } from './i18n'
import router from './router'
import { useAuthStore } from './stores/auth'
import './style.css'

const pinia = createPinia()
createApp(App).use(pinia).use(router).use(createAppI18n()).mount('#app')

// An expired session sends people to the login page and back afterwards.
onUnauthorized(() => {
  const auth = useAuthStore(pinia)
  if (!auth.user) return
  auth.user = null
  router.push({ name: 'login', query: { next: router.currentRoute.value.fullPath } })
})
