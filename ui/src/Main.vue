<script lang="ts" setup>
import App from "./App.vue";
import {darkTheme, GlobalThemeOverrides} from 'naive-ui'
import {createHoverColor, createPressedColor} from "naive-ui/es/_utils/color";

// https://www.realtimecolors.com/?colors=fbe9e9-010104-990000-6d3535-dd852c&fonts=Inter-Inter
const primary = 'rgba(169, 41, 41, 1)';
const info = '#dd852c'
const text = '#ffffff';


const themeOverrides: GlobalThemeOverrides = {
  common: {
    primaryColor: primary,
    primaryColorHover: createPressedColor(primary),
    primaryColorPressed: createPressedColor(createPressedColor(primary)),
    infoColor: info,
    infoColorHover: createHoverColor(info),
    infoColorPressed: createPressedColor(info)
  },
  Button: {
    textColorPrimary: text,
    textColorHoverPrimary: text,
    textColorPressedPrimary: text,
    textColorFocusPrimary: text,
    textColorDisabledPrimary: text,
  },
}

</script>

<template>
  <n-config-provider :theme="darkTheme" :theme-overrides="themeOverrides">
    <!--    <n-theme-editor>-->
    <n-global-style/>
    <Suspense>
      <!-- component with nested async dependencies -->
      <App/>

      <!-- loading state via #fallback slot -->
      <template #fallback>
        Loading...
      </template>
    </Suspense>
    <!--    </n-theme-editor>-->
  </n-config-provider>
</template>

<style scoped>

</style>

