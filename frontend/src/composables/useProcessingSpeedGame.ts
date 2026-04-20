import { computed, onScopeDispose, ref, toValue, type MaybeRefOrGetter } from 'vue'

import { useGameSession } from '@/composables/useGameSession'
import {
  createProcessingSpeedTrial,
  summarizeProcessingSpeedAttempts,
  type ProcessingSpeedAttempt,
  type ProcessingSpeedTrial,
} from '@/lib/play/processingSpeed'
import { createGameResult } from '@/lib/play/result'

export type ProcessingSpeedPhase = 'idle' | 'countdown' | 'running' | 'finished'

const COUNTDOWN_SECONDS = 3
const GAME_DURATION_MS = 20000

interface UseProcessingSpeedGameOptions {
  gameCode: MaybeRefOrGetter<string>
}

export function useProcessingSpeedGame(options: UseProcessingSpeedGameOptions) {
  const session = useGameSession()

  const phase = ref<ProcessingSpeedPhase>('idle')
  const countdownRemaining = ref(COUNTDOWN_SECONDS)
  const remainingMs = ref(GAME_DURATION_MS)
  const currentTrial = ref<ProcessingSpeedTrial | null>(null)
  const attempts = ref<ProcessingSpeedAttempt[]>([])

  let countdownInterval: ReturnType<typeof setInterval> | null = null
  let clockInterval: ReturnType<typeof setInterval> | null = null
  let gameStartedAt: number | null = null
  let currentTrialStartedAt: number | null = null
  let hasFinished = false

  function clearCountdownInterval(): void {
    if (countdownInterval !== null) {
      clearInterval(countdownInterval)
      countdownInterval = null
    }
  }

  function clearGameTimers(): void {
    if (clockInterval !== null) {
      clearInterval(clockInterval)
      clockInterval = null
    }
  }

  function clearTimers(): void {
    clearCountdownInterval()
    clearGameTimers()
  }

  function resetLocalState(): void {
    phase.value = 'idle'
    countdownRemaining.value = COUNTDOWN_SECONDS
    remainingMs.value = GAME_DURATION_MS
    currentTrial.value = null
    attempts.value = []
    gameStartedAt = null
    currentTrialStartedAt = null
    hasFinished = false
  }

  function nextTrial(): void {
    currentTrial.value = createProcessingSpeedTrial()
    currentTrialStartedAt = performance.now()
  }

  async function finishGame(): Promise<void> {
    if (hasFinished) {
      return
    }

    hasFinished = true
    clearGameTimers()
    phase.value = 'finished'
    remainingMs.value = 0
    currentTrial.value = null

    const summary = summarizeProcessingSpeedAttempts(attempts.value)
    const durationMs =
      gameStartedAt !== null ? Math.max(1, Math.round(performance.now() - gameStartedAt)) : 1

    const result = createGameResult({
      score: summary.correctAnswers,
      durationMs,
      states: ['countdown', 'running', 'finished'],
      metrics: {
        time_limit_ms: GAME_DURATION_MS,
        total_answers: summary.totalAnswers,
        correct_answers: summary.correctAnswers,
        incorrect_answers: summary.incorrectAnswers,
        accuracy_pct: summary.accuracyPercent,
        average_response_ms: summary.averageResponseMs,
      },
    })

    await session.submitResult(result)
  }

  function startRunning(): void {
    phase.value = 'running'
    remainingMs.value = GAME_DURATION_MS
    gameStartedAt = performance.now()
    hasFinished = false
    nextTrial()

    clockInterval = setInterval(() => {
      if (gameStartedAt === null) {
        return
      }

      const elapsed = performance.now() - gameStartedAt
      remainingMs.value = Math.max(0, GAME_DURATION_MS - Math.round(elapsed))

      if (remainingMs.value <= 0) {
        void finishGame()
      }
    }, 100)
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

  function answer(isPresentAnswer: boolean): void {
    if (phase.value !== 'running' || !currentTrial.value) {
      return
    }

    const responseMs =
      currentTrialStartedAt !== null ? Math.round(performance.now() - currentTrialStartedAt) : 0

    attempts.value.push({
      wasCorrect: isPresentAnswer === currentTrial.value.isMatchPresent,
      responseMs,
    })

    nextTrial()
  }

  async function startGame(): Promise<void> {
    if (phase.value === 'countdown' || phase.value === 'running') {
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

    runCountdown(() => startRunning())
  }

  function resetGame(): void {
    clearTimers()
    session.resetSession()
    resetLocalState()
  }

  onScopeDispose(() => {
    resetGame()
  })

  const summary = computed(() => summarizeProcessingSpeedAttempts(attempts.value))
  const isBusy = computed(() => session.isBusy.value)
  const canStart = computed(
    () => (phase.value === 'idle' || phase.value === 'finished') && !isBusy.value,
  )

  return {
    phase,
    countdownRemaining,
    remainingMs,
    durationMs: GAME_DURATION_MS,
    currentTrial,
    score: computed(() => summary.value.correctAnswers),
    totalAnswers: computed(() => summary.value.totalAnswers),
    incorrectAnswers: computed(() => summary.value.incorrectAnswers),
    accuracyPercent: computed(() => summary.value.accuracyPercent),
    averageResponseMs: computed(() => summary.value.averageResponseMs),
    isBusy,
    canStart,
    isSubmitting: session.isSubmitting,
    errorMessage: session.errorMessage,
    startGame,
    resetGame,
    answerPresent: () => answer(true),
    answerMissing: () => answer(false),
  }
}
