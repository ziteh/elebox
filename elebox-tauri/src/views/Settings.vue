<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import DbPath from "../components/DbPath.vue";

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
  <v-container>
    <v-row>
      <h1 class="my-8">Settings</h1>
    </v-row>
    <DbPath />
    <v-row class="align-center">
      <v-col>
        <v-text-field
          label="CSV Path"
          variant="outlined"
          v-model="csv_path"
          placeholder="path\to\folder\"
        ></v-text-field>
      </v-col>
      <v-col cols="auto" class="mb-6">
        <v-btn @click="export_csv">Export</v-btn>
      </v-col>
      <v-col cols="auto" class="mb-6">
        <v-btn @click="import_csv">Import</v-btn>
      </v-col>
    </v-row>
    <v-row class="align-center">
      <v-spacer></v-spacer>
      <v-col cols="auto">
        <v-label>Version</v-label>
      </v-col>
      <v-col cols="auto">
        <code>1.0.0-beta.2</code>
      </v-col>
    </v-row>
  </v-container>
</template>
