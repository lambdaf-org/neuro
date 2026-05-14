import { computed, ref } from 'vue'
import { useRouter } from 'vue-router'

import { ApiError } from '@/lib/auth'
import { type GameResult } from '@/lib/play/result'
import {
  openGameEventsSocket,
  startGameSession,
  submitGameResult,
  type AnticheatVerdictFrame,
  type FinalizeSessionResult,
} from '@/lib/play/session'
import { getErrorMessage } from '@/lib/utils/errorHandling'
import { useAuthStore } from '@/stores/auth'

function isBanError(error: unknown): boolean {
  return error instanceof ApiError && error.status === 403 && /suspend|banned/i.test(error.message)
}

interface AnticheatFramePayload {
  message_type?: unknown
  action?: unknown
  flags?: unknown
}

function parseAnticheatFrame(raw: string): AnticheatVerdictFrame | null {
  let parsed: AnticheatFramePayload
  try {
    parsed = JSON.parse(raw) as AnticheatFramePayload
  } catch {
    return null
  }

  if (parsed.message_type !== 'anticheat') return null
  const action = parsed.action
  if (action !== 'flag' && action !== 'ban') return null

  const flagsContainer =
    parsed.flags && typeof parsed.flags === 'object'
      ? (parsed.flags as { flags?: unknown }).flags
      : undefined
  const flagsArray = Array.isArray(flagsContainer) ? flagsContainer : []
  const flags = flagsArray
    .filter((f): f is { code: unknown; reason: unknown } => typeof f === 'object' && f !== null)
    .map((f) => ({
      code: typeof f.code === 'string' ? f.code : '',
      reason: typeof f.reason === 'string' ? f.reason : '',
    }))

  return { action, flags }
}

export function useGameSession() {
  const auth = useAuthStore()
  const router = useRouter()

  function escalateBan(reason: string): void {
    isBanned.value = true
    errorMessage.value = reason
    auth.flagBanned(reason)
    closeSocket()
    void router.replace({ name: 'banned' })
  }

  const sessionId = ref<string | null>(null)
  const isStarting = ref(false)
  const isSubmitting = ref(false)
  const errorMessage = ref('')
  const isBanned = ref(false)
  const lastResult = ref<GameResult | null>(null)
  const lastFinalizeResult = ref<FinalizeSessionResult | null>(null)

  let lifecycleVersion = 0
  let socket: WebSocket | null = null

  function closeSocket(): void {
    if (socket) {
      try {
        socket.close()
      } catch {
        // ignore
      }
      socket = null
    }
  }

  function handleAnticheatVerdict(verdict: AnticheatVerdictFrame): void {
    if (verdict.action === 'ban') {
      const reason = verdict.flags[0]?.reason
      escalateBan(
        reason
          ? `Anti-cheat ${verdict.flags[0]?.code}: ${reason}`
          : 'Account suspended due to anti-cheat violation.',
      )
    } else {
      const reason = verdict.flags[0]?.reason
      if (reason) {
        errorMessage.value = `Anti-cheat warning (${verdict.flags[0]?.code}): ${reason}`
      }
    }
  }

  function openSocket(id: string): void {
    closeSocket()
    const token = auth.accessToken
    if (!token) return

    const ws = openGameEventsSocket(id, token)
    socket = ws

    ws.addEventListener('message', (ev) => {
      if (typeof ev.data !== 'string') return
      const verdict = parseAnticheatFrame(ev.data)
      if (verdict) handleAnticheatVerdict(verdict)
    })

    ws.addEventListener('close', () => {
      if (socket === ws) socket = null
    })

    ws.addEventListener('error', (ev) => {
      console.warn('events ws error', ev)
    })
  }

  function sendRoundEvent(round: number, eventValue: number, correct?: boolean): void {
    if (!socket || socket.readyState !== WebSocket.OPEN) return
    const payload: Record<string, unknown> = {
      round,
      event_value: eventValue,
      client_ts: new Date().toISOString(),
    }
    if (correct !== undefined) payload.correct = correct
    socket.send(JSON.stringify(payload))
  }

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
      openSocket(id)
      return { version, ok: true }
    } catch (error) {
      if (isStale(version)) return { version, ok: false }

      const message = getErrorMessage(error, 'Could not start game session.')
      if (isBanError(error)) {
        escalateBan(message)
      } else {
        errorMessage.value = message
      }
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
      const message = getErrorMessage(error, 'Could not submit game result.')
      if (isBanError(error)) {
        escalateBan(message)
      } else {
        errorMessage.value = message
      }
    } finally {
      if (!isStale(version)) isSubmitting.value = false
    }
  }

  function resetSession(): void {
    invalidate()
    closeSocket()
    sessionId.value = null
    isStarting.value = false
    isSubmitting.value = false
    // Preserve isBanned across resetSession — a ban survives the round.
    if (!isBanned.value) {
      errorMessage.value = ''
    }
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
    isBanned,
    lastResult,
    lastFinalizeResult,

    // Lifecycle versioning utilities
    isStale,
    invalidate,

    // Actions
    beginSession,
    submitResult,
    resetSession,
    sendRoundEvent,
  }
}
