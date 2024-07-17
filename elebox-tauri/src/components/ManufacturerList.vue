<script setup lang="ts">
import { onMounted, reactive } from "vue";
import { DbManufacturer as Db } from "../db_cmd_manufacturer";

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
          <a
            v-if="m.url !== '' && m.url !== undefined"
            :title="m.url"
            :href="m.url"
            target="_blank"
            ><v-icon>mdi-open-in-new</v-icon>
          </a>
        </td>
        <td>{{ m.alias }}</td>
        <td>
          <v-btn
            density="comfortable"
            icon="mdi-square-edit-outline"
            title="Edit"
            :to="{
              name: 'update_manufacturer',
              params: { origin_name: m.name },
            }"
          ></v-btn>
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
