import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'
import AppLayout from '../layouts/AppLayout.vue'
import AuthLayout from '../layouts/AuthLayout.vue'
import RouteViewComponent from '../layouts/RouterBypass.vue'
import RouterOmics from '../layouts/RouterOmics.vue'
import RouterPassthrough from '../layouts/RouterPassthrough.vue'
import { useAuthStore } from '../stores/auth'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/:pathMatch(.*)*',
    redirect: { name: '404' },
  },
  {
    name: 'user',
    path: '/user',
    component: AppLayout,
    props: { isAdmin: false },
    meta: {
      guard: 'user',
    },
    redirect: { name: 'dashboard' },
    children: [
      {
        name: 'dashboard',
        path: 'dashboard',
        component: () => import('../pages/user/Dashboard.vue'),
      },
      {
        name: 'quartet',
        path: '/quartet',
        component: RouterOmics,
        children: [
          {
            name: 'dna',
            path: 'dna',
            component: () => import('../pages/omics/DNA.vue'),
          },
          {
            name: 'rna',
            path: 'rna',
            component: () => import('../pages/omics/RNA.vue'),
          },
          {
            name: 'protein',
            path: 'protein',
            component: () => import('../pages/omics/Protein.vue'),
          },
          {
            name: 'metabolism',
            path: 'metabolism',
            component: () => import('../pages/omics/Metabolism.vue'),
          },
        ],
      },
      {
        name: 'plasmix',
        path: '/plasmix',
        component: RouterOmics,
        children: [
          {
            name: 'plasmix_protein',
            path: 'protein',
            component: () => import('../pages/omics/PlasmixProtein.vue'),
          },
          {
            name: 'plasmix_metabolism',
            path: 'metabolism',
            component: () => import('../pages/omics/PlasmixMetabolism.vue'),
          },
        ],
      },
      {
        name: 'preferences',
        path: 'preferences',
        component: () => import('../pages/user/preferences/Preferences.vue'),
      },
      {
        name: 'profile',
        path: 'profile',
        component: () => import('../pages/user/profile/Profile.vue'),
      },
      {
        name: 'payments',
        path: '/payments',
        component: RouteViewComponent,
        children: [
          {
            name: 'payment-methods',
            path: 'payment-methods',
            component: () => import('../pages/payments/PaymentsPage.vue'),
          },
          {
            name: 'billing',
            path: 'billing',
            component: () => import('../pages/billing/BillingPage.vue'),
          },
          {
            name: 'pricing-plans',
            path: 'pricing-plans',
            component: () => import('../pages/pricing-plans/PricingPlans.vue'),
          },
        ],
      },
      {
        name: 'faq',
        path: '/faq',
        component: () => import('../pages/faq/FaqPage.vue'),
      },
    ],
  },
  {
    name: 'admin',
    path: '/admin',
    component: AppLayout,
    props: { isAdmin: true },
    meta: {
      guard: 'admin',
    },
    redirect: { name: 'admin_dashboard' },
    children: [
      {
        name: 'admin_dashboard',
        path: 'dashboard',
        component: () => import('../pages/admin/dashboard/Dashboard.vue'),
      },
      {
        name: 'admin_users',
        path: 'users',
        component: () => import('../pages/admin/users/UsersPage.vue'),
      },
      {
        name: 'admin_quartet',
        path: 'quartet',
        component: RouterPassthrough,
        children: [
          {
            name: 'admin_dna',
            path: 'dna',
            component: () => import('../pages/admin/omics/DNA.vue'),
          },
          {
            name: 'admin_rna',
            path: 'rna',
            component: () => import('../pages/admin/omics/RNA.vue'),
          },
          {
            name: 'admin_protein',
            path: 'protein',
            component: () => import('../pages/admin/omics/Protein.vue'),
          },
          {
            name: 'admin_metabolism',
            path: 'metabolism',
            component: () => import('../pages/admin/omics/Metabolism.vue'),
          },
        ],
      },
      {
        name: 'admin_plasmix',
        path: 'plasmix',
        component: RouterPassthrough,
        children: [
          {
            name: 'admin_plasmix_protein',
            path: 'protein',
            component: () => import('../pages/admin/omics/PlasmixProtein.vue'),
          },
          {
            name: 'admin_plasmix_metabolism',
            path: 'metabolism',
            component: () => import('../pages/admin/omics/PlasmixMetabolism.vue'),
          },
        ],
      },
    ],
  },
  {
    path: '/auth',
    component: AuthLayout,
    meta: {
      guard: 'guest',
    },
    children: [
      {
        name: 'login',
        path: 'login',
        component: () => import('../pages/auth/Login.vue'),
      },
    ],
  },
  {
    name: '404',
    path: '/404',
    component: () => import('../pages/404.vue'),
  },
  {
    name: 'index',
    path: '/',
    component: () => import('../pages/Index.vue'),
  },
  {
    name: 'order',
    path: '/order',
    component: () => import('../pages/Order.vue'),
  },
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  scrollBehavior(to, from, savedPosition) {
    if (savedPosition) {
      return savedPosition
    }
    // For some reason using documentation example doesn't scroll on page navigation.
    if (to.hash) {
      return { el: to.hash, behavior: 'smooth' }
    } else {
      window.scrollTo(0, 0)
    }
  },
  routes,
})

router.beforeEach(async (to, from) => {
  console.log(`${new Date(Date.now()).toLocaleString('en-CA')}: route to ${to.fullPath}, from ${from.fullPath}`)

  const authStore = useAuthStore()
  await authStore.getProfile()
  if (
    (to.meta && to.meta.guard == 'guest' && authStore.isLoggedIn) ||
    (to.meta && to.meta.guard == 'user' && !authStore.isLoggedIn) ||
    (to.meta && to.meta.guard == 'admin' && !authStore.isAdmin)
  ) {
    return { name: 'index' }
  }
})

export default router
