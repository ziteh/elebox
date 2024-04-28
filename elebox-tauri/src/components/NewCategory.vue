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

async function getCategories() {
  const cs = await invoke("get_categories", {});
  Object.assign(categories, cs);
  console.debug(`get categories: ${categories}`);
}

onMounted(getCategories);
</script>

<template>
  <v-form>
    <v-container>
      <v-row>
        <v-text-field label="Name" variant="outlined" v-model="catName" placeholder="MCU"></v-text-field>
        <v-select label="Category" :items="Object.values(categories).map(cat => cat.name)"
          variant="outlined"></v-select>
        <v-btn @click="newCategory">Save</v-btn>
      </v-row>
    </v-container>
  </v-form>
</template>
