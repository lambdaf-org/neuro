import { ApiError, parseEmptyBody, parseJsonBody, request } from '@/lib/auth/http'
import type { GameResult } from '@/lib/play/result'

interface StartSessionResponse {
  id: string
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null
}

function parseStartSessionBody(rawBody: string): StartSessionResponse {
  const parsed = parseJsonBody(rawBody)

  if (!isRecord(parsed)) {
    throw new ApiError('Unexpected game session response from server.', 500)
  }

  if (typeof parsed.id === 'string') {
    if (!parsed.id.trim()) {
      throw new ApiError('Unexpected game session response from server.', 500)
    }

    return { id: parsed.id }
  }

  if (typeof parsed.id === 'number') {
    return { id: String(parsed.id) }
  }

  throw new ApiError('Unexpected game session response from server.', 500)
}

function withAuthorization(accessToken: string, init: RequestInit): RequestInit {
  const headers = new Headers(init.headers)
  headers.set('Authorization', `Bearer ${accessToken}`)

  return {
    ...init,
    headers,
  }
}

export async function startGameSession(gameCode: string, accessToken: string): Promise<string> {
  const response = await request<StartSessionResponse>(
    `/api/game/${encodeURIComponent(gameCode)}`,
    withAuthorization(accessToken, {
      method: 'POST',
    }),
    parseStartSessionBody,
  )

  return response.id
}

export async function submitGameResult(
  sessionId: string,
  result: GameResult,
  accessToken: string,
): Promise<void> {
  await request<void>(
    `/api/game/session/${encodeURIComponent(sessionId)}`,
    withAuthorization(accessToken, {
      method: 'PATCH',
      body: JSON.stringify({ score: result.score }),
    }),
    parseEmptyBody,
  )
}
