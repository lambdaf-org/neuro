<script setup lang="ts">
import { reactive, ref } from 'vue'
import { useRouter } from 'vue-router'
import type { FormError, FormSubmitEvent } from '@nuxt/ui'

import { ApiError, register } from '@/lib/auth-api'

interface RegisterFormState {
  email: string
  username: string
  password: string
  confirmPassword: string
}

const router = useRouter()

const state = reactive<RegisterFormState>({
  email: '',
  username: '',
  password: '',
  confirmPassword: '',
})

const isSubmitting = ref(false)
const errorMessage = ref('')

function getRegisterErrorMessage(error: ApiError): string {
  if (error.status === 409) {
    if (error.message.toLowerCase().includes('email')) {
      return "If this email can be used, you'll receive a verification email."
    }
    return error.message || 'Username already taken.'
  }

  if (error.status === 400) {
    return error.message || 'Please review the form fields and try again.'
  }

  if (error.status === 429) {
    return error.message || 'Too many attempts. Please wait and try again.'
  }

  return error.message || 'Registration failed. Please try again.'
}

function validate(formState: Partial<RegisterFormState>): FormError[] {
  const errors: FormError[] = []

  if (!formState.email?.trim()) {
    errors.push({ name: 'email', message: 'Email is required.' })
  }

  if (!formState.username?.trim()) {
    errors.push({ name: 'username', message: 'Username is required.' })
  }

  if (!formState.password) {
    errors.push({ name: 'password', message: 'Password is required.' })
  } else if (formState.password.length < 8) {
    errors.push({ name: 'password', message: 'Use at least 8 characters.' })
  }

  if (!formState.confirmPassword) {
    errors.push({ name: 'confirmPassword', message: 'Confirm your password.' })
  } else if (formState.password !== formState.confirmPassword) {
    errors.push({ name: 'confirmPassword', message: 'Passwords must match.' })
  }

  return errors
}

async function onSubmit(event: FormSubmitEvent<RegisterFormState>) {
  isSubmitting.value = true
  errorMessage.value = ''

  try {
    await register({
      email: event.data.email.trim(),
      username: event.data.username.trim(),
      password: event.data.password,
    })

    await router.push({
      path: '/login',
      query: {
        email: event.data.email.trim(),
        registered: '1',
      },
    })
  } catch (error) {
    if (error instanceof ApiError) {
      errorMessage.value = getRegisterErrorMessage(error)
    } else {
      errorMessage.value = 'Registration failed. Please try again.'
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
          <h1 class="text-2xl font-semibold text-highlighted">Create account</h1>
        </div>
      </template>

      <div class="space-y-4">
        <UAlert
          v-if="errorMessage"
          color="error"
          variant="soft"
          title="Registration failed"
          :description="errorMessage"
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
              autocomplete="new-password"
              placeholder="Choose a strong password"
              class="w-full"
            />
          </UFormField>

          <UFormField name="confirmPassword" label="Confirm password" required>
            <UInput
              v-model="state.confirmPassword"
              type="password"
              autocomplete="new-password"
              placeholder="Repeat your password"
              class="w-full"
            />
          </UFormField>

          <UButton type="submit" block :loading="isSubmitting">
            Register
          </UButton>
        </UForm>
      </div>

      <template #footer>
        <div class="flex items-center justify-between text-sm text-toned">
          <span>Already registered?</span>
          <UButton to="/login" variant="link" color="primary">
            Go to sign in
          </UButton>
        </div>
      </template>
    </UCard>
  </main>
</template>
