import { describe, expect, it } from 'vitest'

import { resolveAuthRedirect } from './navigation'

describe('resolveAuthRedirect', () => {
  it('returns fallback for missing redirect', () => {
    expect(resolveAuthRedirect(undefined)).toBe('/play')
  })

  it('keeps safe internal redirects with query and hash', () => {
    expect(resolveAuthRedirect('/profile?tab=security#token')).toBe('/profile?tab=security#token')
  })

  it('blocks external redirects', () => {
    expect(resolveAuthRedirect('https://evil.test/steal')).toBe('/play')
    expect(resolveAuthRedirect('//evil.test/steal')).toBe('/play')
  })

  it('blocks redirects back to auth-only guest pages', () => {
    expect(resolveAuthRedirect('/login')).toBe('/play')
    expect(resolveAuthRedirect('/register')).toBe('/play')
  })
})
