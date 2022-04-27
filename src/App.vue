<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";

async function get_system_info(i: Number) {
  const res = await invoke("get_system_info");
  console.log(i, res);
  return res
}

let i = 0;
setInterval(() => {
  get_system_info(i);
  i++;
}, 2000);
</script>

<template>
  <div class="app" data-tauri-drag-region>Hello World</div>
</template>

<style>
html,
body,
#app {
  padding: 0;
  margin: 0;
  width: 100%;
  height: 100%;
}

.app {
  width: 100%;
  height: 100%;
  overflow: hidden;
}
</style>