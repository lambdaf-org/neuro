import type { NavigationGuard } from 'vue-router'

import { resolveAuthRedirect } from '@/lib/auth/navigation'
import { useAuthStore } from '@/stores/auth'

export const authGuard: NavigationGuard = (to) => {
  const auth = useAuthStore()

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
