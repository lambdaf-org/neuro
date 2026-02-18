<script setup lang="ts">
import { computed } from 'vue'
import type { NavigationMenuItem } from '@nuxt/ui'
import { useRoute, useRouter } from 'vue-router'

import { useAuthStore } from '@/stores/auth'

const route = useRoute()
const router = useRouter()
const auth = useAuthStore()

const email = computed(() => auth.session?.email ?? '')

const items = computed<NavigationMenuItem[]>(() => [
  {
    label: 'Play',
    to: '/play',
    active: route.path.startsWith('/play'),
  },
  {
    label: 'Profile',
    to: '/profile',
    active: route.path.startsWith('/profile'),
  },
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
  <header class="sticky top-0 z-20 border-b border-default bg-elevated/90 backdrop-blur">
    <div class="mx-auto flex w-full max-w-5xl items-center gap-4 px-4 py-3 sm:px-6">
      <p class="shrink-0 text-xs font-semibold uppercase tracking-[0.16em] text-secondary">Neuro</p>

      <UNavigationMenu
        :items="items"
        color="neutral"
        variant="link"
        :highlight="false"
        class="min-w-0 flex-1"
      />

      <div class="flex items-center gap-3">
        <span class="hidden text-sm text-toned md:block">{{ email }}</span>
        <UButton color="neutral" variant="outline" @click="onLogout">Logout</UButton>
      </div>
    </div>
  </header>
</template>
