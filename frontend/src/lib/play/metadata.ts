import { parseJsonBody, request } from '@/lib/auth/http'

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

function parseGameMetadataBody(rawBody: string): GameMetadata {
  return parseJsonBody(rawBody) as GameMetadata
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
