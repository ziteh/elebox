<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import PartList from "@/components/Part/List.vue";
import NewDatabase from "@/components/NewDatabase.vue";

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
        <NewDatabase v-else @notify="checkDb" />
      </v-container>
    </v-row>
  </v-container>
</template>
