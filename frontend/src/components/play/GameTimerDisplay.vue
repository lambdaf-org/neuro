<script setup lang="ts">
import { computed } from 'vue'

import type { GameState } from '@/lib/play/result'

const props = defineProps<{
  state: GameState
  elapsedMs: number
  countdownRemaining: number
  formattedElapsed: string
}>()

const headline = computed(() => {
  if (props.state === 'countdown') {
    return `Start in ${props.countdownRemaining}`
  }

  if (props.state === 'running') {
    return 'Running'
  }

  if (props.state === 'finished') {
    return 'Finished'
  }

  return 'Ready'
})

const detail = computed(() => {
  if (props.state === 'countdown') {
    return 'Get ready.'
  }

  if (props.state === 'running' || props.state === 'finished') {
    if (props.elapsedMs === 0 && props.state === 'finished') {
      return 'Press start to run the dummy game.'
    }

    return `${props.formattedElapsed} (${props.elapsedMs} ms)`
  }

  return ''
})
</script>

<template>
  <div class="rounded-xl border border-default/60 bg-muted/30 p-5">
    <p class="text-xs font-semibold uppercase tracking-[0.16em] text-toned">Timer</p>
    <p class="mt-2 text-2xl font-semibold text-highlighted">{{ headline }}</p>
    <p class="mt-1 text-sm text-toned">{{ detail }}</p>
  </div>
</template>
