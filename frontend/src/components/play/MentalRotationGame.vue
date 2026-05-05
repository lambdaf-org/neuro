<script setup lang="ts">
import { computed } from 'vue'

import { useMentalRotationGame } from '@/composables/useMentalRotationGame'
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
  hasSubmittedResult,
  score,
  correctAnswers,
  incorrectAnswers,
  averageResponseMs,
  isBusy,
  canStart,
  errorMessage,
  startGame,
  resetGame,
  selectOption,
} = useMentalRotationGame({
  gameCode: computed(() => props.gameCode),
})

const displayName = computed(() => props.metadata?.display_name || 'Mental Rotation')
const taskSummary = computed(
  () =>
    props.metadata?.task_summary ||
    'Compare a reference figure to four candidates and select the one that is a rotated match.',
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
  { label: 'Avg. response', value: `${averageResponseMs.value} ms` },
])
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
          <UIcon name="i-lucide-cuboid" class="h-8 w-8 text-secondary" />
        </div>
        <p class="play-surface__title">{{ displayName }}</p>
        <p class="play-surface__text">{{ taskSummary }}</p>
        <p class="play-surface__text">
          Choose the matching rotation. Mirror images and distractors are incorrect.
        </p>
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
        <p class="play-surface__eyebrow">Loading rotations</p>
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
        <div class="flex w-full items-center gap-3">
          <UBadge color="secondary" variant="soft" size="lg">{{ progressLabel }}</UBadge>
        </div>

        <div class="h-2 w-full overflow-hidden rounded-full bg-muted">
          <div
            class="h-full rounded-full bg-secondary transition-[width] duration-200 ease-out"
            :style="{ width: `${progressPercent}%` }"
          />
        </div>

        <div class="w-full space-y-3">
          <p class="text-[0.7rem] font-semibold uppercase tracking-[0.14em] text-toned">
            Reference
          </p>
          <div class="mr-reference-frame">
            <img
              :src="currentPuzzle.reference.imageUrl"
              :alt="`${currentPuzzle.label} reference figure`"
              class="mr-reference-image"
              draggable="false"
            />
          </div>
        </div>

        <div class="w-full space-y-3">
          <p class="text-[0.7rem] font-semibold uppercase tracking-[0.14em] text-toned">
            Rotated match
          </p>
          <div class="grid w-full grid-cols-2 gap-3" aria-label="Answer choices">
            <button
              v-for="(option, index) in currentPuzzle.options"
              :key="option.id"
              type="button"
              class="mr-option-button"
              @click.stop="selectOption(option.id)"
            >
              <span class="mr-option-index">{{ index + 1 }}</span>
              <img
                :src="option.imageUrl"
                :alt="`Option ${index + 1}`"
                class="mr-option-image"
                draggable="false"
              />
            </button>
          </div>
        </div>

        <p class="max-w-md text-sm leading-6 text-toned">
          Tap the candidate that matches the reference after rotation.
        </p>
      </div>

      <div v-else-if="phase === 'submitting'" class="play-surface__content">
        <UIcon name="i-lucide-loader-2" class="h-10 w-10 animate-spin text-secondary" />
        <p class="play-surface__eyebrow">Saving result</p>
      </div>

      <div
        v-else-if="phase === 'finished' && hasSubmittedResult"
        class="play-surface__content play-surface__content--wide max-w-md"
      >
        <UBadge color="secondary" variant="soft" size="lg">Final accuracy {{ score }}%</UBadge>

        <div class="space-y-2">
          <p class="play-surface__eyebrow">Mental rotation score</p>
          <p class="play-surface__display">{{ score }}<span class="text-2xl text-toned">%</span></p>
        </div>

        <div class="grid w-full grid-cols-2 gap-3">
          <UCard
            v-for="(item, index) in resultStats"
            :key="item.label"
            variant="subtle"
            :ui="{ body: 'p-4' }"
            :class="index === resultStats.length - 1 ? 'col-span-2' : ''"
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
          <UButton color="secondary" :disabled="isBusy" :loading="isBusy" @click.stop="startGame">
            Play again
          </UButton>
        </div>
      </div>

      <div
        v-else-if="phase === 'finished'"
        class="play-surface__content play-surface__content--wide max-w-md"
      >
        <UIcon name="i-lucide-alert-triangle" class="h-10 w-10 text-error" />
        <div class="space-y-2">
          <p class="play-surface__eyebrow">Result unavailable</p>
          <p class="play-surface__text">Your answers were not scored.</p>
        </div>
        <UButton color="secondary" :disabled="isBusy" :loading="isBusy" @click.stop="startGame">
          Play again
        </UButton>
      </div>
    </div>
  </div>
</template>

<style scoped>
.mr-reference-frame {
  display: flex;
  min-height: 11rem;
  align-items: center;
  justify-content: center;
  overflow: hidden;
  border-radius: 0.75rem;
  border: 1px solid color-mix(in oklab, var(--ui-secondary) 24%, var(--ui-border));
  background: color-mix(in oklab, var(--ui-bg-elevated) 76%, transparent);
}

.mr-reference-image {
  max-height: 15rem;
  width: 100%;
  object-fit: contain;
  padding: 1rem;
}

.mr-option-button {
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

.mr-option-button:hover,
.mr-option-button:focus-visible {
  border-color: color-mix(in oklab, var(--ui-secondary) 52%, var(--ui-border));
  background: color-mix(in oklab, var(--ui-secondary) 10%, var(--ui-bg-elevated));
}

.mr-option-button:focus-visible {
  outline: 3px solid color-mix(in oklab, var(--ui-secondary) 55%, white);
  outline-offset: 3px;
}

.mr-option-button:active {
  transform: scale(0.99);
}

.mr-option-index {
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

.mr-option-image {
  max-height: 8rem;
  width: 100%;
  object-fit: contain;
  padding: 1.2rem;
}
</style>
