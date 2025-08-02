<script lang="ts" setup>
import {invoke} from "@tauri-apps/api/core";
import {darkTheme} from 'naive-ui'

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
])

let text = ref("");
let speed = ref(4)
let animation = ref('left')
let effects = ref([])
let fontSize = ref(9)

async function setText() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  invoke("set_text", {
    text: text.value,
    speed: speed.value,
    animation: animation.value,
    effects: effects.value,
    fontSize: fontSize.value
  })
}
</script>

<template>
  <n-config-provider :theme="darkTheme">
    <n-global-style/>
    <n-flex :style="{ margin: '20px' }" vertical>
      <!-- Text input -->
      <n-flex>
        <n-input v-model:value="text" :style="{ width: 'calc(750px - 84px)' }" clearable placeholder="Text"
                 type="text"/>
        <n-button type="primary" @click="setText">
          Push
        </n-button>
      </n-flex>

      <div :style="{ width: 'calc(750px - 34px)', paddingInline: '5px', height: '1px', backgroundColor: '#3E3E42' }"/>

      <!-- Text input settings-->
      <n-flex>
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
    </n-flex>
  </n-config-provider>
</template>

<style scope>
.noStrikethrough input {
  text-decoration-line: none !important;
}

.n-input-group {
  width: unset;
}
</style>