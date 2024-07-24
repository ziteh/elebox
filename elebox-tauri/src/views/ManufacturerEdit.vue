<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import ManufacturerField from "@/components/ManufacturerField.vue";

const route = useRoute();
const name = ref();

onMounted(() => {
  name.value = route.params.origin_name;
});
</script>

<template>
  <v-container v-if="name != undefined">
    <h1 class="mb-8">Edit Manufacturer</h1>
    <ManufacturerField :ori_name="name" />
  </v-container>
</template>
