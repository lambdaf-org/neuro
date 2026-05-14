<script setup lang="ts">
import ProtectedNav from '@/components/ProtectedNav.vue'
import PlayModuleCard from '@/components/play/PlayModuleCard.vue'
import { PLAY_MODULES } from '@/lib/play/modules'
</script>

<template>
  <div class="min-h-svh">
    <ProtectedNav />

    <main class="mx-auto w-full max-w-6xl px-4 py-10 sm:px-6">
      <section class="space-y-8">
        <header class="space-y-3">
          <p class="text-xs font-semibold uppercase tracking-[0.18em] text-secondary/90">
            Play Hub
          </p>
          <h1 class="text-2xl font-semibold text-highlighted sm:text-3xl">Cognitive Modules</h1>
          <p class="max-w-2xl text-sm leading-6 text-toned sm:text-base">
            Select a CHC-based benchmark.
          </p>
        </header>

        <div class="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
          <PlayModuleCard v-for="module in PLAY_MODULES" :key="module.id" :module="module" />
        </div>
      </section>
    </main>
  </div>
</template>
