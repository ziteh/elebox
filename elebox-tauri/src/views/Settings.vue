<script setup lang="ts">
import DbPath from "../components/DbPath.vue";
</script>

<template>
  <v-container class="d-flex flex-column">
    <DbPath />
    <div class="flex-fill"></div>
    <div class="d-flex align-end ga-2 mt-auto">
      <v-label>Version</v-label>
      <code>1.0.0-alpha.1</code>
    </div>
  </v-container>
</template>
