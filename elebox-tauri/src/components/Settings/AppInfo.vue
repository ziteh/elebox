<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { getVersion } from "@tauri-apps/api/app";

const assets_dir = ref("");
const app_version = ref("");

async function fetchAssetsDir() {
  assets_dir.value = await invoke("get_assets_path", {});
}

async function getAppVersion() {
  app_version.value = await getVersion();
}

onMounted(() => {
  fetchAssetsDir();
  getAppVersion();
});
</script>

<template>
  <v-row class="align-center">
    <v-col cols="auto">
      <v-label>Assets Folder</v-label>
    </v-col>
    <v-col>
      <code>{{ assets_dir }}</code>
    </v-col>
  </v-row>
  <v-row class="align-center">
    <!-- <v-spacer></v-spacer> -->
    <v-col cols="auto">
      <v-label>Version</v-label>
    </v-col>
    <v-col cols="auto">
      <code>{{ app_version }}</code>
    </v-col>
  </v-row>
</template>
