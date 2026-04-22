<script setup lang="ts">
import { computed } from 'vue'

import { useReactionGame, type ReactionPhase } from '@/composables/useReactionGame'
import type { GameMetadata } from '@/lib/play/metadata'

const props = defineProps<{
  gameCode: string
  metadata: GameMetadata | null
}>()

const {
  phase,
  currentRound,
  roundResults,
  countdownRemaining,
  totalRounds,
  isBusy,
  canStart,
  isSubmitting,
  errorMessage,
  medianMs,
  startGame,
  resetGame,
  onAreaClick,
} = useReactionGame({
  gameCode: computed(() => props.gameCode),
})

function ratingLabel(ms: number): string {
  if (ms <= 0) return 'No score'
  if (ms < 200) return 'Incredible'
  if (ms < 250) return 'Excellent'
  if (ms < 300) return 'Great'
  if (ms < 350) return 'Good'
  if (ms < 400) return 'Average'
  if (ms < 500) return 'Below average'
  return 'Slow'
}

function ratingColor(ms: number): 'neutral' | 'success' | 'secondary' | 'warning' | 'error' {
  if (ms <= 0) return 'neutral'
  if (ms < 250) return 'success'
  if (ms < 350) return 'secondary'
  if (ms < 450) return 'warning'
  return 'error'
}

const lastRoundMs = computed(() => {
  const results = roundResults.value
  return results.length > 0 ? results[results.length - 1]!.reactionMs : 0
})

const displayName = computed(() => props.metadata?.display_name || 'Reaction Time')
const taskSummary = computed(
  () =>
    props.metadata?.task_summary ||
    'Wait for green, then click as fast as you can. Your score uses the median valid reaction time.',
)
const benchmarkLabel = computed(() => props.metadata?.metric_name || '')

const isInteractive = computed(
  () =>
    phase.value === 'waiting' ||
    phase.value === 'signal' ||
    phase.value === 'tooEarly' ||
    phase.value === 'result',
)

const zoneAriaLabel = computed(() => {
  if (phase.value === 'waiting') {
    return 'Reaction test area. Wait for the signal, then press Enter or Space as fast as you can.'
  }

  if (phase.value === 'signal') {
    return 'Reaction test area. Signal shown. Press Enter or Space now.'
  }

  if (phase.value === 'tooEarly') {
    return 'Reaction test area. Too early. Press Enter or Space to retry this round.'
  }

  if (phase.value === 'result') {
    return 'Reaction test area. Round result shown. Press Enter or Space to continue.'
  }

  return undefined
})

function zoneClass(p: ReactionPhase): string {
  if (p === 'waiting') return 'zone-red'
  if (p === 'signal') return 'zone-green'
  if (p === 'tooEarly') return 'zone-amber'
  return 'zone-neutral'
}

function onZoneKeydown(event: KeyboardEvent): void {
  if (!isInteractive.value) {
    return
  }

  if (event.target !== event.currentTarget) {
    return
  }

  if (event.key !== 'Enter' && event.key !== ' ') {
    return
  }

  event.preventDefault()
  onAreaClick()
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

    <div
      class="play-surface rt-zone"
      :class="[zoneClass(phase), isInteractive ? 'cursor-pointer select-none' : '']"
      :role="isInteractive ? 'button' : undefined"
      :tabindex="isInteractive ? 0 : undefined"
      :aria-label="zoneAriaLabel"
      @click="isInteractive ? onAreaClick() : undefined"
      @keydown="onZoneKeydown"
    >
      <div v-if="phase !== 'idle' && phase !== 'finished'" class="play-surface__pill">
        {{ currentRound }} / {{ totalRounds }}
      </div>

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
          <UIcon name="i-lucide-mouse-pointer-click" class="h-8 w-8 text-secondary" />
        </div>
        <p class="play-surface__title">{{ displayName }}</p>
        <p class="play-surface__text">{{ taskSummary }} {{ totalRounds }} rounds.</p>
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
        <p class="play-surface__display">{{ countdownRemaining }}</p>
      </div>

      <div v-else-if="phase === 'waiting'" class="play-surface__content">
        <UIcon name="i-lucide-hand" class="h-14 w-14 text-white/80" />
        <p class="play-surface__display text-white">Wait...</p>
        <p class="rt-sub-light">Don't click yet</p>
      </div>

      <div v-else-if="phase === 'signal'" class="play-surface__content">
        <UIcon name="i-lucide-zap" class="h-14 w-14 text-white animate-bounce" />
        <p class="play-surface__display text-white">Click!</p>
      </div>

      <div v-else-if="phase === 'tooEarly'" class="play-surface__content">
        <UIcon name="i-lucide-alert-triangle" class="h-14 w-14 text-white/80" />
        <p class="play-surface__display text-white">Too early</p>
        <p class="rt-sub-light">Tap to retry this round</p>
      </div>

      <div v-else-if="phase === 'result'" class="play-surface__content">
        <UBadge :color="ratingColor(lastRoundMs)" variant="soft" size="lg">
          {{ ratingLabel(lastRoundMs) }}
        </UBadge>
        <p class="play-surface__display">
          {{ lastRoundMs }}<span class="text-2xl text-toned"> ms</span>
        </p>
        <p class="play-surface__eyebrow text-dimmed">
          Round {{ currentRound }} of {{ totalRounds }} - tap to continue
        </p>
      </div>

      <div
        v-else-if="phase === 'finished'"
        class="play-surface__content play-surface__content--wide w-full max-w-sm"
      >
        <UBadge :color="ratingColor(medianMs)" variant="soft" size="lg">
          {{ ratingLabel(medianMs) }}
        </UBadge>
        <div>
          <p class="play-surface__eyebrow text-toned">Median reaction time</p>
          <p class="play-surface__display">
            {{ medianMs }}<span class="text-2xl text-toned"> ms</span>
          </p>
        </div>

        <div class="w-full space-y-1.5">
          <div v-for="r in roundResults" :key="r.round" class="flex items-center gap-2">
            <span class="w-5 shrink-0 text-xs text-dimmed">{{ r.round }}</span>
            <div class="relative h-1.5 flex-1 overflow-hidden rounded-full bg-accented/40">
              <div
                class="absolute inset-y-0 left-0 rounded-full transition-all duration-500"
                :class="{
                  'bg-emerald-500': r.reactionMs < 250,
                  'bg-secondary': r.reactionMs >= 250 && r.reactionMs < 350,
                  'bg-amber-500': r.reactionMs >= 350 && r.reactionMs < 450,
                  'bg-red-500': r.reactionMs >= 450,
                }"
                :style="{ width: `${Math.min(100, (r.reactionMs / 600) * 100)}%` }"
              />
            </div>
            <span class="w-14 shrink-0 text-right text-xs text-toned">{{ r.reactionMs }} ms</span>
          </div>
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
      </div>
    </div>
  </div>
</template>

<style scoped>
.rt-zone {
  transition:
    background-color 120ms ease,
    border-color 120ms ease,
    box-shadow 120ms ease;
  box-shadow: 0 8px 16px -8px color-mix(in oklab, var(--ui-secondary) 20%, transparent);
}

.zone-neutral {
  border-color: color-mix(in oklab, var(--ui-secondary) 24%, var(--ui-border));
  background: color-mix(in oklab, var(--ui-bg-elevated) 48%, transparent);
}

.zone-red {
  border-color: color-mix(in oklab, #ef4444 48%, var(--ui-border));
  background: color-mix(in oklab, #ef4444 82%, var(--ui-bg));
  box-shadow: inset 0 0 80px rgba(0, 0, 0, 0.12);
}

.zone-green {
  border-color: color-mix(in oklab, #10b981 48%, var(--ui-border));
  background: color-mix(in oklab, #10b981 82%, var(--ui-bg));
  box-shadow: inset 0 0 80px rgba(0, 0, 0, 0.1);
}

.zone-amber {
  border-color: color-mix(in oklab, #f59e0b 48%, var(--ui-border));
  background: color-mix(in oklab, #f59e0b 82%, var(--ui-bg));
  box-shadow: inset 0 0 80px rgba(0, 0, 0, 0.12);
}

.rt-zone:active.cursor-pointer {
  transform: scale(0.998);
}

.rt-zone:focus-visible {
  outline: 3px solid color-mix(in oklab, var(--ui-secondary) 55%, white);
  outline-offset: 3px;
}

.rt-sub-light {
  font-size: 0.875rem;
  color: rgba(255, 255, 255, 0.65);
}
</style>
