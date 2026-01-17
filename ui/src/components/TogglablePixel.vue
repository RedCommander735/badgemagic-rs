<script setup lang="ts">
interface TogglablePixelProps {
  size: number
  enabled: boolean
  border?: boolean
}

interface TogglablePixelEmits {
  clicked: [],
  toggleOn: [],
  toggleOff: []
}

let { size, enabled, border = true} = defineProps<TogglablePixelProps>()

defineEmits<TogglablePixelEmits>()
</script>

<template>
  <div :style="{ height: `${size}px`, aspectRatio: 1, backgroundColor: enabled ? 'red' : '#1F0000', border: border ? '1px solid grey' : 'unset', boxSizing: 'border-box', cursor: 'pointer'}" @click="$emit('clicked')"></div>
</template>

<style scoped>

</style>