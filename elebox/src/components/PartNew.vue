<script setup lang="ts">
import { onMounted, ref, reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

interface Types {
  [index: number]: {
    name: string;
    parent: string;
  }
}

const partName = ref("");
const partQty = ref("");
const partType = ref("");
let types = reactive<Types>({});

async function newPart() {
  await invoke("part_new", { name: partName.value, qty: parseInt(partQty.value), ptype: partType.value });
}

async function getTypes() {
  types = await invoke("get_types", {});
  console.debug(`Types: ${types}`);
}

onMounted(async () => {
  await getTypes();
});
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
      <select v-model="partType">
        <option disabled value="Category">Category</option>
        <option v-for="(t, index) in types" :key="index" :title="t.parent">
          {{ t.name }}
        </option>
      </select>
    </div>

    <button type="submit">Add</button>
  </form>
</template>


