<script setup lang="ts">
import { computed, nextTick, ref, watch } from 'vue'

import { useFluidIntelligenceGame } from '@/composables/useFluidIntelligenceGame'
import type { GameMetadata } from '@/lib/play/metadata'

const props = defineProps<{
  gameCode: string
  metadata: GameMetadata | null
}>()

const {
  phase,
  countdownRemaining,
  currentPuzzle,
  currentIndex,
  totalPuzzles,
  answers,
  score,
  correctAnswers,
  incorrectAnswers,
  averageResponseMs,
  isBusy,
  canStart,
  isSubmitting,
  errorMessage,
  startGame,
  resetGame,
  selectOption,
} = useFluidIntelligenceGame({
  gameCode: computed(() => props.gameCode),
})

const displayName = computed(() => props.metadata?.display_name || 'Pattern Logic')
const taskSummary = computed(
  () =>
    props.metadata?.task_summary ||
    'Examine the visual pattern with a missing piece and select the option that completes it.',
)
const benchmarkLabel = computed(() => props.metadata?.metric_name || 'Accuracy (%)')
const progressLabel = computed(() =>
  totalPuzzles.value > 0 ? `${currentIndex.value + 1} / ${totalPuzzles.value}` : '',
)
const progressPercent = computed(() =>
  totalPuzzles.value > 0 ? ((currentIndex.value + 1) / totalPuzzles.value) * 100 : 0,
)
const resultStats = computed(() => [
  { label: 'Correct', value: correctAnswers.value },
  { label: 'Wrong', value: incorrectAnswers.value },
  { label: 'Items', value: answers.value.length },
  { label: 'Avg. response', value: `${averageResponseMs.value} ms` },
])
const gameSurface = ref<HTMLElement | null>(null)

function onSurfaceKeydown(event: KeyboardEvent): void {
  if (phase.value !== 'running' || event.target !== event.currentTarget) {
    return
  }

  const optionIndex = Number(event.key) - 1
  const option = currentPuzzle.value?.options[optionIndex]
  if (!option) {
    return
  }

  event.preventDefault()
  selectOption(option.id)
}

watch(phase, async (nextPhase) => {
  if (nextPhase !== 'running') {
    return
  }

  await nextTick()
  gameSurface.value?.focus()
})
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
      ref="gameSurface"
      class="play-surface p-6 sm:p-8"
      :tabindex="phase === 'running' ? 0 : undefined"
      :role="phase === 'running' ? 'group' : undefined"
      :aria-label="
        phase === 'running'
          ? 'Pattern logic game. Press number keys 1 through 4 to choose an option.'
          : undefined
      "
      @keydown="onSurfaceKeydown"
    >
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
          <UIcon name="i-lucide-grid-3x3" class="h-8 w-8 text-secondary" />
        </div>
        <p class="play-surface__title">{{ displayName }}</p>
        <p class="play-surface__text">{{ taskSummary }}</p>
        <p class="play-surface__text">Choose one completion for each pattern. Final score is accuracy.</p>
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

      <div v-else-if="phase === 'loading'" class="play-surface__content">
        <UIcon name="i-lucide-loader-2" class="h-10 w-10 animate-spin text-secondary" />
        <p class="play-surface__eyebrow">Loading patterns</p>
      </div>

      <div v-else-if="phase === 'countdown'" class="play-surface__content">
        <p class="play-surface__eyebrow">Get ready</p>
        <p class="play-surface__display">
          {{ countdownRemaining }}
        </p>
      </div>

      <div
        v-else-if="phase === 'running' && currentPuzzle"
        class="play-surface__content play-surface__content--wide max-w-2xl"
      >
        <div class="flex w-full items-center justify-between gap-3">
          <UBadge color="secondary" variant="soft" size="lg">{{ progressLabel }}</UBadge>
          <UBadge color="neutral" variant="soft" size="lg">Score {{ score }}%</UBadge>
        </div>

        <div class="h-2 w-full overflow-hidden rounded-full bg-muted">
          <div
            class="h-full rounded-full bg-secondary transition-[width] duration-200 ease-out"
            :style="{ width: `${progressPercent}%` }"
          />
        </div>

        <div class="w-full space-y-3">
          <p class="text-[0.7rem] font-semibold uppercase tracking-[0.14em] text-toned">
            Complete the pattern
          </p>
          <div class="fi-matrix-frame">
            <img
              :src="currentPuzzle.matrix.imageUrl"
              :alt="`${currentPuzzle.label} matrix pattern`"
              class="fi-matrix-image"
              draggable="false"
            />
          </div>
        </div>

        <div class="grid w-full grid-cols-2 gap-3" aria-label="Answer choices">
          <button
            v-for="(option, index) in currentPuzzle.options"
            :key="option.id"
            type="button"
            class="fi-option-button"
            @click.stop="selectOption(option.id)"
          >
            <span class="fi-option-index">{{ index + 1 }}</span>
            <img
              :src="option.imageUrl"
              :alt="`Option ${index + 1}`"
              class="fi-option-image"
              draggable="false"
            />
          </button>
        </div>

        <p class="max-w-md text-sm leading-6 text-toned">
          Press 1-4 or tap an option.
        </p>
      </div>

      <div
        v-else-if="phase === 'finished'"
        class="play-surface__content play-surface__content--wide max-w-md"
      >
        <UBadge color="secondary" variant="soft" size="lg">Final accuracy {{ score }}%</UBadge>

        <div class="space-y-2">
          <p class="play-surface__eyebrow">Pattern logic score</p>
          <p class="play-surface__display">
            {{ score }}<span class="text-2xl text-toned">%</span>
          </p>
        </div>

        <div class="grid w-full grid-cols-2 gap-3">
          <UCard
            v-for="item in resultStats"
            :key="item.label"
            variant="subtle"
            :ui="{ body: 'p-4' }"
          >
            <div class="flex flex-col gap-1">
              <span class="text-[0.7rem] font-semibold uppercase tracking-[0.12em] text-toned">
                {{ item.label }}
              </span>
              <span class="text-lg font-bold text-highlighted">
                {{ item.value }}
              </span>
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
      </div>
    </div>
  </div>
</template>

<style scoped>
.fi-matrix-frame {
  display: flex;
  min-height: 12rem;
  align-items: center;
  justify-content: center;
  overflow: hidden;
  border-radius: 0.75rem;
  border: 1px solid color-mix(in oklab, var(--ui-secondary) 24%, var(--ui-border));
  background: color-mix(in oklab, var(--ui-bg-elevated) 76%, transparent);
}

.fi-matrix-image {
  max-height: 18rem;
  width: 100%;
  object-fit: contain;
  padding: 1rem;
}

.fi-option-button {
  position: relative;
  display: flex;
  min-height: 8rem;
  align-items: center;
  justify-content: center;
  overflow: hidden;
  border-radius: 0.75rem;
  border: 1px solid color-mix(in oklab, var(--ui-border) 70%, transparent);
  background: color-mix(in oklab, var(--ui-bg-elevated) 68%, transparent);
  transition:
    border-color 120ms ease,
    background-color 120ms ease,
    transform 120ms ease;
}

.fi-option-button:hover,
.fi-option-button:focus-visible {
  border-color: color-mix(in oklab, var(--ui-secondary) 52%, var(--ui-border));
  background: color-mix(in oklab, var(--ui-secondary) 10%, var(--ui-bg-elevated));
}

.fi-option-button:focus-visible {
  outline: 3px solid color-mix(in oklab, var(--ui-secondary) 55%, white);
  outline-offset: 3px;
}

.fi-option-button:active {
  transform: scale(0.99);
}

.fi-option-index {
  position: absolute;
  top: 0.65rem;
  left: 0.65rem;
  display: inline-flex;
  height: 1.45rem;
  min-width: 1.45rem;
  align-items: center;
  justify-content: center;
  border-radius: 9999px;
  border: 1px solid color-mix(in oklab, var(--ui-border) 65%, transparent);
  background: color-mix(in oklab, var(--ui-bg-elevated) 72%, transparent);
  font-size: 0.75rem;
  font-weight: 700;
  color: var(--ui-text-toned);
}

.fi-option-image {
  max-height: 8rem;
  width: 100%;
  object-fit: contain;
  padding: 1.2rem;
}
</style>
