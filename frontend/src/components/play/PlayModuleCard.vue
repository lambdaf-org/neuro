<script setup lang="ts">
import { useRouter } from 'vue-router'

import type { PlayModule } from '@/lib/play/modules'

const props = defineProps<{
  module: PlayModule
}>()

const router = useRouter()

function openModule(): void {
  void router.push(`/play/${props.module.id}`)
}

function onCardKeydown(event: KeyboardEvent): void {
  if (event.key !== 'Enter' && event.key !== ' ') {
    return
  }

  event.preventDefault()
  openModule()
}
</script>

<template>
  <div
    class="play-module-tile group relative h-full cursor-pointer rounded-xl border border-default/50 bg-elevated/40 p-5 backdrop-blur-sm transition-all"
    tabindex="0"
    role="link"
    :aria-label="`Open ${module.name} module`"
    @click="openModule"
    @keydown="onCardKeydown"
  >
    <div class="space-y-4">
      <div class="flex items-start justify-between gap-3">
        <div
          class="inline-flex h-10 w-10 items-center justify-center rounded-lg border border-secondary/30 bg-secondary/10"
        >
          <UIcon :name="module.icon" class="h-5 w-5 text-secondary" />
        </div>
        <UBadge color="secondary" variant="soft" size="sm">
          {{ module.chcCode }}
        </UBadge>
      </div>

      <div class="space-y-2">
        <h2 class="text-lg font-semibold text-highlighted">{{ module.name }}</h2>
        <p class="text-sm leading-6 text-toned">{{ module.description }}</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.play-module-tile {
  outline: none;
}

.play-module-tile:hover,
.play-module-tile:focus-visible {
  border-color: color-mix(in oklab, var(--ui-secondary) 30%, var(--ui-border));
  background: color-mix(in oklab, var(--ui-elevated) 50%, transparent);
  box-shadow: 0 8px 16px -8px color-mix(in oklab, var(--ui-secondary) 20%, transparent);
  transform: translateY(-2px);
}
</style>
