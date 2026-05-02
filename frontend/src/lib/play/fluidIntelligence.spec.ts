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
      },
      {
        id: 11,
        group_id: 1,
        label: 'option_0',
        image_url: 'https://example.test/option_0.svg',
      },
      {
        id: 12,
        group_id: 1,
        label: 'option_1',
        image_url: 'https://example.test/option_1.svg',
      },
      {
        id: 13,
        group_id: 1,
        label: 'option_2',
        image_url: 'https://example.test/option_2.svg',
      },
      {
        id: 14,
        group_id: 1,
        label: 'option_3',
        image_url: 'https://example.test/option_3.svg',
      },
    ],
  },
]

describe('createFluidPuzzles', () => {
  it('creates playable puzzles from asset groups', () => {
    const puzzles = createFluidPuzzles(groups, { random: () => 0 })

    expect(puzzles).toHaveLength(1)
    expect(puzzles[0]?.matrix.imageUrl).toContain('matrix.svg')
    expect(puzzles[0]?.options).toHaveLength(4)
  })

  it('skips malformed groups without four answer options', () => {
    const malformedGroups = [
      {
        ...groups[0]!,
        assets: groups[0]!.assets.slice(0, 3),
      },
    ]

    expect(createFluidPuzzles(malformedGroups)).toEqual([])
  })
})

describe('summarizeFluidAnswers', () => {
  it('summarizes progress and response time before server scoring', () => {
    expect(
      summarizeFluidAnswers([
        {
          puzzleId: 1,
          selectedOptionId: 11,
          responseMs: 1000,
        },
        {
          puzzleId: 2,
          selectedOptionId: 21,
          responseMs: 3000,
        },
      ]),
    ).toEqual({
      totalAnswers: 2,
      correctAnswers: 0,
      incorrectAnswers: 0,
      accuracyPercent: 0,
      averageResponseMs: 2000,
    })
  })
})
