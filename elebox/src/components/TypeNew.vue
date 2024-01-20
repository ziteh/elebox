<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const typeName = ref("");
const typeParent = ref("");
const types = ref("");

async function newType() {
  await invoke("type_new", { name: typeName.value, parent: typeParent.value });
  console.debug(`Types: ${typeName.value}, ${typeParent.value}`);

  await getTypes();
}

async function getTypes() {
  types.value = await invoke("get_types", {});
  console.debug(`Types: ${types.value}`);
}

onMounted(async () => {
  await getTypes();
});
</script>

<template>
  <form class="row" @submit.prevent="newType">
    <div class="form-group">
      <label for="part-name-in">Name: </label>
      <input id="part-name-in" v-model="typeName" placeholder="LDO" />
    </div>

    <div class="form-group">
      <label for="part-type-in">Type: </label>
      <select v-model="typeParent">
        <option disabled value="Category">Category</option>
        <option v-for="(t, index) in types" :key="index" :title="t.parent">
          {{ t.name }}
        </option>
      </select>
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


