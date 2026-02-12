const API_BASE_URL = (import.meta.env.VITE_API_BASE_URL || '/backend').replace(/\/$/, '')

export interface LoginPayload {
  email: string
  password: string
}

export interface RegisterPayload {
  email: string
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

async function request<T>(path: string, init: RequestInit): Promise<T> {
  const response = await fetch(`${API_BASE_URL}${path}`, {
    ...init,
    headers: {
      'Content-Type': 'application/json',
      ...init.headers,
    },
  })

  const rawBody = await response.text()
  if (!response.ok) {
    throw new ApiError(rawBody || `Request failed with status ${response.status}`, response.status)
  }

  if (!rawBody) {
    return undefined as T
  }

  return JSON.parse(rawBody) as T
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
