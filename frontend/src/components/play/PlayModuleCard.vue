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
  <UCard
    variant="subtle"
    tabindex="0"
    role="link"
    :aria-label="`Open ${module.name} module`"
    class="group h-full cursor-pointer border border-default/50 transition-all duration-150 hover:-translate-y-0.5 hover:border-secondary/40 hover:shadow-lg hover:shadow-secondary/10 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-secondary/40"
    :ui="{
      body: 'p-5 sm:p-6',
    }"
    @click="openModule"
    @keydown="onCardKeydown"
  >
    <div class="space-y-4">
      <div class="flex items-start justify-between gap-3">
        <div class="inline-flex h-10 w-10 items-center justify-center rounded-lg border border-secondary/30 bg-secondary/10">
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
  </UCard>
</template>
