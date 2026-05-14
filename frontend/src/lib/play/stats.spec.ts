import { describe, expect, it, vi, afterEach } from 'vitest'

import { ApiError } from '@/lib/auth'

import { getPlayerStats, type PlayerGameStats } from './stats'

const validStats: PlayerGameStats[] = [
  {
    user_id: '123e4567-e89b-12d3-a456-426614174000',
    username: 'testuser',
    game_code: 'gt',
    latest_metric: 245.5,
    best_metric: 210.3,
    avg_metric: 230.1,
    session_count: 5,
    completed_at: '2026-05-01T12:00:00Z',
  },
  {
    user_id: '123e4567-e89b-12d3-a456-426614174000',
    username: 'testuser',
    game_code: 'gwm',
    latest_metric: 7,
    best_metric: 9,
    avg_metric: 6.8,
    session_count: 3,
    completed_at: '2026-05-02T14:30:00Z',
  },
]

afterEach(() => {
  vi.unstubAllGlobals()
})

describe('getPlayerStats', () => {
  it('returns stats when the response matches the expected shape', async () => {
    vi.stubGlobal(
      'fetch',
      vi.fn().mockResolvedValue(
        new Response(JSON.stringify(validStats), {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        }),
      ),
    )

    await expect(getPlayerStats('token')).resolves.toEqual(validStats)
  })

  it('returns an empty array for a valid empty response', async () => {
    vi.stubGlobal(
      'fetch',
      vi.fn().mockResolvedValue(
        new Response(JSON.stringify([]), {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        }),
      ),
    )

    await expect(getPlayerStats('token')).resolves.toEqual([])
  })

  it('throws an ApiError when the response is not an array', async () => {
    vi.stubGlobal(
      'fetch',
      vi.fn().mockResolvedValue(
        new Response(JSON.stringify({ game_code: 'gt' }), {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        }),
      ),
    )

    await expect(getPlayerStats('token')).rejects.toThrow(ApiError)
  })

  it('throws an ApiError when an item has invalid metric values', async () => {
    const invalidStats = [{ ...validStats[0], best_metric: Infinity }]

    vi.stubGlobal(
      'fetch',
      vi.fn().mockResolvedValue(
        new Response(JSON.stringify(invalidStats), {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        }),
      ),
    )

    await expect(getPlayerStats('token')).rejects.toThrow(ApiError)
  })

  it('throws an ApiError when session_count is not an integer', async () => {
    const invalidStats = [{ ...validStats[0], session_count: 2.5 }]

    vi.stubGlobal(
      'fetch',
      vi.fn().mockResolvedValue(
        new Response(JSON.stringify(invalidStats), {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        }),
      ),
    )

    await expect(getPlayerStats('token')).rejects.toThrow(ApiError)
  })
})
