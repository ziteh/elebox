<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";

defineProps({
  part: String
})

async function partAdd(part: string, qty: Number) {
  await invoke("part_add", { part, qty });
  console.log(part);
}
</script>

<template>
  <v-btn @click="partAdd(part!, 1)">+</v-btn>
  <v-btn @click="partAdd(part!, -1)">-</v-btn>
</template>
