/**
 * Validates if a string is a valid HTTP/HTTPS URL.
 * Returns true if the URL is valid, false otherwise.
 */
export function isValidHttpUrl(value: string): boolean {
  if (!value || typeof value !== 'string') {
    return false
  }

  try {
    const url = new URL(value)
    return url.protocol === 'http:' || url.protocol === 'https:'
  } catch {
    return false
  }
}
