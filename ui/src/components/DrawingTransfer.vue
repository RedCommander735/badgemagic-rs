<script setup lang="ts">

import CloseFilled from "@vicons/material/CloseFilled";
import ModeEditFilled from "@vicons/material/ModeEditFilled";
import ListComponent from "./ListComponent.vue";
import {MessageTransferEmits, MessageTransferProps} from "./TextTransfer.vue";
import {invoke} from "@tauri-apps/api/core";
import {createDiscreteApi} from "naive-ui";


let props = defineProps<MessageTransferProps>()
let db = props.db;

let emit = defineEmits<MessageTransferEmits>()

let editMode = ref(false);
let editId: number = 0;

let matrix = reactive<boolean[]>([])

const WIDTH = 44
const HEIGHT = 11

for (let h = 0; h < WIDTH * HEIGHT; h++) {
  matrix.push(false)
}

function toggleValue(index: number) {
  matrix[index] = !matrix[index]
}


let userMessage = createDiscreteApi(['message'])

interface DrawableMessage {
  drawable: boolean[]
  width: number
  speed: number
  animation: string
  effects: string[]
}
function setDrawable() {
  invoke("set_drawable", {
    drawable: matrix,
    width: WIDTH,
    speed: 4,
    animation: 'center',
    effects: []
  });
}

function addMessage() {
  if (props.messages.length >= 8) {
    userMessage.message.error('No more than 8 messages can be held at a time.')
    return
  }

  id--;

  let message: Message = {
    id: id,
    text: text.value,
    speed: speed.value,
    animation: animation.value,
    effects: effects.value,
    font: font.value,
    font_subtype: subtype.value,
    m_type: 'text',
  };

  emit('add', message)
}

function deleteMessage(m: Message) {
  emit('delete', m)
}

function editMessage(m: Message) {
  editId = m.id;
  editMode.value = true;

  text.value = m.text;
  speed.value = m.speed;
  animation.value = m.animation;
  effects.value = m.effects;
  font.value = m.font;
  subtype.value = m.font_subtype;
}

function updateMessage() {
  emit('update', editId, text.value, speed.value, animation.value, effects.value, font.value, subtype.value)

  editMode.value = false;
  editId = 0;
}
</script>

<template>
  <DrawableMatrix :width="WIDTH" :height="HEIGHT" :values="matrix" @toggleValueAtIndex="toggleValue"/>

  <DrawableMatrix :width="WIDTH" :height="HEIGHT" :values="matrix" @toggleValueAtIndex="toggleValue" :pixelSpacing="1" :pixelSize="4" :border="false"/>
  <n-button type="primary" @click="setDrawable">
    Push
  </n-button>
  <n-divider dashed/>
  <n-flex v-if="props.messages.length > 0"
          :style="{ borderRadius: '6px', border: '1px solid #3E3E42', padding: '12px' }"
          vertical>
    <n-flex v-for="message in props.messages" :key="message.id" :style="{ alignItems: 'center' }">
      <ListComponent :message="message">
        <ListComponentButton :message="message" :icon="CloseFilled" :function="deleteMessage" />
        <ListComponentButton :message="message" :icon="ModeEditFilled" :function="editMessage" :disabled="editMode" />
      </ListComponent>
    </n-flex>
    <n-button type="primary" @click="setMessages">
      Push all messages
    </n-button>
  </n-flex>
</template>

<style scoped>

</style>