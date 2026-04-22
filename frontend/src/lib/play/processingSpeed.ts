export interface ProcessingSpeedTrial {
  target: string
  candidates: string[]
  isMatchPresent: boolean
}

export interface ProcessingSpeedAttempt {
  wasCorrect: boolean
  responseMs: number
}

export interface ProcessingSpeedSummary {
  totalAnswers: number
  correctAnswers: number
  incorrectAnswers: number
  accuracyPercent: number
  averageResponseMs: number
}

interface CreateProcessingSpeedTrialOptions {
  candidateCount?: number
  isMatchPresent?: boolean
  random?: () => number
}

const SYMBOL_POOL = ['A', 'B', 'C', 'D', 'E', 'F', 'H', 'K', 'M', 'N', 'P', 'R'] as const

function pickIndex(length: number, random: () => number): number {
  if (length <= 1) {
    return 0
  }

  return Math.min(length - 1, Math.floor(random() * length))
}

function sampleUniqueSymbols(pool: readonly string[], count: number, random: () => number): string[] {
  const available = [...pool]
  const result: string[] = []

  while (available.length > 0 && result.length < count) {
    const index = pickIndex(available.length, random)
    const [picked] = available.splice(index, 1)

    if (picked) {
      result.push(picked)
    }
  }

  return result
}

function shuffle<T>(items: readonly T[], random: () => number): T[] {
  const copy = [...items]

  for (let index = copy.length - 1; index > 0; index -= 1) {
    const swapIndex = pickIndex(index + 1, random)
    ;[copy[index], copy[swapIndex]] = [copy[swapIndex]!, copy[index]!]
  }

  return copy
}

export function createProcessingSpeedTrial(
  options: CreateProcessingSpeedTrialOptions = {},
): ProcessingSpeedTrial {
  const random = options.random ?? Math.random
  const requestedCandidateCount = Math.max(2, Math.round(options.candidateCount ?? 4))
  const target = SYMBOL_POOL[pickIndex(SYMBOL_POOL.length, random)]!
  const isMatchPresent = options.isMatchPresent ?? random() >= 0.5
  const maxCandidateCount = isMatchPresent ? SYMBOL_POOL.length : SYMBOL_POOL.length - 1
  const candidateCount = Math.min(requestedCandidateCount, maxCandidateCount)
  const distractorPool = SYMBOL_POOL.filter((symbol) => symbol !== target)
  const distractorCount = isMatchPresent ? candidateCount - 1 : candidateCount
  const distractors = sampleUniqueSymbols(distractorPool, distractorCount, random)
  const candidates = isMatchPresent ? [...distractors, target] : distractors

  return {
    target,
    candidates: shuffle(candidates, random),
    isMatchPresent,
  }
}

export function summarizeProcessingSpeedAttempts(
  attempts: readonly ProcessingSpeedAttempt[],
): ProcessingSpeedSummary {
  const totalAnswers = attempts.length
  const correctAnswers = attempts.filter((attempt) => attempt.wasCorrect).length
  const incorrectAnswers = totalAnswers - correctAnswers
  const averageResponseMs =
    totalAnswers > 0
      ? Math.round(attempts.reduce((sum, attempt) => sum + attempt.responseMs, 0) / totalAnswers)
      : 0
  const accuracyPercent =
    totalAnswers > 0 ? Math.round((correctAnswers / totalAnswers) * 100) : 0

  return {
    totalAnswers,
    correctAnswers,
    incorrectAnswers,
    accuracyPercent,
    averageResponseMs,
  }
}
