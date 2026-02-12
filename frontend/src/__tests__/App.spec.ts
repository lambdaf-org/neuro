import { describe, it, expect } from 'vitest'

import { mount } from '@vue/test-utils'
import App from '../App.vue'

describe('App', () => {
  it('mounts renders properly', () => {
    const wrapper = mount(App, {
      global: {
        stubs: {
          UApp: { template: '<div><slot /></div>' },
          RouterView: { template: '<div>Auth Routes</div>' },
        },
      },
    })

    expect(wrapper.text()).toContain('Auth Routes')
  })
})
