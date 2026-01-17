<script setup lang="ts">

import {Reactive} from "vue";

interface ButtonMatrixProps {
  width: number
  values: Reactive<boolean[]>
  pixelSize?: number
  pixelSpacing?: number
  border?: boolean
}
interface TogglablePixelEmits {
  toggleValueAtIndex: [index: number],
}

let { width, values, pixelSize = 16, pixelSpacing = 4, border = true} = defineProps<ButtonMatrixProps>()

defineEmits<TogglablePixelEmits>()

</script>



<template>
  <n-checkbox-group>
    <n-grid :y-gap="0" :cols="width" :style="{ width: `${width * (pixelSize + pixelSpacing)}px`, height: `${values.length / width * (pixelSize + pixelSpacing)}px`}">
      <n-gi v-for="(_, index) in values" :key="index">
        <TogglablePixel :size="pixelSize" :enabled="values[index]" @clicked="$emit('toggleValueAtIndex', index)" :border="border"/>
      </n-gi>
    </n-grid>
  </n-checkbox-group>
</template>

<style scoped>

</style>