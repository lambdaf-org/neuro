import { ref, toValue, watch, type MaybeRefOrGetter } from 'vue'

import { ApiError } from '@/lib/auth'
import { getGameMetadata, type GameMetadata } from '@/lib/play/metadata'
import { getErrorMessage } from '@/lib/utils/errorHandling'
import { useAuthStore } from '@/stores/auth'

interface UseGameMetadataOptions {
  gameCode: MaybeRefOrGetter<string>
}

export function useGameMetadata(options: UseGameMetadataOptions) {
  const auth = useAuthStore()

  const metadata = ref<GameMetadata | null>(null)
  const isLoading = ref(false)
  const errorMessage = ref('')

  let requestVersion = 0

  function getAccessTokenOrThrow(): string {
    auth.hydrate()

    const token = auth.accessToken
    if (!token) throw new ApiError('No active session. Please sign in again.', 401)
    return token
  }

  async function loadMetadata(gameCode: string): Promise<void> {
    const normalizedGameCode = gameCode.trim().toLowerCase()
    const version = ++requestVersion

    if (!normalizedGameCode) {
      metadata.value = null
      errorMessage.value = ''
      isLoading.value = false
      return
    }

    isLoading.value = true
    errorMessage.value = ''

    try {
      const nextMetadata = await getGameMetadata(normalizedGameCode, getAccessTokenOrThrow())
      if (version !== requestVersion) return

      metadata.value = nextMetadata
    } catch (error) {
      if (version !== requestVersion) return

      metadata.value = null
      errorMessage.value = getErrorMessage(error, 'Could not load game details.')
    } finally {
      if (version === requestVersion) {
        isLoading.value = false
      }
    }
  }

  watch(
    () => toValue(options.gameCode),
    (gameCode) => {
      void loadMetadata(gameCode)
    },
    { immediate: true },
  )

  return {
    metadata,
    isLoading,
    errorMessage,
    reload: async () => loadMetadata(toValue(options.gameCode)),
  }
}
