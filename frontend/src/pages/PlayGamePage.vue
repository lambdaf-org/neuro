<script setup lang="ts">
import { computed, defineAsyncComponent } from 'vue'
import { useRoute } from 'vue-router'

import ProtectedNav from '@/components/ProtectedNav.vue'
import { useGameMetadata } from '@/composables/useGameMetadata'
import type { GameId } from '@/lib/play/modules'
import { findPlayModuleById } from '@/lib/play/modules'

/**
 * Game component registry.
 * To add a new game: create the component, then add one line here.
 * PlayGamePage never needs to change again.
 */
const GAME_REGISTRY: Partial<Record<GameId, ReturnType<typeof defineAsyncComponent>>> = {
  'reaction-time': defineAsyncComponent(() => import('@/components/play/ReactionTestGame.vue')),
  'symbol-matching': defineAsyncComponent(
    () => import('@/components/play/ProcessingSpeedGame.vue'),
  ),
  'pattern-logic': defineAsyncComponent(
    () => import('@/components/play/FluidIntelligenceGame.vue'),
  ),
  'sequence-memory': defineAsyncComponent(() => import('@/components/play/WorkingMemoryGame.vue')),
  'mental-rotation': defineAsyncComponent(() => import('@/components/play/MentalRotationGame.vue')),
}

const route = useRoute()

const gameId = computed(() => {
  const raw = route.params.gameId
  if (typeof raw === 'string') return raw
  return Array.isArray(raw) ? raw[0] || '' : ''
})

const selectedModule = computed(() => findPlayModuleById(gameId.value))
const gameComponent = computed(() => GAME_REGISTRY[gameId.value as GameId] ?? null)
const comingSoonModule = computed(() =>
  selectedModule.value && !gameComponent.value ? selectedModule.value : null,
)
const backendGameCode = computed(() => selectedModule.value?.chcCode.toLowerCase() ?? '')
const {
  metadata,
  isLoading: isMetadataLoading,
  errorMessage: metadataError,
} = useGameMetadata({
  gameCode: backendGameCode,
})
const pageChcCode = computed(
  () => metadata.value?.chc_factor || selectedModule.value?.chcCode || '',
)
const pageTitle = computed(() => metadata.value?.display_name || selectedModule.value?.name || '')
</script>

<template>
  <div class="min-h-svh">
    <ProtectedNav />

    <main class="mx-auto w-full max-w-3xl px-4 py-8 sm:px-6">
      <div class="space-y-6">
        <!-- Page header -->
        <div class="flex items-center gap-3">
          <UButton to="/play" variant="ghost" color="neutral" size="sm" icon="i-lucide-arrow-left">
            Back
          </UButton>
          <template v-if="selectedModule">
            <span class="text-default/30">/</span>
            <UBadge color="primary" variant="soft" size="sm">{{ pageChcCode }}</UBadge>
            <h1 class="text-lg font-semibold text-highlighted">
              {{ pageTitle }}
            </h1>
          </template>
        </div>

        <!-- Module not found -->
        <UAlert
          v-if="!selectedModule"
          color="error"
          variant="soft"
          title="Module not found"
          :description="`No module registered for '${gameId}'.`"
        />

        <UAlert
          v-if="selectedModule && metadataError"
          color="warning"
          variant="soft"
          title="Game details unavailable"
          :description="metadataError"
        />

        <USkeleton v-if="selectedModule && isMetadataLoading" class="h-96 w-full rounded-xl" />

        <!-- Registered game component -->
        <component
          v-else-if="gameComponent"
          :is="gameComponent"
          :game-code="backendGameCode"
          :metadata="metadata"
        />

        <!-- Module exists but no component registered yet -->
        <div
          v-else-if="comingSoonModule"
          class="flex min-h-72 flex-col items-center justify-center gap-3 rounded-xl border border-default/50 bg-elevated/30 p-10 text-center backdrop-blur-sm"
        >
          <div
            class="inline-flex h-12 w-12 items-center justify-center rounded-xl border border-secondary/30 bg-secondary/10"
          >
            <UIcon :name="comingSoonModule.icon" class="h-6 w-6 text-secondary" />
          </div>
          <p class="font-semibold text-highlighted">{{ comingSoonModule.name }}</p>
          <p class="text-sm text-toned">This module is coming soon.</p>
        </div>
      </div>
    </main>
  </div>
</template>
