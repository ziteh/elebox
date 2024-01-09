<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";

defineProps({
  part: String
})

async function partAdd(part: String, qty: Number) {
  await invoke("part_add", { part, qty });
  console.log(part);
}
</script>

<template>
  <button @click="partAdd(part, 1)">+</button>
  <button @click="partAdd(part, -1)">-</button>
</template>
