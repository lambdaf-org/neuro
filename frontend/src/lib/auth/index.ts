import type { LoginPayload, LoginResponse, RegisterPayload, StoredSession } from './types'
import { ApiError, parseEmptyBody, parseJsonBody, request } from './http'
import { isLoginResponse, isStoredSession } from './types'

export { ApiError } from './http'
export type { LoginPayload, LoginResponse, RegisterPayload, StoredSession } from './types'
export { isLoginResponse, isStoredSession } from './types'

const SESSION_STORAGE_KEY = 'neuro.session'

function parseLoginBody(rawBody: string): LoginResponse {
  const parsed = parseJsonBody(rawBody)
  if (!isLoginResponse(parsed)) {
    throw new ApiError('Unexpected login response from server.', 500)
  }
  return parsed
}

export async function register(payload: RegisterPayload): Promise<void> {
  await request<void>(
    '/register',
    {
      method: 'POST',
      body: JSON.stringify(payload),
    },
    parseEmptyBody,
  )
}

export async function login(payload: LoginPayload): Promise<LoginResponse> {
  return request<LoginResponse>(
    '/login',
    {
      method: 'POST',
      body: JSON.stringify(payload),
    },
    parseLoginBody,
  )
}

export function saveSession(session: LoginResponse): void {
  const persisted: StoredSession = {
    userId: session.user_id,
    email: session.email,
    accessToken: session.access_token,
    savedAt: new Date().toISOString(),
  }

  localStorage.setItem(SESSION_STORAGE_KEY, JSON.stringify(persisted))
}

export function getSession(): StoredSession | null {
  try {
    const raw = localStorage.getItem(SESSION_STORAGE_KEY)
    if (!raw) {
      return null
    }

    const parsed: unknown = JSON.parse(raw)
    if (!isStoredSession(parsed)) {
      localStorage.removeItem(SESSION_STORAGE_KEY)
      return null
    }
    return parsed
  } catch {
    // Handle both JSON parsing errors and storage access failures (SecurityError, etc.)
    try {
      localStorage.removeItem(SESSION_STORAGE_KEY)
    } catch {
      // Ignore if we can't even access storage to clean up
    }
    return null
  }
}

export function clearSession(): void {
  localStorage.removeItem(SESSION_STORAGE_KEY)
}
