<script lang="ts" setup>
import {invoke} from "@tauri-apps/api/core";
import {createDiscreteApi} from 'naive-ui'
import Database from "@tauri-apps/plugin-sql";
import CloseFilled from '@vicons/material/CloseFilled'
import ModeEditFilled from '@vicons/material/ModeEditFilled'
import ListComponent from "./ListComponent.vue";
import {Message} from "../App.vue";
import {UnwrapRef} from "vue";

export interface MessageTransferProps {
  db: Database,
  messages: UnwrapRef<Message[]>
}

export interface MessageTransferEmits {
  add: [message: Message],
  delete: [message: Message],
  update: [id: number, text: string, speed: number, animation: string, effects: string[], font: number, subtype: string]
}

interface Id {
  id: number
}

// TODO Move Id up to app

let props = defineProps<MessageTransferProps>()
let db = props.db;

let emit = defineEmits<MessageTransferEmits>()

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

let effectsOptionsWithInvert = ref([
  {value: 'inverted', label: 'Inverted'},
  {value: 'flashing', label: 'Flashing'},
  {value: 'border', label: 'Border'},
]);

let effectsOptionsNoInvert = ref([
  {value: 'inverted', label: 'Inverted'},
  {value: 'flashing', label: 'Flashing'},
  {value: 'border', label: 'Border'},
]);

let fontOptions = ref([
  {value: 0, label: 'Mono'},
  {value: 1, label: 'U8G2'},
]);

let monoOptions = ref([
  {value: '5x8', label: '5x8'},
  {value: '6x9', label: '6x9'},
])

let u8g2Options = ref([
  {value: 'lucasfont alternate tf', label: 'Lucasfont alternate TF'},
  {value: 'spleen 5x8 me', label: 'Spleen 5x8 ME'},
  {value: '6x10 tf', label: '6x10 TF'},
])

let text = ref("");
let speed = ref(4);
let animation = ref('left');
let effects = ref<string[]>([]);
let font = ref(0)
let subtype = ref(monoOptions.value[0].value)

let effectsOptions = ref(effectsOptionsWithInvert.value);
let subtypeOptions = ref(monoOptions.value);

watch(font, () => {
  switch (font.value) {
    case 0:
      subtypeOptions.value = monoOptions.value;
      effectsOptions.value = effectsOptionsWithInvert.value;
      subtype.value = monoOptions.value[0].value;
      break
    case 1:
      subtypeOptions.value = u8g2Options.value;
      effectsOptions.value = effectsOptionsNoInvert.value;
      subtype.value = u8g2Options.value[0].value;
      effects.value.splice(effects.value.indexOf('inverted'), 1)
      break
    default:
      subtypeOptions.value = monoOptions.value;
      effectsOptions.value = effectsOptionsWithInvert.value;
      subtype.value = monoOptions.value[0].value;
      break
  }
})

let id = -1;

if (props.messages) {
  for (let m of props.messages) {
    if (m.id < id) {
      id = m.id;
    }
  }
}

let editMode = ref(false);
let editId: number = 0;

function setText() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  invoke("set_text", {
    text: text.value,
    speed: speed.value,
    animation: animation.value,
    effects: effects.value,
    font: font.value,
    fontSubtype: subtype.value,
  });
}

function setMessages() {
  invoke("set_messages", {
    messages: props.messages,
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

async function updateMessageDb() {
  let content_id = (await db.select<Id[]>("select content_id as id from messages where id = $1", [editId]))[0].id;

  db.execute("update text_messages set content = $1, speed = $2, animation = $3, effects = $4, font = $5, subtype = $6 where id = $7",
      [text.value, speed.value, animation.value, effects.value, font.value, subtype.value, content_id])
}

async function saveMessage() {
  let content_id = (await db.select<Id[]>("insert into text_messages (content, speed, animation, effects, font, subtype) values ($1, $2, $3, $4, $5, $6) returning id",
      [text.value, speed.value, animation.value, effects.value, font.value, subtype.value]))[0].id;

  db.execute("insert into messages (content_id, type) values ($1, 'text')",
      [content_id])
}
</script>

<template>
  <n-flex vertical>
    <!-- Text input -->
    <n-input v-model:value="text" clearable placeholder="Text"
             type="text"/>

    <!-- Text input settings -->
    <!--  TODO Cancel button  -->
    <n-flex
        :style="{ borderRadius: '6px', border: '1px solid #3E3E42', padding: '12px'}">
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
        <n-input-group-label>Font</n-input-group-label>
        <n-select v-model:value="font" :options="fontOptions"
                        :style="{ width: '90px' }" class="noStrikethrough"/>
      </n-input-group>
      <n-input-group>
        <n-input-group-label>Subtype</n-input-group-label>
        <n-select v-model:value="subtype" :options="subtypeOptions"
                  :style="{ width: '320px' }" class="noStrikethrough"/>
      </n-input-group>
    </n-flex>
    <n-flex>
      <n-button type="primary" @click="setText">
        Push
      </n-button>
      <n-button v-if="editMode" type="primary" @click="updateMessage">
        Save
      </n-button>
      <n-button v-else type="primary" @click="addMessage">
        Add to messages
      </n-button>
      <n-button type="primary" @click="saveMessage">
        Save Message to storage
      </n-button>
      <n-button v-if="editMode && editId >= 0" type="info" @click="updateMessageDb">
        Update Message in storage
      </n-button>
    </n-flex>
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