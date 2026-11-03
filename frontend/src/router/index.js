import { createRouter, createWebHistory } from 'vue-router'

import StatusView from '@/views/StatusView.vue'

export const routes = [
  { path: '/', name: 'status', component: StatusView },
  { path: '/:pathMatch(.*)*', redirect: '/' },
]

export default createRouter({
  history: createWebHistory(),
  routes,
})
