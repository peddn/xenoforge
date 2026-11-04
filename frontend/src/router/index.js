import { createRouter, createWebHistory } from 'vue-router'

import { useAuthStore } from '@/stores/auth'

export const routes = [
  { path: '/', name: 'home', component: () => import('@/views/HomeView.vue') },
  { path: '/login', name: 'login', component: () => import('@/views/LoginView.vue'), meta: { public: true } },
  { path: '/characters/:id', name: 'character', component: () => import('@/views/CharacterView.vue') },
  { path: '/teams/:id', name: 'team', component: () => import('@/views/TeamView.vue') },
  { path: '/join/:token', name: 'join', component: () => import('@/views/JoinView.vue') },
  { path: '/:pathMatch(.*)*', redirect: '/' },
]

/** Everything except the login page needs an account. */
export async function guard(to) {
  const auth = useAuthStore()
  if (!auth.checked) await auth.load()
  if (!to.meta.public && !auth.user) return { name: 'login', query: { next: to.fullPath } }
  return true
}

const router = createRouter({ history: createWebHistory(), routes })
router.beforeEach(guard)

export default router
