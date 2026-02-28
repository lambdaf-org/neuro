import { computed, onScopeDispose, ref, toValue, type MaybeRefOrGetter } from 'vue'

import { ApiError } from '@/lib/auth'
import { createGameResult, type GameResult, type GameState } from '@/lib/play/result'
import { startGameSession, submitGameResult } from '@/lib/play/session'
import { getErrorMessage } from '@/lib/utils/errorHandling'
import { useAuthStore } from '@/stores/auth'

const DEFAULT_COUNTDOWN_SECONDS = 3

interface UseGameRuntimeOptions {
  gameCode: MaybeRefOrGetter<string>
  countdownSeconds?: number
}

function toCountdownSeconds(value: number | undefined): number {
  if (value === undefined || !Number.isFinite(value)) {
    return DEFAULT_COUNTDOWN_SECONDS
  }

  return Math.max(1, Math.round(value))
}

function buildDummyScore(durationMs: number): number {
  // Inverse duration keeps the dummy game usable as a base for reaction-time scoring.
  return Number((100000 / Math.max(1, durationMs)).toFixed(2))
}

export function useGameRuntime(options: UseGameRuntimeOptions) {
  const auth = useAuthStore()
  const countdownSeconds = toCountdownSeconds(options.countdownSeconds)

  const sessionId = ref<string | null>(null)
  const isStarting = ref(false)
  const isSubmitting = ref(false)

  const state = ref<GameState>('finished')
  const stateHistory = ref<GameState[]>([])
  const elapsedMs = ref(0)
  const countdownRemaining = ref(countdownSeconds)
  const errorMessage = ref('')
  const lastResult = ref<GameResult | null>(null)

  let countdownHandle: ReturnType<typeof setInterval> | null = null
  let animationHandle: number | null = null
  let runningStartAt: number | null = null

  function getAccessTokenOrThrow(): string {
    const accessToken = auth.accessToken
    if (!accessToken) {
      throw new ApiError('No active session. Please sign in again.', 401)
    }

    return accessToken
  }

  function clearSession(): void {
    sessionId.value = null
  }

  function clearCountdown(): void {
    if (countdownHandle !== null) {
      clearInterval(countdownHandle)
      countdownHandle = null
    }
  }

  function clearAnimation(): void {
    if (animationHandle !== null) {
      cancelAnimationFrame(animationHandle)
      animationHandle = null
    }
  }

  function clearTimers(): void {
    clearCountdown()
    clearAnimation()
  }

  function setState(nextState: GameState): void {
    state.value = nextState

    const currentHistory = stateHistory.value
    const lastState = currentHistory[currentHistory.length - 1]
    if (lastState !== nextState) {
      currentHistory.push(nextState)
    }
  }

  function enterRunning(): void {
    clearCountdown()
    setState('running')
    runningStartAt = performance.now()

    const tick = (now: number) => {
      const startedAt = runningStartAt
      if (startedAt === null) {
        return
      }

      elapsedMs.value = Math.max(0, Math.round(now - startedAt))
      animationHandle = requestAnimationFrame(tick)
    }

    animationHandle = requestAnimationFrame(tick)
  }

  async function startGame(): Promise<void> {
    if (state.value === 'countdown' || state.value === 'running') {
      return
    }

    const gameCode = toValue(options.gameCode).trim()
    if (!gameCode) {
      errorMessage.value = 'Missing game code.'
      return
    }

    clearTimers()
    clearSession()

    state.value = 'finished'
    stateHistory.value = []
    elapsedMs.value = 0
    countdownRemaining.value = countdownSeconds
    lastResult.value = null
    errorMessage.value = ''

    isStarting.value = true
    try {
      sessionId.value = await startGameSession(gameCode, getAccessTokenOrThrow())
    } catch (error) {
      state.value = 'finished'
      sessionId.value = null
      errorMessage.value = getErrorMessage(error, 'Could not start game session.')
      return
    } finally {
      isStarting.value = false
    }

    setState('countdown')
    countdownHandle = setInterval(() => {
      if (countdownRemaining.value <= 1) {
        countdownRemaining.value = 0
        enterRunning()
        return
      }

      countdownRemaining.value -= 1
    }, 1000)
  }

  async function stopGame(): Promise<void> {
    if (state.value !== 'running') {
      return
    }

    clearAnimation()

    const startedAt = runningStartAt
    runningStartAt = null

    const measuredDuration =
      startedAt === null ? Math.max(1, elapsedMs.value) : Math.max(1, Math.round(performance.now() - startedAt))

    elapsedMs.value = measuredDuration
    setState('finished')

    const result = createGameResult({
      score: buildDummyScore(measuredDuration),
      durationMs: measuredDuration,
      states: [...stateHistory.value],
      metrics: {
        elapsed_ms: measuredDuration,
        rounds: 1,
      },
    })

    lastResult.value = result

    const currentSessionId = sessionId.value
    if (!currentSessionId) {
      errorMessage.value = 'No active game session to submit.'
      return
    }

    isSubmitting.value = true
    try {
      await submitGameResult(currentSessionId, result, getAccessTokenOrThrow())
      errorMessage.value = ''
    } catch (error) {
      errorMessage.value = getErrorMessage(error, 'Could not submit game result.')
    } finally {
      isSubmitting.value = false
    }
  }

  function resetGame(): void {
    clearTimers()
    clearSession()

    runningStartAt = null
    state.value = 'finished'
    stateHistory.value = []
    elapsedMs.value = 0
    countdownRemaining.value = countdownSeconds
    errorMessage.value = ''
    lastResult.value = null
  }

  onScopeDispose(clearTimers)

  const isBusy = computed(() => isStarting.value || isSubmitting.value)
  const canStart = computed(() => state.value === 'finished' && !isBusy.value)
  const canStop = computed(() => state.value === 'running' && !isSubmitting.value)
  const formattedElapsed = computed(() => `${(elapsedMs.value / 1000).toFixed(3)} s`)

  return {
    state,
    countdownRemaining,
    elapsedMs,
    formattedElapsed,
    canStart,
    canStop,
    isBusy,
    errorMessage,
    lastResult,
    startGame,
    stopGame,
    resetGame,
  }
}
