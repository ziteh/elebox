<script setup lang="ts">
import { onMounted, reactive, ref } from "vue";
import { DbManufacturer as Db } from "@/utils/db_cmd_manufacturer";
import ItemEditButton from "@/components/ItemEditButton.vue";
import ItemDeleteButton from "@/components/ItemDeleteButton.vue";

const search = ref("");
const headers = ref([
  { key: "name", title: "Name", sortable: true },
  { key: "alias", title: "Alias", sortable: true },
  { key: "edit", title: "Edit", sortable: false },
]);

const existing = reactive<Db.Manufacturer[]>([]);

async function fetchExisting() {
  const data = await Db.list();
  Object.assign(existing, data);
}

async function deleteItem(name: string) {
  await Db.remove(name);
  await fetchExisting();
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
      <template v-slot:item.name="{ item }">
        {{ item.name }}
        <a v-if="item.url" :title="item.url" :href="item.url" target="_blank"
          ><v-icon>mdi-open-in-new</v-icon>
        </a>
      </template>
      <template v-slot:item.edit="{ item }">
        <ItemEditButton :item="'manufacturer'" :name="item.name" />
        <ItemDeleteButton :name="item.name" @delete="deleteItem(item.name)" />
      </template>
    </v-data-table>
  </v-card>
</template>
