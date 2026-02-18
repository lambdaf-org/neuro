<script setup lang="ts">
import { computed } from 'vue'

import ProtectedNav from '@/components/ProtectedNav.vue'
import { useAuthStore } from '@/stores/auth'

const auth = useAuthStore()

const session = computed(() => auth.session)

const tokenPreview = computed(() => {
  const token = session.value?.accessToken
  if (!token) {
    return '-'
  }

  if (token.length <= 24) {
    return token
  }

  return `${token.slice(0, 12)}...${token.slice(-12)}`
})
</script>

<template>
  <div class="min-h-svh">
    <ProtectedNav />

    <main class="mx-auto w-full max-w-5xl px-4 py-10 sm:px-6">
      <UCard variant="subtle" class="border border-default/60">
        <template #header>
          <div class="space-y-1">
            <p class="text-xs font-semibold uppercase tracking-[0.16em] text-secondary">Profile</p>
            <h1 class="text-2xl font-semibold text-highlighted">Session Details</h1>
          </div>
        </template>

        <dl class="grid gap-5 text-sm sm:grid-cols-2">
          <div class="space-y-1">
            <dt class="font-medium text-muted">Email</dt>
            <dd class="text-highlighted">{{ session?.email || '-' }}</dd>
          </div>

          <div class="space-y-1">
            <dt class="font-medium text-muted">User ID</dt>
            <dd class="break-all text-highlighted">{{ session?.userId || '-' }}</dd>
          </div>

          <div class="space-y-1 sm:col-span-2">
            <dt class="font-medium text-muted">Access Token</dt>
            <dd class="break-all font-mono text-xs text-toned">{{ tokenPreview }}</dd>
          </div>
        </dl>
      </UCard>
    </main>
  </div>
</template>
