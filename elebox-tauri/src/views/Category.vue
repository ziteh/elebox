<script setup lang="ts">
import router from '../router.js';
import NewCategory from "../components/NewCategory.vue";
import CategoryList from "../components/CategoryList.vue";

function toHome() {
  router.replace({ path: "/" })
}
</script>

<template>
  <div class="container">
    <button @click="toHome">&#11013; Back</button>
    <NewCategory />
    <CategoryList />
  </div>
</template>
