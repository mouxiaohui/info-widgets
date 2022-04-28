<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { onMounted, ref } from "vue";
import { appWindow } from "@tauri-apps/api/window";

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
const refBatteryIcon = ref("battery_4");

function updateIcon() {
  const battery_cap = refSystemInfo.value.battery_remaining_capacity;
  if (battery_cap) {
    if (battery_cap >= 85) {
      refBatteryIcon.value = "battery_4";
    } else if (battery_cap >= 55) {
      refBatteryIcon.value = "battery_3";
    } else if (battery_cap >= 35) {
      refBatteryIcon.value = "battery_2";
    } else {
      refBatteryIcon.value = "battery_1";
    }
  }
}

function start() {
  setInterval(async () => {
    let data = (await invoke("get_system_info")) as SystemInfo;

    refSystemInfo.value = {
      battery_remaining_capacity: data.battery_remaining_capacity,
      cpu_load: data.cpu_load,
      memory: data.memory,
    };

    updateIcon();
  }, 2000);
}

start();

onMounted(() => {
  let w = document.getElementById("widgets");
  if (w) {
    addEventListener("mousedown", () => appWindow.startDragging());
  }
});
</script>

<template>
  <div id="widgets">
    <div class="info">
      <div class="battery">
        <img :src="'src/assets/img/battery/' + refBatteryIcon + '.svg'" />
        <span>
          <p>{{ refSystemInfo.battery_remaining_capacity }}%</p>
        </span>
      </div>
    </div>
  </div>
</template>

<style>
#widgets {
  width: 100%;
  height: 100%;
  color: white;
  cursor: pointer;
  background-color: black;
  user-select: none;
}

.info {
  height: 30px;
  display: flex;
}

/* ====== battery ====== */
.battery {
  height: 30px;
  display: flex;
}
.battery img {
  width: 30px;
  height: 30px;
}
.battery span {
  width: 40px;
  height: 30px;
}
.battery span p {
  margin-top: 8px;
}
</style>