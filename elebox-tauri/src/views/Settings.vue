<script setup lang="ts">
import DbPath from "../components/DbPath.vue";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const csv_path = ref("");

async function export_csv() {
  await invoke("export_csv", { csvPath: csv_path.value });
  console.debug(`Export path: ${csv_path.value}`);
}

async function import_csv() {
  await invoke("import_csv", { csvPath: csv_path.value });
  console.debug(`Import path: ${csv_path.value}`);
}
</script>

<template>
  <v-container class="d-flex flex-column">
    <DbPath />
    <v-row>
      <v-text-field
        label="CSV Path"
        variant="outlined"
        v-model="csv_path"
        placeholder="path\to\folder\"
      ></v-text-field>
      <v-btn @click="export_csv">Export</v-btn>
      <v-btn @click="import_csv">Import</v-btn>
    </v-row>
    <div class="flex-fill"></div>
    <div class="d-flex align-end ga-2 mt-auto">
      <v-label>Version</v-label>
      <code>1.0.0-alpha.1</code>
    </div>
  </v-container>
</template>
