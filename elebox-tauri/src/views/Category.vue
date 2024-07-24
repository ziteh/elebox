<script setup lang="ts">
import { onMounted, reactive, ref } from "vue";
import { DbCategory as Db } from "@/utils/db_cmd_category";
import CategoryField from "@/components/Category/Field.vue";
import CategoryList from "@/components/Category/List.vue";
import CategoriesTree from "@/components/Category/Tree.vue";

const displayTree = ref(false);

const existing = reactive<Db.Category[]>([]);

async function fetchExisting() {
  const data = await Db.list();
  Object.assign(existing, data);

  console.debug(`Get categories: ${existing.length}`);
}

onMounted(fetchExisting);
</script>

<template>
  <v-container>
    <CategoryField @update="fetchExisting" />

    <v-switch
      label="Display"
      false-icon="mdi-view-list"
      true-icon="mdi-file-tree"
      v-model="displayTree"
      class="my-n4"
    ></v-switch>

    <CategoriesTree v-if="displayTree" />
    <CategoryList v-else :existing="existing" @update="fetchExisting" />
  </v-container>
</template>
