<script setup lang="ts">
withDefaults(
  defineProps<{
    title?: string
    description: string
    icon?: string
    color?: 'error' | 'warning' | 'neutral'
    retryLabel?: string
  }>(),
  {
    title: 'Something went wrong',
    icon: 'i-lucide-circle-alert',
    color: 'error',
    retryLabel: undefined,
  },
)

const emit = defineEmits<{
  retry: []
}>()
</script>

<template>
  <UAlert :color="color" variant="soft" :icon="icon" :title="title" :description="description">
    <template v-if="retryLabel" #actions>
      <UButton
        :color="color"
        variant="subtle"
        size="sm"
        icon="i-lucide-refresh-cw"
        @click="emit('retry')"
      >
        {{ retryLabel }}
      </UButton>
    </template>
  </UAlert>
</template>
