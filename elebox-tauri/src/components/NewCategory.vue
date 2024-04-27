<script setup lang="ts">
import { onMounted, ref, reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

interface Categories {
  [index: number]: {
    name: string;
    parent: string;
  }
}

const catName = ref("");
const catParent = ref("");
let categories = reactive<Categories>({});

async function newCategory() {
  await invoke("new_category", { name: catName.value, parent: catParent.value });
  console.debug(`new category: ${catName.value}, ${catParent.value}`);

  await getCategories();
}

async function deleteType() {
  let res = await invoke("del_category", { name: catName.value });
  console.debug(`delete category: ${catName.value}, ${res}`);

  await getCategories();
}

async function getCategories() {
  const cs = await invoke("get_categories", {});
  Object.assign(categories, cs);
  console.debug(`get categories: ${categories}`);
}

onMounted(getCategories);
</script>

<template>
  <form class="row" @submit.prevent="newCategory">
    <div class="form-group">
      <label for="name-in">Name: </label>
      <input id="name-in" v-model="catName" placeholder="LDO" />
    </div>

    <div class="form-group">
      <label for="category-in">Category: </label>
      <select v-model="catParent">
        <option disabled value="Category">Category</option>
        <option v-for="(t, index) in categories" :key="index" :title="t.parent">
          {{ t.name }}
        </option>
      </select>
    </div>

    <button type="submit">Add</button>
  </form>
  <button @click="deleteType">Delete</button>
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
