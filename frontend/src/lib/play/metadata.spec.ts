import { describe, expect, it, vi, afterEach } from 'vitest'

import { ApiError } from '@/lib/auth'

import { getGameMetadata, type GameMetadata } from './metadata'

const validMetadata: GameMetadata = {
  game_code: 'rt',
  display_name: 'Reaction Time',
  chc_factor: 'Gs',
  cognitive_domain: 'Processing Speed',
  description: 'Measure simple reaction speed.',
  scientific_basis: 'Classic reaction time task.',
  task_summary: 'Wait, then react quickly.',
  metric_name: 'Median reaction time',
  metric_direction: 'lower_is_better',
  icon_url: null,
  sort_order: 1,
  is_active: true,
}

afterEach(() => {
  vi.unstubAllGlobals()
})

describe('getGameMetadata', () => {
  it('returns metadata when the response matches the expected shape', async () => {
    vi.stubGlobal(
      'fetch',
      vi.fn().mockResolvedValue(
        new Response(JSON.stringify(validMetadata), {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        }),
      ),
    )

    await expect(getGameMetadata('rt', 'token')).resolves.toEqual(validMetadata)
  })

  it('throws an ApiError when the response shape is invalid', async () => {
    vi.stubGlobal(
      'fetch',
      vi.fn().mockResolvedValue(
        new Response(JSON.stringify({ display_name: 'Reaction Time' }), {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        }),
      ),
    )

    await expect(getGameMetadata('rt', 'token')).rejects.toEqual(
      expect.objectContaining<ApiError>({
        message: 'Unexpected game metadata response from server.',
        name: 'ApiError',
        status: 500,
      }),
    )
  })
})
