<script lang="ts" setup>

interface Props {
  options: Option[],
  default?: Option | null,
  tabindex?: number,
}

export interface Option {
  value: string,
  name: string,
}

const props = withDefaults(defineProps<Props>(), {
  default: null,
  tabindex: 0
})

let selectedInitial: Option | null = null;

if (props.default) {
  selectedInitial = props.default;
} else if (props.options.length > 0) {
  selectedInitial = props.options[0];
}

let selected = ref<Option | null>(selectedInitial);
let open = ref<boolean>(false)

</script>



<template>
  <div :tabindex="tabindex" class="custom-select" @blur="open = false">
    <div :class="{ open: open }" class="selected" @click="open = !open">
      {{ selected ? selected?.name : "" }}
    </div>
    <div :class="{ selectHide: !open }" class="items">
      <div
          v-for="(option, i) of options"
          :key="i"
          @click="
          selected = option;
          open = false;
          $emit('input', option);
        "
      >
        {{ option.name }}
      </div>
    </div>
  </div>
</template>

<style scoped>
.custom-select {
  --accent-main: #ad8225;
  --bg: #0f0f0f;

  position: relative;
  width: 100%;
  text-align: left;
  outline: none;
  height: 35px;
  line-height: 35px;
}

.custom-select .selected {
  background-color: var(--bg);
  border-radius: 6px;
  border: 1px solid #666666;
  color: #fff;
  padding-left: 1em;
  cursor: pointer;
  user-select: none;
}

.custom-select .selected.open {
  border: 1px solid var(--accent-main);
  border-radius: 6px 6px 0px 0px;
}

.custom-select .selected:after {
  position: absolute;
  content: "";
  top: 22px;
  right: 1em;
  width: 0;
  height: 0;
  border: 5px solid transparent;
  border-color: #fff transparent transparent transparent;
}

.custom-select .items {
  color: #fff;
  border-radius: 0px 0px 6px 6px;
  overflow: hidden;
  border-right: 1px solid var(--accent-main);
  border-left: 1px solid var(--accent-main);
  border-bottom: 1px solid var(--accent-main);
  position: absolute;
  background-color: var(--bg);
  left: 0;
  right: 0;
  z-index: 1;
}

.custom-select .items div {
  color: #fff;
  padding-left: 1em;
  cursor: pointer;
  user-select: none;
}

.custom-select .items div:hover {
  background-color: var(--accent-main);
}

.selectHide {
  display: none;
}
</style>