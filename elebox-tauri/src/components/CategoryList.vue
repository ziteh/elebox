<script setup lang="ts">
import { onMounted, reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Category } from "../interface";

let categories = reactive<Category[]>([]);

async function getCategories() {
  const cs = await invoke("get_categories", {});
  Object.assign(categories, cs);
  console.debug(`get categories: ${categories}`);
}

async function delCategory(name: string) {
  console.debug(`delete: ${name}`);
  await invoke("del_category", { name });
  // await getCategories();
}

onMounted(getCategories);
</script>

<template>
  <v-table>
    <thead>
      <tr>
        <th>Name</th>
        <th>Parent</th>
        <th>Edit</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="(cat, i) in categories" :key="i">
        <td>{{ cat.name }}</td>
        <td>{{ cat.parent }}</td>
        <td>
          <v-btn
            density="comfortable"
            icon="mdi-trash-can-outline"
            @click="delCategory(cat.name)"
            title="Delete"
          ></v-btn>
          <v-btn
            density="comfortable"
            icon="mdi-square-edit-outline"
            :to="{
              name: 'update_category',
              params: { origin_name: cat.name },
            }"
          ></v-btn>
        </td>
      </tr>
    </tbody>
  </v-table>
</template>
