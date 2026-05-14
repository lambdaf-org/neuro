export interface LoginPayload {
  email: string
  password: string
}

export interface RegisterPayload {
  email: string
  username: string
  password: string
}

export interface LoginResponse {
  user_id: string
  email: string
  access_token: string
  is_banned?: boolean
}

export interface StoredSession {
  userId: string
  email: string
  accessToken: string
  savedAt: string
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null
}

export function isLoginResponse(value: unknown): value is LoginResponse {
  if (!isRecord(value)) {
    return false
  }

  return (
    typeof value.user_id === 'string' &&
    typeof value.email === 'string' &&
    typeof value.access_token === 'string'
  )
}

export function isStoredSession(value: unknown): value is StoredSession {
  if (!isRecord(value)) {
    return false
  }

  return (
    typeof value.userId === 'string' &&
    typeof value.email === 'string' &&
    typeof value.accessToken === 'string' &&
    typeof value.savedAt === 'string'
  )
}
