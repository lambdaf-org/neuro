<script setup lang="ts">
import type { FormError, FormSubmitEvent } from '@nuxt/ui'

import AppEmptyState from '@/components/ui/AppEmptyState.vue'
import AppErrorState from '@/components/ui/AppErrorState.vue'
import AppLoadingState from '@/components/ui/AppLoadingState.vue'
import type { AssetGroup } from '@/lib/admin/assets'
import type { GroupFormState } from '@/composables/useAdminAssets'

const groupFormState = defineModel<GroupFormState>('groupFormState', { required: true })

defineProps<{
  groups: AssetGroup[]
  selectedGroupId: number | null
  editingGroupId: number | null
  isGroupsLoading: boolean
  isGroupSubmitting: boolean
  deletingGroupId: number | null
  groupErrorMessage: string
  validateGroupForm: (state: Partial<GroupFormState>) => FormError[]
  refreshGroups: () => Promise<void>
  selectGroup: (groupId: number) => void
  startGroupEdit: (group: AssetGroup) => void
  cancelGroupEdit: () => void
  onGroupSubmit: (event: FormSubmitEvent<GroupFormState>) => Promise<void>
  onGroupDelete: (group: AssetGroup) => Promise<void>
}>()
</script>

<template>
  <section class="space-y-4">
    <div class="flex items-center justify-between gap-2">
      <h2 class="text-lg font-semibold text-highlighted">Asset Groups</h2>
      <UButton color="neutral" variant="outline" :loading="isGroupsLoading" @click="refreshGroups">
        Reload
      </UButton>
    </div>

    <AppErrorState
      v-if="groupErrorMessage"
      title="Group operation failed"
      :description="groupErrorMessage"
      retry-label="Reload"
      @retry="refreshGroups"
    />

    <UForm
      :state="groupFormState"
      :validate="validateGroupForm"
      class="space-y-3"
      @submit="onGroupSubmit"
    >
      <UFormField name="gameCode" label="Game Code" required>
        <UInput v-model="groupFormState.gameCode" placeholder="gv" class="w-full" />
      </UFormField>

      <UFormField name="label" label="Label" required>
        <UInput v-model="groupFormState.label" placeholder="group_00" class="w-full" />
      </UFormField>

      <div class="flex gap-2">
        <UButton type="submit" :loading="isGroupSubmitting">
          {{ editingGroupId !== null ? 'Update Group' : 'Create Group' }}
        </UButton>
        <UButton
          v-if="editingGroupId !== null"
          type="button"
          color="neutral"
          variant="outline"
          @click="cancelGroupEdit"
        >
          Cancel
        </UButton>
      </div>
    </UForm>

    <div class="space-y-2">
      <AppLoadingState v-if="isGroupsLoading && groups.length === 0" variant="list" :count="4" />

      <AppEmptyState
        v-else-if="groups.length === 0"
        icon="i-lucide-folder-open"
        title="No groups available"
        description="Create your first asset group."
      />

      <div v-else class="space-y-2">
        <div
          v-for="group in groups"
          :key="group.id"
          class="flex flex-wrap items-center justify-between gap-3 rounded-md border border-default/70 bg-elevated p-3"
        >
          <div>
            <p class="font-medium text-highlighted">{{ group.label }}</p>
            <p class="text-sm text-toned">{{ group.game_code }}</p>
          </div>

          <div class="flex flex-wrap gap-2">
            <UButton
              size="xs"
              :variant="selectedGroupId === group.id ? 'solid' : 'outline'"
              color="neutral"
              @click="selectGroup(group.id)"
            >
              Select
            </UButton>
            <UButton size="xs" color="neutral" variant="outline" @click="startGroupEdit(group)"
              >Edit</UButton
            >
            <UButton
              size="xs"
              color="error"
              variant="soft"
              :loading="deletingGroupId === group.id"
              @click="onGroupDelete(group)"
            >
              Delete
            </UButton>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>
