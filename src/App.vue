<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { onMounted, ref, computed } from "vue";
import { appWindow } from "@tauri-apps/api/window";

interface Memory {
  percentage: number;
  total: string;
  used: string;
}

interface SystemInfo {
  battery_remaining_capacity: number | null;
  cpu_load: number | null;
  memory: Memory | null;
}

const refSystemInfo = ref<SystemInfo>({
  battery_remaining_capacity: null,
  cpu_load: null,
  memory: null,
});
const refBatteryIcon = ref("battery_4");

function getImageUrl(name: String) {
  return new URL(`./assets/img/battery/${name}.svg`, import.meta.url).href;
}

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
        <img
          id="battery-icon"
          style="width: 30px; height: 30px"
          :src="getImageUrl(refBatteryIcon)"
        />
        <span>
          <p>{{ refSystemInfo.battery_remaining_capacity }}%</p>
        </span>
      </div>
      <div class="cpu">
        <img style="width: 30px; height: 30px" src="./assets/img/cpu.svg" />
        <span>
          <p>
            {{
              refSystemInfo.cpu_load >= 0
                ? (refSystemInfo.cpu_load * 100).toFixed(0)
                : ""
            }}%
          </p>
        </span>
      </div>
      <div class="memory">
        <img style="width: 30px; height: 30px" src="./assets/img/memory.svg" />
        <span>
          <p>
            {{
              refSystemInfo.memory?.percentage >= 0
                ? (refSystemInfo.memory?.percentage * 100).toFixed(0)
                : ""
            }}%
          </p>
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
  /* background-color: black; */
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
.battery span {
  width: 40px;
  height: 30px;
}
.battery span p {
  margin-top: 6px;
}

/* ====== apu ====== */
.cpu {
  height: 30px;
  display: flex;
}
.cpu span {
  width: 35px;
  height: 30px;
  padding-left: 3px;
}
.cpu span p {
  margin-top: 6px;
}

/* ====== memory ====== */
.memory {
  height: 30px;
  display: flex;
}
.memory span {
  height: 30px;
  padding-left: 3px;
}
.memory span p {
  margin-top: 6px;
}
</style>