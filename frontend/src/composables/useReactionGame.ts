import { computed, onScopeDispose, ref, toValue, type MaybeRefOrGetter } from 'vue'

import { createGameResult } from '@/lib/play/result'
import { useGameSession } from '@/composables/useGameSession'

/**
 * Phases of one reaction round:
 *  idle      – nothing happening, game not started yet
 *  countdown – "3… 2… 1…" before the first round or between rounds
 *  waiting   – red "wait…" screen; random delay before signal
 *  signal    – green "click NOW!" screen; timer is ticking
 *  tooEarly  – user clicked during the wait phase
 *  result    – individual round result shown
 *  finished  – all rounds done; summary visible
 */
export type ReactionPhase =
  | 'idle'
  | 'countdown'
  | 'waiting'
  | 'signal'
  | 'tooEarly'
  | 'result'
  | 'finished'

const TOTAL_ROUNDS = 5
const MIN_WAIT_MS = 1500
const MAX_WAIT_MS = 4500
const COUNTDOWN_SECONDS = 3
const MIN_VALID_REACTION_MS = 150
const MAX_VALID_REACTION_MS = 1500

function randomWaitMs(): number {
  return Math.round(MIN_WAIT_MS + Math.random() * (MAX_WAIT_MS - MIN_WAIT_MS))
}

function median(values: number[]): number {
  if (values.length === 0) return 0
  const sorted = [...values].sort((a, b) => a - b)
  const mid = Math.floor(sorted.length / 2)
  return sorted.length % 2 === 0 ? Math.round((sorted[mid - 1]! + sorted[mid]!) / 2) : sorted[mid]!
}

interface ReactionTimeSummary {
  scoredTimes: number[]
  averageMs: number
  medianMs: number
  bestMs: number
  worstMs: number
  totalMs: number
  excludedCount: number
}

function summarizeReactionTimes(times: number[]): ReactionTimeSummary {
  const scoredTimes = times.filter((ms) => ms >= MIN_VALID_REACTION_MS && ms <= MAX_VALID_REACTION_MS)
  const averageMs =
    scoredTimes.length > 0
      ? Math.round(scoredTimes.reduce((a, b) => a + b, 0) / scoredTimes.length)
      : 0

  return {
    scoredTimes,
    averageMs,
    medianMs: median(scoredTimes),
    bestMs: scoredTimes.length > 0 ? Math.min(...scoredTimes) : 0,
    worstMs: scoredTimes.length > 0 ? Math.max(...scoredTimes) : 0,
    totalMs: times.reduce((a, b) => a + b, 0),
    excludedCount: times.length - scoredTimes.length,
  }
}

export interface RoundResult {
  round: number
  reactionMs: number
}

interface UseReactionGameOptions {
  gameCode: MaybeRefOrGetter<string>
}

export function useReactionGame(options: UseReactionGameOptions) {
  // ── Shared session infrastructure ─────────────────────────────────────
  const session = useGameSession()

  // ── Game-specific state ───────────────────────────────────────────────
  const phase = ref<ReactionPhase>('idle')
  const currentRound = ref(0)
  const roundResults = ref<RoundResult[]>([])
  const countdownRemaining = ref(COUNTDOWN_SECONDS)

  // Internal timing
  let signalTimestamp: number | null = null
  let waitTimeout: ReturnType<typeof setTimeout> | null = null
  let countdownInterval: ReturnType<typeof setInterval> | null = null

  // ── Timer helpers ─────────────────────────────────────────────────────

  function clearWaitTimeout(): void {
    if (waitTimeout !== null) {
      clearTimeout(waitTimeout)
      waitTimeout = null
    }
  }

  function clearCountdownInterval(): void {
    if (countdownInterval !== null) {
      clearInterval(countdownInterval)
      countdownInterval = null
    }
  }

  function clearTimers(): void {
    clearWaitTimeout()
    clearCountdownInterval()
  }

  // ── Countdown (before round 1) ────────────────────────────────────────
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

  // ── Waiting → Signal transition ─────────────────────────────────────
  function enterWaiting(): void {
    phase.value = 'waiting'
    signalTimestamp = null

    const delay = randomWaitMs()
    waitTimeout = setTimeout(() => {
      phase.value = 'signal'
      signalTimestamp = performance.now()
    }, delay)
  }

  // ── User click handler (the core mechanic) ──────────────────────────
  function onAreaClick(): void {
    if (phase.value === 'waiting') {
      // Clicked too early
      clearWaitTimeout()
      phase.value = 'tooEarly'
      return
    }

    if (phase.value === 'signal') {
      // Good click — measure reaction
      const now = performance.now()
      const reactionMs = signalTimestamp !== null ? Math.round(now - signalTimestamp) : 0
      signalTimestamp = null

      const round = currentRound.value
      roundResults.value.push({ round, reactionMs })
      phase.value = 'result'
      return
    }

    if (phase.value === 'tooEarly') {
      // Retry the same round
      enterWaiting()
      return
    }

    if (phase.value === 'result') {
      // Advance to next round or finish
      if (currentRound.value >= TOTAL_ROUNDS) {
        finishGame()
      } else {
        currentRound.value += 1
        enterWaiting()
      }
    }
  }

  // ── Finish & submit ───────────────────────────────────────────────────

  async function finishGame(): Promise<void> {
    phase.value = 'finished'

    const times = roundResults.value.map((r) => r.reactionMs)
    const summary = summarizeReactionTimes(times)

    const score = summary.medianMs > 0 ? Number((100000 / summary.medianMs).toFixed(2)) : 0

    const result = createGameResult({
      score,
      durationMs: summary.totalMs,
      states: ['countdown', 'running', 'finished'],
      metrics: {
        rounds: TOTAL_ROUNDS,
        average_ms: summary.averageMs,
        median_ms: summary.medianMs,
        best_ms: summary.bestMs,
        worst_ms: summary.worstMs,
        n_excluded: summary.excludedCount,
        ...Object.fromEntries(times.map((t, i) => [`round_${i + 1}_ms`, t])),
      },
    })

    await session.submitResult(result)
  }

  // ── Public: start ─────────────────────────────────────────────────────

  async function startGame(): Promise<void> {
    if (phase.value === 'countdown' || phase.value === 'waiting' || phase.value === 'signal') return

    const gameCode = toValue(options.gameCode).trim()
    if (!gameCode) {
      // gameCode comes from route params — empty is a programming error, not user-facing
      return
    }

    // Reset game-specific state before awaiting so the UI shows a clean slate
    clearTimers()
    phase.value = 'idle'
    currentRound.value = 1
    roundResults.value = []
    signalTimestamp = null

    const { version, ok } = await session.beginSession(gameCode)
    if (!ok || session.isStale(version)) return

    runCountdown(() => enterWaiting())
  }

  // ── Public: reset ─────────────────────────────────────────────────────

  function resetGame(): void {
    clearTimers()
    session.resetSession()

    signalTimestamp = null
    phase.value = 'idle'
    currentRound.value = 0
    roundResults.value = []
    countdownRemaining.value = COUNTDOWN_SECONDS
  }

  onScopeDispose(() => {
    session.invalidate()
    clearTimers()
  })

  // ── Derived state ─────────────────────────────────────────────────────

  const isBusy = computed(() => session.isBusy.value)
  const canStart = computed(
    () => (phase.value === 'idle' || phase.value === 'finished') && !isBusy.value,
  )

  const medianMs = computed(() =>
    summarizeReactionTimes(roundResults.value.map((r) => r.reactionMs)).medianMs,
  )

  return {
    // Game state
    phase,
    currentRound,
    roundResults,
    countdownRemaining,
    totalRounds: TOTAL_ROUNDS,

    // Forwarded from session
    isBusy,
    canStart,
    isSubmitting: session.isSubmitting,
    errorMessage: session.errorMessage,
    lastResult: session.lastResult,

    // Derived
    medianMs,

    // Actions
    startGame,
    resetGame,
    onAreaClick,
  }
}
