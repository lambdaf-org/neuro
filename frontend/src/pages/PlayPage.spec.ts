import { mount } from '@vue/test-utils'
import { describe, expect, it } from 'vitest'

import PlayPage from './PlayPage.vue'
import { PLAY_MODULES } from '@/lib/play/modules'

describe('PlayPage', () => {
  it('renders every configured play module', () => {
    const wrapper = mount(PlayPage, {
      global: {
        stubs: {
          ProtectedNav: { template: '<nav>Protected Navigation</nav>' },
          PlayModuleCard: {
            props: ['module'],
            template:
              '<section data-test="module-card">{{ module.name }} {{ module.chcCode }} {{ module.description }}</section>',
          },
        },
      },
    })

    const cards = wrapper.findAll('[data-test="module-card"]')

    expect(cards).toHaveLength(PLAY_MODULES.length)
    for (const module of PLAY_MODULES) {
      expect(wrapper.text()).toContain(module.name)
      expect(wrapper.text()).toContain(module.chcCode)
    }
  })
})
