import { ApiError, parseJsonBody, request } from '@/lib/auth/http'

export interface GameMetadata {
  game_code: string
  display_name: string
  chc_factor: string
  cognitive_domain: string
  description: string
  scientific_basis: string
  task_summary: string
  metric_name: string
  metric_direction: string
  icon_url: string | null
  sort_order: number
  is_active: boolean
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null
}

function isGameMetadata(value: unknown): value is GameMetadata {
  if (!isRecord(value)) {
    return false
  }

  return (
    typeof value.game_code === 'string' &&
    typeof value.display_name === 'string' &&
    typeof value.chc_factor === 'string' &&
    typeof value.cognitive_domain === 'string' &&
    typeof value.description === 'string' &&
    typeof value.scientific_basis === 'string' &&
    typeof value.task_summary === 'string' &&
    typeof value.metric_name === 'string' &&
    typeof value.metric_direction === 'string' &&
    (typeof value.icon_url === 'string' || value.icon_url === null) &&
    typeof value.sort_order === 'number' &&
    Number.isFinite(value.sort_order) &&
    typeof value.is_active === 'boolean'
  )
}

function parseGameMetadataBody(rawBody: string): GameMetadata {
  const parsed = parseJsonBody(rawBody)

  if (!isGameMetadata(parsed)) {
    throw new ApiError('Unexpected game metadata response from server.', 500)
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

export async function getGameMetadata(
  gameCode: string,
  accessToken: string,
): Promise<GameMetadata> {
  return request<GameMetadata>(
    `/api/game/metadata/${encodeURIComponent(gameCode)}`,
    withAuthorization(accessToken, {
      method: 'GET',
    }),
    parseGameMetadataBody,
  )
}
