<script setup lang="ts">
import { reactive, ref, watch } from 'vue'
import { useRoute } from 'vue-router'
import type { FormError, FormSubmitEvent } from '@nuxt/ui'

import { ApiError, login, saveSession } from '@/lib/auth-api'

interface LoginFormState {
  email: string
  username: string
  password: string
}

const route = useRoute()

const state = reactive<LoginFormState>({
  email: '',
  username: '',
  password: '',
})

const isSubmitting = ref(false)
const errorMessage = ref('')
const successMessage = ref('')

function syncRouteState() {
  const emailQuery = route.query.email
  if (typeof emailQuery === 'string' && emailQuery.length > 0 && !state.email) {
    state.email = emailQuery
  }

  if (route.query.registered === '1') {
    successMessage.value = 'Account created. Sign in with your credentials.'
  }
}

syncRouteState()
watch(() => route.query, syncRouteState)

function validate(formState: Partial<LoginFormState>): FormError[] {
  const errors: FormError[] = []

  if (!formState.email?.trim()) {
    errors.push({ name: 'email', message: 'Email is required.' })
  }

  if (!formState.username?.trim()) {
    errors.push({ name: 'username', message: 'Username is required.' })
  }

  if (!formState.password) {
    errors.push({ name: 'password', message: 'Password is required.' })
  }

  return errors
}

async function onSubmit(event: FormSubmitEvent<LoginFormState>) {
  isSubmitting.value = true
  errorMessage.value = ''
  successMessage.value = ''

  try {
    const session = await login({
      email: event.data.email.trim(),
      username: event.data.username.trim(),
      password: event.data.password,
    })

    saveSession(session)
    state.password = ''
    successMessage.value = 'Authenticated. Access token has been stored in local storage.'
  } catch (error) {
    if (error instanceof ApiError) {
      errorMessage.value = error.message
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
        <UAlert
          v-if="errorMessage"
          color="error"
          variant="soft"
          title="Authentication failed"
          :description="errorMessage"
        />

        <UAlert
          v-if="successMessage"
          color="success"
          variant="soft"
          title="Success"
          :description="successMessage"
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

          <UFormField name="username" label="Username" required>
            <UInput
              v-model="state.username"
              autocomplete="username"
              placeholder="yourhandle1"
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
