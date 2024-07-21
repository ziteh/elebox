<script setup lang="ts">
// TODO this file unused
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
  <!-- <v-file-input label="Database file" variant="outlined" v-model="path" placeholder="elebox.db"></v-file-input> -->
  <v-row class="align-center">
    <v-col>
      <v-text-field
        label="Database Path"
        variant="outlined"
        v-model="path"
        placeholder="elebox.db"
      ></v-text-field>
    </v-col>
    <v-col cols="auto" class="mb-6">
      <v-btn @click="setPath" text="Apply"></v-btn>
    </v-col>
  </v-row>
</template>
