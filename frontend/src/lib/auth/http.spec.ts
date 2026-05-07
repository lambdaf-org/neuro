import { afterEach, describe, expect, it, vi } from 'vitest'

import { ApiError, parseJsonBody, request, setUnauthorizedHandler } from './http'

afterEach(() => {
  setUnauthorizedHandler(null)
  vi.unstubAllGlobals()
})

describe('request', () => {
  it('notifies the unauthorized handler for authenticated 401 responses', async () => {
    const onUnauthorized = vi.fn()
    setUnauthorizedHandler(onUnauthorized)
    vi.stubGlobal(
      'fetch',
      vi.fn().mockResolvedValue(
        new Response(JSON.stringify({ message: 'JWT expired' }), {
          status: 401,
          headers: { 'Content-Type': 'application/json' },
        }),
      ),
    )

    await expect(
      request(
        '/api/game/stats',
        {
          method: 'GET',
          headers: { Authorization: 'Bearer expired-token' },
        },
        parseJsonBody,
      ),
    ).rejects.toThrow(ApiError)

    expect(onUnauthorized).toHaveBeenCalledOnce()
  })

  it('does not notify the unauthorized handler for unauthenticated 401 responses', async () => {
    const onUnauthorized = vi.fn()
    setUnauthorizedHandler(onUnauthorized)
    vi.stubGlobal(
      'fetch',
      vi.fn().mockResolvedValue(
        new Response(JSON.stringify({ message: 'Invalid credentials' }), {
          status: 401,
          headers: { 'Content-Type': 'application/json' },
        }),
      ),
    )

    await expect(
      request('/login', { method: 'POST', body: JSON.stringify({}) }, parseJsonBody),
    ).rejects.toThrow(ApiError)

    expect(onUnauthorized).not.toHaveBeenCalled()
  })

  it('still throws the ApiError when the unauthorized handler rejects', async () => {
    setUnauthorizedHandler(vi.fn().mockRejectedValue(new Error('Navigation failed')))
    vi.stubGlobal(
      'fetch',
      vi.fn().mockResolvedValue(
        new Response(JSON.stringify({ message: 'JWT expired' }), {
          status: 401,
          headers: { 'Content-Type': 'application/json' },
        }),
      ),
    )

    await expect(
      request(
        '/api/game/stats',
        {
          method: 'GET',
          headers: { Authorization: 'Bearer expired-token' },
        },
        parseJsonBody,
      ),
    ).rejects.toThrow(ApiError)
  })
})
