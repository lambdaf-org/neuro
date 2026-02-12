<script setup lang="ts">
import { reactive, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'

import { ApiError, login, saveSession } from '@/lib/auth-api'

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

<style scoped>
.auth-container {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 100vh;
  padding: 2.5rem 1rem;
}

.auth-card {
  width: 100%;
  max-width: 36rem;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 0.5rem;
  padding: 2rem;
  backdrop-filter: blur(10px);
}

.auth-header {
  margin-bottom: 2rem;
}

.auth-brand {
  font-size: 0.75rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.16em;
  color: rgba(255, 255, 255, 0.6);
  margin-bottom: 0.5rem;
}

.auth-title {
  font-size: 1.5rem;
  font-weight: 600;
  color: rgba(255, 255, 255, 0.9);
}

.auth-content {
  margin-bottom: 1.5rem;
}

.alert {
  padding: 1rem;
  border-radius: 0.375rem;
  margin-bottom: 1rem;
}

.alert strong {
  display: block;
  margin-bottom: 0.25rem;
  font-weight: 600;
}

.alert-error {
  background: rgba(239, 68, 68, 0.1);
  border: 1px solid rgba(239, 68, 68, 0.3);
  color: #fca5a5;
}

.alert-success {
  background: rgba(34, 197, 94, 0.1);
  border: 1px solid rgba(34, 197, 94, 0.3);
  color: #86efac;
}

.auth-form {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.form-field {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.form-field label {
  font-size: 0.875rem;
  font-weight: 500;
  color: rgba(255, 255, 255, 0.8);
}

.form-field input {
  padding: 0.625rem 0.875rem;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 0.375rem;
  color: rgba(255, 255, 255, 0.9);
  font-size: 0.875rem;
  transition: all 0.2s;
}

.form-field input:focus {
  outline: none;
  border-color: rgba(59, 130, 246, 0.5);
  background: rgba(255, 255, 255, 0.08);
}

.form-field input::placeholder {
  color: rgba(255, 255, 255, 0.4);
}

.btn {
  padding: 0.625rem 1rem;
  border-radius: 0.375rem;
  font-weight: 500;
  font-size: 0.875rem;
  cursor: pointer;
  transition: all 0.2s;
  border: none;
}

.btn-primary {
  background: #3b82f6;
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: #2563eb;
}

.btn-primary:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.auth-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-size: 0.875rem;
  color: rgba(255, 255, 255, 0.6);
  padding-top: 1.5rem;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
}

.link {
  color: #3b82f6;
  text-decoration: none;
  font-weight: 500;
}

.link:hover {
  color: #60a5fa;
  text-decoration: underline;
}
</style>
