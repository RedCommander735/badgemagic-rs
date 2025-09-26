<script setup lang="ts">
import Database from "@tauri-apps/plugin-sql";
import {Message} from "../App.vue";
import ListComponent from "./ListComponent.vue";
import AddSharp from '@vicons/material/AddSharp'
import {createDiscreteApi} from "naive-ui";
import {UnwrapRef} from "vue";

interface SavedMessagesProps {
  db: Database,
  messages: UnwrapRef<Message[]>
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

interface SavedMessagesEmits {
  add: [message: Message]
}

let props = defineProps<SavedMessagesProps>()
let emit = defineEmits<SavedMessagesEmits>()

let userMessage = createDiscreteApi(['message'])

let dbMessages = await props.db.select<DBMessage[]>(
    "select m.id, tm.content as text, tm.speed, tm.animation, tm.effects, tm.font_size, m.type as m_type \
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

function addToBuffer(m: Message) {
  if (props.messages.length >= 8) {
    userMessage.message.error('No more than 8 messages can be held at a time.')
    return
  }

  userMessage.message.success('Successfully added message to buffer.')

  emit('add', m)
}
</script>

<template>
  <n-flex v-if="messages.length > 0"
          :style="{ borderRadius: '6px', border: '1px solid #3E3E42', padding: '12px'}"
          vertical>
    <n-flex v-for="message in messages" :key="message.id">
      <ListComponent :message="message" >
        <ListComponentButton :message="message" :icon="AddSharp" :function="addToBuffer"/>
      </ListComponent>
      <!-- TODO Button "Add to selection", Button "Delete", (Button "Edit") -->
    </n-flex>
  </n-flex>
</template>

<style scoped>

</style>