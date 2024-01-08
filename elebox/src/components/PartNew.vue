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
    <div class="form-group">
      <label for="part-name-in">Name: </label>
      <input id="part-name-in" v-model="partName" placeholder="AMS1117" />
    </div>
    <div class="form-group">
      <label for="part-qty-in">Quantity: </label>
      <input id="part-qty-in" v-model="partQty" placeholder="15" type="number" pattern="[0-9]*" />
    </div>
    <div class="form-group">
      <label for="part-type-in">Type: </label>
      <input id="part-type-in" v-model="partType" placeholder="LDO" />
    </div>
    <button type="submit">Add</button>
  </form>
</template>

<style>
.row {
  display: flex;
  justify-content: space-between;
}

.form-group {
  margin-right: 10px;
}
</style>

