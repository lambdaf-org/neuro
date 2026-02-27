import { ref } from 'vue'

import { ApiError } from '@/lib/auth'
import type { GameResult } from '@/lib/play/result'
import { startGameSession, submitGameResult } from '@/lib/play/session'
import { getErrorMessage } from '@/lib/utils/errorHandling'
import { useAuthStore } from '@/stores/auth'

export function useGameSubmit() {
  const auth = useAuthStore()

  const sessionId = ref<string | null>(null)
  const isStarting = ref(false)
  const isSubmitting = ref(false)
  const errorMessage = ref('')

  function getAccessTokenOrThrow(): string {
    const accessToken = auth.accessToken
    if (!accessToken) {
      throw new ApiError('No active session. Please sign in again.', 401)
    }

    return accessToken
  }

  async function startSession(gameCode: string): Promise<void> {
    isStarting.value = true
    errorMessage.value = ''

    try {
      sessionId.value = await startGameSession(gameCode, getAccessTokenOrThrow())
    } catch (error) {
      sessionId.value = null
      errorMessage.value = getErrorMessage(error, 'Could not start game session.')
      throw error
    } finally {
      isStarting.value = false
    }
  }

  async function submitResult(result: GameResult): Promise<void> {
    const currentSessionId = sessionId.value
    if (!currentSessionId) {
      throw new ApiError('No active game session to submit.', 400)
    }

    isSubmitting.value = true
    errorMessage.value = ''

    try {
      await submitGameResult(currentSessionId, result, getAccessTokenOrThrow())
    } catch (error) {
      errorMessage.value = getErrorMessage(error, 'Could not submit game result.')
      throw error
    } finally {
      isSubmitting.value = false
    }
  }

  function resetSession(): void {
    sessionId.value = null
    errorMessage.value = ''
  }

  return {
    sessionId,
    isStarting,
    isSubmitting,
    errorMessage,
    startSession,
    submitResult,
    resetSession,
  }
}
