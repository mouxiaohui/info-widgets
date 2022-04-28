<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref, onMounted } from "vue";

interface Memory {
  percentage: Number;
  total: string;
  used: string;
}

interface SystemInfo {
  battery_remaining_capacity: Number | null;
  cpu_load: Number | null;
  memory: Memory | null;
}

const refSystemInfo = ref<SystemInfo>({
  battery_remaining_capacity: null,
  cpu_load: null,
  memory: null,
});

function start() {
  setInterval(async () => {
    let data = (await invoke("get_system_info")) as SystemInfo;

    refSystemInfo.value = {
      battery_remaining_capacity: data.battery_remaining_capacity,
      cpu_load: data.cpu_load,
      memory: data.memory,
    };
  }, 2000);
}

start();

onMounted(() => {
  let w = document.getElementById("widgets");
  if (w) {
    w.children[0].setAttribute("data-tauri-drag-region", "");
  }
});
</script>

<template>
  <div id="widgets" class="no-select" data-tauri-drag-region>
    <div>
      <div>电池:{{ refSystemInfo.battery_remaining_capacity }}</div>
      <div>cpu_load:{{ refSystemInfo.cpu_load }}</div>
      <div>
        ram
        <span>{{ refSystemInfo.memory?.percentage }} | </span>
        <span>{{ refSystemInfo.memory?.used }} | </span>
        <span>{{ refSystemInfo.memory?.total }}</span>
      </div>
    </div>
  </div>
</template>

<style>
html,
body,
#app {
  padding: 0;
  margin: 0;
  width: 100%;
  height: 100%;
  overflow: hidden;
}

.no-select {
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}

#widgets {
  width: 100%;
  height: 100%;
  color: white;
  cursor: pointer;
  background-color: black;
}
</style>