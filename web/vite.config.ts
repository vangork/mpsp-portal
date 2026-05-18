import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { resolve, dirname } from 'node:path'
import { fileURLToPath } from 'url'
import VueI18nPlugin from '@intlify/unplugin-vue-i18n/vite'
import { vuestic } from '@vuestic/compiler/vite'

// https://vitejs.dev/config/
export default defineConfig(({ command }) => {
  const data = {
    build: {
      sourcemap: true,
    },
    plugins: [
      vuestic({ devtools: false }),
      vue(),
      VueI18nPlugin({
        include: resolve(dirname(fileURLToPath(import.meta.url)), './src/i18n/locales/**'),
      }),
    ],
  }

  if (command === 'serve') {
    return {
      ...data,
      server: {
        proxy: {
          // Proxy all requests starting with /api
          '/api': {
            target: 'http://127.0.0.1:8080', // Replace with your backend API URL
            changeOrigin: true,
            secure: false,
          },
        },
      },
    }
  } else {
    return data
  }
})
