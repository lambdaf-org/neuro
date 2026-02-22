<script setup lang="ts">
import ProtectedNav from '@/components/ProtectedNav.vue'
import PlayModuleCard from '@/components/play/PlayModuleCard.vue'
import { PLAY_MODULES } from '@/lib/play/modules'
</script>

<template>
  <div class="min-h-svh">
    <ProtectedNav />

    <main class="mx-auto w-full max-w-6xl px-4 py-10 sm:px-6">
      <section class="space-y-6">
        <header class="space-y-2">
          <p class="text-xs font-semibold uppercase tracking-[0.18em] text-secondary/90">
            Play Hub
          </p>
          <h1 class="text-2xl font-semibold text-highlighted sm:text-3xl">Cognitive Modules</h1>
          <p class="text-sm text-toned sm:text-base">Select a CHC-based benchmark.</p>
        </header>

        <div class="module-grid grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
          <PlayModuleCard v-for="module in PLAY_MODULES" :key="module.id" :module="module" />
        </div>
      </section>
    </main>
  </div>
</template>

<style scoped>
.module-grid > * {
  animation: module-enter 320ms ease-out both;
}

.module-grid > *:nth-child(2) {
  animation-delay: 45ms;
}

.module-grid > *:nth-child(3) {
  animation-delay: 80ms;
}

.module-grid > *:nth-child(4) {
  animation-delay: 115ms;
}

.module-grid > *:nth-child(5) {
  animation-delay: 150ms;
}

@keyframes module-enter {
  from {
    opacity: 0;
    transform: translateY(10px);
  }

  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>
