<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const partName = ref("");
const partQty = ref("");
const partType = ref("");

async function newPart() {
  await invoke("part_new", { name: partName.value, qty: parseInt(partQty.value), ptype: partType.value });
}
</script>

<template>
  <form class="row" @submit.prevent="newPart">
    <input id="part-name-in" v-model="partName" placeholder="Name" />
    <input id="part-qty-in" v-model="partQty" placeholder="Quantity" type="number" pattern="[0-9]*" />
    <input id="part-type-in" v-model="partType" placeholder="Type" />
    <button type="submit">Add</button>
  </form>
</template>


