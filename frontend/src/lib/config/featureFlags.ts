function isTrueFlag(value: string | undefined): boolean {
  return value === 'true'
}

export const isAdminUiEnabled = isTrueFlag(import.meta.env.VITE_ADMIN_UI)
