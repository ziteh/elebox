<script setup lang="ts">
import { onMounted, reactive } from "vue";
import { DbPackage as Db } from "../db_cmd_package";

let pkgs = reactive<Db.Package[]>([]);

async function get() {
  const data = await Db.list();
  Object.assign(pkgs, data);
}

async function remove(name: string) {
  await Db.remove(name);
}

onMounted(get);
</script>

<template>
  <v-table>
    <thead>
      <tr>
        <th>Name</th>
        <th>Type</th>
        <th>Alias</th>
        <th>Edit</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="(p, i) in pkgs" :key="i">
        <td>{{ p.name }}</td>
        <td>{{ p.pkg_type.toUpperCase() }}</td>
        <td>{{ p.alias }}</td>
        <td>
          <v-btn
            density="comfortable"
            icon="mdi-square-edit-outline"
            title="Edit"
            :to="{
              name: 'update_package',
              params: { origin_name: p.name },
            }"
          ></v-btn>
          <v-btn
            density="comfortable"
            icon="mdi-trash-can-outline"
            :title="`Delete: ${p.name}`"
            @click="remove(p.name)"
          ></v-btn>
        </td>
      </tr>
    </tbody>
  </v-table>
</template>
