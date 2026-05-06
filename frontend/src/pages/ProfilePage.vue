<script setup lang="ts">
import { computed } from 'vue'

import ProtectedNav from '@/components/ProtectedNav.vue'
import { useProfileHistory } from '@/composables/useProfileHistory'
import type { GameId } from '@/lib/play/modules'
import { useAuthStore } from '@/stores/auth'

const auth = useAuthStore()
const { histories, isLoading, errorMessage, hasSessions, reload } = useProfileHistory()

const session = computed(() => auth.session)

const dateFormatter = new Intl.DateTimeFormat(undefined, {
  dateStyle: 'medium',
  timeStyle: 'short',
})

const scoreFormatter = new Intl.NumberFormat(undefined, {
  maximumFractionDigits: 2,
})

const percentFormatter = new Intl.NumberFormat(undefined, {
  maximumFractionDigits: 0,
  style: 'percent',
})

function formatCompletedAt(value: string): string {
  const date = new Date(value)
  if (Number.isNaN(date.getTime())) {
    return '-'
  }

  return dateFormatter.format(date)
}

function formatMetric(value: number, gameId: GameId): string {
  if (gameId === 'reaction-time') {
    return `${scoreFormatter.format(value)} ms`
  }

  if (gameId === 'pattern-logic' || gameId === 'mental-rotation') {
    return percentFormatter.format(value)
  }

  return scoreFormatter.format(value)
}

function metricLabel(gameId: GameId): string {
  if (gameId === 'reaction-time') {
    return 'Median RT'
  }

  if (gameId === 'pattern-logic' || gameId === 'mental-rotation') {
    return 'Accuracy'
  }

  if (gameId === 'symbol-matching') {
    return 'Correct'
  }

  if (gameId === 'sequence-memory') {
    return 'Max span'
  }

  return 'Metric'
}
</script>

<template>
  <div class="min-h-svh">
    <ProtectedNav />

    <main class="mx-auto w-full max-w-5xl px-4 py-10 sm:px-6">
      <div class="space-y-8">
        <header class="space-y-3">
          <p class="text-xs font-semibold uppercase tracking-[0.18em] text-secondary/90">Profile</p>
          <h1 class="text-2xl font-semibold text-highlighted sm:text-3xl">Your History</h1>
          <p class="max-w-2xl text-sm leading-6 text-toned sm:text-base">
            Review your recent sessions for the active cognitive modules.
          </p>
        </header>

        <UCard variant="subtle" class="border border-default/60">
          <dl class="grid gap-5 text-sm sm:grid-cols-2">
            <div class="space-y-1">
              <dt class="font-medium text-muted">Email</dt>
              <dd class="text-highlighted">{{ session?.email || '-' }}</dd>
            </div>

            <div class="space-y-1">
              <dt class="font-medium text-muted">User ID</dt>
              <dd class="break-all text-highlighted">{{ session?.userId || '-' }}</dd>
            </div>
          </dl>
        </UCard>

        <section class="space-y-4">
          <div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
            <div class="space-y-1">
              <h2 class="text-lg font-semibold text-highlighted">Game Sessions</h2>
              <p class="text-sm text-toned">Active benchmark sessions.</p>
            </div>

            <UButton
              color="neutral"
              variant="outline"
              size="sm"
              icon="i-lucide-refresh-cw"
              :loading="isLoading"
              @click="reload"
            >
              Refresh
            </UButton>
          </div>

          <UAlert
            v-if="errorMessage"
            color="error"
            variant="soft"
            icon="i-lucide-circle-alert"
            title="History unavailable"
            :description="errorMessage"
          />

          <div v-if="isLoading" class="grid gap-4 md:grid-cols-2">
            <UCard
              v-for="index in 3"
              :key="index"
              variant="subtle"
              class="border border-default/50"
            >
              <div class="space-y-5">
                <div class="flex items-center justify-between gap-3">
                  <div class="flex items-center gap-3">
                    <USkeleton class="h-10 w-10 rounded-lg" />
                    <div class="space-y-2">
                      <USkeleton class="h-4 w-32" />
                      <USkeleton class="h-3 w-20" />
                    </div>
                  </div>
                  <USkeleton class="h-6 w-12 rounded-full" />
                </div>

                <div class="space-y-3">
                  <USkeleton v-for="row in 3" :key="row" class="h-12 w-full" />
                </div>
              </div>
            </UCard>
          </div>

          <UEmpty
            v-else-if="!errorMessage && !hasSessions"
            variant="subtle"
            icon="i-lucide-history"
            title="No sessions yet"
            description="Play an active benchmark to see your results here."
            :actions="[
              {
                label: 'Start playing',
                icon: 'i-lucide-play',
                to: '/play',
              },
            ]"
          />

          <div v-else class="grid gap-4 md:grid-cols-2">
            <UCard
              v-for="history in histories"
              :key="history.gameCode"
              variant="subtle"
              class="border border-default/50"
            >
              <template #header>
                <div class="flex items-start justify-between gap-3">
                  <div class="flex min-w-0 items-center gap-3">
                    <div
                      class="inline-flex h-10 w-10 shrink-0 items-center justify-center rounded-lg border border-secondary/30 bg-secondary/10"
                    >
                      <UIcon :name="history.module.icon" class="h-5 w-5 text-secondary" />
                    </div>
                    <div class="min-w-0">
                      <h3 class="truncate text-base font-semibold text-highlighted">
                        {{ history.module.name }}
                      </h3>
                      <p class="text-xs text-toned">
                        {{ history.sessions.length }} recent sessions
                      </p>
                    </div>
                  </div>

                  <UBadge color="secondary" variant="soft" size="sm">
                    {{ history.module.chcCode }}
                  </UBadge>
                </div>
              </template>

              <ol v-if="history.sessions.length" class="divide-y divide-default/60">
                <li
                  v-for="(gameSession, index) in history.sessions"
                  :key="`${history.gameCode}-${gameSession.completed_at}-${index}`"
                  class="flex items-center justify-between gap-4 py-3 first:pt-0 last:pb-0"
                >
                  <div class="min-w-0">
                    <p class="truncate text-sm font-medium text-highlighted">
                      {{ formatCompletedAt(gameSession.completed_at) }}
                    </p>
                  </div>

                  <div class="shrink-0 text-right">
                    <p class="text-sm font-semibold text-highlighted">
                      {{ formatMetric(gameSession.metric_value, history.module.id) }}
                    </p>
                    <p class="text-xs text-muted">{{ metricLabel(history.module.id) }}</p>
                  </div>
                </li>
              </ol>

              <UEmpty
                v-else
                variant="naked"
                size="sm"
                icon="i-lucide-history"
                title="No sessions"
                description="Results appear after your first completed run."
              />
            </UCard>
          </div>
        </section>
      </div>
    </main>
  </div>
</template>
