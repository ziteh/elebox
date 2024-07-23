<script setup lang="ts">
import { onMounted, reactive, ref } from "vue";
import { DbCategory as Db } from "@/utils/db_cmd_category";
import ItemEditButton from "@/components/ItemEditButton.vue";
import ItemDeleteButton from "@/components/ItemDeleteButton.vue";

const search = ref("");
const headers = ref([
  { key: "name", title: "Name", sortable: true },
  { key: "alias", title: "Alias", sortable: true },
  { key: "parent", title: "Parent", sortable: true },
  { key: "edit", title: "Edit", sortable: false },
]);

const existing = reactive<Db.Category[]>([]);

async function fetchExisting() {
  const data = await Db.list();
  Object.assign(existing, data);
}

async function deleteItem(name: string) {
  await Db.remove(name);
}

onMounted(fetchExisting);
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

    <v-data-table :headers="headers" :items="existing" :search="search">
      <template v-slot:item.edit="{ item }">
        <ItemEditButton :path_name="'update_category'" :item_name="item.name" />
        <ItemDeleteButton :name="item.name" @delete="deleteItem(item.name)" />
      </template>
    </v-data-table>
  </v-card>
</template>
