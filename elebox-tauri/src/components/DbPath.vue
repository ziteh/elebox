<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const path = ref("");

async function getPath() {
  path.value = await invoke("get_db_path", {});
  console.debug(`DB path: ${path.value}`);
}

async function setPath() {
  await invoke("set_db_path", { new_path: path.value });
  console.debug(`DB path: ${path.value}`);
}

onMounted(getPath);
</script>

<template>
  <input id="db_path" v-model="path" placeholder=".db" />
  <button @click="setPath">Set DB Path</button>
</template>

