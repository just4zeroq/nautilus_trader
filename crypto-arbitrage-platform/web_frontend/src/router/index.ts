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
    },
    {
      path: '/monitor',
      name: 'Monitor',
      component: () => import('@/views/Monitor.vue')
    },
    {
      path: '/blacklist',
      name: 'Blacklist',
      component: () => import('@/views/Blacklist.vue')
    },
    {
      path: '/alerts',
      name: 'Alerts',
      component: () => import('@/views/Alerts.vue')
    },
    {
      path: '/alerts/rules',
      redirect: '/alerts'
    },
    {
      path: '/alerts/channels',
      name: 'AlertChannels',
      component: () => import('@/views/AlertChannels.vue')
    },
    {
      path: '/alerts/history',
      name: 'AlertHistory',
      component: () => import('@/views/AlertHistory.vue')
    },
    {
      path: '/accounts',
      name: 'Accounts',
      component: () => import('@/views/Accounts.vue')
    }
  ]
})

export default router
