<script setup lang="ts">
import {UnwrapRef} from "vue";
import {Message} from "./TextTransfer.vue";

interface ListComponentButtonProps {
  message: Message,
  icon: Component
  function: (m: Message) => void,
  disabled?: UnwrapRef<boolean>,
}

let props = defineProps<ListComponentButtonProps>()
</script>

<template>
  <n-button type="primary" @click="props.function(props.message)" :style="{padding: '0 10px'}" :disabled="disabled">
    <n-icon>
      <props.icon/>
    </n-icon>
  </n-button>
</template>

<style scoped>

</style>