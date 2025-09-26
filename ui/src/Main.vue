<script lang="ts" setup>
import App from "./App.vue";
import {darkTheme, GlobalThemeOverrides} from 'naive-ui'
import {createPressedColor} from "naive-ui/es/_utils/color";

const primary = 'rgba(169, 41, 41, 1)';
const text = '#ffffff';


const themeOverrides: GlobalThemeOverrides = {
  common: {
    primaryColor: primary,
    primaryColorHover: createPressedColor(primary),
    primaryColorPressed: createPressedColor(createPressedColor(primary)),
    textColorBase: text
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

