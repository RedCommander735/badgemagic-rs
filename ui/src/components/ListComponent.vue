<script setup lang="ts">
import {Message} from "../App.vue";
import {UnwrapRef} from "vue";

interface ListComponentProps {
  message: UnwrapRef<Message>
}

let props = defineProps<ListComponentProps>()

function capitalize(str: string) {
  return str.charAt(0).toUpperCase() + str.slice(1);
}
</script>

<template>
  <n-flex :style="{ alignItems: 'center', justifyContent: 'space-between', width: '100%' }">
    <p>"{{ props.message.text }}" | Speed: {{ props.message.speed }} | Animation: {{
        capitalize(props.message.animation)
      }} | Effects:
      {{ props.message.effects.length > 0 ? props.message.effects.join(", ") : "None" }} | Font size: {{
        props.message.font_size
      }} | Saved to storage: {{ props.message.id >= 0 }}</p>
    <n-flex>
      <slot/>
    </n-flex>
  </n-flex>
</template>

<style scoped>

</style>