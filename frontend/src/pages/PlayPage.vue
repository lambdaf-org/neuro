<script setup lang="ts">
import ProtectedNav from '@/components/ProtectedNav.vue'
</script>

<template>
  <div class="min-h-svh">
    <ProtectedNav />

    <main class="mx-auto w-full max-w-5xl px-4 py-10 sm:px-6">
      <UCard variant="subtle" class="border border-default/60">
        <template #header>
          <div class="space-y-1">
            <p class="text-xs font-semibold uppercase tracking-[0.16em] text-secondary">Play</p>
            <h1 class="text-2xl font-semibold text-highlighted">Game Hub</h1>
          </div>
        </template>
      </UCard>
    </main>
  </div>
</template>
