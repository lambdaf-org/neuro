import { computed, ref, toValue, watch, type MaybeRefOrGetter } from 'vue'

import { ApiError } from '@/lib/auth'
import { getGameLeaderboard, type LeaderboardEntry } from '@/lib/play/leaderboard'
import { findPlayModuleById, type GameId } from '@/lib/play/modules'
import { getErrorMessage } from '@/lib/utils/errorHandling'
import { useAuthStore } from '@/stores/auth'

interface UseLeaderboardOptions {
  gameId: MaybeRefOrGetter<GameId>
}

function toBackendGameCode(gameId: GameId): string {
  return findPlayModuleById(gameId)?.chcCode.toLowerCase() ?? ''
}

export function useLeaderboard(options: UseLeaderboardOptions) {
  const auth = useAuthStore()

  const entries = ref<LeaderboardEntry[]>([])
  const isLoading = ref(false)
  const errorMessage = ref('')

  let requestVersion = 0

  const selectedModule = computed(() => findPlayModuleById(toValue(options.gameId)))
  const hasEntries = computed(() => entries.value.length > 0)

  function getAccessTokenOrThrow(): string {
    auth.hydrate()

    const token = auth.accessToken
    if (!token) throw new ApiError('No active session. Please sign in again.', 401)
    return token
  }

  async function loadLeaderboard(gameId: GameId): Promise<void> {
    const version = ++requestVersion
    const gameCode = toBackendGameCode(gameId)

    if (!gameCode) {
      entries.value = []
      errorMessage.value = ''
      isLoading.value = false
      return
    }

    isLoading.value = true
    errorMessage.value = ''

    try {
      const nextEntries = await getGameLeaderboard(gameCode, getAccessTokenOrThrow())
      if (version !== requestVersion) return

      entries.value = nextEntries
    } catch (error) {
      if (version !== requestVersion) return

      entries.value = []
      errorMessage.value = getErrorMessage(error, 'Could not load leaderboard.')
    } finally {
      if (version === requestVersion) {
        isLoading.value = false
      }
    }
  }

  watch(
    () => toValue(options.gameId),
    (gameId) => {
      void loadLeaderboard(gameId)
    },
    { immediate: true },
  )

  return {
    entries,
    isLoading,
    errorMessage,
    selectedModule,
    hasEntries,
    reload: async () => loadLeaderboard(toValue(options.gameId)),
  }
}
