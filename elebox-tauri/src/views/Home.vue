<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import PartList from "../components/PartList.vue";
import NewDatabase from "../components/NewDatabase.vue";

const db_exists = ref(false);

async function checkDb() {
  db_exists.value = await invoke("is_db_exists");
}

onMounted(checkDb);
</script>

<template>
  <v-container>
    <v-row>
      <v-container>
        <PartList v-if="db_exists" />
        <NewDatabase v-else @update="checkDb" />
      </v-container>
    </v-row>
  </v-container>
</template>
