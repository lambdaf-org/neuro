import type { NavigationGuard } from 'vue-router'

import { resolveAuthRedirect } from '@/lib/auth/navigation'
import { isAdminUiEnabled } from '@/lib/config/featureFlags'
import { useAuthStore } from '@/stores/auth'

const BAN_ALLOWED_ROUTES = new Set(['banned', 'login', 'register'])

export const authGuard: NavigationGuard = (to) => {
  const auth = useAuthStore()

  if (
    auth.isBanned &&
    auth.isAuthenticated &&
    !BAN_ALLOWED_ROUTES.has(typeof to.name === 'string' ? to.name : '')
  ) {
    return { name: 'banned' }
  }

  if (to.matched.some((r) => r.meta.requiresAdminUi) && !isAdminUiEnabled) {
    return '/'
  }

  const requiresAuth = to.matched.some((r) => r.meta.requiresAuth)
  const guestOnly = to.matched.some((r) => r.meta.guestOnly)

  if (requiresAuth && !auth.isAuthenticated) {
    return {
      name: 'login',
      query: { redirect: to.fullPath },
    }
  }

  if (guestOnly && auth.isAuthenticated) {
    return resolveAuthRedirect(to.query.redirect)
  }

  return true
}
