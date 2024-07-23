<script setup lang="ts">
import { ref } from "vue";
import CategoryField from "@/components/CategoryField.vue";
import CategoryList from "@/components/CategoryList.vue";
import CategoriesTree from "@/components/CategoryTree.vue";

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
      class="my-n4"
    ></v-switch>

    <CategoriesTree v-if="displayTree" />
    <CategoryList v-else />
  </v-container>
</template>
