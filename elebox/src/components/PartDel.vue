<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";

defineProps({
  part: String
})

async function partDel(part: String) {
  console.debug(`Part delete: ${part}`);
  await invoke("part_del", { part });
}
</script>

<template>
  <button @click="partDel(part)">🗑️</button>
</template>
