<script setup lang="ts">
import { computed } from 'vue'

import { useWorkingMemoryGame } from '@/composables/useWorkingMemoryGame'
import type { GameMetadata } from '@/lib/play/metadata'

const props = defineProps<{
  gameCode: string
  metadata: GameMetadata | null
}>()

const {
  phase,
  countdownRemaining,
  currentSpan,
  sequence,
  response,
  activeTile,
  attempts,
  lastAttempt,
  tileCount,
  maxSpan,
  averageResponseMs,
  isBusy,
  canStart,
  isSubmitting,
  errorMessage,
  startGame,
  resetGame,
  selectTile,
} = useWorkingMemoryGame({
  gameCode: computed(() => props.gameCode),
})

const displayName = computed(() => props.metadata?.display_name || 'Sequence Memory')
const taskSummary = computed(
  () =>
    props.metadata?.task_summary ||
    'Watch a highlighted sequence, then reproduce the same positions in order. Span increases after each correct recall.',
)
const benchmarkLabel = computed(() => props.metadata?.metric_name || 'Max Sequence Length')
const tileIndexes = computed(() => Array.from({ length: tileCount }, (_, index) => index))
const recallProgressLabel = computed(() => `${response.value.length} / ${sequence.value.length}`)
const feedbackColor = computed(() => (lastAttempt.value?.wasCorrect ? 'success' : 'error'))
const feedbackTitle = computed(() => {
  if (lastAttempt.value?.wasCorrect) {
    return 'Correct'
  }

  return 'Sequence ended'
})

function tileClass(tile: number): string {
  if (activeTile.value === tile) {
    return 'wm-tile--active'
  }

  if (phase.value === 'recalling' && response.value.includes(tile)) {
    return 'wm-tile--recalled'
  }

  if (phase.value === 'feedback' && lastAttempt.value?.sequence.includes(tile)) {
    return lastAttempt.value.wasCorrect ? 'wm-tile--correct' : 'wm-tile--missed'
  }

  return ''
}
</script>

<template>
  <div class="w-full">
    <UAlert
      v-if="errorMessage"
      color="error"
      variant="soft"
      title="Error"
      :description="errorMessage"
      class="mb-4"
    />

    <div class="play-surface p-6 sm:p-8">
      <UButton
        v-if="phase !== 'idle' && phase !== 'finished'"
        icon="i-lucide-x"
        color="neutral"
        variant="ghost"
        size="sm"
        square
        class="play-surface__reset"
        :disabled="isBusy"
        @click.stop="resetGame"
      />

      <div v-if="phase === 'idle'" class="play-surface__content">
        <div class="play-surface__icon">
          <UIcon name="i-lucide-brain-circuit" class="h-8 w-8 text-secondary" />
        </div>
        <p class="play-surface__title">{{ displayName }}</p>
        <p class="play-surface__text">{{ taskSummary }}</p>
        <p v-if="benchmarkLabel" class="play-surface__meta">Benchmark: {{ benchmarkLabel }}</p>
        <UButton
          color="secondary"
          size="lg"
          :loading="isBusy"
          :disabled="!canStart"
          class="mt-2"
          @click.stop="startGame"
        >
          Start
        </UButton>
      </div>

      <div v-else-if="phase === 'countdown'" class="play-surface__content">
        <p class="play-surface__eyebrow">Get ready</p>
        <p class="play-surface__display">
          {{ countdownRemaining }}
        </p>
      </div>

      <div
        v-else-if="phase === 'presenting' || phase === 'recalling' || phase === 'feedback'"
        class="play-surface__content play-surface__content--wide max-w-xl"
      >
        <div class="flex w-full items-center justify-between gap-3">
          <UBadge color="secondary" variant="soft" size="lg">Span {{ currentSpan }}</UBadge>
          <UBadge color="neutral" variant="soft" size="lg">Best {{ maxSpan }}</UBadge>
        </div>

        <div class="space-y-2 text-center">
          <p v-if="phase === 'presenting'" class="play-surface__eyebrow">Watch</p>
          <p v-else-if="phase === 'recalling'" class="play-surface__eyebrow">Repeat</p>
          <UBadge v-else :color="feedbackColor" variant="soft" size="lg">
            {{ feedbackTitle }}
          </UBadge>
        </div>

        <div class="wm-grid" aria-label="Sequence positions">
          <button
            v-for="tile in tileIndexes"
            :key="tile"
            type="button"
            class="wm-tile"
            :class="tileClass(tile)"
            :disabled="phase !== 'recalling'"
            :aria-label="`Memory tile ${tile + 1}`"
            @click.stop="selectTile(tile)"
          />
        </div>

        <div v-if="phase === 'recalling'" class="w-full max-w-sm space-y-2">
          <div class="flex items-center justify-between text-xs text-toned">
            <span>Recall progress</span>
            <span>{{ recallProgressLabel }}</span>
          </div>
          <UProgress
            :model-value="response.length"
            :max="sequence.length"
            color="secondary"
            size="sm"
          />
        </div>

        <p v-if="phase === 'presenting'" class="max-w-md text-sm leading-6 text-toned">
          Memorize the order of the highlighted positions.
        </p>
        <p v-else-if="phase === 'recalling'" class="max-w-md text-sm leading-6 text-toned">
          Tap the positions in the same order.
        </p>
        <p v-else-if="lastAttempt?.wasCorrect" class="max-w-md text-sm leading-6 text-toned">
          Next span starts now.
        </p>
        <p v-else class="max-w-md text-sm leading-6 text-toned">
          Your score is the longest sequence recalled in exact order.
        </p>
      </div>

      <div
        v-else-if="phase === 'finished'"
        class="play-surface__content play-surface__content--wide max-w-md"
      >
        <UBadge color="secondary" variant="soft" size="lg">Max span {{ maxSpan }}</UBadge>

        <div class="space-y-2">
          <p class="play-surface__eyebrow">Sequence memory score</p>
          <p class="play-surface__display">
            {{ maxSpan }}
          </p>
        </div>

        <div class="w-full">
          <UCard variant="subtle" :ui="{ body: 'p-4' }">
            <div class="flex flex-col gap-1">
              <span class="text-[0.7rem] font-semibold uppercase tracking-[0.12em] text-toned">
                Avg. recall
              </span>
              <span class="text-lg font-bold text-highlighted"> {{ averageResponseMs }} ms </span>
            </div>
          </UCard>
        </div>

        <div class="flex items-center gap-3">
          <span v-if="isSubmitting" class="flex items-center gap-1.5 text-xs text-dimmed">
            <UIcon name="i-lucide-loader-2" class="h-3.5 w-3.5 animate-spin" />
            Saving...
          </span>
          <UButton color="secondary" :disabled="isBusy" :loading="isBusy" @click.stop="startGame">
            Play again
          </UButton>
        </div>

        <p v-if="attempts.length === 0" class="text-sm text-toned">No recall attempt was scored.</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.wm-grid {
  display: grid;
  width: min(100%, 21rem);
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 0.75rem;
}

.wm-tile {
  aspect-ratio: 1;
  border-radius: 0.75rem;
  border: 1px solid color-mix(in oklab, var(--ui-border) 72%, transparent);
  background: color-mix(in oklab, var(--ui-bg-elevated) 70%, transparent);
  font-size: 1rem;
  font-weight: 700;
  color: var(--ui-text-toned);
  transition:
    background-color 120ms ease,
    border-color 120ms ease,
    color 120ms ease,
    transform 120ms ease,
    box-shadow 120ms ease;
}

.wm-tile:not(:disabled):hover,
.wm-tile:not(:disabled):focus-visible {
  border-color: color-mix(in oklab, var(--ui-secondary) 52%, var(--ui-border));
  background: color-mix(in oklab, var(--ui-secondary) 10%, var(--ui-bg-elevated));
}

.wm-tile:not(:disabled):focus-visible {
  outline: 3px solid color-mix(in oklab, var(--ui-secondary) 55%, white);
  outline-offset: 3px;
}

.wm-tile:not(:disabled):active {
  transform: scale(0.98);
}

.wm-tile--active {
  border-color: color-mix(in oklab, var(--ui-secondary) 74%, white);
  background: var(--ui-secondary);
  color: white;
  box-shadow: 0 0 0 6px color-mix(in oklab, var(--ui-secondary) 18%, transparent);
  transform: scale(1.03);
}

.wm-tile--recalled {
  border-color: color-mix(in oklab, var(--ui-primary) 48%, var(--ui-border));
  background: color-mix(in oklab, var(--ui-primary) 14%, var(--ui-bg-elevated));
  color: var(--ui-text-highlighted);
}

.wm-tile--correct {
  border-color: color-mix(in oklab, var(--ui-success) 58%, var(--ui-border));
  background: color-mix(in oklab, var(--ui-success) 15%, var(--ui-bg-elevated));
}

.wm-tile--missed {
  border-color: color-mix(in oklab, var(--ui-warning) 58%, var(--ui-border));
  background: color-mix(in oklab, var(--ui-warning) 14%, var(--ui-bg-elevated));
}
</style>
