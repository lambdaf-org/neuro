import type { AssetGroup, GameAsset } from '@/lib/play/assets'

export interface FluidOption {
  id: number
  label: string
  imageUrl: string
  isCorrect: boolean
}

export interface FluidPuzzle {
  id: number
  label: string
  matrix: {
    id: number
    label: string
    imageUrl: string
  }
  options: FluidOption[]
}

export interface FluidAnswer {
  puzzleId: number
  selectedOptionId: number
  correctOptionId: number
  wasCorrect: boolean
  responseMs: number
}

export interface FluidSummary {
  totalAnswers: number
  correctAnswers: number
  incorrectAnswers: number
  accuracyPercent: number
  averageResponseMs: number
}

interface CreateFluidPuzzlesOptions {
  random?: () => number
}

function pickIndex(length: number, random: () => number): number {
  if (length <= 1) {
    return 0
  }

  return Math.min(length - 1, Math.floor(random() * length))
}

function shuffle<T>(items: readonly T[], random: () => number): T[] {
  const copy = [...items]

  for (let index = copy.length - 1; index > 0; index -= 1) {
    const swapIndex = pickIndex(index + 1, random)
    ;[copy[index], copy[swapIndex]] = [copy[swapIndex]!, copy[index]!]
  }

  return copy
}

function isMatrixAsset(asset: GameAsset): boolean {
  return asset.label.trim().toLowerCase() === 'matrix'
}

function toPuzzle(group: AssetGroup, random: () => number): FluidPuzzle | null {
  const matrix = group.assets.find(isMatrixAsset)
  const options = group.assets.filter((asset) => !isMatrixAsset(asset))
  const correctOptions = options.filter((asset) => asset.is_correct)

  if (!matrix || options.length < 2 || correctOptions.length !== 1) {
    return null
  }

  return {
    id: group.id,
    label: group.label,
    matrix: {
      id: matrix.id,
      label: matrix.label,
      imageUrl: matrix.image_url,
    },
    options: shuffle(
      options.map((asset) => ({
        id: asset.id,
        label: asset.label,
        imageUrl: asset.image_url,
        isCorrect: asset.is_correct,
      })),
      random,
    ),
  }
}

export function createFluidPuzzles(
  groups: readonly AssetGroup[],
  options: CreateFluidPuzzlesOptions = {},
): FluidPuzzle[] {
  const random = options.random ?? Math.random

  return shuffle(
    groups
      .map((group) => toPuzzle(group, random))
      .filter((puzzle): puzzle is FluidPuzzle => puzzle !== null),
    random,
  )
}

export function summarizeFluidAnswers(answers: readonly FluidAnswer[]): FluidSummary {
  const totalAnswers = answers.length
  const correctAnswers = answers.filter((answer) => answer.wasCorrect).length
  const incorrectAnswers = totalAnswers - correctAnswers
  const accuracyPercent =
    totalAnswers > 0 ? Math.round((correctAnswers / totalAnswers) * 100) : 0
  const averageResponseMs =
    totalAnswers > 0
      ? Math.round(answers.reduce((sum, answer) => sum + answer.responseMs, 0) / totalAnswers)
      : 0

  return {
    totalAnswers,
    correctAnswers,
    incorrectAnswers,
    accuracyPercent,
    averageResponseMs,
  }
}
