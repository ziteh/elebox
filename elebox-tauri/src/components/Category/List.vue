<script setup lang="ts">
import { reactive, ref } from "vue";
import { DbCategory as Db } from "@/utils/db_cmd_category";
import ItemEditButton from "@/components/ItemEditButton.vue";
import ItemDeleteButton from "@/components/ItemDeleteButton.vue";

const props = defineProps<{
  existing: Db.Category[];
}>();

const existing = reactive(props.existing);

const emit = defineEmits(["update"]);

const search = ref("");
const headers = ref([
  { key: "name", title: "Name", sortable: true },
  { key: "alias", title: "Alias", sortable: true },
  { key: "parent", title: "Parent", sortable: true },
  { key: "edit", title: "Edit", sortable: false },
]);

async function deleteItem(name: string) {
  await Db.remove(name);
  emit("update");
}
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
        <ItemEditButton :item="'category'" :name="item.name" />
        <ItemDeleteButton :name="item.name" @delete="deleteItem(item.name)" />
      </template>
    </v-data-table>
  </v-card>
</template>
