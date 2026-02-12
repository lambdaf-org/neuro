<script setup lang="ts">
import { reactive, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'

import { ApiError, login, saveSession } from '@/lib/auth-api'
import '@/assets/auth.css'

interface LoginFormState {
  email: string
  password: string
}

const route = useRoute()
const router = useRouter()

const state = reactive<LoginFormState>({
  email: '',
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

async function onSubmit(event: Event) {
  event.preventDefault()
  isSubmitting.value = true
  errorMessage.value = ''
  successMessage.value = ''

  if (!state.email.trim() || !state.password) {
    errorMessage.value = 'Email and password are required.'
    isSubmitting.value = false
    return
  }

  try {
    const session = await login({
      email: state.email.trim(),
      password: state.password,
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
  <main class="auth-container">
    <div class="auth-card">
      <div class="auth-header">
        <p class="auth-brand">Neuro</p>
        <h1 class="auth-title">Sign in</h1>
      </div>

      <div class="auth-content">
        <div v-if="errorMessage" class="alert alert-error">
          <strong>Authentication failed</strong>
          <p>{{ errorMessage }}</p>
        </div>

        <div v-if="successMessage" class="alert alert-success">
          <strong>Success</strong>
          <p>{{ successMessage }}</p>
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
              autocomplete="current-password"
              placeholder="********"
              required
            />
          </div>

          <button type="submit" class="btn btn-primary" :disabled="isSubmitting">
            {{ isSubmitting ? 'Signing in...' : 'Sign in' }}
          </button>
        </form>
      </div>

      <div class="auth-footer">
        <span>No account yet?</span>
        <router-link to="/register" class="link">Create account</router-link>
      </div>
    </div>
  </main>
</template>
