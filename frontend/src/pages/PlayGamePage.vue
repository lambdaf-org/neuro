<script setup lang="ts">
import { computed, watch } from 'vue'
import { useRoute } from 'vue-router'

import { useGameRuntime } from '@/composables/useGameRuntime'
import ProtectedNav from '@/components/ProtectedNav.vue'
import GameTimerDisplay from '@/components/play/GameTimerDisplay.vue'
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

const {
  state,
  countdownRemaining,
  elapsedMs,
  formattedElapsed,
  canStart,
  canStop,
  isBusy,
  errorMessage,
  lastResult,
  startGame,
  stopGame,
  resetGame,
} = useGameRuntime({
  gameCode: computed(() => selectedModule.value?.id ?? ''),
})

watch(gameId, () => {
  resetGame()
})

const resultPreview = computed(() => {
  return lastResult.value ? JSON.stringify(lastResult.value, null, 2) : ''
})
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
          class="game-container min-h-150 space-y-6 rounded-xl border border-default/50 bg-elevated/30 p-6 backdrop-blur-sm"
        >
          <div class="grid gap-4 md:grid-cols-[auto_1fr]">
            <div class="space-y-3">
              <div class="flex flex-wrap gap-2">
                <UButton
                  color="primary"
                  :disabled="!canStart"
                  :loading="isBusy"
                  @click="startGame"
                >
                  Start
                </UButton>
                <UButton
                  color="warning"
                  variant="soft"
                  :disabled="!canStop"
                  :loading="isBusy"
                  @click="stopGame"
                >
                  Stop
                </UButton>
                <UButton color="neutral" variant="ghost" @click="resetGame">Reset</UButton>
              </div>
            </div>

            <GameTimerDisplay
              :state="state"
              :elapsed-ms="elapsedMs"
              :countdown-remaining="countdownRemaining"
              :formatted-elapsed="formattedElapsed"
            />
          </div>

          <UAlert
            v-if="errorMessage"
            color="error"
            variant="soft"
            title="Submit failed"
            :description="errorMessage"
          />

          <div class="space-y-2 rounded-xl border border-default/60 bg-muted/20 p-4">
            <p class="text-sm text-toned">
              Dummy game flow: <code>countdown -> running -> finished</code>. You can reuse
              this directly for a reaction-time game by replacing the stop trigger and scoring logic.
            </p>
          </div>

          <div v-if="lastResult" class="space-y-2">
            <p class="text-sm font-medium text-highlighted">Standard Result Payload</p>
            <pre class="result-preview">{{ resultPreview }}</pre>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<style scoped>
.result-preview {
  max-height: 18rem;
  overflow: auto;
  border: 1px solid color-mix(in oklab, var(--ui-border) 70%, transparent);
  border-radius: 0.75rem;
  background: color-mix(in oklab, var(--ui-bg-elevated) 60%, transparent);
  padding: 0.9rem;
  font-size: 0.75rem;
  line-height: 1.4;
}
</style>
