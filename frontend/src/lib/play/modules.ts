export type GameId =
  | 'reaction-time'
  | 'processing-speed'
  | 'fluid-intelligence'
  | 'working-memory'
  | 'visual-processing'

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
    description: 'Measure speed of response under rapidly changing stimuli.',
    icon: 'i-lucide-mouse-pointer-click',
  },
  {
    id: 'processing-speed',
    chcCode: 'Gs',
    name: 'Processing Speed',
    description: 'Track accuracy and throughput while handling timed visual tasks.',
    icon: 'i-lucide-gauge',
  },
  {
    id: 'fluid-intelligence',
    chcCode: 'Gf',
    name: 'Fluid Intelligence',
    description: 'Solve novel patterns and abstract logic problems under pressure.',
    icon: 'i-lucide-grid-3x3',
  },
  {
    id: 'working-memory',
    chcCode: 'Gwm',
    name: 'Working Memory',
    description: 'Retain and manipulate information across short cognitive intervals.',
    icon: 'i-lucide-circuit-board',
  },
  {
    id: 'visual-processing',
    chcCode: 'Gv',
    name: 'Visual Processing',
    description: 'Interpret spatial relations and transformations in dynamic scenes.',
    icon: 'i-lucide-cuboid',
  },
] as const satisfies readonly PlayModule[]

export function findPlayModuleById(gameId: string): PlayModule | undefined {
  return PLAY_MODULES.find((module) => module.id === gameId)
}
