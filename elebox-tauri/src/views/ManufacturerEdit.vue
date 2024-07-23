<script setup lang="ts">
import ManufacturerField from "../components/ManufacturerField.vue";
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
    <h1 class="mb-8">Edit Manufacturer</h1>
    <ManufacturerField :ori_name="name" />
  </v-container>
</template>
