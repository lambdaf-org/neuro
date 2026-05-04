import { computed, ref } from 'vue'

import { ApiError } from '@/lib/auth'
import { type GameResult } from '@/lib/play/result'
import { startGameSession, submitGameResult, type FinalizeSessionResult } from '@/lib/play/session'
import { getErrorMessage } from '@/lib/utils/errorHandling'
import { useAuthStore } from '@/stores/auth'

export function useGameSession() {
  const auth = useAuthStore()

  const sessionId = ref<string | null>(null)
  const isStarting = ref(false)
  const isSubmitting = ref(false)
  const errorMessage = ref('')
  const lastResult = ref<GameResult | null>(null)
  const lastFinalizeResult = ref<FinalizeSessionResult | null>(null)

  let lifecycleVersion = 0

  function getAccessTokenOrThrow(): string {
    const token = auth.accessToken
    if (!token) throw new ApiError('No active session. Please sign in again.', 401)
    return token
  }

  /**
   * Bump the version counter and return the new version.
   * Store the returned value in a local const before any await, then check
   * `isStale(version)` after every await to detect cancellation.
   */
  function bumpVersion(): number {
    return ++lifecycleVersion
  }

  /**
   * Returns true if the given version no longer matches the current one,
   * meaning the game was reset/restarted while an async op was in flight.
   */
  function isStale(version: number): boolean {
    return version !== lifecycleVersion
  }

  /**
   * Increment version without returning it.  Call this when you want to
   * cancel in-flight async work (e.g. in `resetGame` or `onScopeDispose`).
   */
  function invalidate(): void {
    lifecycleVersion += 1
  }

  async function beginSession(gameCode: string): Promise<{ version: number; ok: boolean }> {
    const version = bumpVersion()

    sessionId.value = null
    isStarting.value = true
    errorMessage.value = ''
    lastResult.value = null
    lastFinalizeResult.value = null

    try {
      const id = await startGameSession(gameCode, getAccessTokenOrThrow())
      if (isStale(version)) return { version, ok: false }

      sessionId.value = id
      return { version, ok: true }
    } catch (error) {
      if (isStale(version)) return { version, ok: false }

      errorMessage.value = getErrorMessage(error, 'Could not start game session.')
      return { version, ok: false }
    } finally {
      if (!isStale(version)) isStarting.value = false
    }
  }

  async function submitResult(result: GameResult): Promise<void> {
    lastResult.value = result
    lastFinalizeResult.value = null

    const sid = sessionId.value
    if (!sid) {
      errorMessage.value = 'No active game session to submit.'
      return
    }

    const version = lifecycleVersion
    isSubmitting.value = true
    try {
      const finalizeResult = await submitGameResult(sid, result, getAccessTokenOrThrow())
      if (isStale(version)) return

      lastFinalizeResult.value = finalizeResult
      if (finalizeResult.status === 'invalid') {
        errorMessage.value =
          typeof finalizeResult.metrics.reason === 'string'
            ? finalizeResult.metrics.reason
            : 'Game result was marked invalid.'
      } else {
        errorMessage.value = ''
      }
    } catch (error) {
      if (isStale(version)) return
      errorMessage.value = getErrorMessage(error, 'Could not submit game result.')
    } finally {
      if (!isStale(version)) isSubmitting.value = false
    }
  }

  function resetSession(): void {
    invalidate()
    sessionId.value = null
    isStarting.value = false
    isSubmitting.value = false
    errorMessage.value = ''
    lastResult.value = null
    lastFinalizeResult.value = null
  }

  const isBusy = computed(() => isStarting.value || isSubmitting.value)

  return {
    // Refs (read-only from callers; mutated via actions)
    sessionId,
    isStarting,
    isSubmitting,
    isBusy,
    errorMessage,
    lastResult,
    lastFinalizeResult,

    // Lifecycle versioning utilities
    isStale,
    invalidate,

    // Actions
    beginSession,
    submitResult,
    resetSession,
  }
}
