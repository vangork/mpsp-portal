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
      name: 'quartet',
      displayName: 'menu.quartet',
      meta: {
        icon: 'folder_shared',
      },
      children: [
        {
          name: 'dna',
          displayName: 'menu.dna',
        },
        {
          name: 'rna',
          displayName: 'menu.rna',
        },
        {
          name: 'protein',
          displayName: 'menu.protein',
        },
        {
          name: 'metabolism',
          displayName: 'menu.metabolism',
        },
      ],
    },
    {
      name: 'plasmix',
      displayName: 'menu.plasmix',
      meta: {
        icon: 'groups',
      },
      children: [
        {
          name: 'plasmix_protein',
          displayName: 'menu.plasmix_protein',
        },
        {
          name: 'plasmix_metabolism',
          displayName: 'menu.plasmix_metabolism',
        },
      ],
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
    {
      name: 'faq',
      displayName: 'menu.faq',
      meta: {
        icon: 'quiz',
      },
    },
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
      name: 'admin_quartet',
      displayName: 'menu.quartet',
      meta: {
        icon: 'folder_shared',
      },
      children: [
        {
          name: 'admin_dna',
          displayName: 'menu.dna',
        },
        {
          name: 'admin_rna',
          displayName: 'menu.rna',
        },
        {
          name: 'admin_protein',
          displayName: 'menu.protein',
        },
        {
          name: 'admin_metabolism',
          displayName: 'menu.metabolism',
        },
      ],
    },
    {
      name: 'admin_plasmix',
      displayName: 'menu.plasmix',
      meta: {
        icon: 'groups',
      },
      children: [
        {
          name: 'admin_plasmix_protein',
          displayName: 'menu.plasmix_protein',
        },
        {
          name: 'admin_plasmix_metabolism',
          displayName: 'menu.plasmix_metabolism',
        },
      ],
    },
  ] as INavigationRoute[],
}
