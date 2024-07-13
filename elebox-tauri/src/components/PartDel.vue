<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";

defineProps({
  part: String,
});

async function partDel(part: string) {
  console.debug(`Part delete: ${part}`);
  await invoke("part_del", { part });
}
</script>

<template>
  <v-btn
    density="comfortable"
    icon="mdi-trash-can-outline"
    @click="partDel(part!)"
  ></v-btn>
</template>
