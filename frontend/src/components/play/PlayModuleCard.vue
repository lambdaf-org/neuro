<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'

import type { PlayModule } from '@/lib/play/modules'

const props = defineProps<{
  module: PlayModule
}>()

const router = useRouter()
const routeTarget = computed(() => `/play/${props.module.id}`)

function isInteractiveTarget(target: EventTarget | null): boolean {
  if (!(target instanceof Element)) {
    return false
  }

  // Check if click is on an interactive child element (exclude role="link" to allow card clicks)
  return Boolean(
    target.closest(
      'a, button, input, textarea, select, [role="button"], [data-interactive="true"]',
    ),
  )
}

function openModule(): void {
  void router.push(routeTarget.value)
}

function onCardClick(event: MouseEvent): void {
  if (isInteractiveTarget(event.target)) {
    return
  }

  openModule()
}

function onCardKeydown(event: KeyboardEvent): void {
  if (event.key !== 'Enter' && event.key !== ' ') {
    return
  }

  event.preventDefault()
  event.stopPropagation()
  openModule()
}
</script>

<template>
  <div
    class="play-module-tile group relative h-full cursor-pointer rounded-xl border border-default/50 bg-elevated/40 p-5 backdrop-blur-sm transition-all"
    tabindex="0"
    role="link"
    @click="onCardClick"
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

      <div class="pt-2">
        <UButton block :to="routeTarget" color="primary" variant="soft" data-interactive="true">
          Open module
        </UButton>
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
