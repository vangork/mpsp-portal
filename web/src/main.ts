import './scss/main.scss'

import { createGtm } from '@gtm-support/vue-gtm'
import { createApp } from 'vue'
import { createVuestic } from 'vuestic-ui'
import App from './App.vue'
import i18n from './i18n'
import router from './router'
import stores from './stores'
import vuesticGlobalConfig from './services/vuestic-ui/global-config'

const app = createApp(App)

app.use(stores)
app.use(router)
app.use(i18n)
app.use(createVuestic({ config: vuesticGlobalConfig }))

if (import.meta.env.VITE_APP_GTM_ENABLED) {
  app.use(
    createGtm({
      id: import.meta.env.VITE_APP_GTM_KEY,
      debug: false,
      vueRouter: router,
    }),
  )
}

app.mount('#app')
