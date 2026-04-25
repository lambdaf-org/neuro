import { ApiError, parseJsonBody, request } from '@/lib/auth/http'

export interface GameAsset {
  id: number
  group_id: number
  label: string
  image_url: string
  is_correct: boolean
}

export interface AssetGroup {
  id: number
  game_code: string
  label: string
  assets: GameAsset[]
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null
}

function isGameAsset(value: unknown): value is GameAsset {
  if (!isRecord(value)) {
    return false
  }

  return (
    typeof value.id === 'number' &&
    typeof value.group_id === 'number' &&
    typeof value.label === 'string' &&
    typeof value.image_url === 'string' &&
    typeof value.is_correct === 'boolean'
  )
}

function isAssetGroup(value: unknown): value is AssetGroup {
  if (!isRecord(value)) {
    return false
  }

  return (
    typeof value.id === 'number' &&
    typeof value.game_code === 'string' &&
    typeof value.label === 'string' &&
    Array.isArray(value.assets) &&
    value.assets.every(isGameAsset)
  )
}

function parseAssetGroupsBody(rawBody: string): AssetGroup[] {
  const parsed = parseJsonBody(rawBody)

  if (!Array.isArray(parsed) || !parsed.every(isAssetGroup)) {
    throw new ApiError('Unexpected game assets response from server.', 500)
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

export async function getAssetGroupsByCode(
  gameCode: string,
  accessToken: string,
): Promise<AssetGroup[]> {
  return request<AssetGroup[]>(
    `/api/asset-groups/${encodeURIComponent(gameCode)}`,
    withAuthorization(accessToken, { method: 'GET' }),
    parseAssetGroupsBody,
  )
}
