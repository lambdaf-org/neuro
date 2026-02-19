import './assets/main.css'

import { createApp } from 'vue'
import ui from '@nuxt/ui/vue-plugin'

import App from './App.vue'
import router from './router'
import { useAuthStore } from './stores/auth'
import { pinia } from './stores/pinia'

const app = createApp(App)

app.use(pinia)
app.use(router)
app.use(ui)

useAuthStore().hydrate()

app.mount('#app')
