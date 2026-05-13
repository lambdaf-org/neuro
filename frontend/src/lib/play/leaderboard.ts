import { ApiError, parseJsonBody, request } from '@/lib/auth/http'

export interface LeaderboardEntry {
  game_code: string
  user_id: string
  username: string
  metric_value: number
  completed_at: string
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null
}

function isLeaderboardEntry(value: unknown): value is LeaderboardEntry {
  if (!isRecord(value)) {
    return false
  }

  return (
    typeof value.game_code === 'string' &&
    typeof value.user_id === 'string' &&
    typeof value.username === 'string' &&
    typeof value.metric_value === 'number' &&
    Number.isFinite(value.metric_value) &&
    typeof value.completed_at === 'string'
  )
}

function parseLeaderboardBody(rawBody: string): LeaderboardEntry[] {
  const parsed = parseJsonBody(rawBody)

  if (!Array.isArray(parsed) || !parsed.every(isLeaderboardEntry)) {
    throw new ApiError('Unexpected leaderboard response from server.', 500)
  }

  return parsed
}

function withAuthorization(accessToken: string, init: RequestInit): RequestInit {
  const headers = new Headers(init.headers)
  headers.set('Authorization', `Bearer ${accessToken}`)

  return {
    ...init,
    headers,
  }
}

export async function getGameLeaderboard(
  gameCode: string,
  accessToken: string,
): Promise<LeaderboardEntry[]> {
  return request<LeaderboardEntry[]>(
    `/api/game/leaderboard/${encodeURIComponent(gameCode)}`,
    withAuthorization(accessToken, {
      method: 'GET',
    }),
    parseLeaderboardBody,
  )
}
