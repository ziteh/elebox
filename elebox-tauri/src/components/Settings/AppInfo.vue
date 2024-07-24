<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const assets_dir = ref("");

async function fetchAssetsDir() {
  assets_dir.value = await invoke("get_assets_path", {});
}

onMounted(() => {
  fetchAssetsDir();
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
      <code>1.0.0-beta.4</code>
    </v-col>
  </v-row>
</template>
