import { computed, ref } from 'vue'

import { ApiError } from '@/lib/auth'
import { getRecentGameSessions, type RecentGameSession } from '@/lib/play/history'
import { PLAY_MODULES, type GameId, type PlayModule } from '@/lib/play/modules'
import { getErrorMessage } from '@/lib/utils/errorHandling'
import { useAuthStore } from '@/stores/auth'

const PROFILE_HISTORY_GAME_IDS = new Set<GameId>([
  'reaction-time',
  'symbol-matching',
  'pattern-logic',
  'sequence-memory',
])

const PROFILE_HISTORY_MODULES = PLAY_MODULES.filter((module) =>
  PROFILE_HISTORY_GAME_IDS.has(module.id),
)

export interface ProfileGameHistory {
  module: PlayModule
  gameCode: string
  sessions: RecentGameSession[]
}

function toBackendGameCode(module: PlayModule): string {
  return module.chcCode.toLowerCase()
}

function createEmptyHistory(): ProfileGameHistory[] {
  return PROFILE_HISTORY_MODULES.map((module) => ({
    module,
    gameCode: toBackendGameCode(module),
    sessions: [],
  }))
}

export function useProfileHistory() {
  const auth = useAuthStore()

  const histories = ref<ProfileGameHistory[]>(createEmptyHistory())
  const isLoading = ref(false)
  const errorMessage = ref('')

  let requestVersion = 0

  const totalSessionCount = computed(() =>
    histories.value.reduce((count, history) => count + history.sessions.length, 0),
  )
  const hasSessions = computed(() => totalSessionCount.value > 0)

  function getAccessTokenOrThrow(): string {
    auth.hydrate()

    const token = auth.accessToken
    if (!token) throw new ApiError('No active session. Please sign in again.', 401)
    return token
  }

  async function loadHistory(): Promise<void> {
    const version = ++requestVersion

    isLoading.value = true
    errorMessage.value = ''

    try {
      const accessToken = getAccessTokenOrThrow()
      const nextHistories = await Promise.all(
        PROFILE_HISTORY_MODULES.map(async (module) => {
          const gameCode = toBackendGameCode(module)

          return {
            module,
            gameCode,
            sessions: await getRecentGameSessions(gameCode, accessToken),
          }
        }),
      )

      if (version !== requestVersion) return

      histories.value = nextHistories
    } catch (error) {
      if (version !== requestVersion) return

      errorMessage.value = getErrorMessage(error, 'Could not load profile history.')
    } finally {
      if (version === requestVersion) {
        isLoading.value = false
      }
    }
  }

  void loadHistory()

  return {
    histories,
    isLoading,
    errorMessage,
    totalSessionCount,
    hasSessions,
    reload: loadHistory,
  }
}
