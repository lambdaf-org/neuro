export type GameId =
  | 'reaction-time'
  | 'symbol-matching'
  | 'pattern-logic'
  | 'sequence-memory'
  | 'mental-rotation'

export interface PlayModule {
  readonly id: GameId
  readonly chcCode: 'Gt' | 'Gs' | 'Gf' | 'Gwm' | 'Gv'
  readonly name: string
  readonly description: string
  readonly icon: string
}

export const PLAY_MODULES = [
  {
    id: 'reaction-time',
    chcCode: 'Gt',
    name: 'Reaction Time',
    description: 'Wait for a visual cue, then respond as quickly as possible.',
    icon: 'i-lucide-mouse-pointer-click',
  },
  {
    id: 'symbol-matching',
    chcCode: 'Gs',
    name: 'Symbol Matching',
    description: 'Decide whether the target symbol appears in the candidate set.',
    icon: 'i-lucide-gauge',
  },
  {
    id: 'pattern-logic',
    chcCode: 'Gf',
    name: 'Pattern Logic',
    description: 'Choose the option that correctly completes the visual matrix.',
    icon: 'i-lucide-grid-3x3',
  },
  {
    id: 'sequence-memory',
    chcCode: 'Gwm',
    name: 'Sequence Memory',
    description: 'Repeat increasingly long sequences in the correct order.',
    icon: 'i-lucide-circuit-board',
  },
  {
    id: 'mental-rotation',
    chcCode: 'Gv',
    name: 'Mental Rotation',
    description: 'Find the candidate that matches the reference after rotation.',
    icon: 'i-lucide-cuboid',
  },
] as const satisfies readonly PlayModule[]

export function findPlayModuleById(gameId: string): PlayModule | undefined {
  return PLAY_MODULES.find((module) => module.id === gameId)
}
