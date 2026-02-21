import { ApiError } from '@/lib/auth'

/**
 * Extracts a user-friendly error message from an error object.
 * Prioritizes ApiError messages and falls back to a default message.
 */
export function getErrorMessage(error: unknown, fallback: string): string {
  if (error instanceof ApiError) {
    return error.message || fallback
  }
  return fallback
}

/**
 * Checks if an error is a 401 Unauthorized error.
 * Used to detect expired sessions.
 */
export function isUnauthorizedError(error: unknown): boolean {
  return error instanceof ApiError && error.status === 401
}
