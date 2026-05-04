import { computed, onScopeDispose, ref, toValue, type MaybeRefOrGetter } from 'vue'

import { useGameSession } from '@/composables/useGameSession'
import { createGameResult } from '@/lib/play/result'
import {
  createWorkingMemorySequence,
  isWorkingMemoryRecallCorrect,
  summarizeWorkingMemoryAttempts,
  type WorkingMemoryAttempt,
} from '@/lib/play/workingMemory'

export type WorkingMemoryPhase =
  | 'idle'
  | 'countdown'
  | 'presenting'
  | 'recalling'
  | 'feedback'
  | 'finished'

const COUNTDOWN_SECONDS = 3
const STARTING_SPAN = 2
const TILE_COUNT = 9
const TILE_FLASH_MS = 560
const TILE_GAP_MS = 180
const FEEDBACK_MS = 720

interface UseWorkingMemoryGameOptions {
  gameCode: MaybeRefOrGetter<string>
}

export function useWorkingMemoryGame(options: UseWorkingMemoryGameOptions) {
  const session = useGameSession()

  const phase = ref<WorkingMemoryPhase>('idle')
  const countdownRemaining = ref(COUNTDOWN_SECONDS)
  const currentSpan = ref(STARTING_SPAN)
  const sequence = ref<number[]>([])
  const response = ref<number[]>([])
  const activeTile = ref<number | null>(null)
  const attempts = ref<WorkingMemoryAttempt[]>([])
  const lastAttempt = ref<WorkingMemoryAttempt | null>(null)

  let countdownInterval: ReturnType<typeof setInterval> | null = null
  let presentationTimeout: ReturnType<typeof setTimeout> | null = null
  let feedbackTimeout: ReturnType<typeof setTimeout> | null = null
  let gameStartedAt: number | null = null
  let recallStartedAt: number | null = null
  let hasFinished = false

  function clearCountdownInterval(): void {
    if (countdownInterval !== null) {
      clearInterval(countdownInterval)
      countdownInterval = null
    }
  }

  function clearPresentationTimeout(): void {
    if (presentationTimeout !== null) {
      clearTimeout(presentationTimeout)
      presentationTimeout = null
    }
  }

  function clearFeedbackTimeout(): void {
    if (feedbackTimeout !== null) {
      clearTimeout(feedbackTimeout)
      feedbackTimeout = null
    }
  }

  function clearTimers(): void {
    clearCountdownInterval()
    clearPresentationTimeout()
    clearFeedbackTimeout()
  }

  function resetLocalState(): void {
    phase.value = 'idle'
    countdownRemaining.value = COUNTDOWN_SECONDS
    currentSpan.value = STARTING_SPAN
    sequence.value = []
    response.value = []
    activeTile.value = null
    attempts.value = []
    lastAttempt.value = null
    gameStartedAt = null
    recallStartedAt = null
    hasFinished = false
  }

  function runCountdown(onComplete: () => void): void {
    countdownRemaining.value = COUNTDOWN_SECONDS
    phase.value = 'countdown'

    countdownInterval = setInterval(() => {
      if (countdownRemaining.value <= 1) {
        countdownRemaining.value = 0
        clearCountdownInterval()
        onComplete()
        return
      }

      countdownRemaining.value -= 1
    }, 1000)
  }

  function enterRecall(): void {
    activeTile.value = null
    response.value = []
    recallStartedAt = performance.now()
    phase.value = 'recalling'
  }

  function presentTileAt(index: number): void {
    const tile = sequence.value[index]

    if (tile === undefined) {
      enterRecall()
      return
    }

    activeTile.value = tile
    presentationTimeout = setTimeout(() => {
      activeTile.value = null
      presentationTimeout = setTimeout(() => presentTileAt(index + 1), TILE_GAP_MS)
    }, TILE_FLASH_MS)
  }

  function startRound(span: number): void {
    clearPresentationTimeout()
    clearFeedbackTimeout()
    currentSpan.value = span
    sequence.value = createWorkingMemorySequence(span, { tileCount: TILE_COUNT })
    response.value = []
    activeTile.value = null
    recallStartedAt = null
    phase.value = 'presenting'

    presentationTimeout = setTimeout(() => presentTileAt(0), TILE_GAP_MS)
  }

  async function finishGame(): Promise<void> {
    if (hasFinished) {
      return
    }

    hasFinished = true
    clearTimers()
    phase.value = 'finished'
    activeTile.value = null

    const summary = summarizeWorkingMemoryAttempts(attempts.value)
    const durationMs =
      gameStartedAt !== null ? Math.max(1, Math.round(performance.now() - gameStartedAt)) : 1

    const result = createGameResult({
      score: summary.maxSpan,
      durationMs,
      states: ['countdown', 'running', 'finished'],
      metrics: {
        total_attempts: summary.totalAttempts,
        correct_attempts: summary.correctAttempts,
        failed_attempts: summary.failedAttempts,
        failed_span: summary.failedSpan,
        average_response_ms: summary.averageResponseMs,
      },
      trials: attempts.value.map((attempt) => ({
        span: attempt.span,
        correct: attempt.wasCorrect,
        ms: attempt.responseMs,
      })),
    })

    await session.submitResult(result)
  }

  function completeRecall(): void {
    const currentResponse = [...response.value]
    const wasCorrect = isWorkingMemoryRecallCorrect(sequence.value, currentResponse)
    const responseMs =
      recallStartedAt !== null ? Math.max(0, Math.round(performance.now() - recallStartedAt)) : 0
    const attempt: WorkingMemoryAttempt = {
      span: currentSpan.value,
      wasCorrect,
      responseMs,
      sequence: [...sequence.value],
      response: currentResponse,
    }

    attempts.value.push(attempt)
    lastAttempt.value = attempt
    phase.value = 'feedback'
    recallStartedAt = null

    feedbackTimeout = setTimeout(() => {
      if (wasCorrect) {
        startRound(currentSpan.value + 1)
        return
      }

      void finishGame()
    }, FEEDBACK_MS)
  }

  function selectTile(tile: number): void {
    if (phase.value !== 'recalling') {
      return
    }

    if (tile < 0 || tile >= TILE_COUNT || response.value.length >= sequence.value.length) {
      return
    }

    response.value = [...response.value, tile]

    if (response.value.length >= sequence.value.length) {
      completeRecall()
    }
  }

  async function startGame(): Promise<void> {
    if (
      phase.value === 'countdown' ||
      phase.value === 'presenting' ||
      phase.value === 'recalling' ||
      phase.value === 'feedback'
    ) {
      return
    }

    const gameCode = toValue(options.gameCode).trim()
    if (!gameCode) {
      return
    }

    clearTimers()
    resetLocalState()

    const { version, ok } = await session.beginSession(gameCode)
    if (!ok || session.isStale(version)) {
      return
    }

    gameStartedAt = performance.now()
    runCountdown(() => startRound(STARTING_SPAN))
  }

  function resetGame(): void {
    clearTimers()
    session.resetSession()
    resetLocalState()
  }

  onScopeDispose(() => {
    resetGame()
  })

  const summary = computed(() => summarizeWorkingMemoryAttempts(attempts.value))
  const isBusy = computed(() => session.isBusy.value)
  const canStart = computed(
    () => (phase.value === 'idle' || phase.value === 'finished') && !isBusy.value,
  )

  return {
    phase,
    countdownRemaining,
    currentSpan,
    sequence,
    response,
    activeTile,
    attempts,
    lastAttempt,
    tileCount: TILE_COUNT,
    maxSpan: computed(() => summary.value.maxSpan),
    correctAttempts: computed(() => summary.value.correctAttempts),
    failedAttempts: computed(() => summary.value.failedAttempts),
    failedSpan: computed(() => summary.value.failedSpan),
    averageResponseMs: computed(() => summary.value.averageResponseMs),
    isBusy,
    canStart,
    isSubmitting: session.isSubmitting,
    errorMessage: session.errorMessage,
    startGame,
    resetGame,
    selectTile,
  }
}
