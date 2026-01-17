<script lang="ts" setup>

import Database from '@tauri-apps/plugin-sql';
import {createDiscreteApi} from "naive-ui";

export interface Message {
  id: number
  text: string
  speed: number
  animation: string
  effects: string[]
  font: number
  font_subtype: string
  m_type: string
}

const messagesKey = "currentMessages";
let userMessage = createDiscreteApi(['message'])

let db = await Database.load('sqlite:messages.db');
let activeTab = ref('text')

let storedMessages: string | null = localStorage.getItem(messagesKey)
let parsedMessages: Message[] | null = storedMessages ? JSON.parse(storedMessages) : null;
let messages = ref<Message[]>(parsedMessages ? parsedMessages : []);

watch(messages, () => {
  localStorage.setItem(messagesKey, JSON.stringify(messages.value))
  console.log('updated localstorage')
}, {deep: true})

function addMessage(m: Message) {
  if (messages.value.length >= 8) {
    userMessage.message.error('No more than 8 messages can be held at a time.')
    return
  }

  messages.value.push(m)
}
function deleteMessage(m: Message) {
  messages.value = messages.value.filter((x: Message) => x !== m);
}

function updateMessage(id: number, text: string, speed: number, animation: string, effects: string[], font: number, font_subtype: string) {
  for (let m of messages.value) {
    if (m.id === id) {
      m.text = text;
      m.speed = speed;
      m.animation = animation;
      m.effects = effects;
      m.font = font;
      m.font_subtype = font_subtype
      break;
    }
  }
}
</script>

<template>
  <n-flex :style="{ margin: '20px' }">
    <n-tabs default-value="text" type="segment" v-model:value="activeTab">
      <n-tab-pane name="text" tab="Text">
        <TextTransfer :db="db" :messages="messages" @add="addMessage" @delete="deleteMessage" @update="updateMessage"/>
      </n-tab-pane>
      <n-tab-pane name="draw" tab="Draw">
        <DrawingTransfer :db="db" :messages="messages" @add="addMessage" @delete="deleteMessage" @update="updateMessage"/>
      </n-tab-pane>
      <n-tab-pane name="saved" tab="Saved Messages">
        <SavedMessages :db="db" :messages="messages" @add="addMessage"/>
      </n-tab-pane>
    </n-tabs>
  </n-flex>
</template>