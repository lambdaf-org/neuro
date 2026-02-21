import type { NavigationGuard } from 'vue-router'

import { resolveAuthRedirect } from '@/lib/auth/navigation'
import { isAdminUiEnabled } from '@/lib/config/featureFlags'
import { useAuthStore } from '@/stores/auth'

export const authGuard: NavigationGuard = (to) => {
  const auth = useAuthStore()

  if (to.meta.requiresAdminUi && !isAdminUiEnabled) {
    return '/'
  }

  const requiresAuth = Boolean(to.meta.requiresAuth)
  const guestOnly = Boolean(to.meta.guestOnly)

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
