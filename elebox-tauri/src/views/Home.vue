<script setup lang="ts">
import { onMounted, ref, reactive } from "vue";
import { useRouter, useRoute } from "vue-router";
import { invoke } from "@tauri-apps/api/tauri";
import PartList from "../components/PartList.vue";
import NewDatabase from "../components/NewDatabase.vue";

const isPath = ref(false);

async function getDbPath() {
  const path = await invoke("get_db_path", {});
  if (path !== "") {
    isPath.value = true;
  }
}

onMounted(getDbPath);
</script>

<template>
  <v-container>
    <v-row>
      <v-container>
        <PartList v-if="isPath" />
        <NewDatabase v-else @update="getDbPath" />
      </v-container>
    </v-row>
  </v-container>
</template>
