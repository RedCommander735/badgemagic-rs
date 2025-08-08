<script setup lang="ts">
import Database from "@tauri-apps/plugin-sql";
import {Message} from "./TextTransfer.vue";
import {effect} from "vue";

interface SavedMessagesProps {
  db: Database
}

interface DBMessage {
  id: number
  text: string
  speed: number
  animation: string
  effects: string
  font_size: number
  m_type: string
}

let props = defineProps<SavedMessagesProps>()

let dbMessages = await props.db.select<DBMessage[]>(
    "select tm.id, tm.content as text, tm.speed, tm.animation, tm.effects, tm.font_size, m.type as m_type \
    from messages m join text_messages tm on  m.content_id = tm.id"
)

let messages = ref<Message[]>(dbMessages.map<Message>((m) => {
  return {
    id: m.id,
    text: m.text,
    speed: m.speed,
    animation: m.animation,
    effects: JSON.parse(m.effects),
    font_size: m.font_size,
    m_type: m.m_type
  }
}));
</script>

<template>
  <n-flex v-if="messages.length > 0"
          :style="{ width: 'calc(750px - 2 * 12px)', borderRadius: '6px', border: '1px solid #3E3E42', padding: '12px'}"
          vertical>
    <n-flex v-for="message in messages" :key="message.id">
      <p>"{{ message.text }}" | Speed: {{ message.speed }} | Animation: {{ message.animation }} | Effects:
        {{ message.effects.length > 0 ? message.effects.join(", ") : "None" }} | Font size: {{
          message.font_size
        }}</p>
      <!-- TODO Button "Add to selection", Button "Delete", (Button "Edit") -->
    </n-flex>
  </n-flex>
</template>

<style scoped>

</style>