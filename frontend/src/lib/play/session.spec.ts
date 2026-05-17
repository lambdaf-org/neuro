import { afterEach, describe, expect, it, vi } from 'vitest'

import { ApiError } from '@/lib/auth'
import { GAME_RESULT_SCHEMA_VERSION, type GameResult } from '@/lib/play/result'

import {
  startGameSession,
  submitFluidIntelligenceAnswers,
  submitGameResult,
} from './session'

const token = 'access-token'

function jsonResponse(body: unknown, status = 200): Response {
  return new Response(JSON.stringify(body), {
    status,
    headers: { 'Content-Type': 'application/json' },
  })
}

function gameResult(overrides: Partial<GameResult> = {}): GameResult {
  return {
    schema_version: GAME_RESULT_SCHEMA_VERSION,
    score: 42,
    duration_ms: 1200,
    states: ['countdown', 'running', 'finished'],
    metrics: {},
    ...overrides,
  }
}

afterEach(() => {
  vi.unstubAllGlobals()
})

describe('startGameSession', () => {
  it('posts to the encoded game endpoint with authorization', async () => {
    const fetchMock = vi.fn().mockResolvedValue(jsonResponse({ id: 'session-123' }, 201))
    vi.stubGlobal('fetch', fetchMock)

    await expect(startGameSession('gt', token)).resolves.toBe('session-123')

    expect(fetchMock).toHaveBeenCalledWith(
      '/backend/api/game/gt',
      expect.objectContaining({
        method: 'POST',
        headers: expect.any(Headers),
      }),
    )
    const headers = fetchMock.mock.calls[0]?.[1].headers as Headers
    expect(headers.get('Authorization')).toBe(`Bearer ${token}`)
  })

  it('rejects an empty session id from the server', async () => {
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue(jsonResponse({ id: '' }, 201)))

    await expect(startGameSession('gt', token)).rejects.toEqual(
      expect.objectContaining<ApiError>({
        name: 'ApiError',
        message: 'Unexpected game session response from server.',
        status: 500,
      }),
    )
  })
})

describe('submitGameResult', () => {
  it('submits trial payloads when present', async () => {
    const fetchMock = vi.fn().mockResolvedValue(
      jsonResponse({
        status: 'completed',
        metric_value: 320,
        metrics: { n_valid: 3 },
        scoring_version: 1,
      }),
    )
    vi.stubGlobal('fetch', fetchMock)

    const result = await submitGameResult(
      'session-123',
      gameResult({
        trials: [{ ms: 300 }, { ms: 320 }, { ms: 340 }],
      }),
      token,
    )

    expect(result.metric_value).toBe(320)
    expect(JSON.parse(fetchMock.mock.calls[0]?.[1].body as string)).toEqual({
      trials: [{ ms: 300 }, { ms: 320 }, { ms: 340 }],
    })
  })

  it('falls back to the legacy score payload when no trials are present', async () => {
    const fetchMock = vi.fn().mockResolvedValue(
      jsonResponse({
        status: 'completed',
        metric_value: 42,
        metrics: {},
        scoring_version: 0,
      }),
    )
    vi.stubGlobal('fetch', fetchMock)

    await submitGameResult('session-123', gameResult(), token)

    expect(JSON.parse(fetchMock.mock.calls[0]?.[1].body as string)).toEqual({
      score: 42,
    })
  })
})

describe('submitFluidIntelligenceAnswers', () => {
  it('maps image-choice answers to backend trials and summarizes the scored response', async () => {
    const fetchMock = vi.fn().mockResolvedValue(
      jsonResponse({
        status: 'completed',
        metric_value: 0.6,
        metrics: {
          n_trials: 5,
          misses: 2,
          mean_rt: 1800.4,
        },
        scoring_version: 1,
      }),
    )
    vi.stubGlobal('fetch', fetchMock)

    const summary = await submitFluidIntelligenceAnswers(
      'session-123',
      [
        {
          puzzle_id: 10,
          selected_option_id: 12,
          response_ms: 1500,
        },
      ],
      token,
    )

    expect(JSON.parse(fetchMock.mock.calls[0]?.[1].body as string)).toEqual({
      trials: [
        {
          puzzle_id: 10,
          selected_option_id: 12,
          ms: 1500,
        },
      ],
    })
    expect(summary).toEqual({
      score: 60,
      total_answers: 5,
      correct_answers: 3,
      incorrect_answers: 2,
      average_response_ms: 1800,
    })
  })

  it('throws an ApiError when the server marks the answer set invalid', async () => {
    vi.stubGlobal(
      'fetch',
      vi.fn().mockResolvedValue(
        jsonResponse({
          status: 'invalid',
          metric_value: null,
          metrics: { reason: 'not enough valid trials' },
          scoring_version: 1,
        }),
      ),
    )

    await expect(submitFluidIntelligenceAnswers('session-123', [], token)).rejects.toEqual(
      expect.objectContaining<ApiError>({
        name: 'ApiError',
        message: 'not enough valid trials',
        status: 422,
      }),
    )
  })
})
