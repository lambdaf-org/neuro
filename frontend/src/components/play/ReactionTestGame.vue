<script setup lang="ts">
import { computed } from 'vue'

import { useReactionGame, type ReactionPhase } from '@/composables/useReactionGame'

const props = defineProps<{
  gameCode: string
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
  averageMs,
  startGame,
  resetGame,
  onAreaClick,
} = useReactionGame({
  gameCode: computed(() => props.gameCode),
})

function ratingLabel(ms: number): string {
  if (ms < 200) return 'Incredible'
  if (ms < 250) return 'Excellent'
  if (ms < 300) return 'Great'
  if (ms < 350) return 'Good'
  if (ms < 400) return 'Average'
  if (ms < 500) return 'Below average'
  return 'Slow'
}

function ratingColor(ms: number): 'success' | 'secondary' | 'warning' | 'error' {
  if (ms < 250) return 'success'
  if (ms < 350) return 'secondary'
  if (ms < 450) return 'warning'
  return 'error'
}

const lastRoundMs = computed(() => {
  const results = roundResults.value
  return results.length > 0 ? results[results.length - 1]!.reactionMs : 0
})

const isInteractive = computed(
  () =>
    phase.value === 'waiting' ||
    phase.value === 'signal' ||
    phase.value === 'tooEarly' ||
    phase.value === 'result',
)

function zoneClass(p: ReactionPhase): string {
  if (p === 'waiting') return 'zone-red'
  if (p === 'signal') return 'zone-green'
  if (p === 'tooEarly') return 'zone-amber'
  return 'zone-neutral'
}
</script>

<template>
  <div class="rt-root">
    <UAlert
      v-if="errorMessage"
      color="error"
      variant="soft"
      title="Error"
      :description="errorMessage"
      class="mb-4"
    />

    <div
      class="rt-zone"
      :class="[zoneClass(phase), isInteractive ? 'cursor-pointer select-none' : '']"
      @click="isInteractive ? onAreaClick() : undefined"
    >
      <div v-if="phase !== 'idle' && phase !== 'finished'" class="rt-round-pill">
        {{ currentRound }} / {{ totalRounds }}
      </div>

      <button
        v-if="phase !== 'idle' && phase !== 'finished'"
        class="rt-reset-btn"
        :disabled="isBusy"
        @click.stop="resetGame"
      >
        <UIcon name="i-lucide-x" class="h-4 w-4" />
      </button>

      <div v-if="phase === 'idle'" class="rt-content">
        <div class="rt-icon-wrap">
          <UIcon name="i-lucide-mouse-pointer-click" class="h-8 w-8 text-secondary" />
        </div>
        <p class="rt-headline">Reaction Time</p>
        <p class="rt-sub">
          Wait for <span class="text-emerald-500 font-semibold">green</span>, then click as fast as
          you can. {{ totalRounds }} rounds.
        </p>
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

      <div v-else-if="phase === 'countdown'" class="rt-content">
        <p class="rt-label">Get ready</p>
        <p class="rt-huge">{{ countdownRemaining }}</p>
      </div>

      <div v-else-if="phase === 'waiting'" class="rt-content">
        <UIcon name="i-lucide-hand" class="h-14 w-14 text-white/80" />
        <p class="rt-huge text-white">Wait...</p>
        <p class="rt-sub-light">Don't click yet</p>
      </div>

      <div v-else-if="phase === 'signal'" class="rt-content">
        <UIcon name="i-lucide-zap" class="h-14 w-14 text-white animate-bounce" />
        <p class="rt-huge text-white">Click!</p>
      </div>

      <div v-else-if="phase === 'tooEarly'" class="rt-content">
        <UIcon name="i-lucide-alert-triangle" class="h-14 w-14 text-white/80" />
        <p class="rt-huge text-white">Too early</p>
        <p class="rt-sub-light">Tap to retry this round</p>
      </div>

      <div v-else-if="phase === 'result'" class="rt-content">
        <UBadge :color="ratingColor(lastRoundMs)" variant="soft" size="lg">
          {{ ratingLabel(lastRoundMs) }}
        </UBadge>
        <p class="rt-huge">{{ lastRoundMs }}<span class="text-2xl text-toned"> ms</span></p>
        <p class="rt-label text-dimmed">
          Round {{ currentRound }} of {{ totalRounds }} - tap to continue
        </p>
      </div>

      <div v-else-if="phase === 'finished'" class="rt-content w-full max-w-sm">
        <UBadge :color="ratingColor(averageMs)" variant="soft" size="lg">
          {{ ratingLabel(averageMs) }}
        </UBadge>
        <div>
          <p class="rt-label text-toned">Average reaction time</p>
          <p class="rt-huge">{{ averageMs }}<span class="text-2xl text-toned"> ms</span></p>
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
.rt-root {
  width: 100%;
}

.rt-zone {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 22rem;
  border-radius: 1rem;
  overflow: hidden;
  border: 1px solid color-mix(in oklab, var(--ui-border) 82%, transparent);
  background: color-mix(in oklab, var(--ui-bg-elevated) 42%, transparent);
  backdrop-filter: blur(8px);
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

.rt-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
  padding: 2.5rem 1.5rem;
  text-align: center;
}

.rt-round-pill {
  position: absolute;
  top: 1rem;
  left: 1.25rem;
  font-size: 0.7rem;
  font-weight: 600;
  letter-spacing: 0.12em;
  text-transform: uppercase;
  color: color-mix(in oklab, var(--ui-text-toned) 70%, transparent);
  background: color-mix(in oklab, var(--ui-bg-elevated) 60%, transparent);
  border: 1px solid color-mix(in oklab, var(--ui-border) 50%, transparent);
  padding: 0.2rem 0.6rem;
  border-radius: 9999px;
  backdrop-filter: blur(6px);
}

.rt-reset-btn {
  position: absolute;
  top: 0.9rem;
  right: 1.1rem;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 1.75rem;
  height: 1.75rem;
  border-radius: 9999px;
  color: color-mix(in oklab, var(--ui-text-toned) 60%, transparent);
  background: color-mix(in oklab, var(--ui-bg-elevated) 50%, transparent);
  border: 1px solid color-mix(in oklab, var(--ui-border) 50%, transparent);
  cursor: pointer;
  transition: opacity 120ms;
  backdrop-filter: blur(6px);
}

.rt-reset-btn:hover:not(:disabled) {
  opacity: 0.8;
}

.rt-reset-btn:disabled {
  opacity: 0.35;
  cursor: default;
}

.rt-icon-wrap {
  display: inline-flex;
  height: 4rem;
  width: 4rem;
  align-items: center;
  justify-content: center;
  border-radius: 1rem;
  border: 1px solid color-mix(in oklab, var(--ui-secondary) 32%, transparent);
  background: color-mix(in oklab, var(--ui-secondary) 12%, transparent);
}

.rt-headline {
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--ui-text-highlighted);
}

.rt-huge {
  font-size: 4rem;
  font-weight: 700;
  line-height: 1;
  color: var(--ui-text-highlighted);
}

.rt-label {
  font-size: 0.7rem;
  font-weight: 600;
  letter-spacing: 0.14em;
  text-transform: uppercase;
  color: var(--ui-text-toned);
}

.rt-sub {
  font-size: 0.875rem;
  line-height: 1.6;
  color: var(--ui-text-toned);
  max-width: 22rem;
}

.rt-sub-light {
  font-size: 0.875rem;
  color: rgba(255, 255, 255, 0.65);
}
</style>
