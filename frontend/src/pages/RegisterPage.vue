<script setup lang="ts">
import { reactive, ref } from 'vue'
import { useRouter } from 'vue-router'

import { ApiError, register } from '@/lib/auth-api'

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
