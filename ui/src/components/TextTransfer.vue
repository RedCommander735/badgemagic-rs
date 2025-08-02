<script lang="ts" setup>
import {invoke} from "@tauri-apps/api/core";
import { createDiscreteApi } from 'naive-ui'

export interface Message {
  id: number
  text: string
  speed: number
  animation: string
  effects: string[]
  font_size: number
  m_type: string
}

let userMessage = createDiscreteApi(['message'])

let animationOptions = ref([
  {value: 'left', label: 'Left'},
  {value: 'right', label: 'Right'},
  {value: 'up', label: 'Up'},
  {value: 'down', label: 'Down'},
  {value: 'center', label: 'Center'},
  {value: 'fast', label: 'Fast'},
  {value: 'drop', label: 'Drop'},
  {value: 'curtain', label: 'Curtain'},
  {value: 'laser', label: 'Laser'},
])

let effectsOptions = ref([
  {value: 'inverted', label: 'Inverted'},
  {value: 'flashing', label: 'Flashing'},
  {value: 'border', label: 'Border'},
]);

let text = ref("");
let speed = ref(4);
let animation = ref('left');
let effects = ref([]);
let fontSize = ref(9);

let id = ref(0);
let messages = ref<Message[]>([]);

function capitalize(str: string) {
  return str.charAt(0).toUpperCase() + str.slice(1);
}

function setText() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  invoke("set_text", {
    text: text.value,
    speed: speed.value,
    animation: animation.value,
    effects: effects.value,
    fontSize: fontSize.value
  });
}

function setMessages() {
  invoke("set_messages", {
    messages: messages.value,
  });
}

function addMessage() {
  if (messages.value.length >= 8) {
    userMessage.message.error('No more than 8 messages can be held at a time.')
    return
  }

  let message: Message = {
    id: id.value,
    text: text.value,
    speed: speed.value,
    animation: animation.value,
    effects: effects.value,
    font_size: fontSize.value,
    m_type: 'text',
  };

  id.value++;
  messages.value.push(message);
}

function deleteMessage(m: Message) {
  messages.value = messages.value.filter((x: Message) => x !== m);
}
</script>

<template>
  <n-flex vertical>
    <!-- Text input -->
    <n-input v-model:value="text" :style="{ width: '750px' }" clearable placeholder="Text"
             type="text"/>

    <!-- Text input settings -->
    <n-flex
        :style="{ width: 'calc(750px - 2 * 12px)', borderRadius: '6px', border: '1px solid #3E3E42', padding: '12px'}">
      <n-input-group>
        <n-input-group-label>Speed</n-input-group-label>
        <n-input-number v-model:value="speed" :max="8" :min="1"
                        :style="{ width: '80px' }" class="noStrikethrough"/>
      </n-input-group>
      <n-input-group>
        <n-input-group-label>Animation</n-input-group-label>
        <n-select v-model:value="animation" :options="animationOptions" :style="{ width: '100px' }"/>
      </n-input-group>
      <n-input-group>
        <n-input-group-label>Effects</n-input-group-label>
        <n-select v-model:value="effects" :options="effectsOptions" :style="{ width: '320px' }" multiple
                  placeholder="Select additional effects"/>
      </n-input-group>
      <n-input-group>
        <n-input-group-label>Font size</n-input-group-label>
        <n-input-number v-model:value="fontSize" :max="9" :min="8"
                        :style="{ width: '80px' }" class="noStrikethrough"/>
      </n-input-group>
    </n-flex>
    <n-flex>
      <n-button type="primary" @click="setText">
        Push
      </n-button>
      <n-button type="primary" @click="addMessage">
        Add to messages
      </n-button>
      <n-button type="primary" @click="saveMessage">
        Save Message
      </n-button>
    </n-flex>
    <n-flex vertical
        :style="{ width: 'calc(750px - 2 * 12px)', borderRadius: '6px', border: '1px solid #3E3E42', padding: '12px'}">
      <n-flex v-for="message in messages" :key="message.id">
        <p>"{{ message.text }}" | Speed: {{ message.speed }} | Animation: {{capitalize(message.animation) }} | Effects:
          {{ message.effects.length > 0 ? message.effects.join(", ") : "None" }} | Font size: {{ message.fontSize }}</p>
        <p @click="deleteMessage(message)" :style="{cursor: 'pointer'}">x</p>
      </n-flex>
      <n-button type="primary" @click="setMessages" v-if="messages.length > 0">
        Push all messages
      </n-button>
    </n-flex>
  </n-flex>
</template>

<style scoped>
.noStrikethrough input {
  text-decoration-line: none !important;
}

.n-input-group {
  width: unset;
}
</style>