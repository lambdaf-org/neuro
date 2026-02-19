import { computed, ref } from 'vue'
import { defineStore } from 'pinia'

import type { LoginResponse, StoredSession } from '@/lib/auth'
import { clearSession, getSession, saveSession } from '@/lib/auth'

export const useAuthStore = defineStore('auth', () => {
  const session = ref<StoredSession | null>(null)
  const hydrated = ref(false)

  const accessToken = computed(() => session.value?.accessToken ?? null)
  const isAuthenticated = computed(() => Boolean(accessToken.value))

  function readPersistedSession(): StoredSession | null {
    if (typeof window === 'undefined') {
      return null
    }

    return getSession()
  }

  function hydrate(): void {
    if (hydrated.value) {
      return
    }

    session.value = readPersistedSession()
    hydrated.value = true
  }

  function refresh(): void {
    session.value = readPersistedSession()
    hydrated.value = true
  }

  function signIn(payload: LoginResponse): void {
    saveSession(payload)
    refresh()
  }

  function logout(): void {
    clearSession()
    session.value = null
    hydrated.value = true
  }

  return {
    session,
    hydrated,
    accessToken,
    isAuthenticated,
    hydrate,
    refresh,
    signIn,
    logout,
  }
})
