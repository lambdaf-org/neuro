<script setup lang="ts">
import AssetGroupsPanel from '@/components/admin/AssetGroupsPanel.vue'
import GameAssetsPanel from '@/components/admin/GameAssetsPanel.vue'
import ProtectedNav from '@/components/ProtectedNav.vue'
import { useAdminAssets } from '@/composables/useAdminAssets'

const {
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
} = useAdminAssets()
</script>

<template>
  <div class="min-h-svh">
    <ProtectedNav />

    <main class="mx-auto w-full max-w-6xl px-4 py-8 sm:px-6">
      <UCard variant="subtle" class="border border-default/60">
        <template #header>
          <div class="space-y-1">
            <p class="text-xs font-semibold uppercase tracking-[0.16em] text-secondary">Admin</p>
            <h1 class="text-2xl font-semibold text-highlighted">Asset Management</h1>
            <p class="text-sm text-toned">Manage `asset_groups` and `game_assets`.</p>
          </div>
        </template>

        <div class="grid gap-6 lg:grid-cols-2">
          <AssetGroupsPanel
            :groups="groups"
            :selected-group-id="selectedGroupId"
            v-model:group-form-state="groupFormState"
            :editing-group-id="editingGroupId"
            :is-groups-loading="isGroupsLoading"
            :is-group-submitting="isGroupSubmitting"
            :deleting-group-id="deletingGroupId"
            :group-error-message="groupErrorMessage"
            :validate-group-form="validateGroupForm"
            :refresh-groups="refreshGroups"
            :select-group="selectGroup"
            :start-group-edit="startGroupEdit"
            :cancel-group-edit="cancelGroupEdit"
            :on-group-submit="onGroupSubmit"
            :on-group-delete="onGroupDelete"
          />

          <GameAssetsPanel
            :selected-group="selectedGroup"
            :selected-group-id="selectedGroupId"
            :assets="assets"
            v-model:asset-form-state="assetFormState"
            :editing-asset-id="editingAssetId"
            :is-assets-loading="isAssetsLoading"
            :is-asset-submitting="isAssetSubmitting"
            :deleting-asset-id="deletingAssetId"
            :asset-error-message="assetErrorMessage"
            :validate-asset-form="validateAssetForm"
            :refresh-assets="refreshAssets"
            :start-asset-edit="startAssetEdit"
            :cancel-asset-edit="cancelAssetEdit"
            :on-asset-submit="onAssetSubmit"
            :on-asset-delete="onAssetDelete"
          />
        </div>
      </UCard>
    </main>
  </div>
</template>
