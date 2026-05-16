import { shallowMount } from '@vue/test-utils'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import AppErrorState from '@/components/ui/AppErrorState.vue'
import PlayGamePage from './PlayGamePage.vue'

const { route } = vi.hoisted(() => ({
  route: {
    params: {
      gameId: 'missing-module',
    },
  },
}))

vi.mock('vue-router', async (importOriginal) => {
  const actual = await importOriginal<typeof import('vue-router')>()

  return {
    ...actual,
    useRoute: () => route,
  }
})

vi.mock('@/composables/useGameMetadata', () => ({
  useGameMetadata: () => ({
    metadata: { value: null },
    isLoading: { value: false },
    errorMessage: { value: '' },
  }),
}))

describe('PlayGamePage', () => {
  beforeEach(() => {
    route.params.gameId = 'missing-module'
  })

  it('shows a clear error state for unknown game ids', () => {
    const wrapper = shallowMount(PlayGamePage)
    const errorState = wrapper.getComponent(AppErrorState)

    expect(errorState.props('title')).toBe('Module not found')
    expect(errorState.props('description')).toBe("No module registered for 'missing-module'.")
  })
})
