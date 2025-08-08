<script lang="ts" setup>

import Database from '@tauri-apps/plugin-sql';

let db = await Database.load('sqlite:messages.db');
</script>

<template>
  <n-flex :style="{ margin: '20px' }">
    <n-tabs default-value="text" type="segment">
      <n-tab-pane name="text" tab="Text">
        <TextTransfer :db="db"/>
      </n-tab-pane>
      <n-tab-pane name="draw" tab="Draw">
        <p>Placeholder</p>
      </n-tab-pane>
      <n-tab-pane name="saved" tab="Saved Messages">
        <SavedMessages :db="db"/>
      </n-tab-pane>
    </n-tabs>
  </n-flex>
</template>