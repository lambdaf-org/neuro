<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'

import { useAuthStore } from '@/stores/auth'

const auth = useAuthStore()
const router = useRouter()

const reason = computed(
  () => auth.banReason || 'Your account was flagged by the anti-cheat system.',
)

async function onSignOut() {
  auth.logout()
  await router.push({ path: '/login', query: { loggedOut: '1' } })
}

async function onCreateNew() {
  auth.logout()
  await router.push('/register')
}
</script>

<template>
  <main class="mx-auto flex min-h-svh w-full max-w-xl items-center px-4 py-10 sm:px-6">
    <UCard variant="subtle" class="w-full border border-error/40 backdrop-blur-sm">
      <template #header>
        <div class="space-y-1">
          <p class="text-xs font-semibold uppercase tracking-[0.16em] text-error">Neuro</p>
          <h1 class="text-2xl font-semibold text-highlighted">Account suspended</h1>
        </div>
      </template>

      <div class="space-y-5">
        <div class="banned-hero">
          <div class="banned-hero__icon">
            <UIcon name="i-lucide-shield-alert" class="h-8 w-8 text-error" />
          </div>
          <p class="banned-hero__title">Anti-cheat triggered</p>
          <p class="banned-hero__text">{{ reason }}</p>
        </div>

        <UAlert
          color="error"
          variant="soft"
          title="What this means"
          description="Your reactions tripped one of our automated checks. The session was discarded and your account can no longer start new games."
          icon="i-lucide-info"
        />

        <div class="flex flex-col gap-2 sm:flex-row sm:justify-end">
          <UButton color="neutral" variant="subtle" icon="i-lucide-log-out" @click="onSignOut">
            Sign out
          </UButton>
          <UButton color="secondary" icon="i-lucide-user-plus" @click="onCreateNew">
            Create a new account
          </UButton>
        </div>
      </div>

      <template #footer>
        <p class="text-xs text-dimmed">
          If you think this is a mistake, sign out and contact support before opening another
          account.
        </p>
      </template>
    </UCard>
  </main>
</template>

<style scoped>
.banned-hero {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
  padding: 1.5rem 1rem;
  border-radius: 0.75rem;
  background: color-mix(in oklab, var(--ui-error) 10%, transparent);
  border: 1px solid color-mix(in oklab, var(--ui-error) 25%, var(--ui-border));
  text-align: center;
}

.banned-hero__icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 3.5rem;
  height: 3.5rem;
  border-radius: 9999px;
  background: color-mix(in oklab, var(--ui-error) 18%, var(--ui-bg));
}

.banned-hero__title {
  font-size: 1rem;
  font-weight: 600;
  color: var(--ui-text-highlighted);
}

.banned-hero__text {
  font-size: 0.875rem;
  color: var(--ui-text-toned);
  max-width: 28rem;
}
</style>
