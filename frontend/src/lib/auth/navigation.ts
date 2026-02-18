const DEFAULT_AUTH_REDIRECT = '/play'
const BLOCKED_REDIRECT_PATHS = new Set(['/login', '/register'])

export function resolveAuthRedirect(rawRedirect: unknown, fallback = DEFAULT_AUTH_REDIRECT): string {
  if (typeof rawRedirect !== 'string') {
    return fallback
  }

  if (!rawRedirect.startsWith('/') || rawRedirect.startsWith('//')) {
    return fallback
  }

  try {
    const parsed = new URL(rawRedirect, 'http://localhost')

    if (BLOCKED_REDIRECT_PATHS.has(parsed.pathname)) {
      return fallback
    }

    return `${parsed.pathname}${parsed.search}${parsed.hash}`
  } catch {
    return fallback
  }
}
