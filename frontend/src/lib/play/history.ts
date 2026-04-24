import { ApiError, parseJsonBody, request } from '@/lib/auth/http'

export interface RecentGameSession {
  score: number
  completed_at: string
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null
}

function isRecentGameSession(value: unknown): value is RecentGameSession {
  if (!isRecord(value)) {
    return false
  }

  return (
    typeof value.score === 'number' &&
    Number.isFinite(value.score) &&
    typeof value.completed_at === 'string'
  )
}

function parseRecentSessionsBody(rawBody: string): RecentGameSession[] {
  const parsed = parseJsonBody(rawBody)

  if (!Array.isArray(parsed) || !parsed.every(isRecentGameSession)) {
    throw new ApiError('Unexpected game history response from server.', 500)
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

export async function getRecentGameSessions(
  gameCode: string,
  accessToken: string,
): Promise<RecentGameSession[]> {
  return request<RecentGameSession[]>(
    `/api/game/${encodeURIComponent(gameCode)}/recent`,
    withAuthorization(accessToken, {
      method: 'GET',
    }),
    parseRecentSessionsBody,
  )
}
