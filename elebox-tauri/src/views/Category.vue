<script setup lang="ts">
import { ref } from "vue";
import CategoryField from "../components/CategoryField.vue";
import CategoryList from "../components/CategoryList.vue";
import CategoriesTree from "./CategoriesTree.vue";

const displayTree = ref(false);
</script>

<template>
  <v-container>
    <CategoryField />

    <v-switch
      label="Display"
      false-icon="mdi-view-list"
      true-icon="mdi-file-tree"
      v-model="displayTree"
    ></v-switch>

    <CategoryList v-if="!displayTree" />
    <CategoriesTree v-else />
  </v-container>
</template>
