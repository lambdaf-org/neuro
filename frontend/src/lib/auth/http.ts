const API_BASE_URL = (import.meta.env.VITE_API_BASE_URL || '/backend').replace(/\/$/, '')

export class ApiError extends Error {
  status: number
  cause?: unknown

  constructor(message: string, status: number, cause?: unknown) {
    super(message)
    this.name = 'ApiError'
    this.status = status
    this.cause = cause
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
  if (typeof body.message === 'string' && body.message.trim()) {
    return body.message
  }

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

type ResponseParser<T> = (rawBody: string) => T

function buildHeaders(init: RequestInit): HeadersInit {
  const headers = new Headers(init.headers)

  if (!headers.has('Accept')) {
    headers.set('Accept', 'application/json')
  }

  if (init.body && !headers.has('Content-Type')) {
    headers.set('Content-Type', 'application/json')
  }

  return headers
}

export async function request<T>(path: string, init: RequestInit, parse: ResponseParser<T>): Promise<T> {
  let response: Response

  try {
    response = await fetch(`${API_BASE_URL}${path}`, {
      ...init,
      headers: buildHeaders(init),
    })
  } catch (error) {
    throw new ApiError('Network error. Please check your connection and try again.', 0, error)
  }

  const rawBody = await response.text()

  if (!response.ok) {
    const fallback = rawBody || `Request failed with status ${response.status}`
    throw new ApiError(parseErrorMessage(rawBody, fallback), response.status)
  }

  return parse(rawBody)
}

export function parseJsonBody(rawBody: string): unknown {
  const parsed = parseJson(rawBody)
  if (!parsed.ok) {
    throw new ApiError('Invalid JSON response from server.', 500)
  }
  return parsed.value
}
