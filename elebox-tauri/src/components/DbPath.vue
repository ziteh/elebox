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
  <v-row>
    <!-- <v-file-input label="Database file" variant="outlined" v-model="path" placeholder="elebox.db"></v-file-input> -->
    <v-text-field label="Database file" variant="outlined" v-model="path" placeholder="elebox.db"></v-text-field>
    <v-btn @click="setPath">Apply</v-btn>
  </v-row>
</template>
