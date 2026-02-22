<script setup lang="ts">
import { computed } from 'vue'
import type { NavigationMenuItem } from '@nuxt/ui'
import { RouterLink, useRoute, useRouter } from 'vue-router'

import { isAdminUiEnabled } from '@/lib/config/featureFlags'
import { useAuthStore } from '@/stores/auth'

const route = useRoute()
const router = useRouter()
const auth = useAuthStore()

const email = computed(() => auth.session?.email ?? '')

const items = computed<NavigationMenuItem[]>(() => [
  {
    label: 'Profile',
    to: '/profile',
    active: route.path.startsWith('/profile'),
  },
  ...(isAdminUiEnabled
    ? [{ label: 'Admin', to: '/admin/assets', active: route.path.startsWith('/admin/assets') }]
    : []),
])

async function onLogout() {
  auth.logout()
  await router.push({
    path: '/login',
    query: { loggedOut: '1' },
  })
}
</script>

<template>
  <header class="nav-header sticky top-0 z-20 border-b bg-elevated/90 backdrop-blur">
    <div class="mx-auto flex w-full max-w-5xl items-center gap-6 px-4 py-3 sm:px-6">
      <RouterLink
        to="/play"
        class="shrink-0 text-sm font-semibold text-secondary transition-colors hover:text-primary"
      >
        Neuro
      </RouterLink>

      <UNavigationMenu
        :items="items"
        color="neutral"
        variant="link"
        :highlight="false"
        class="min-w-0 flex-1"
      />

      <div class="flex items-center gap-4">
        <span class="hidden text-sm text-muted md:block">{{ email }}</span>
        <UButton color="neutral" variant="outline" size="sm" @click="onLogout">Logout</UButton>
      </div>
    </div>
  </header>
</template>

<style scoped>
.nav-header {
  border-color: color-mix(in oklab, var(--ui-border) 85%, var(--ui-primary) 15%);
}
</style>
