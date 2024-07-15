<script setup lang="ts">
import CategoryField from "../components/CategoryField.vue";
import { Category } from "../interface";
import { onMounted, ref, reactive } from "vue";
import { useRoute } from "vue-router";

const route = useRoute();

const category = ref<Category>({
  name: "aa",
  alias: "ali",
  parent: "none",
});
const name = ref();

onMounted(() => {
  name.value = route.params.origin_name;
});
</script>

<template>
  <v-container>
    <h1>Edit Category</h1>
    <CategoryField :category="category" :origin_name="name" />
  </v-container>
</template>
