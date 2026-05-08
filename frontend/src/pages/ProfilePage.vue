<script setup lang="ts">
import { reactive, ref } from 'vue'

import ProtectedNav from '@/components/ProtectedNav.vue'
import AppEmptyState from '@/components/ui/AppEmptyState.vue'
import AppErrorState from '@/components/ui/AppErrorState.vue'
import AppLoadingState from '@/components/ui/AppLoadingState.vue'
import { useProfileHistory, type ProfileGameEntry } from '@/composables/useProfileHistory'
import { getRecentGameSessions, type RecentGameSession } from '@/lib/play/history'
import type { GameId } from '@/lib/play/modules'
import { useAuthStore } from '@/stores/auth'

const auth = useAuthStore()
const { entries, isLoading, errorMessage, hasSessions, reload } = useProfileHistory()

const modalOpen = ref(false)
const activeEntry = ref<ProfileGameEntry | null>(null)
const recentSessions = reactive<Record<string, RecentGameSession[]>>({})
const recentLoading = reactive<Record<string, boolean>>({})

const scoreFormatter = new Intl.NumberFormat(undefined, {
  maximumFractionDigits: 2,
})

const percentFormatter = new Intl.NumberFormat(undefined, {
  maximumFractionDigits: 0,
  style: 'percent',
})

const dateFormatter = new Intl.DateTimeFormat(undefined, {
  dateStyle: 'medium',
  timeStyle: 'short',
})

function formatMetric(value: number, gameId: GameId): string {
  if (gameId === 'reaction-time') {
    return `${scoreFormatter.format(value)} ms`
  }

  if (gameId === 'pattern-logic' || gameId === 'mental-rotation') {
    return percentFormatter.format(value)
  }

  return scoreFormatter.format(value)
}

function formatDate(value: string): string {
  const date = new Date(value)
  if (Number.isNaN(date.getTime())) return '-'
  return dateFormatter.format(date)
}

function metricLabel(gameId: GameId): string {
  if (gameId === 'reaction-time') return 'Median RT'
  if (gameId === 'pattern-logic' || gameId === 'mental-rotation') return 'Accuracy'
  if (gameId === 'symbol-matching') return 'Correct'
  if (gameId === 'sequence-memory') return 'Max Span'
  return 'Score'
}

function directionHint(gameId: GameId): string {
  return gameId === 'reaction-time' ? 'lower is better' : 'higher is better'
}

async function openDetail(entry: ProfileGameEntry) {
  if (!entry.stats) return

  activeEntry.value = entry
  modalOpen.value = true

  if (recentSessions[entry.gameCode]) return

  recentLoading[entry.gameCode] = true
  try {
    auth.hydrate()
    const token = auth.accessToken
    if (!token) return
    recentSessions[entry.gameCode] = await getRecentGameSessions(entry.gameCode, token)
  } catch {
    recentSessions[entry.gameCode] = []
  } finally {
    recentLoading[entry.gameCode] = false
  }
}
</script>

<template>
  <div class="min-h-svh">
    <ProtectedNav />

    <main class="mx-auto w-full max-w-5xl px-4 py-10 sm:px-6">
      <div class="space-y-8">
        <!-- Header -->
        <header class="space-y-3">
          <p class="text-xs font-semibold uppercase tracking-[0.18em] text-secondary/90">Profile</p>
          <h1 class="text-2xl font-semibold text-highlighted sm:text-3xl">Cognitive Profile</h1>
          <p class="max-w-2xl text-sm leading-6 text-toned sm:text-base">
            Your best, average, and most recent scores across all cognitive benchmarks.
          </p>
        </header>

        <!-- Stats Section -->
        <section class="space-y-4">
          <div class="space-y-1">
            <h2 class="text-lg font-semibold text-highlighted">Performance Overview</h2>
            <p class="text-sm text-toned">Best · Average · Recent per benchmark.</p>
          </div>

          <!-- Error -->
          <AppErrorState
            v-if="errorMessage"
            title="Stats unavailable"
            :description="errorMessage"
            retry-label="Retry"
            @retry="reload"
          />

          <!-- Loading skeletons -->
          <AppLoadingState v-if="isLoading" variant="cards" :count="5" />

          <!-- Empty state -->
          <AppEmptyState
            v-else-if="!errorMessage && !hasSessions"
            icon="i-lucide-brain"
            title="No sessions yet"
            description="Complete a benchmark to see your cognitive profile here."
            :actions="[
              {
                label: 'Start playing',
                icon: 'i-lucide-play',
                to: '/play',
              },
            ]"
          />

          <!-- Game stat cards -->
          <div v-else-if="!errorMessage" class="grid gap-4 md:grid-cols-2">
            <UCard
              v-for="entry in entries"
              :key="entry.gameCode"
              variant="subtle"
              :tabindex="entry.stats ? 0 : undefined"
              :role="entry.stats ? 'button' : undefined"
              :aria-label="entry.stats ? `View ${entry.module.name} details` : undefined"
              class="border border-default/50 transition-colors"
              :class="{
                'cursor-pointer hover:border-secondary/40 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-secondary/40':
                  entry.stats,
              }"
              @click="openDetail(entry)"
              @keydown.enter.prevent="openDetail(entry)"
              @keydown.space.prevent="openDetail(entry)"
            >
              <template #header>
                <div class="flex items-start justify-between gap-3">
                  <div class="flex min-w-0 items-center gap-3">
                    <div
                      class="inline-flex h-10 w-10 shrink-0 items-center justify-center rounded-lg border border-secondary/30 bg-secondary/10"
                    >
                      <UIcon :name="entry.module.icon" class="h-5 w-5 text-secondary" />
                    </div>
                    <div class="min-w-0">
                      <h3 class="truncate text-base font-semibold text-highlighted">
                        {{ entry.module.name }}
                      </h3>
                      <p class="text-xs text-toned">
                        {{ metricLabel(entry.module.id) }}
                        <span class="text-muted"
                          >&middot; {{ directionHint(entry.module.id) }}</span
                        >
                      </p>
                    </div>
                  </div>

                  <UBadge color="secondary" variant="soft" size="sm">
                    {{ entry.module.chcCode }}
                  </UBadge>
                </div>
              </template>

              <!-- Stats grid: Best / Avg / Recent -->
              <div v-if="entry.stats" class="space-y-4">
                <div class="grid grid-cols-3 gap-3">
                  <div
                    class="rounded-lg border border-default/40 bg-elevated/50 px-3 py-2.5 text-center"
                  >
                    <p class="text-xs font-medium text-muted">Best</p>
                    <p class="mt-1 text-base font-semibold text-highlighted">
                      {{ formatMetric(entry.stats.best_metric, entry.module.id) }}
                    </p>
                  </div>
                  <div
                    class="rounded-lg border border-default/40 bg-elevated/50 px-3 py-2.5 text-center"
                  >
                    <p class="text-xs font-medium text-muted">Avg</p>
                    <p class="mt-1 text-base font-semibold text-highlighted">
                      {{ formatMetric(entry.stats.avg_metric, entry.module.id) }}
                    </p>
                  </div>
                  <div
                    class="rounded-lg border border-default/40 bg-elevated/50 px-3 py-2.5 text-center"
                  >
                    <p class="text-xs font-medium text-muted">Recent</p>
                    <p class="mt-1 text-base font-semibold text-highlighted">
                      {{ formatMetric(entry.stats.latest_metric, entry.module.id) }}
                    </p>
                  </div>
                </div>

                <p class="text-xs text-muted">
                  {{ entry.stats.session_count }} completed session{{
                    entry.stats.session_count === 1 ? '' : 's'
                  }}
                </p>
              </div>

              <!-- No sessions for this game -->
              <div v-else class="py-2 text-center">
                <p class="text-sm text-muted">No sessions yet</p>
                <UButton size="xs" variant="link" color="secondary" to="/play" class="mt-1">
                  Play now
                </UButton>
              </div>
            </UCard>
          </div>
        </section>
      </div>
    </main>

    <!-- Detail Modal -->
    <UModal v-model:open="modalOpen">
      <template #content>
        <div v-if="activeEntry" class="p-6 space-y-5">
          <!-- Modal header -->
          <div class="flex items-center gap-3">
            <div
              class="inline-flex h-10 w-10 shrink-0 items-center justify-center rounded-lg border border-secondary/30 bg-secondary/10"
            >
              <UIcon :name="activeEntry.module.icon" class="h-5 w-5 text-secondary" />
            </div>
            <div>
              <h3 class="text-lg font-semibold text-highlighted">
                {{ activeEntry.module.name }}
              </h3>
              <p class="text-xs text-toned">
                {{ metricLabel(activeEntry.module.id) }}
                <span class="text-muted">&middot; {{ directionHint(activeEntry.module.id) }}</span>
              </p>
            </div>
          </div>

          <!-- Stats summary in modal -->
          <div v-if="activeEntry.stats" class="grid grid-cols-3 gap-3">
            <div class="rounded-lg border border-default/40 bg-elevated/50 px-3 py-2.5 text-center">
              <p class="text-xs font-medium text-muted">Best</p>
              <p class="mt-1 text-base font-semibold text-highlighted">
                {{ formatMetric(activeEntry.stats.best_metric, activeEntry.module.id) }}
              </p>
            </div>
            <div class="rounded-lg border border-default/40 bg-elevated/50 px-3 py-2.5 text-center">
              <p class="text-xs font-medium text-muted">Avg</p>
              <p class="mt-1 text-base font-semibold text-highlighted">
                {{ formatMetric(activeEntry.stats.avg_metric, activeEntry.module.id) }}
              </p>
            </div>
            <div class="rounded-lg border border-default/40 bg-elevated/50 px-3 py-2.5 text-center">
              <p class="text-xs font-medium text-muted">Recent</p>
              <p class="mt-1 text-base font-semibold text-highlighted">
                {{ formatMetric(activeEntry.stats.latest_metric, activeEntry.module.id) }}
              </p>
            </div>
          </div>

          <!-- Recent runs list -->
          <div class="space-y-2">
            <p class="text-xs font-medium text-muted uppercase tracking-wide">Recent Runs</p>

            <AppLoadingState v-if="recentLoading[activeEntry.gameCode]" variant="list" :count="5" />

            <ul
              v-else-if="recentSessions[activeEntry.gameCode]?.length"
              class="divide-y divide-default/40"
            >
              <li
                v-for="run in recentSessions[activeEntry.gameCode]"
                :key="`${run.completed_at}-${run.metric_value}`"
                class="flex items-center justify-between py-2.5 first:pt-0 last:pb-0"
              >
                <span class="text-sm text-toned">{{ formatDate(run.completed_at) }}</span>
                <span class="text-sm font-medium text-highlighted">
                  {{ formatMetric(run.metric_value, activeEntry.module.id) }}
                </span>
              </li>
            </ul>

            <p v-else class="text-sm text-muted">No run data available.</p>
          </div>
        </div>
      </template>
    </UModal>
  </div>
</template>
