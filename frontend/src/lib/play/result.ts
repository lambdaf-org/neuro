export const GAME_RESULT_SCHEMA_VERSION = 1 as const

export type GameState = 'countdown' | 'running' | 'finished'

export interface GameResult {
  schema_version: typeof GAME_RESULT_SCHEMA_VERSION
  score: number
  duration_ms: number
  states: GameState[]
  metrics: Record<string, number>
}

export interface CreateGameResultInput {
  score: number
  durationMs: number
  states: GameState[]
  metrics?: Record<string, number>
}

function toPositiveNumber(value: number): number {
  if (!Number.isFinite(value)) {
    return 0
  }

  return Math.max(0, value)
}

export function createGameResult(input: CreateGameResultInput): GameResult {
  const safeDuration = Math.max(1, Math.round(toPositiveNumber(input.durationMs)))

  return {
    schema_version: GAME_RESULT_SCHEMA_VERSION,
    score: toPositiveNumber(input.score),
    duration_ms: safeDuration,
    states: input.states,
    metrics: input.metrics ?? {},
  }
}
