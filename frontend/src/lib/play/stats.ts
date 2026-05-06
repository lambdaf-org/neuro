import { ApiError, parseJsonBody, request } from '@/lib/auth/http'

export interface PlayerGameStats {
  user_id: string
  username: string
  game_code: string
  latest_metric: number
  best_metric: number
  avg_metric: number
  session_count: number
  completed_at: string | null
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null
}

function isPlayerGameStats(value: unknown): value is PlayerGameStats {
  if (!isRecord(value)) return false

  return (
    typeof value.game_code === 'string' &&
    typeof value.latest_metric === 'number' &&
    Number.isFinite(value.latest_metric) &&
    typeof value.best_metric === 'number' &&
    Number.isFinite(value.best_metric) &&
    typeof value.avg_metric === 'number' &&
    Number.isFinite(value.avg_metric) &&
    typeof value.session_count === 'number' &&
    Number.isInteger(value.session_count)
  )
}

function parsePlayerStatsBody(rawBody: string): PlayerGameStats[] {
  const parsed = parseJsonBody(rawBody)

  if (!Array.isArray(parsed) || !parsed.every(isPlayerGameStats)) {
    throw new ApiError('Unexpected player stats response from server.', 500)
  }

  return parsed
}

function withAuthorization(accessToken: string, init: RequestInit): RequestInit {
  const headers = new Headers(init.headers)
  headers.set('Authorization', `Bearer ${accessToken}`)
  return { ...init, headers }
}

export async function getPlayerStats(accessToken: string): Promise<PlayerGameStats[]> {
  return request<PlayerGameStats[]>(
    '/api/game/stats',
    withAuthorization(accessToken, { method: 'GET' }),
    parsePlayerStatsBody,
  )
}
