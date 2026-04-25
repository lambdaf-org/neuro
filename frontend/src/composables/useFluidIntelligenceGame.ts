import { computed, onScopeDispose, ref, toValue, type MaybeRefOrGetter } from 'vue'

import { useGameSession } from '@/composables/useGameSession'
import { ApiError } from '@/lib/auth'
import { createGameResult } from '@/lib/play/result'
import { getAssetGroupsByCode } from '@/lib/play/assets'
import {
  createFluidPuzzles,
  summarizeFluidAnswers,
  type FluidAnswer,
  type FluidPuzzle,
} from '@/lib/play/fluidIntelligence'
import { getErrorMessage } from '@/lib/utils/errorHandling'
import { useAuthStore } from '@/stores/auth'

export type FluidIntelligencePhase = 'idle' | 'loading' | 'countdown' | 'running' | 'finished'

const COUNTDOWN_SECONDS = 3

interface UseFluidIntelligenceGameOptions {
  gameCode: MaybeRefOrGetter<string>
}

export function useFluidIntelligenceGame(options: UseFluidIntelligenceGameOptions) {
  const auth = useAuthStore()
  const session = useGameSession()

  const phase = ref<FluidIntelligencePhase>('idle')
  const countdownRemaining = ref(COUNTDOWN_SECONDS)
  const puzzles = ref<FluidPuzzle[]>([])
  const currentIndex = ref(0)
  const answers = ref<FluidAnswer[]>([])
  const localErrorMessage = ref('')
  const isLoadingAssets = ref(false)

  let countdownInterval: ReturnType<typeof setInterval> | null = null
  let gameStartedAt: number | null = null
  let puzzleStartedAt: number | null = null
  let lifecycleVersion = 0

  function getAccessTokenOrThrow(): string {
    const token = auth.accessToken
    if (!token) throw new ApiError('No active session. Please sign in again.', 401)
    return token
  }

  function clearCountdownInterval(): void {
    if (countdownInterval !== null) {
      clearInterval(countdownInterval)
      countdownInterval = null
    }
  }

  function resetLocalState(): void {
    phase.value = 'idle'
    countdownRemaining.value = COUNTDOWN_SECONDS
    puzzles.value = []
    currentIndex.value = 0
    answers.value = []
    localErrorMessage.value = ''
    isLoadingAssets.value = false
    gameStartedAt = null
    puzzleStartedAt = null
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

  function startRunning(): void {
    phase.value = 'running'
    currentIndex.value = 0
    gameStartedAt = performance.now()
    puzzleStartedAt = gameStartedAt
  }

  async function finishGame(): Promise<void> {
    clearCountdownInterval()
    phase.value = 'finished'

    const summary = summarizeFluidAnswers(answers.value)
    const durationMs =
      gameStartedAt !== null ? Math.max(1, Math.round(performance.now() - gameStartedAt)) : 1

    const perPuzzleMetrics = Object.fromEntries(
      answers.value.flatMap((answer, index) => [
        [`item_${index + 1}_correct`, answer.wasCorrect ? 1 : 0],
        [`item_${index + 1}_response_ms`, answer.responseMs],
      ]),
    )

    const result = createGameResult({
      score: summary.accuracyPercent,
      durationMs,
      states: ['countdown', 'running', 'finished'],
      metrics: {
        total_items: summary.totalAnswers,
        correct_answers: summary.correctAnswers,
        incorrect_answers: summary.incorrectAnswers,
        accuracy_pct: summary.accuracyPercent,
        average_response_ms: summary.averageResponseMs,
        ...perPuzzleMetrics,
      },
    })

    await session.submitResult(result)
  }

  function selectOption(optionId: number): void {
    if (phase.value !== 'running') {
      return
    }

    const puzzle = puzzles.value[currentIndex.value]
    if (!puzzle) {
      return
    }

    const selectedOption = puzzle.options.find((option) => option.id === optionId)
    const correctOption = puzzle.options.find((option) => option.isCorrect)
    if (!selectedOption || !correctOption) {
      return
    }

    const responseMs =
      puzzleStartedAt !== null ? Math.max(0, Math.round(performance.now() - puzzleStartedAt)) : 0

    answers.value.push({
      puzzleId: puzzle.id,
      selectedOptionId: selectedOption.id,
      correctOptionId: correctOption.id,
      wasCorrect: selectedOption.id === correctOption.id,
      responseMs,
    })

    if (currentIndex.value >= puzzles.value.length - 1) {
      void finishGame()
      return
    }

    currentIndex.value += 1
    puzzleStartedAt = performance.now()
  }

  async function startGame(): Promise<void> {
    if (phase.value === 'loading' || phase.value === 'countdown' || phase.value === 'running') {
      return
    }

    const gameCode = toValue(options.gameCode).trim()
    if (!gameCode) {
      return
    }

    const version = ++lifecycleVersion
    clearCountdownInterval()
    resetLocalState()
    phase.value = 'loading'
    isLoadingAssets.value = true

    try {
      const groups = await getAssetGroupsByCode(gameCode, getAccessTokenOrThrow())
      if (version !== lifecycleVersion) {
        return
      }

      const loadedPuzzles = createFluidPuzzles(groups)
      if (loadedPuzzles.length === 0) {
        throw new ApiError('No playable Pattern Logic assets were found.', 500)
      }

      puzzles.value = loadedPuzzles
    } catch (error) {
      if (version !== lifecycleVersion) {
        return
      }

      localErrorMessage.value = getErrorMessage(error, 'Could not load Pattern Logic assets.')
      phase.value = 'idle'
      return
    } finally {
      if (version === lifecycleVersion) {
        isLoadingAssets.value = false
      }
    }

    const { version: sessionVersion, ok } = await session.beginSession(gameCode)
    if (!ok || session.isStale(sessionVersion) || version !== lifecycleVersion) {
      if (version === lifecycleVersion) {
        phase.value = 'idle'
      }
      return
    }

    runCountdown(() => startRunning())
  }

  function resetGame(): void {
    lifecycleVersion += 1
    clearCountdownInterval()
    session.resetSession()
    resetLocalState()
  }

  onScopeDispose(() => {
    resetGame()
  })

  const currentPuzzle = computed(() => puzzles.value[currentIndex.value] ?? null)
  const summary = computed(() => summarizeFluidAnswers(answers.value))
  const isBusy = computed(() => session.isBusy.value || isLoadingAssets.value)
  const canStart = computed(
    () => (phase.value === 'idle' || phase.value === 'finished') && !isBusy.value,
  )
  const errorMessage = computed(() => localErrorMessage.value || session.errorMessage.value)

  return {
    phase,
    countdownRemaining,
    currentPuzzle,
    currentIndex,
    totalPuzzles: computed(() => puzzles.value.length),
    answers,
    score: computed(() => summary.value.accuracyPercent),
    correctAnswers: computed(() => summary.value.correctAnswers),
    incorrectAnswers: computed(() => summary.value.incorrectAnswers),
    averageResponseMs: computed(() => summary.value.averageResponseMs),
    isBusy,
    canStart,
    isSubmitting: session.isSubmitting,
    errorMessage,
    startGame,
    resetGame,
    selectOption,
  }
}
