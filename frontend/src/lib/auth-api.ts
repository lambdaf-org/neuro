const API_BASE_URL = (import.meta.env.VITE_API_BASE_URL || '/backend').replace(/\/$/, '')

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
}

export interface StoredSession {
  userId: string
  email: string
  accessToken: string
  savedAt: string
}

export class ApiError extends Error {
  status: number

  constructor(message: string, status: number) {
    super(message)
    this.name = 'ApiError'
    this.status = status
  }
}

type ParseJsonResult = {
  ok: boolean
  value: unknown
}

function parseJson(rawBody: string): ParseJsonResult {
  try {
    return { ok: true, value: JSON.parse(rawBody) as unknown }
  } catch {
    return { ok: false, value: null }
  }
}

function parseErrorMessage(rawBody: string, fallback: string): string {
  const parsed = parseJson(rawBody)
  if (!parsed.ok || !parsed.value || typeof parsed.value !== 'object') {
    return fallback
  }

  const body = parsed.value as Record<string, unknown>
  if (typeof body.error === 'string' && body.error.trim()) {
    return body.error
  }

  if (Array.isArray(body.errors)) {
    const messages = body.errors.filter((entry): entry is string => typeof entry === 'string')
    if (messages.length) {
      return messages.join(', ')
    }
  }

  return fallback
}

async function request<T>(path: string, init: RequestInit): Promise<T> {
  const response = await fetch(`${API_BASE_URL}${path}`, {
    ...init,
    headers: {
      'Content-Type': 'application/json',
      ...init.headers,
    },
  })

  const rawBody = await response.text()
  const parsedBody = parseJson(rawBody)

  if (!response.ok) {
    const fallback = rawBody || `Request failed with status ${response.status}`
    throw new ApiError(parseErrorMessage(rawBody, fallback), response.status)
  }

  if (!rawBody) {
    return undefined as T
  }

  if (parsedBody.ok) {
    return parsedBody.value as T
  }

  return rawBody as T
}

export async function register(payload: RegisterPayload): Promise<void> {
  await request<void>('/register', {
    method: 'POST',
    body: JSON.stringify(payload),
  })
}

export async function login(payload: LoginPayload): Promise<LoginResponse> {
  return request<LoginResponse>('/login', {
    method: 'POST',
    body: JSON.stringify(payload),
  })
}

export function saveSession(session: LoginResponse): void {
  const persisted: StoredSession = {
    userId: session.user_id,
    email: session.email,
    accessToken: session.access_token,
    savedAt: new Date().toISOString(),
  }

  localStorage.setItem('neuro.session', JSON.stringify(persisted))
}
