<script setup lang="ts">
import { computed, ref } from 'vue'
import type { ButtonProps, SelectItem } from '@nuxt/ui'

import ProtectedNav from '@/components/ProtectedNav.vue'
import AppEmptyState from '@/components/ui/AppEmptyState.vue'
import AppErrorState from '@/components/ui/AppErrorState.vue'
import AppLoadingState from '@/components/ui/AppLoadingState.vue'
import { useLeaderboard } from '@/composables/useLeaderboard'
import { PLAY_MODULES, type GameId } from '@/lib/play/modules'

const selectedGameId = ref<GameId>(PLAY_MODULES[0].id)

const abilityItems = computed<SelectItem[]>(() =>
  PLAY_MODULES.map((module) => ({
    label: `${module.name} (${module.chcCode})`,
    value: module.id,
    icon: module.icon,
  })),
)

const { entries, isLoading, errorMessage, selectedModule, hasEntries, reload } = useLeaderboard({
  gameId: selectedGameId,
})

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

const selectedMetricLabel = computed(() => metricLabel(selectedGameId.value))
const selectedDirectionHint = computed(() => directionHint(selectedGameId.value))
const emptyActions = computed<ButtonProps[]>(() => [
  {
    label: 'Play this benchmark',
    icon: 'i-lucide-play',
    to: `/play/${selectedGameId.value}`,
  },
])

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
</script>

<template>
  <div class="min-h-svh">
    <ProtectedNav />

    <main class="mx-auto w-full max-w-5xl px-4 py-10 sm:px-6">
      <div class="space-y-8">
        <header class="space-y-3">
          <p class="text-xs font-semibold uppercase tracking-[0.18em] text-secondary/90">
            Leaderboard
          </p>
          <h1 class="text-2xl font-semibold text-highlighted sm:text-3xl">Top Scores</h1>
          <p class="max-w-2xl text-sm leading-6 text-toned sm:text-base">
            Compare completed benchmark results by CHC ability.
          </p>
        </header>

        <section class="space-y-4">
          <div class="flex flex-col gap-4 sm:flex-row sm:items-end sm:justify-between">
            <div class="flex items-center gap-3">
              <div
                class="inline-flex h-10 w-10 shrink-0 items-center justify-center rounded-lg border border-secondary/30 bg-secondary/10"
              >
                <UIcon
                  :name="selectedModule?.icon ?? 'i-lucide-trophy'"
                  class="h-5 w-5 text-secondary"
                />
              </div>
              <div class="min-w-0">
                <div class="flex items-center gap-2">
                  <h2 class="truncate text-lg font-semibold text-highlighted">
                    {{ selectedModule?.name ?? 'Ability' }}
                  </h2>
                  <UBadge v-if="selectedModule" color="secondary" variant="soft" size="sm">
                    {{ selectedModule.chcCode }}
                  </UBadge>
                </div>
                <p class="text-sm text-toned">
                  {{ selectedMetricLabel }} - {{ selectedDirectionHint }}
                </p>
              </div>
            </div>

            <UFormField label="Ability" name="ability" class="w-full sm:w-64">
              <USelect
                v-model="selectedGameId"
                :items="abilityItems"
                value-key="value"
                color="neutral"
                variant="subtle"
                class="w-full"
              />
            </UFormField>
          </div>

          <AppErrorState
            v-if="errorMessage"
            title="Leaderboard unavailable"
            :description="errorMessage"
            retry-label="Retry"
            @retry="reload"
          />

          <AppLoadingState v-else-if="isLoading" variant="list" :count="8" />

          <AppEmptyState
            v-else-if="!hasEntries"
            icon="i-lucide-trophy"
            title="No leaderboard entries yet"
            description="Complete this benchmark to place on the leaderboard."
            :actions="emptyActions"
          />

          <UCard
            v-else
            variant="subtle"
            class="border border-default/50"
            :ui="{ body: 'p-0 sm:p-0' }"
          >
            <div
              class="hidden grid-cols-[3rem_minmax(0,1fr)_8rem_11rem] gap-3 border-b border-default/40 px-5 py-3 text-xs font-semibold uppercase tracking-[0.12em] text-muted sm:grid"
            >
              <span>Rank</span>
              <span>Player</span>
              <span class="text-right">{{ selectedMetricLabel }}</span>
              <span class="text-right">Completed</span>
            </div>

            <ol class="divide-y divide-default/40">
              <li
                v-for="(entry, index) in entries"
                :key="`${entry.user_id}-${entry.completed_at}-${entry.metric_value}`"
                class="grid grid-cols-[auto_minmax(0,1fr)] gap-x-3 gap-y-2 px-4 py-4 sm:grid-cols-[3rem_minmax(0,1fr)_8rem_11rem] sm:items-center sm:px-5"
              >
                <div
                  class="flex h-9 w-9 items-center justify-center rounded-lg border border-default/50 bg-elevated/70 text-sm font-semibold text-highlighted"
                >
                  {{ index + 1 }}
                </div>

                <div class="min-w-0">
                  <p class="truncate text-sm font-semibold text-highlighted">
                    {{ entry.username }}
                  </p>
                  <p class="text-xs text-muted sm:hidden">{{ formatDate(entry.completed_at) }}</p>
                </div>

                <p
                  class="col-start-2 text-sm font-semibold text-highlighted sm:col-start-auto sm:text-right"
                >
                  {{ formatMetric(entry.metric_value, selectedGameId) }}
                </p>

                <p class="hidden text-sm text-toned sm:block sm:text-right">
                  {{ formatDate(entry.completed_at) }}
                </p>
              </li>
            </ol>
          </UCard>
        </section>
      </div>
    </main>
  </div>
</template>
