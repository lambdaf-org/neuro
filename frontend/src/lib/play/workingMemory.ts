export interface WorkingMemoryAttempt {
  span: number
  wasCorrect: boolean
  responseMs: number
  sequence: number[]
  response: number[]
}

export interface WorkingMemorySummary {
  totalAttempts: number
  correctAttempts: number
  failedAttempts: number
  maxSpan: number
  failedSpan: number
  averageResponseMs: number
}

interface CreateWorkingMemorySequenceOptions {
  tileCount?: number
  random?: () => number
}

const DEFAULT_TILE_COUNT = 9

function pickIndex(length: number, random: () => number): number {
  if (length <= 1) {
    return 0
  }

  return Math.min(length - 1, Math.floor(random() * length))
}

export function createWorkingMemorySequence(
  span: number,
  options: CreateWorkingMemorySequenceOptions = {},
): number[] {
  const tileCount = Math.max(1, Math.round(options.tileCount ?? DEFAULT_TILE_COUNT))
  const random = options.random ?? Math.random
  const sequenceLength = Math.max(1, Math.round(span))
  const sequence: number[] = []

  for (let index = 0; index < sequenceLength; index += 1) {
    const previous = sequence[index - 1]
    const nextCandidates = Array.from({ length: tileCount }, (_, tileIndex) => tileIndex).filter(
      (tileIndex) => tileIndex !== previous,
    )
    const candidates = nextCandidates.length > 0 ? nextCandidates : [0]

    sequence.push(candidates[pickIndex(candidates.length, random)]!)
  }

  return sequence
}

export function isWorkingMemoryRecallCorrect(
  sequence: readonly number[],
  response: readonly number[],
): boolean {
  return (
    sequence.length === response.length && sequence.every((tile, index) => tile === response[index])
  )
}

export function summarizeWorkingMemoryAttempts(
  attempts: readonly WorkingMemoryAttempt[],
): WorkingMemorySummary {
  const totalAttempts = attempts.length
  const correctAttempts = attempts.filter((attempt) => attempt.wasCorrect)
  const failedAttempts = totalAttempts - correctAttempts.length
  const failedAttempt = attempts.find((attempt) => !attempt.wasCorrect)
  const averageResponseMs =
    totalAttempts > 0
      ? Math.round(attempts.reduce((sum, attempt) => sum + attempt.responseMs, 0) / totalAttempts)
      : 0

  return {
    totalAttempts,
    correctAttempts: correctAttempts.length,
    failedAttempts,
    maxSpan:
      correctAttempts.length > 0 ? Math.max(...correctAttempts.map((attempt) => attempt.span)) : 0,
    failedSpan: failedAttempt?.span ?? 0,
    averageResponseMs,
  }
}
