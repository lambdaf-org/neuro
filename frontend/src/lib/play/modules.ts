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

export const PLAY_MODULE_ICONS: Record<GameId, string> = {
  'reaction-time': 'i-lucide-mouse-pointer-click',
  'symbol-matching': 'i-lucide-scan-search',
  'pattern-logic': 'i-lucide-grid-3x3',
  'sequence-memory': 'i-lucide-brain-circuit',
  'mental-rotation': 'i-lucide-cuboid',
}

export const PLAY_MODULES = [
  {
    id: 'reaction-time',
    chcCode: 'Gt',
    name: 'Reaction Time',
    description: 'Wait for a visual cue, then respond as quickly as possible.',
    icon: PLAY_MODULE_ICONS['reaction-time'],
  },
  {
    id: 'symbol-matching',
    chcCode: 'Gs',
    name: 'Symbol Matching',
    description: 'Decide whether the target symbol appears in the candidate set.',
    icon: PLAY_MODULE_ICONS['symbol-matching'],
  },
  {
    id: 'pattern-logic',
    chcCode: 'Gf',
    name: 'Pattern Logic',
    description: 'Choose the option that correctly completes the visual matrix.',
    icon: PLAY_MODULE_ICONS['pattern-logic'],
  },
  {
    id: 'sequence-memory',
    chcCode: 'Gwm',
    name: 'Sequence Memory',
    description: 'Repeat increasingly long sequences in the correct order.',
    icon: PLAY_MODULE_ICONS['sequence-memory'],
  },
  {
    id: 'mental-rotation',
    chcCode: 'Gv',
    name: 'Mental Rotation',
    description: 'Find the candidate that matches the reference after rotation.',
    icon: PLAY_MODULE_ICONS['mental-rotation'],
  },
] as const satisfies readonly PlayModule[]

export function findPlayModuleById(gameId: string): PlayModule | undefined {
  return PLAY_MODULES.find((module) => module.id === gameId)
}
