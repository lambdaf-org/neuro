import { describe, expect, it } from 'vitest'

import {
  createWorkingMemorySequence,
  isWorkingMemoryRecallCorrect,
  summarizeWorkingMemoryAttempts,
} from './workingMemory'

describe('createWorkingMemorySequence', () => {
  it('creates a sequence with the requested span and no immediate repeats', () => {
    const sequence = createWorkingMemorySequence(5, { tileCount: 3, random: () => 0 })

    expect(sequence).toHaveLength(5)
    expect(sequence.every((tile) => tile >= 0 && tile < 3)).toBe(true)
    expect(sequence.some((tile, index) => index > 0 && tile === sequence[index - 1])).toBe(false)
  })
})

describe('isWorkingMemoryRecallCorrect', () => {
  it('requires exact serial order', () => {
    expect(isWorkingMemoryRecallCorrect([1, 3, 5], [1, 3, 5])).toBe(true)
    expect(isWorkingMemoryRecallCorrect([1, 3, 5], [1, 5, 3])).toBe(false)
    expect(isWorkingMemoryRecallCorrect([1, 3, 5], [1, 3])).toBe(false)
  })
})

describe('summarizeWorkingMemoryAttempts', () => {
  it('uses the highest correct span and tracks the failed attempt separately', () => {
    expect(
      summarizeWorkingMemoryAttempts([
        {
          span: 2,
          wasCorrect: true,
          responseMs: 1000,
          sequence: [0, 1],
          response: [0, 1],
        },
        {
          span: 3,
          wasCorrect: true,
          responseMs: 2000,
          sequence: [0, 1, 2],
          response: [0, 1, 2],
        },
        {
          span: 4,
          wasCorrect: false,
          responseMs: 3000,
          sequence: [0, 1, 2, 3],
          response: [0, 1, 3, 2],
        },
      ]),
    ).toEqual({
      totalAttempts: 3,
      correctAttempts: 2,
      failedAttempts: 1,
      maxSpan: 3,
      failedSpan: 4,
      averageResponseMs: 2000,
    })
  })
})
