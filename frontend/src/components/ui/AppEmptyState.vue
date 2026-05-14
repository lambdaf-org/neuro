<script setup lang="ts">
import type { ButtonProps } from '@nuxt/ui'

withDefaults(
  defineProps<{
    icon?: string
    title: string
    description?: string
    actions?: ButtonProps[]
  }>(),
  {
    icon: 'i-lucide-inbox',
    description: undefined,
    actions: undefined,
  },
)
</script>

<template>
  <UEmpty
    variant="subtle"
    :icon="icon"
    :title="title"
    :description="description"
    :actions="actions"
  />
</template>
