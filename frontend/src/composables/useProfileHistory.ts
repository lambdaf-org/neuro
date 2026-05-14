import { computed, ref } from 'vue'

import { ApiError } from '@/lib/auth'
import { getPlayerStats, type PlayerGameStats } from '@/lib/play/stats'
import { PLAY_MODULES, type GameId, type PlayModule } from '@/lib/play/modules'
import { getErrorMessage } from '@/lib/utils/errorHandling'
import { useAuthStore } from '@/stores/auth'

const PROFILE_GAME_IDS = new Set<GameId>([
  'reaction-time',
  'symbol-matching',
  'pattern-logic',
  'sequence-memory',
  'mental-rotation',
])

const PROFILE_MODULES = PLAY_MODULES.filter((module) => PROFILE_GAME_IDS.has(module.id))

function toBackendGameCode(module: PlayModule): string {
  return module.chcCode.toLowerCase()
}

export interface ProfileGameEntry {
  module: PlayModule
  gameCode: string
  stats: PlayerGameStats | null
}

function createEmptyEntries(): ProfileGameEntry[] {
  return PROFILE_MODULES.map((module) => ({
    module,
    gameCode: toBackendGameCode(module),
    stats: null,
  }))
}

export function useProfileHistory() {
  const auth = useAuthStore()

  const entries = ref<ProfileGameEntry[]>(createEmptyEntries())
  const isLoading = ref(false)
  const errorMessage = ref('')

  let requestVersion = 0

  const totalSessionCount = computed(() =>
    entries.value.reduce((sum, entry) => sum + (entry.stats?.session_count ?? 0), 0),
  )
  const hasSessions = computed(() => totalSessionCount.value > 0)

  function getAccessTokenOrThrow(): string {
    auth.hydrate()

    const token = auth.accessToken
    if (!token) throw new ApiError('No active session. Please sign in again.', 401)
    return token
  }

  async function loadStats(): Promise<void> {
    const version = ++requestVersion

    isLoading.value = true
    errorMessage.value = ''

    try {
      const accessToken = getAccessTokenOrThrow()
      const allStats = await getPlayerStats(accessToken)

      if (version !== requestVersion) return

      const statsByCode = new Map(allStats.map((s) => [s.game_code, s]))

      entries.value = PROFILE_MODULES.map((module) => {
        const gameCode = toBackendGameCode(module)
        return {
          module,
          gameCode,
          stats: statsByCode.get(gameCode) ?? null,
        }
      })
    } catch (error) {
      if (version !== requestVersion) return

      errorMessage.value = getErrorMessage(error, 'Could not load profile stats.')
    } finally {
      if (version === requestVersion) {
        isLoading.value = false
      }
    }
  }

  void loadStats()

  return {
    entries,
    isLoading,
    errorMessage,
    totalSessionCount,
    hasSessions,
    reload: loadStats,
  }
}
