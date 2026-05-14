<script setup lang="ts">
import { computed, defineAsyncComponent } from 'vue'
import { useRoute } from 'vue-router'

import ProtectedNav from '@/components/ProtectedNav.vue'
import AppEmptyState from '@/components/ui/AppEmptyState.vue'
import AppErrorState from '@/components/ui/AppErrorState.vue'
import AppLoadingState from '@/components/ui/AppLoadingState.vue'
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
        <AppErrorState
          v-if="!selectedModule"
          title="Module not found"
          :description="`No module registered for '${gameId}'.`"
        />

        <AppErrorState
          v-if="selectedModule && metadataError"
          color="warning"
          icon="i-lucide-info"
          title="Game details unavailable"
          :description="metadataError"
        />

        <AppLoadingState v-if="selectedModule && isMetadataLoading" />

        <!-- Registered game component -->
        <component
          v-else-if="gameComponent"
          :is="gameComponent"
          :game-code="backendGameCode"
          :metadata="metadata"
        />

        <!-- Module exists but no component registered yet -->
        <div v-else-if="comingSoonModule" class="min-h-72">
          <AppEmptyState
            :icon="comingSoonModule.icon"
            :title="comingSoonModule.name"
            description="This module is coming soon."
          />
        </div>
      </div>
    </main>
  </div>
</template>
