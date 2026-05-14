import { ApiError, parseJsonBody, request } from '@/lib/auth/http'
import type { GameResult } from '@/lib/play/result'

const WS_BASE_URL = (() => {
  const explicit = import.meta.env.VITE_WS_BASE_URL as string | undefined
  if (explicit) return explicit.replace(/\/$/, '')

  const apiBase = (import.meta.env.VITE_API_BASE_URL as string | undefined) || '/backend'
  if (typeof window === 'undefined') return apiBase

  const url = new URL(apiBase, window.location.origin)
  url.protocol = url.protocol === 'https:' ? 'wss:' : 'ws:'
  return url.toString().replace(/\/$/, '')
})()

export interface AnticheatVerdictFrame {
  action: 'flag' | 'ban'
  flags: Array<{ code: string; reason: string }>
}

export function openGameEventsSocket(sessionId: string, accessToken: string): WebSocket {
  const url = `${WS_BASE_URL}/api/game/session/${encodeURIComponent(sessionId)}/events/ws?token=${encodeURIComponent(accessToken)}`
  return new WebSocket(url)
}

interface StartSessionResponse {
  id: string
}

export interface FluidIntelligenceAnswerSubmission {
  puzzle_id: number
  selected_option_id: number
  response_ms: number
}

export type MentalRotationAnswerSubmission = FluidIntelligenceAnswerSubmission

export interface FinalizeSessionResult {
  status: string
  metric_value: number | null
  metrics: Record<string, unknown>
  scoring_version: number
}

export interface FluidIntelligenceSubmissionResult {
  score: number
  total_answers: number
  correct_answers: number
  incorrect_answers: number
  average_response_ms: number
}

export type MentalRotationSubmissionResult = FluidIntelligenceSubmissionResult

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null
}

function numberMetric(metrics: Record<string, unknown>, key: string, fallback = 0): number {
  const value = metrics[key]
  return typeof value === 'number' && Number.isFinite(value) ? value : fallback
}

function parseStartSessionBody(rawBody: string): StartSessionResponse {
  const parsed = parseJsonBody(rawBody)

  if (!isRecord(parsed)) {
    throw new ApiError('Unexpected game session response from server.', 500)
  }

  if (typeof parsed.id === 'string') {
    if (!parsed.id.trim()) {
      throw new ApiError('Unexpected game session response from server.', 500)
    }

    return { id: parsed.id }
  }

  if (typeof parsed.id === 'number') {
    return { id: String(parsed.id) }
  }

  throw new ApiError('Unexpected game session response from server.', 500)
}

function parseFinalizeSessionResultBody(rawBody: string): FinalizeSessionResult {
  const parsed = parseJsonBody(rawBody)

  if (
    !isRecord(parsed) ||
    typeof parsed.status !== 'string' ||
    (typeof parsed.metric_value !== 'number' && parsed.metric_value !== null) ||
    !isRecord(parsed.metrics) ||
    typeof parsed.scoring_version !== 'number'
  ) {
    throw new ApiError('Unexpected game session finalize response from server.', 500)
  }

  return {
    status: parsed.status,
    metric_value: parsed.metric_value,
    metrics: parsed.metrics,
    scoring_version: parsed.scoring_version,
  }
}

function withAuthorization(accessToken: string, init: RequestInit): RequestInit {
  const headers = new Headers(init.headers)
  headers.set('Authorization', `Bearer ${accessToken}`)

  return {
    ...init,
    headers,
  }
}

export async function startGameSession(gameCode: string, accessToken: string): Promise<string> {
  const response = await request<StartSessionResponse>(
    `/api/game/${encodeURIComponent(gameCode)}`,
    withAuthorization(accessToken, {
      method: 'POST',
    }),
    parseStartSessionBody,
  )

  return response.id
}

export async function submitGameResult(
  sessionId: string,
  result: GameResult,
  accessToken: string,
): Promise<FinalizeSessionResult> {
  return request<FinalizeSessionResult>(
    `/api/game/session/${encodeURIComponent(sessionId)}`,
    withAuthorization(accessToken, {
      method: 'PATCH',
      body: JSON.stringify(
        result.trials?.length
          ? {
              trials: result.trials,
            }
          : {
              score: result.score,
            },
      ),
    }),
    parseFinalizeSessionResultBody,
  )
}

export async function submitFluidIntelligenceAnswers(
  sessionId: string,
  answers: FluidIntelligenceAnswerSubmission[],
  accessToken: string,
): Promise<FluidIntelligenceSubmissionResult> {
  return submitImageChoiceAnswers(
    sessionId,
    answers,
    accessToken,
    'Pattern Logic result was not scored.',
  )
}

export async function submitMentalRotationAnswers(
  sessionId: string,
  answers: MentalRotationAnswerSubmission[],
  accessToken: string,
): Promise<MentalRotationSubmissionResult> {
  return submitImageChoiceAnswers(
    sessionId,
    answers,
    accessToken,
    'Mental Rotation result was not scored.',
  )
}

async function submitImageChoiceAnswers(
  sessionId: string,
  answers: FluidIntelligenceAnswerSubmission[],
  accessToken: string,
  unscoredMessage: string,
): Promise<FluidIntelligenceSubmissionResult> {
  const result = await request<FinalizeSessionResult>(
    `/api/game/session/${encodeURIComponent(sessionId)}`,
    withAuthorization(accessToken, {
      method: 'PATCH',
      body: JSON.stringify({
        trials: answers.map((answer) => ({
          puzzle_id: answer.puzzle_id,
          selected_option_id: answer.selected_option_id,
          ms: answer.response_ms,
        })),
      }),
    }),
    parseFinalizeSessionResultBody,
  )

  if (result.status !== 'completed' || result.metric_value === null) {
    const reason =
      typeof result.metrics.reason === 'string' ? result.metrics.reason : unscoredMessage
    throw new ApiError(reason, 422)
  }

  const totalAnswers = Math.max(
    0,
    Math.round(numberMetric(result.metrics, 'n_trials', answers.length)),
  )
  const incorrectAnswers = Math.max(0, Math.round(numberMetric(result.metrics, 'misses')))
  const correctAnswers = Math.max(0, totalAnswers - incorrectAnswers)

  return {
    score: Math.round(result.metric_value * 100),
    total_answers: totalAnswers,
    correct_answers: correctAnswers,
    incorrect_answers: incorrectAnswers,
    average_response_ms: Math.max(0, Math.round(numberMetric(result.metrics, 'mean_rt'))),
  }
}
