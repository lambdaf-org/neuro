import { computed, ref } from 'vue'
import { defineStore } from 'pinia'

import type { LoginResponse, StoredSession } from '@/lib/auth'
import { clearSession, getSession, saveSession } from '@/lib/auth'

const BAN_FLAG_KEY = 'neuro.banned'
const BAN_REASON_KEY = 'neuro.ban_reason'

function safeRead(key: string): string | null {
  try {
    return localStorage.getItem(key)
  } catch {
    return null
  }
}

function safeWrite(key: string, value: string | null): void {
  try {
    if (value === null) localStorage.removeItem(key)
    else localStorage.setItem(key, value)
  } catch {
    // ignore
  }
}

export const useAuthStore = defineStore('auth', () => {
  const session = ref<StoredSession | null>(null)
  const hydrated = ref(false)
  const isBanned = ref(false)
  const banReason = ref('')

  const accessToken = computed(() => session.value?.accessToken ?? null)
  const isAuthenticated = computed(() => Boolean(accessToken.value))

  function readPersistedSession(): StoredSession | null {
    if (typeof window === 'undefined') {
      return null
    }

    return getSession()
  }

  function loadBanFromStorage(): void {
    isBanned.value = safeRead(BAN_FLAG_KEY) === '1'
    banReason.value = safeRead(BAN_REASON_KEY) ?? ''
  }

  function hydrate(): void {
    if (hydrated.value) {
      return
    }

    session.value = readPersistedSession()
    loadBanFromStorage()
    hydrated.value = true
  }

  function refresh(): void {
    session.value = readPersistedSession()
    loadBanFromStorage()
    hydrated.value = true
  }

  function signIn(payload: LoginResponse): void {
    saveSession(payload)
    refresh()
    clearBan()
  }

  function logout(): void {
    clearSession()
    session.value = null
    hydrated.value = true
    clearBan()
  }

  function flagBanned(reason: string): void {
    isBanned.value = true
    banReason.value = reason
    safeWrite(BAN_FLAG_KEY, '1')
    safeWrite(BAN_REASON_KEY, reason)
  }

  function clearBan(): void {
    isBanned.value = false
    banReason.value = ''
    safeWrite(BAN_FLAG_KEY, null)
    safeWrite(BAN_REASON_KEY, null)
  }

  return {
    session,
    hydrated,
    isBanned,
    banReason,
    accessToken,
    isAuthenticated,
    hydrate,
    refresh,
    signIn,
    logout,
    flagBanned,
    clearBan,
  }
})
