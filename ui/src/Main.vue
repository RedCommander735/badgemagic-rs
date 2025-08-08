<script lang="ts" setup>
import App from "./App.vue";
import {darkTheme} from 'naive-ui'
</script>

<template>
  <n-config-provider :theme="darkTheme">
    <n-global-style/>
    <Suspense>
      <!-- component with nested async dependencies -->
      <App/>

      <!-- loading state via #fallback slot -->
      <template #fallback>
        Loading...
      </template>
    </Suspense>
  </n-config-provider>
</template>

<style scoped>

</style>