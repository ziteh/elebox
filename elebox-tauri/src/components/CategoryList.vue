<script setup lang="ts">
import { onMounted, reactive } from "vue";
import { DbCategory as Db } from "../db_cmd_category";

let categories = reactive<Db.Category[]>([]);

async function list() {
  const data = await Db.list();
  Object.assign(categories, data);
}

async function remove(name: string) {
  await Db.remove(name);
}

onMounted(list);
</script>

<template>
  <v-table>
    <thead>
      <tr>
        <th>Name</th>
        <th>Alias</th>
        <th>Parent</th>
        <th>Edit</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="(cat, i) in categories" :key="i">
        <td>{{ cat.name }}</td>
        <td>{{ cat.alias }}</td>
        <td>{{ cat.parent }}</td>
        <td>
          <v-btn
            density="comfortable"
            icon="mdi-square-edit-outline"
            title="Edit"
            :to="{
              name: 'update_category',
              params: { origin_name: cat.name },
            }"
          ></v-btn>
          <v-btn
            density="comfortable"
            icon="mdi-trash-can-outline"
            :title="`Delete: ${cat.name}`"
            @click="remove(cat.name)"
          ></v-btn>
        </td>
      </tr>
    </tbody>
  </v-table>
</template>
