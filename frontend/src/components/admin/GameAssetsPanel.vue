<script setup lang="ts">
import type { FormError, FormSubmitEvent } from '@nuxt/ui'

import type { AssetGroup, GameAsset } from '@/lib/admin/assets'
import type { AssetFormState } from '@/composables/useAdminAssets'

const assetFormState = defineModel<AssetFormState>('assetFormState', { required: true })

defineProps<{
  selectedGroup: AssetGroup | null
  selectedGroupId: number | null
  assets: GameAsset[]
  editingAssetId: number | null
  isAssetsLoading: boolean
  isAssetSubmitting: boolean
  deletingAssetId: number | null
  assetErrorMessage: string
  validateAssetForm: (state: Partial<AssetFormState>) => FormError[]
  refreshAssets: () => Promise<void>
  startAssetEdit: (asset: GameAsset) => void
  cancelAssetEdit: () => void
  onAssetSubmit: (event: FormSubmitEvent<AssetFormState>) => Promise<void>
  onAssetDelete: (asset: GameAsset) => Promise<void>
}>()
</script>

<template>
  <section class="space-y-4">
    <div class="flex items-center justify-between gap-2">
      <div>
        <h2 class="text-lg font-semibold text-highlighted">Game Assets</h2>
        <p class="text-sm text-toned">
          {{ selectedGroup ? `Group: ${selectedGroup.label} (${selectedGroup.game_code})` : 'No group selected' }}
        </p>
      </div>
      <UButton
        color="neutral"
        variant="outline"
        :disabled="selectedGroupId === null"
        :loading="isAssetsLoading"
        @click="refreshAssets"
      >
        Reload
      </UButton>
    </div>

    <UAlert
      v-if="assetErrorMessage"
      color="error"
      variant="soft"
      title="Asset operation failed"
      :description="assetErrorMessage"
    />

    <UForm
      :state="assetFormState"
      :validate="validateAssetForm"
      class="space-y-3"
      :disabled="selectedGroupId === null"
      @submit="onAssetSubmit"
    >
      <UFormField name="label" label="Label" required>
        <UInput v-model="assetFormState.label" placeholder="option_1" class="w-full" />
      </UFormField>

      <UFormField name="imageUrl" label="Image URL" required>
        <UInput v-model="assetFormState.imageUrl" type="url" placeholder="https://.../image.svg" class="w-full" />
      </UFormField>

      <UFormField name="isCorrect">
        <UCheckbox v-model="assetFormState.isCorrect" label="Mark as correct answer" />
      </UFormField>

      <div class="flex gap-2">
        <UButton type="submit" :loading="isAssetSubmitting" :disabled="selectedGroupId === null">
          {{ editingAssetId !== null ? 'Update Asset' : 'Create Asset' }}
        </UButton>
        <UButton
          v-if="editingAssetId !== null"
          type="button"
          color="neutral"
          variant="outline"
          @click="cancelAssetEdit"
        >
          Cancel
        </UButton>
      </div>
    </UForm>

    <div v-if="selectedGroupId === null">
      <UAlert
        color="neutral"
        variant="soft"
        title="No group selected"
        description="Select a group first to manage its assets."
      />
    </div>

    <div v-else class="space-y-2">
      <UAlert
        v-if="isAssetsLoading && assets.length === 0"
        color="neutral"
        variant="soft"
        title="Loading assets..."
      />

      <UAlert
        v-else-if="assets.length === 0"
        color="neutral"
        variant="soft"
        title="No assets available"
        description="Create the first asset for this group."
      />

      <div v-else class="space-y-2">
        <div
          v-for="asset in assets"
          :key="asset.id"
          class="rounded-md border border-default/70 bg-elevated p-3"
        >
          <div class="flex flex-wrap items-start justify-between gap-3">
            <div class="space-y-1">
              <p class="font-medium text-highlighted">
                {{ asset.label }}
                <UBadge v-if="asset.is_correct" color="success" variant="subtle" size="sm" class="ml-2">
                  Correct
                </UBadge>
              </p>
              <a
                :href="asset.image_url"
                target="_blank"
                rel="noreferrer"
                class="break-all text-sm text-primary hover:underline"
              >
                {{ asset.image_url }}
              </a>
            </div>

            <div class="flex flex-wrap gap-2">
              <UButton size="xs" color="neutral" variant="outline" @click="startAssetEdit(asset)">Edit</UButton>
              <UButton
                size="xs"
                color="error"
                variant="soft"
                :loading="deletingAssetId === asset.id"
                @click="onAssetDelete(asset)"
              >
                Delete
              </UButton>
            </div>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>
