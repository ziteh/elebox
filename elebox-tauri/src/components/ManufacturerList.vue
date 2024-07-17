<script setup lang="ts">
import { onMounted, reactive } from "vue";
import { DbManufacturer as Db } from "../db_cmd_manufacturer";
import ItemEditButton from "./ItemEditButton.vue";

let mfrs = reactive<Db.Manufacturer[]>([]);

async function list() {
  const data = await Db.list();
  Object.assign(mfrs, data);
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
        <th>Edit</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="(m, i) in mfrs" :key="i">
        <td>
          {{ m.name }}
          <a v-if="m.url" :title="m.url" :href="m.url" target="_blank"
            ><v-icon>mdi-open-in-new</v-icon>
          </a>
        </td>
        <td>{{ m.alias }}</td>
        <td>
          <ItemEditButton
            :path_name="'update_manufacturer'"
            :item_name="m.name"
          />
          <v-btn
            density="comfortable"
            icon="mdi-trash-can-outline"
            :title="`Delete: ${m.name}`"
            @click="remove(m.name)"
          ></v-btn>
        </td>
      </tr>
    </tbody>
  </v-table>
</template>
