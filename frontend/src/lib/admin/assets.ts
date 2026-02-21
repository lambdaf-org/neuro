import { ApiError, parseEmptyBody, parseJsonBody, request } from '@/lib/auth/http'

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

export interface AssetGroupPayload {
  game_code: string
  label: string
}

export interface GameAssetPayload {
  group_id: number
  label: string
  image_url: string
  is_correct: boolean
}

export interface UpdateGameAssetPayload {
  label: string
  image_url: string
  is_correct: boolean
}

interface CreatedResource {
  id: string | number
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null
}

function withAuthorization(accessToken: string, init: RequestInit): RequestInit {
  const headers = new Headers(init.headers)
  headers.set('Authorization', `Bearer ${accessToken}`)

  return {
    ...init,
    headers,
  }
}

function parseCreatedResource(rawBody: string): CreatedResource {
  const parsed = parseJsonBody(rawBody)
  if (!isRecord(parsed)) {
    throw new ApiError('Unexpected create response from server.', 500)
  }

  if (typeof parsed.id !== 'string' && typeof parsed.id !== 'number') {
    throw new ApiError('Unexpected create response from server.', 500)
  }

  return { id: parsed.id }
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

  if (
    typeof value.id !== 'number' ||
    typeof value.game_code !== 'string' ||
    typeof value.label !== 'string'
  ) {
    return false
  }

  if (value.assets === undefined) {
    return true
  }

  return Array.isArray(value.assets) && value.assets.every(isGameAsset)
}

function parseGameAssetsBody(rawBody: string): GameAsset[] {
  const parsed = parseJsonBody(rawBody)
  if (!Array.isArray(parsed) || !parsed.every(isGameAsset)) {
    throw new ApiError('Unexpected game assets response from server.', 500)
  }
  return parsed
}

function parseAssetGroupsBody(rawBody: string): AssetGroup[] {
  const parsed = parseJsonBody(rawBody)
  if (!Array.isArray(parsed) || !parsed.every(isAssetGroup)) {
    throw new ApiError('Unexpected asset groups response from server.', 500)
  }

  return parsed.map((group) => ({
    ...group,
    assets: Array.isArray(group.assets) ? group.assets : [],
  }))
}

export async function listAssetGroups(accessToken: string): Promise<AssetGroup[]> {
  return request<AssetGroup[]>(
    '/admin/asset-groups',
    withAuthorization(accessToken, { method: 'GET' }),
    parseAssetGroupsBody,
  )
}

export async function createAssetGroup(
  payload: AssetGroupPayload,
  accessToken: string,
): Promise<CreatedResource> {
  return request<CreatedResource>(
    '/admin/asset-groups',
    withAuthorization(accessToken, {
      method: 'POST',
      body: JSON.stringify(payload),
    }),
    parseCreatedResource,
  )
}

export async function updateAssetGroup(
  id: number,
  payload: AssetGroupPayload,
  accessToken: string,
): Promise<void> {
  await request<void>(
    `/admin/asset-groups/${id}`,
    withAuthorization(accessToken, {
      method: 'PUT',
      body: JSON.stringify(payload),
    }),
    parseEmptyBody,
  )
}

export async function deleteAssetGroup(id: number, accessToken: string): Promise<void> {
  await request<void>(
    `/admin/asset-groups/${id}`,
    withAuthorization(accessToken, { method: 'DELETE' }),
    parseEmptyBody,
  )
}

export async function listGameAssets(groupId: number, accessToken: string): Promise<GameAsset[]> {
  return request<GameAsset[]>(
    `/admin/game-assets/${groupId}`,
    withAuthorization(accessToken, { method: 'GET' }),
    parseGameAssetsBody,
  )
}

export async function createGameAsset(
  payload: GameAssetPayload,
  accessToken: string,
): Promise<CreatedResource> {
  return request<CreatedResource>(
    '/admin/game-assets',
    withAuthorization(accessToken, {
      method: 'POST',
      body: JSON.stringify(payload),
    }),
    parseCreatedResource,
  )
}

export async function updateGameAsset(
  id: number,
  payload: UpdateGameAssetPayload,
  accessToken: string,
): Promise<void> {
  await request<void>(
    `/admin/game-assets/${id}`,
    withAuthorization(accessToken, {
      method: 'PUT',
      body: JSON.stringify(payload),
    }),
    parseEmptyBody,
  )
}

export async function deleteGameAsset(id: number, accessToken: string): Promise<void> {
  await request<void>(
    `/admin/game-assets/${id}`,
    withAuthorization(accessToken, { method: 'DELETE' }),
    parseEmptyBody,
  )
}
