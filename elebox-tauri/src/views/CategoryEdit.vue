<script setup lang="ts">
import CategoryField from "../components/CategoryField.vue";
import { onMounted, ref } from "vue";
import { useRoute } from "vue-router";

const route = useRoute();
const name = ref();

onMounted(() => {
  name.value = route.params.origin_name;
});
</script>

<template>
  <v-container v-if="name !== undefined">
    <h1>Edit Category</h1>
    <CategoryField :origin_name="name" />
  </v-container>
</template>
