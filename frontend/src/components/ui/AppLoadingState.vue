<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    variant?: 'cards' | 'list' | 'surface'
    count?: number
  }>(),
  {
    variant: 'surface',
    count: 3,
  },
)
</script>

<template>
  <div v-if="variant === 'cards'" class="grid gap-4 md:grid-cols-2">
    <UCard
      v-for="index in props.count"
      :key="index"
      variant="subtle"
      class="border border-default/50"
    >
      <div class="space-y-5">
        <div class="flex items-center justify-between gap-3">
          <div class="flex items-center gap-3">
            <USkeleton class="h-10 w-10 rounded-lg" />
            <div class="space-y-2">
              <USkeleton class="h-4 w-32" />
              <USkeleton class="h-3 w-20" />
            </div>
          </div>
          <USkeleton class="h-6 w-12 rounded-full" />
        </div>
        <div class="grid grid-cols-3 gap-3">
          <USkeleton v-for="col in 3" :key="col" class="h-16 w-full rounded-lg" />
        </div>
      </div>
    </UCard>
  </div>

  <div v-else-if="variant === 'list'" class="space-y-2">
    <USkeleton v-for="index in props.count" :key="index" class="h-12 w-full rounded-md" />
  </div>

  <USkeleton v-else class="h-96 w-full rounded-xl" />
</template>
