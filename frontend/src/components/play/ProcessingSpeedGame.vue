<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted } from 'vue'

import { useProcessingSpeedGame } from '@/composables/useProcessingSpeedGame'
import type { GameMetadata } from '@/lib/play/metadata'

const props = defineProps<{
  gameCode: string
  metadata: GameMetadata | null
}>()

const {
  phase,
  countdownRemaining,
  remainingMs,
  durationMs,
  currentTrial,
  score,
  totalAnswers,
  incorrectAnswers,
  accuracyPercent,
  averageResponseMs,
  isBusy,
  canStart,
  isSubmitting,
  errorMessage,
  startGame,
  resetGame,
  answerPresent,
  answerMissing,
} = useProcessingSpeedGame({
  gameCode: computed(() => props.gameCode),
})

const displayName = computed(() => props.metadata?.display_name || 'Symbol Matching')
const taskSummary = computed(
  () =>
    props.metadata?.task_summary ||
    'Decide whether the target symbol appears in the set of candidates. Answer as many trials as you can before time runs out.',
)
const benchmarkLabel = computed(() => props.metadata?.metric_name || '')
const choiceHint = computed(
  () => 'Press Present if the target appears. Press Missing if it does not.',
)
const remainingSecondsLabel = computed(() => (remainingMs.value / 1000).toFixed(1))
const progressPercent = computed(() => Math.max(0, (remainingMs.value / durationMs) * 100))

function onWindowKeydown(event: KeyboardEvent): void {
  if (phase.value !== 'running') {
    return
  }

  const target = event.target
  if (target instanceof HTMLElement) {
    const tagName = target.tagName
    if (
      tagName === 'INPUT' ||
      tagName === 'TEXTAREA' ||
      tagName === 'SELECT' ||
      target.isContentEditable
    ) {
      return
    }
  }

  if (event.key === 'ArrowLeft') {
    event.preventDefault()
    answerPresent()
    return
  }

  if (event.key === 'ArrowRight') {
    event.preventDefault()
    answerMissing()
    return
  }

  const normalizedKey = event.key.toLowerCase()
  if (normalizedKey === 'y') {
    event.preventDefault()
    answerPresent()
    return
  }

  if (normalizedKey === 'n') {
    event.preventDefault()
    answerMissing()
  }
}

onMounted(() => {
  window.addEventListener('keydown', onWindowKeydown)
})

onBeforeUnmount(() => {
  window.removeEventListener('keydown', onWindowKeydown)
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
      class="play-surface p-6 sm:p-8"
      :aria-label="
        phase === 'running'
          ? 'Symbol matching game. Press left or Y for present, right or N for missing.'
          : undefined
      "
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
          <UIcon name="i-lucide-scan-search" class="h-8 w-8 text-secondary" />
        </div>
        <p class="play-surface__title">{{ displayName }}</p>
        <p class="play-surface__text">{{ taskSummary }}</p>
        <p class="play-surface__text">{{ choiceHint }}</p>
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
        v-else-if="phase === 'running'"
        class="play-surface__content play-surface__content--wide max-w-xl"
      >
        <div class="flex w-full items-center justify-between gap-3">
          <UBadge color="secondary" variant="soft" size="lg">Score {{ score }}</UBadge>
          <UBadge color="neutral" variant="soft" size="lg">{{ remainingSecondsLabel }}s</UBadge>
        </div>

        <div class="h-2 w-full overflow-hidden rounded-full bg-muted">
          <div
            class="h-full rounded-full bg-[linear-gradient(90deg,#10b981_0%,#f59e0b_75%,#ef4444_100%)] transition-[width] duration-100 ease-linear"
            :style="{ width: `${progressPercent}%` }"
          />
        </div>

        <div v-if="currentTrial" class="w-full space-y-6">
          <div class="space-y-2 text-center">
            <p class="text-[0.7rem] font-semibold uppercase tracking-[0.14em] text-toned">Target</p>
            <div class="ps-target">
              {{ currentTrial.target }}
            </div>
          </div>

          <div class="space-y-2">
            <p class="text-[0.7rem] font-semibold uppercase tracking-[0.14em] text-toned">
              Candidates
            </p>
            <div class="grid grid-cols-2 gap-3">
              <div
                v-for="(symbol, index) in currentTrial.candidates"
                :key="`${symbol}-${index}`"
                class="ps-symbol-card"
              >
                {{ symbol }}
              </div>
            </div>
          </div>
        </div>

        <div class="grid w-full grid-cols-1 gap-3 sm:grid-cols-2" aria-label="Answer choices">
          <UButton
            color="success"
            variant="solid"
            size="xl"
            block
            class="justify-between"
            @click.stop="answerPresent"
          >
            <span>Present</span>
            <span class="text-xs uppercase tracking-[0.08em] opacity-80">Left / Y</span>
          </UButton>

          <UButton
            color="neutral"
            variant="outline"
            size="xl"
            block
            class="justify-between"
            @click.stop="answerMissing"
          >
            <span>Missing</span>
            <span class="text-xs uppercase tracking-[0.08em] opacity-80">Right / N</span>
          </UButton>
        </div>

        <p class="max-w-md text-sm leading-6 text-toned">
          Left arrow or Y = Present. Right arrow or N = Missing.
        </p>
      </div>

      <div
        v-else-if="phase === 'finished'"
        class="play-surface__content play-surface__content--wide max-w-md"
      >
        <UBadge color="secondary" variant="soft" size="lg">Final score {{ score }}</UBadge>

        <div class="space-y-2">
          <p class="play-surface__eyebrow">Correct answers in 20 seconds</p>
          <p class="play-surface__display">
            {{ score }}
          </p>
        </div>

        <div class="grid w-full grid-cols-2 gap-3">
          <UCard
            v-for="item in [
              { label: 'Attempts', value: totalAnswers },
              { label: 'Wrong', value: incorrectAnswers },
              { label: 'Accuracy', value: `${accuracyPercent}%` },
              { label: 'Avg. response', value: `${averageResponseMs} ms` },
            ]"
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
.ps-target {
  display: inline-flex;
  min-width: 5.5rem;
  min-height: 5.5rem;
  align-items: center;
  justify-content: center;
  margin-inline: auto;
  border-radius: 1rem;
  border: 1px solid color-mix(in oklab, var(--ui-secondary) 28%, transparent);
  background: color-mix(in oklab, var(--ui-secondary) 10%, transparent);
  font-size: 2.5rem;
  font-weight: 700;
  color: var(--ui-text-highlighted);
}

.ps-symbol-card {
  display: flex;
  min-height: 4.75rem;
  align-items: center;
  justify-content: center;
  border-radius: 1rem;
  border: 1px solid color-mix(in oklab, var(--ui-border) 60%, transparent);
  background: color-mix(in oklab, var(--ui-bg-elevated) 68%, transparent);
  font-size: 2rem;
  font-weight: 700;
  color: var(--ui-text-highlighted);
}
</style>
