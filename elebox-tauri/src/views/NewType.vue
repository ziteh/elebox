<script setup lang="ts">
import router from '../router.js';
import TypeNew from "../components/TypeNew.vue";
import TypeList from "../components/TypeList.vue";

function toHome() {
  router.replace({ path: "/" })
}
</script>

<template>
  <div class="container">
    <button @click="toHome">&#11013; Back</button>
    <TypeNew />
<<<<<<< HEAD:elebox-tauri/src/views/NewType.vue
    <TypeList />
=======
>>>>>>> origin/core:elebox/src/views/NewType.vue
  </div>
</template>
