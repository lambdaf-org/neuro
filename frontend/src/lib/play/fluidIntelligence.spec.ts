import { describe, expect, it } from 'vitest'

import type { AssetGroup } from '@/lib/play/assets'

import { createFluidPuzzles, summarizeFluidAnswers } from './fluidIntelligence'

const groups: AssetGroup[] = [
  {
    id: 1,
    game_code: 'gf',
    label: 'group_00',
    assets: [
      {
        id: 10,
        group_id: 1,
        label: 'matrix',
        image_url: 'https://example.test/matrix.svg',
        is_correct: false,
      },
      {
        id: 11,
        group_id: 1,
        label: 'option_0',
        image_url: 'https://example.test/option_0.svg',
        is_correct: true,
      },
      {
        id: 12,
        group_id: 1,
        label: 'option_1',
        image_url: 'https://example.test/option_1.svg',
        is_correct: false,
      },
    ],
  },
]

describe('createFluidPuzzles', () => {
  it('creates playable puzzles from asset groups', () => {
    const puzzles = createFluidPuzzles(groups, { random: () => 0 })

    expect(puzzles).toHaveLength(1)
    expect(puzzles[0]?.matrix.imageUrl).toContain('matrix.svg')
    expect(puzzles[0]?.options).toHaveLength(2)
    expect(puzzles[0]?.options.filter((option) => option.isCorrect)).toHaveLength(1)
  })

  it('skips malformed groups without a single correct option', () => {
    const malformedGroups = [
      {
        ...groups[0]!,
        assets: groups[0]!.assets.map((asset) => ({ ...asset, is_correct: false })),
      },
    ]

    expect(createFluidPuzzles(malformedGroups)).toEqual([])
  })
})

describe('summarizeFluidAnswers', () => {
  it('summarizes accuracy and response time', () => {
    expect(
      summarizeFluidAnswers([
        {
          puzzleId: 1,
          selectedOptionId: 11,
          correctOptionId: 11,
          wasCorrect: true,
          responseMs: 1000,
        },
        {
          puzzleId: 2,
          selectedOptionId: 21,
          correctOptionId: 22,
          wasCorrect: false,
          responseMs: 3000,
        },
      ]),
    ).toEqual({
      totalAnswers: 2,
      correctAnswers: 1,
      incorrectAnswers: 1,
      accuracyPercent: 50,
      averageResponseMs: 2000,
    })
  })
})
