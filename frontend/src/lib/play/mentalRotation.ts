import type { AssetGroup, GameAsset } from '@/lib/play/assets'

export interface MentalRotationOption {
  id: number
  label: string
  imageUrl: string
}

export interface MentalRotationPuzzle {
  id: number
  label: string
  reference: {
    id: number
    label: string
    imageUrl: string
  }
  options: MentalRotationOption[]
}

export interface MentalRotationAnswer {
  puzzleId: number
  selectedOptionId: number
  responseMs: number
}

export interface MentalRotationSummary {
  totalAnswers: number
  correctAnswers: number
  incorrectAnswers: number
  accuracyPercent: number
  averageResponseMs: number
}

interface CreateMentalRotationPuzzlesOptions {
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

function isReferenceAsset(asset: GameAsset): boolean {
  return asset.label.trim().toLowerCase() === 'reference'
}

function toPuzzle(group: AssetGroup, random: () => number): MentalRotationPuzzle | null {
  const reference = group.assets.find(isReferenceAsset)
  const options = group.assets.filter((asset) => !isReferenceAsset(asset))

  if (!reference || options.length !== 4) {
    return null
  }

  return {
    id: group.id,
    label: group.label,
    reference: {
      id: reference.id,
      label: reference.label,
      imageUrl: reference.image_url,
    },
    options: shuffle(
      options.map((asset) => ({
        id: asset.id,
        label: asset.label,
        imageUrl: asset.image_url,
      })),
      random,
    ),
  }
}

export function createMentalRotationPuzzles(
  groups: readonly AssetGroup[],
  options: CreateMentalRotationPuzzlesOptions = {},
): MentalRotationPuzzle[] {
  const random = options.random ?? Math.random

  return shuffle(
    groups
      .map((group) => toPuzzle(group, random))
      .filter((puzzle): puzzle is MentalRotationPuzzle => puzzle !== null),
    random,
  )
}

// Correctness is server-scored from the stored Supabase asset metadata.
export function summarizeMentalRotationAnswers(
  answers: readonly MentalRotationAnswer[],
): MentalRotationSummary {
  const totalAnswers = answers.length
  const averageResponseMs =
    totalAnswers > 0
      ? Math.round(answers.reduce((sum, answer) => sum + answer.responseMs, 0) / totalAnswers)
      : 0

  return {
    totalAnswers,
    correctAnswers: 0,
    incorrectAnswers: 0,
    accuracyPercent: 0,
    averageResponseMs,
  }
}
