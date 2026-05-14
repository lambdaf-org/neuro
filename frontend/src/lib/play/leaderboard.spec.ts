import { afterEach, describe, expect, it, vi } from 'vitest'

import { ApiError } from '@/lib/auth'

import { getGameLeaderboard, type LeaderboardEntry } from './leaderboard'

const validEntries: LeaderboardEntry[] = [
  {
    game_code: 'gt',
    user_id: '123e4567-e89b-12d3-a456-426614174000',
    username: 'testuser',
    metric_value: 210.3,
    completed_at: '2026-05-01T12:00:00Z',
  },
  {
    game_code: 'gt',
    user_id: '123e4567-e89b-12d3-a456-426614174001',
    username: 'runnerup',
    metric_value: 245.5,
    completed_at: '2026-05-02T14:30:00Z',
  },
]

afterEach(() => {
  vi.unstubAllGlobals()
})

describe('getGameLeaderboard', () => {
  it('returns leaderboard entries when the response matches the expected shape', async () => {
    vi.stubGlobal(
      'fetch',
      vi.fn().mockResolvedValue(
        new Response(JSON.stringify(validEntries), {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        }),
      ),
    )

    await expect(getGameLeaderboard('gt', 'token')).resolves.toEqual(validEntries)
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

    await expect(getGameLeaderboard('gt', 'token')).resolves.toEqual([])
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

    await expect(getGameLeaderboard('gt', 'token')).rejects.toThrow(ApiError)
  })

  it('throws an ApiError when an entry has an invalid metric value', async () => {
    const invalidEntries = [{ ...validEntries[0], metric_value: Infinity }]

    vi.stubGlobal(
      'fetch',
      vi.fn().mockResolvedValue(
        new Response(JSON.stringify(invalidEntries), {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        }),
      ),
    )

    await expect(getGameLeaderboard('gt', 'token')).rejects.toThrow(ApiError)
  })
})
