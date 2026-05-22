export interface INavigationRoute {
  name: string
  displayName: string
  meta: { icon: string }
  children?: INavigationRoute[]
}

export const userNavigationRoutes = {
  routes: [
    {
      name: 'dashboard',
      displayName: 'menu.dashboard',
      meta: {
        icon: 'vuestic-iconset-dashboard',
      },
    },
    {
      name: 'projects',
      displayName: 'menu.projects',
      meta: {
        icon: 'vuestic-iconset-files',
      },
    },
    // {
    //   name: 'payments',
    //   displayName: 'menu.payments',
    //   meta: {
    //     icon: 'credit_card',
    //   },
    //   children: [
    //     {
    //       name: 'payment-methods',
    //       displayName: 'menu.payment-methods',
    //     },
    //     {
    //       name: 'pricing-plans',
    //       displayName: 'menu.pricing-plans',
    //     },
    //     {
    //       name: 'billing',
    //       displayName: 'menu.billing',
    //     },
    //   ],
    // },
    // {
    //   name: 'faq',
    //   displayName: 'menu.faq',
    //   meta: {
    //     icon: 'quiz',
    //   },
    // },
  ] as INavigationRoute[],
}

export const adminNavigationRoutes = {
  routes: [
    {
      name: 'admin_dashboard',
      displayName: 'menu.overview',
      meta: {
        icon: 'vuestic-iconset-dashboard',
      },
    },
    {
      name: 'admin_users',
      displayName: 'menu.users',
      meta: {
        icon: 'group',
      },
    },
    {
      name: 'admin_projects',
      displayName: 'menu.projects_details',
      meta: {
        icon: 'vuestic-iconset-files',
      },
    },
    {
      name: 'admin_billing',
      displayName: 'menu.billing_info',
      meta: {
        icon: 'credit_card',
      },
    },
  ] as INavigationRoute[],
}
