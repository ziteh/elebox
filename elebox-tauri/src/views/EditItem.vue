<script setup lang="ts">
import { ref } from "vue";
import { useRoute } from "vue-router";
import CategoryField from "@/components/CategoryField.vue";
import PackageField from "@/components/PackageField.vue";
import ManufacturerField from "@/components/ManufacturerField.vue";

const route = useRoute();

const item = ref(toString(route.params.item));
const name = ref(toString(route.params.name));

function toString(value: string | string[]) {
  return Array.isArray(value) ? value[0] : value;
}
</script>

<template>
  <v-container v-if="item === 'category'">
    <h1 class="mb-8">Edit Category</h1>
    <CategoryField :ori_name="name" />
  </v-container>
  <v-container v-else-if="item === 'package'">
    <h1 class="mb-8">Edit Package</h1>
    <PackageField :ori_name="name" />
  </v-container>
  <v-container v-else-if="item === 'manufacturer'">
    <h1 class="mb-8">Edit Manufacturer</h1>
    <ManufacturerField :ori_name="name" />
  </v-container>

  <v-container v-else>
    <v-empty-state headline="404" title="Page not found"></v-empty-state>
  </v-container>
</template>
