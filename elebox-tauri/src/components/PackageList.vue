<script setup lang="ts">
import { onMounted, reactive, ref } from "vue";
import { DbPackage as Db } from "../db_cmd_package";
import ItemEditButton from "./ItemEditButton.vue";
import ItemDeleteButton from "./ItemDeleteButton.vue";

const search = ref("");
const headers = ref([
  { key: "name", title: "Name", sortable: true },
  { key: "pkg_type", title: "Type" },
  { key: "alias", title: "Alias" },
  { key: "edit", title: "Edit", sortable: false },
]);

let pkgs = reactive<Db.Package[]>([]);

async function list() {
  const data = await Db.list();
  Object.assign(pkgs, data);
}

async function deleteItem(name: string) {
  await Db.remove(name);
}

onMounted(list);
</script>

<template>
  <v-card flat>
    <template v-slot:text>
      <v-text-field
        v-model="search"
        label="Search"
        prepend-inner-icon="mdi-magnify"
        variant="outlined"
        hide-details
        single-line
      ></v-text-field>
    </template>

    <v-data-table :headers="headers" :items="pkgs" :search="search">
      <template v-slot:item.pkg_type="{ item }">
        {{ item.pkg_type.toUpperCase() }}
      </template>
      <template v-slot:item.edit="{ item }">
        <ItemEditButton :path_name="'update_package'" :item_name="item.name" />
        <ItemDeleteButton :name="item.name" @delete="deleteItem(item.name)" />
      </template>
    </v-data-table>
  </v-card>
</template>
