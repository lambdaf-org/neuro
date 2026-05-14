<script setup lang="ts">
import { reactive, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import type { FormError, FormSubmitEvent } from '@nuxt/ui'

import AppErrorState from '@/components/ui/AppErrorState.vue'
import { resolveAuthRedirect } from '@/lib/auth/navigation'
import { ApiError, login } from '@/lib/auth'
import { useAuthStore } from '@/stores/auth'

interface LoginFormState {
  email: string
  password: string
}

const route = useRoute()
const router = useRouter()
const auth = useAuthStore()

const state = reactive<LoginFormState>({
  email: '',
  password: '',
})

const isSubmitting = ref(false)
const errorMessage = ref('')
const successMessage = ref('')
const warningMessage = ref('')

function syncRouteState() {
  const emailQuery = route.query.email
  if (typeof emailQuery === 'string' && emailQuery.length > 0 && !state.email) {
    state.email = emailQuery
  }

  successMessage.value = ''
  warningMessage.value = ''

  if (route.query.loggedOut === '1') {
    successMessage.value = 'You have been signed out.'
  } else if (route.query.sessionExpired === '1') {
    warningMessage.value = 'Your session has expired. Please sign in again.'
  } else if (route.query.registered === '1') {
    successMessage.value =
      "If this email can be used, you'll receive a verification email. Check your inbox before signing in."
  }
}

watch(() => route.query, syncRouteState, { immediate: true })

function validate(formState: Partial<LoginFormState>): FormError[] {
  const errors: FormError[] = []

  if (!formState.email?.trim()) {
    errors.push({ name: 'email', message: 'Email is required.' })
  }

  if (!formState.password) {
    errors.push({ name: 'password', message: 'Password is required.' })
  }

  return errors
}

function getLoginErrorMessage(error: ApiError): string {
  if (error.status === 403) {
    return 'Please verify your email first. Check your inbox for the verification link.'
  }

  if (error.status === 401) {
    return 'Invalid email or password.'
  }

  if (error.status === 429) {
    return 'Too many login attempts. Please wait and try again.'
  }

  return error.message || 'Login failed. Please try again.'
}

async function onSubmit(event: FormSubmitEvent<LoginFormState>) {
  isSubmitting.value = true
  errorMessage.value = ''
  successMessage.value = ''

  try {
    const session = await login({
      email: event.data.email.trim(),
      password: event.data.password,
    })

    auth.signIn(session)
    state.password = ''

    if (session.is_banned) {
      auth.flagBanned('Account suspended due to anti-cheat violation.')
      await router.replace({ name: 'banned' })
      return
    }

    await router.push(resolveAuthRedirect(route.query.redirect))
  } catch (error) {
    if (error instanceof ApiError) {
      errorMessage.value = getLoginErrorMessage(error)
    } else {
      errorMessage.value = 'Login failed. Please try again.'
    }
  } finally {
    isSubmitting.value = false
  }
}
</script>

<template>
  <main class="mx-auto flex min-h-svh w-full max-w-xl items-center px-4 py-10 sm:px-6">
    <UCard variant="subtle" class="w-full border border-default/60 backdrop-blur-sm">
      <template #header>
        <div class="space-y-1">
          <p class="text-xs font-semibold uppercase tracking-[0.16em] text-secondary">Neuro</p>
          <h1 class="text-2xl font-semibold text-highlighted">Sign in</h1>
        </div>
      </template>

      <div class="space-y-4">
        <AppErrorState
          v-if="errorMessage"
          title="Authentication failed"
          :description="errorMessage"
        />

        <UAlert
          v-if="successMessage"
          color="success"
          variant="soft"
          title="Notice"
          :description="successMessage"
        />

        <UAlert
          v-if="warningMessage"
          color="warning"
          variant="soft"
          title="Session expired"
          :description="warningMessage"
        />

        <UForm :state="state" :validate="validate" class="space-y-4" @submit="onSubmit">
          <UFormField name="email" label="Email" required>
            <UInput
              v-model="state.email"
              type="email"
              autocomplete="email"
              placeholder="you@example.com"
              class="w-full"
            />
          </UFormField>

          <UFormField name="password" label="Password" required>
            <UInput
              v-model="state.password"
              type="password"
              autocomplete="current-password"
              placeholder="********"
              class="w-full"
            />
          </UFormField>

          <UButton type="submit" block :loading="isSubmitting"> Sign in </UButton>
        </UForm>
      </div>

      <template #footer>
        <div class="flex items-center justify-between text-sm text-toned">
          <span>No account yet?</span>
          <UButton to="/register" variant="link" color="primary"> Create account </UButton>
        </div>
      </template>
    </UCard>
  </main>
</template>
