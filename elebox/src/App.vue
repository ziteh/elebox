<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import PartList from "./components/PartList.vue";
import PartNew from "./components/PartNew.vue";
import TypeNew from "./components/TypeNew.vue";

</script>

<template>
  <div class="container">

    <TypeNew />
    <PartNew />
    <PartList />

  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
