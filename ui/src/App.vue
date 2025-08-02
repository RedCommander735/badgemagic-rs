<script lang="ts" setup>
import {darkTheme} from 'naive-ui'
</script>

<template>
  <n-config-provider :theme="darkTheme">
    <n-global-style/>
    <n-flex :style="{ margin: '20px' }">
      <n-tabs default-value="text" type="segment">
        <n-tab-pane name="text" tab="Text">
          <TextTransfer/>
        </n-tab-pane>
        <n-tab-pane name="draw" tab="Draw">
          <p>Placeholder</p>
        </n-tab-pane>
        <n-tab-pane name="saved" tab="Saved Messages">
          <p>Placeholder</p>
        </n-tab-pane>
      </n-tabs>
    </n-flex>
  </n-config-provider>
</template>