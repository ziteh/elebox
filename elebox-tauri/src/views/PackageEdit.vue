<script setup lang="ts">
import "../styles.css";
import PackageField from "../components/PackageField.vue";
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
    <h1 class="mb-8">Edit Package</h1>
    <PackageField :origin_name="name" />
  </v-container>
</template>
