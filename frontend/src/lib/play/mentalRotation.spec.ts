import { describe, expect, it } from 'vitest'

import type { AssetGroup } from '@/lib/play/assets'

import { createMentalRotationPuzzles, summarizeMentalRotationAnswers } from './mentalRotation'

const groups: AssetGroup[] = [
  {
    id: 1,
    game_code: 'gv',
    label: 'group_00',
    assets: [
      {
        id: 10,
        group_id: 1,
        label: 'reference',
        image_url: 'https://example.test/reference.svg',
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

describe('createMentalRotationPuzzles', () => {
  it('creates playable puzzles from reference asset groups', () => {
    const puzzles = createMentalRotationPuzzles(groups, { random: () => 0 })

    expect(puzzles).toHaveLength(1)
    expect(puzzles[0]?.reference.imageUrl).toContain('reference.svg')
    expect(puzzles[0]?.options).toHaveLength(4)
  })

  it('skips malformed groups without four answer options', () => {
    const malformedGroups = [
      {
        ...groups[0]!,
        assets: groups[0]!.assets.slice(0, 3),
      },
    ]

    expect(createMentalRotationPuzzles(malformedGroups)).toEqual([])
  })

  it('skips malformed groups with duplicate reference assets', () => {
    const malformedGroups = [
      {
        ...groups[0]!,
        assets: [
          ...groups[0]!.assets,
          {
            id: 15,
            group_id: 1,
            label: 'reference',
            image_url: 'https://example.test/reference-extra.svg',
          },
        ],
      },
    ]

    expect(createMentalRotationPuzzles(malformedGroups)).toEqual([])
  })
})

describe('summarizeMentalRotationAnswers', () => {
  it('summarizes progress and response time before server scoring', () => {
    expect(
      summarizeMentalRotationAnswers([
        {
          puzzleId: 1,
          selectedOptionId: 11,
          responseMs: 1200,
        },
        {
          puzzleId: 2,
          selectedOptionId: 21,
          responseMs: 2800,
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
