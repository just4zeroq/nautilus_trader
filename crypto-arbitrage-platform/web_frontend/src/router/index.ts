import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'Dashboard',
      component: () => import('@/views/Dashboard.vue')
    },
    {
      path: '/strategies',
      name: 'Strategies',
      component: () => import('@/views/Strategies.vue')
    },
    {
      path: '/symbols',
      name: 'Symbols',
      component: () => import('@/views/Symbols.vue')
    },
    {
      path: '/positions',
      name: 'Positions',
      component: () => import('@/views/Positions.vue')
    }
  ]
})

export default router
