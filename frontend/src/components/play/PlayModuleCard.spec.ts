import { mount } from '@vue/test-utils'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import PlayModuleCard from './PlayModuleCard.vue'
import { PLAY_MODULES } from '@/lib/play/modules'

const { push } = vi.hoisted(() => ({
  push: vi.fn(),
}))

vi.mock('vue-router', () => ({
  useRouter: () => ({
    push,
  }),
}))

function mountCard() {
  return mount(PlayModuleCard, {
    props: {
      module: PLAY_MODULES[0],
    },
    global: {
      stubs: {
        UBadge: { template: '<span><slot /></span>' },
        UCard: { template: '<article v-bind="$attrs"><slot /></article>' },
        UIcon: { template: '<span />' },
      },
    },
  })
}

describe('PlayModuleCard', () => {
  beforeEach(() => {
    push.mockClear()
  })

  it('renders the module name, CHC code and description', () => {
    const wrapper = mountCard()

    expect(wrapper.text()).toContain('Reaction Time')
    expect(wrapper.text()).toContain('Gt')
    expect(wrapper.text()).toContain('Wait for a visual cue')
  })

  it('navigates to the module route when clicked', async () => {
    const wrapper = mountCard()

    await wrapper.get('[role="link"]').trigger('click')

    expect(push).toHaveBeenCalledWith('/play/reaction-time')
  })

  it('supports keyboard activation with Enter and Space', async () => {
    const wrapper = mountCard()
    const card = wrapper.get('[role="link"]')

    await card.trigger('keydown', { key: 'Enter' })
    await card.trigger('keydown', { key: ' ' })
    await card.trigger('keydown', { key: 'Tab' })

    expect(push).toHaveBeenCalledTimes(2)
    expect(push).toHaveBeenNthCalledWith(1, '/play/reaction-time')
    expect(push).toHaveBeenNthCalledWith(2, '/play/reaction-time')
  })
})
