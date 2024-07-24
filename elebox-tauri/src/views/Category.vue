<script setup lang="ts">
import { ref } from "vue";
import CategoryField from "@/components/Category/Field.vue";
import CategoryList from "@/components/Category/List.vue";
import CategoriesTree from "@/components/Category/Tree.vue";

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
