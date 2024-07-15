<script setup lang="ts">
import { onMounted, ref, reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Category } from "../interface";

const props = defineProps<{
  category?: Category;
  origin_name?: string;
}>();

const category = ref<Category>(
  props.category || { name: "", alias: "", parent: "" }
);

let categories = reactive<Category[]>([]);

async function newCategory() {
  await invoke("new_category", {
    name: category.value.name,
    parent: category.value.parent,
    alias: category.value.alias,
  });

  await getCategories();
}

async function updateCategory() {
  if (props.origin_name === undefined) {
    return;
  }
  await invoke("update_category", {
    originName: props.origin_name,
    name: category.value.name,
    parent: category.value.parent,
    alias: category.value.alias,
  });

  await getCategories();
}

async function getCategories() {
  const p_cat = await invoke("get_categories", {});
  Object.assign(categories, p_cat);
  console.debug(`get categories: ${categories}`);
}

onMounted(() => {
  getCategories();
});
</script>

<template>
  <v-form>
    <v-container>
      <v-row class="ga-8">
        <v-text-field
          label="Name"
          variant="outlined"
          v-model="category.name"
          placeholder="MCU"
          :rules="[(v: any) => !!v || 'Required']"
          required
        ></v-text-field>
        <v-text-field
          label="Alias"
          variant="outlined"
          v-model="category.alias"
          placeholder=""
        ></v-text-field>
        <v-select
          label="Parent"
          :items="Object.values(categories).map((c) => c.name)"
          variant="outlined"
          v-model="category.parent"
        ></v-select>
        <v-btn v-if="props.origin_name === undefined" @click="newCategory"
          >Add</v-btn
        >
        <v-btn v-else @click="updateCategory">Update</v-btn>
      </v-row>
    </v-container>
  </v-form>
</template>
