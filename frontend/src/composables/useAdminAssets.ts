import { computed, onMounted, reactive, ref, watch } from 'vue'
import { useRouter } from 'vue-router'
import type { FormError, FormSubmitEvent } from '@nuxt/ui'

import { ApiError } from '@/lib/auth'
import {
  createAssetGroup,
  createGameAsset,
  deleteAssetGroup,
  deleteGameAsset,
  listAssetGroups,
  listGameAssets,
  updateAssetGroup,
  updateGameAsset,
  type AssetGroup,
  type GameAsset,
} from '@/lib/admin/assets'
import { useAuthStore } from '@/stores/auth'
import { getErrorMessage, isUnauthorizedError } from '@/lib/utils/errorHandling'
import { isValidHttpUrl } from '@/lib/utils/validation'

export interface GroupFormState {
  gameCode: string
  label: string
}

export interface AssetFormState {
  label: string
  imageUrl: string
  isCorrect: boolean
}

export function useAdminAssets() {
  const router = useRouter()
  const auth = useAuthStore()

  const groups = ref<AssetGroup[]>([])
  const assets = ref<GameAsset[]>([])
  const selectedGroupId = ref<number | null>(null)

  const groupFormState = reactive<GroupFormState>({
    gameCode: '',
    label: '',
  })

  const assetFormState = reactive<AssetFormState>({
    label: '',
    imageUrl: '',
    isCorrect: false,
  })

  const editingGroupId = ref<number | null>(null)
  const editingAssetId = ref<number | null>(null)

  const isGroupsLoading = ref(false)
  const isAssetsLoading = ref(false)
  const isGroupSubmitting = ref(false)
  const isAssetSubmitting = ref(false)
  const deletingGroupId = ref<number | null>(null)
  const deletingAssetId = ref<number | null>(null)

  const groupErrorMessage = ref('')
  const assetErrorMessage = ref('')

  let assetFetchId = 0

  const selectedGroup = computed(
    () => groups.value.find((group) => group.id === selectedGroupId.value) ?? null,
  )

  function handleAuthError(error: unknown): void {
    if (isUnauthorizedError(error)) {
      auth.logout()
      void router.push({
        path: '/login',
        query: { redirect: '/admin/assets', sessionExpired: '1' },
      })
    }
  }

  function getAccessTokenOrThrow(): string {
    const accessToken = auth.accessToken
    if (!accessToken) {
      throw new ApiError('No active session. Please sign in again.', 401)
    }
    return accessToken
  }

  function resetGroupForm(): void {
    groupFormState.gameCode = ''
    groupFormState.label = ''
  }

  function resetAssetForm(): void {
    assetFormState.label = ''
    assetFormState.imageUrl = ''
    assetFormState.isCorrect = false
  }

  function validateGroupForm(state: Partial<GroupFormState>): FormError[] {
    const errors: FormError[] = []

    if (!state.gameCode?.trim()) {
      errors.push({ name: 'gameCode', message: 'Game code is required.' })
    }

    if (!state.label?.trim()) {
      errors.push({ name: 'label', message: 'Label is required.' })
    }

    return errors
  }

  function validateAssetForm(state: Partial<AssetFormState>): FormError[] {
    const errors: FormError[] = []

    if (!state.label?.trim()) {
      errors.push({ name: 'label', message: 'Label is required.' })
    }

    if (!state.imageUrl?.trim()) {
      errors.push({ name: 'imageUrl', message: 'Image URL is required.' })
    } else if (!isValidHttpUrl(state.imageUrl.trim())) {
      errors.push({ name: 'imageUrl', message: 'Please enter a valid HTTP or HTTPS URL.' })
    }

    return errors
  }

  function getCreatedResourceId(id: string | number): number | null {
    const parsed = typeof id === 'number' ? id : Number(id)
    return Number.isInteger(parsed) ? parsed : null
  }

  async function refreshGroups(): Promise<void> {
    isGroupsLoading.value = true
    groupErrorMessage.value = ''

    try {
      const loadedGroups = await listAssetGroups(getAccessTokenOrThrow())
      groups.value = loadedGroups

      if (loadedGroups.length === 0) {
        selectedGroupId.value = null
        assets.value = []
        return
      }

      if (
        selectedGroupId.value === null ||
        !loadedGroups.some((group) => group.id === selectedGroupId.value)
      ) {
        selectedGroupId.value = loadedGroups[0]?.id ?? null
      }
    } catch (error) {
      handleAuthError(error)
      groups.value = []
      selectedGroupId.value = null
      assets.value = []
      groupErrorMessage.value = getErrorMessage(error, 'Failed to load asset groups.')
    } finally {
      isGroupsLoading.value = false
    }
  }

  async function refreshAssets(): Promise<void> {
    const groupId = selectedGroupId.value
    if (groupId === null) {
      assets.value = []
      return
    }

    const currentFetchId = ++assetFetchId

    isAssetsLoading.value = true
    assetErrorMessage.value = ''

    try {
      const loadedAssets = await listGameAssets(groupId, getAccessTokenOrThrow())
      if (currentFetchId === assetFetchId) {
        assets.value = loadedAssets
      }
    } catch (error) {
      handleAuthError(error)
      if (currentFetchId === assetFetchId) {
        assets.value = []
        assetErrorMessage.value = getErrorMessage(error, 'Failed to load game assets.')
      }
    } finally {
      if (currentFetchId === assetFetchId) {
        isAssetsLoading.value = false
      }
    }
  }

  function selectGroup(groupId: number): void {
    selectedGroupId.value = groupId
  }

  function startGroupEdit(group: AssetGroup): void {
    editingGroupId.value = group.id
    groupFormState.gameCode = group.game_code
    groupFormState.label = group.label
  }

  function cancelGroupEdit(): void {
    editingGroupId.value = null
    resetGroupForm()
  }

  async function onGroupSubmit(event: FormSubmitEvent<GroupFormState>): Promise<void> {
    isGroupSubmitting.value = true
    groupErrorMessage.value = ''

    try {
      const accessToken = getAccessTokenOrThrow()
      const payload = {
        game_code: event.data.gameCode.trim(),
        label: event.data.label.trim(),
      }

      if (editingGroupId.value !== null) {
        await updateAssetGroup(editingGroupId.value, payload, accessToken)
        await refreshGroups()
        cancelGroupEdit()
        return
      }

      const created = await createAssetGroup(payload, accessToken)
      await refreshGroups()

      const createdId = getCreatedResourceId(created.id)
      if (createdId !== null && groups.value.some((group) => group.id === createdId)) {
        selectedGroupId.value = createdId
      }

      resetGroupForm()
    } catch (error) {
      handleAuthError(error)
      groupErrorMessage.value = getErrorMessage(error, 'Saving asset group failed.')
    } finally {
      isGroupSubmitting.value = false
    }
  }

  async function onGroupDelete(group: AssetGroup): Promise<void> {
    if (!window.confirm(`Delete group "${group.label}"? This also deletes all related assets.`)) {
      return
    }

    deletingGroupId.value = group.id
    groupErrorMessage.value = ''

    try {
      await deleteAssetGroup(group.id, getAccessTokenOrThrow())
      await refreshGroups()
    } catch (error) {
      handleAuthError(error)
      groupErrorMessage.value = getErrorMessage(error, 'Deleting asset group failed.')
    } finally {
      deletingGroupId.value = null
    }
  }

  function startAssetEdit(asset: GameAsset): void {
    editingAssetId.value = asset.id
    assetFormState.label = asset.label
    assetFormState.imageUrl = asset.image_url
    assetFormState.isCorrect = asset.is_correct
  }

  function cancelAssetEdit(): void {
    editingAssetId.value = null
    resetAssetForm()
  }

  async function onAssetSubmit(event: FormSubmitEvent<AssetFormState>): Promise<void> {
    const groupId = selectedGroupId.value
    if (groupId === null) {
      assetErrorMessage.value = 'Select an asset group first.'
      return
    }

    isAssetSubmitting.value = true
    assetErrorMessage.value = ''

    try {
      const accessToken = getAccessTokenOrThrow()

      if (editingAssetId.value !== null) {
        await updateGameAsset(
          editingAssetId.value,
          {
            label: event.data.label.trim(),
            image_url: event.data.imageUrl.trim(),
            is_correct: event.data.isCorrect,
          },
          accessToken,
        )
        await refreshAssets()
        cancelAssetEdit()
        return
      }

      await createGameAsset(
        {
          group_id: groupId,
          label: event.data.label.trim(),
          image_url: event.data.imageUrl.trim(),
          is_correct: event.data.isCorrect,
        },
        accessToken,
      )
      await refreshAssets()
      resetAssetForm()
    } catch (error) {
      handleAuthError(error)
      assetErrorMessage.value = getErrorMessage(error, 'Saving game asset failed.')
    } finally {
      isAssetSubmitting.value = false
    }
  }

  async function onAssetDelete(asset: GameAsset): Promise<void> {
    if (!window.confirm(`Delete asset "${asset.label}"?`)) {
      return
    }

    deletingAssetId.value = asset.id
    assetErrorMessage.value = ''

    try {
      await deleteGameAsset(asset.id, getAccessTokenOrThrow())
      await refreshAssets()
    } catch (error) {
      handleAuthError(error)
      assetErrorMessage.value = getErrorMessage(error, 'Deleting game asset failed.')
    } finally {
      deletingAssetId.value = null
    }
  }

  watch(selectedGroupId, () => {
    cancelAssetEdit()
    void refreshAssets()
  })

  onMounted(() => {
    void refreshGroups()
  })

  return {
    groups,
    assets,
    selectedGroupId,
    groupFormState,
    assetFormState,
    editingGroupId,
    editingAssetId,
    isGroupsLoading,
    isAssetsLoading,
    isGroupSubmitting,
    isAssetSubmitting,
    deletingGroupId,
    deletingAssetId,
    groupErrorMessage,
    assetErrorMessage,
    selectedGroup,
    validateGroupForm,
    validateAssetForm,
    refreshGroups,
    refreshAssets,
    selectGroup,
    startGroupEdit,
    cancelGroupEdit,
    onGroupSubmit,
    onGroupDelete,
    startAssetEdit,
    cancelAssetEdit,
    onAssetSubmit,
    onAssetDelete,
  }
}
