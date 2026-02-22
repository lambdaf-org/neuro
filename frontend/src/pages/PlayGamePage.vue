<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'

import ProtectedNav from '@/components/ProtectedNav.vue'
import { findPlayModuleById } from '@/lib/play/modules'

const route = useRoute()

const gameId = computed(() => {
  const raw = route.params.gameId
  if (typeof raw === 'string') {
    return raw
  }
  // Handle array case (edge case in Vue Router)
  return Array.isArray(raw) ? raw[0] || '' : ''
})

const selectedModule = computed(() => findPlayModuleById(gameId.value))
</script>

<template>
  <div class="min-h-svh">
    <ProtectedNav />

    <main class="mx-auto w-full max-w-7xl px-4 py-8 sm:px-6">
      <div class="space-y-6">
        <div class="flex items-center gap-4">
          <UButton to="/play" variant="ghost" color="neutral" size="sm" icon="i-lucide-arrow-left">
            Back
          </UButton>
          <div class="flex items-center gap-3" v-if="selectedModule">
            <UBadge color="primary" variant="soft" size="sm">{{ selectedModule.chcCode }}</UBadge>
            <h1 class="text-xl font-semibold text-highlighted sm:text-2xl">
              {{ selectedModule.name }}
            </h1>
          </div>
        </div>

        <UAlert
          v-if="!selectedModule"
          color="error"
          variant="soft"
          title="Module not found"
          :description="`No module is registered for id '${gameId}'.`"
        />

        <div
          v-else
          class="game-container min-h-150 rounded-xl border border-default/50 bg-elevated/30 p-6 backdrop-blur-sm"
        >
          <p class="text-center text-sm text-toned">
            Game interface for {{ selectedModule.name }}.
          </p>
        </div>
      </div>
    </main>
  </div>
</template>
