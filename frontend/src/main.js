import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import { createAppI18n } from './i18n'
import router from './router'
import './style.css'

createApp(App).use(createPinia()).use(router).use(createAppI18n()).mount('#app')
