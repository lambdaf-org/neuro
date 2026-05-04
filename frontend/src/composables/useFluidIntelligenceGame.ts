import { computed, onScopeDispose, ref, toValue, type MaybeRefOrGetter } from 'vue'

import { useGameSession } from '@/composables/useGameSession'
import { ApiError } from '@/lib/auth'
import { getAssetGroupsByCode } from '@/lib/play/assets'
import {
  createFluidPuzzles,
  summarizeFluidAnswers,
  type FluidAnswer,
  type FluidSummary,
  type FluidPuzzle,
} from '@/lib/play/fluidIntelligence'
import { submitFluidIntelligenceAnswers } from '@/lib/play/session'
import { getErrorMessage } from '@/lib/utils/errorHandling'
import { useAuthStore } from '@/stores/auth'

export type FluidIntelligencePhase =
  | 'idle'
  | 'loading'
  | 'countdown'
  | 'running'
  | 'submitting'
  | 'finished'

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
  const submittedSummary = ref<FluidSummary | null>(null)
  const localErrorMessage = ref('')
  const isLoadingAssets = ref(false)
  const isSubmittingAnswers = ref(false)

  let countdownInterval: ReturnType<typeof setInterval> | null = null
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
    submittedSummary.value = null
    localErrorMessage.value = ''
    isLoadingAssets.value = false
    isSubmittingAnswers.value = false
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
    puzzleStartedAt = performance.now()
  }

  async function finishGame(): Promise<void> {
    clearCountdownInterval()
    phase.value = 'submitting'

    const sid = session.sessionId.value
    if (!sid) {
      localErrorMessage.value = 'No active game session to submit.'
      phase.value = 'finished'
      return
    }

    isSubmittingAnswers.value = true
    try {
      const result = await submitFluidIntelligenceAnswers(
        sid,
        answers.value.map((answer) => ({
          puzzle_id: answer.puzzleId,
          selected_option_id: answer.selectedOptionId,
          response_ms: answer.responseMs,
        })),
        getAccessTokenOrThrow(),
      )

      submittedSummary.value = {
        totalAnswers: result.total_answers,
        correctAnswers: result.correct_answers,
        incorrectAnswers: result.incorrect_answers,
        accuracyPercent: result.score,
        averageResponseMs: result.average_response_ms,
      }
      localErrorMessage.value = ''
    } catch (error) {
      localErrorMessage.value = getErrorMessage(error, 'Could not submit Pattern Logic result.')
    } finally {
      isSubmittingAnswers.value = false
      phase.value = 'finished'
    }
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
    if (!selectedOption) {
      return
    }

    const responseMs =
      puzzleStartedAt !== null ? Math.max(0, Math.round(performance.now() - puzzleStartedAt)) : 0

    answers.value.push({
      puzzleId: puzzle.id,
      selectedOptionId: selectedOption.id,
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
  const summary = computed(() => submittedSummary.value ?? summarizeFluidAnswers(answers.value))
  const hasSubmittedResult = computed(() => submittedSummary.value !== null)
  const isBusy = computed(
    () => session.isBusy.value || isLoadingAssets.value || isSubmittingAnswers.value,
  )
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
    hasSubmittedResult,
    score: computed(() => summary.value.accuracyPercent),
    correctAnswers: computed(() => summary.value.correctAnswers),
    incorrectAnswers: computed(() => summary.value.incorrectAnswers),
    averageResponseMs: computed(() => summary.value.averageResponseMs),
    isBusy,
    canStart,
    errorMessage,
    startGame,
    resetGame,
    selectOption,
  }
}
