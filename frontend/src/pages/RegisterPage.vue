<script setup lang="ts">
import { reactive, ref } from 'vue'
import { useRouter } from 'vue-router'

import { ApiError, register } from '@/lib/auth-api'
import '@/assets/auth.css'

interface RegisterFormState {
  email: string
  password: string
  confirmPassword: string
}

const router = useRouter()

const state = reactive<RegisterFormState>({
  email: '',
  password: '',
  confirmPassword: '',
})

const isSubmitting = ref(false)
const errorMessage = ref('')

async function onSubmit(event: Event) {
  event.preventDefault()
  isSubmitting.value = true
  errorMessage.value = ''

  if (!state.email.trim() || !state.password || !state.confirmPassword) {
    errorMessage.value = 'All fields are required.'
    isSubmitting.value = false
    return
  }

  if (state.password.length < 8) {
    errorMessage.value = 'Password must be at least 8 characters.'
    isSubmitting.value = false
    return
  }

  if (state.password !== state.confirmPassword) {
    errorMessage.value = 'Passwords must match.'
    isSubmitting.value = false
    return
  }

  try {
    await register({
      email: state.email.trim(),
      password: state.password,
    })

    await router.push({
      path: '/login',
      query: {
        email: state.email.trim(),
        registered: '1',
      },
    })
  } catch (error) {
    if (error instanceof ApiError) {
      errorMessage.value = error.message
    } else {
      errorMessage.value = 'Registration failed. Please try again.'
    }
  } finally {
    isSubmitting.value = false
  }
}
</script>

<template>
  <main class="auth-container">
    <div class="auth-card">
      <div class="auth-header">
        <p class="auth-brand">Neuro</p>
        <h1 class="auth-title">Create account</h1>
      </div>

      <div class="auth-content">
        <div v-if="errorMessage" class="alert alert-error">
          <strong>Registration failed</strong>
          <p>{{ errorMessage }}</p>
        </div>

        <form class="auth-form" @submit="onSubmit">
          <div class="form-field">
            <label for="email">Email</label>
            <input
              id="email"
              v-model="state.email"
              type="email"
              autocomplete="email"
              placeholder="you@example.com"
              required
            />
          </div>

          <div class="form-field">
            <label for="password">Password</label>
            <input
              id="password"
              v-model="state.password"
              type="password"
              autocomplete="new-password"
              placeholder="Choose a strong password"
              required
            />
          </div>

          <div class="form-field">
            <label for="confirmPassword">Confirm password</label>
            <input
              id="confirmPassword"
              v-model="state.confirmPassword"
              type="password"
              autocomplete="new-password"
              placeholder="Repeat your password"
              required
            />
          </div>

          <button type="submit" class="btn btn-primary" :disabled="isSubmitting">
            {{ isSubmitting ? 'Creating account...' : 'Register' }}
          </button>
        </form>
      </div>

      <div class="auth-footer">
        <span>Already registered?</span>
        <router-link to="/login" class="link">Go to sign in</router-link>
      </div>
    </div>
  </main>
</template>
