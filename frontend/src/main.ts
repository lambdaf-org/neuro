import './assets/main.css'

import { createApp } from 'vue'
import ui from '@nuxt/ui/vue-plugin'

import App from './App.vue'
import { setUnauthorizedHandler } from './lib/auth/http'
import router from './router'
import { useAuthStore } from './stores/auth'
import { pinia } from './stores/pinia'

const app = createApp(App)

app.use(pinia)
app.use(router)
app.use(ui)

const auth = useAuthStore()
auth.hydrate()

let isHandlingUnauthorized = false

setUnauthorizedHandler(() => {
  if (isHandlingUnauthorized) {
    return
  }

  isHandlingUnauthorized = true
  const currentRoute = router.currentRoute.value

  auth.logout()

  if (currentRoute.name !== 'login') {
    void router.replace({
      name: 'login',
      query: {
        redirect: currentRoute.fullPath,
        sessionExpired: '1',
      },
    })
  }
})

app.mount('#app')
