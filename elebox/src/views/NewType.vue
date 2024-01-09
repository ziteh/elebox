<script setup lang="ts">
import router from '../router.js';
import TypeNew from "../components/TypeNew.vue";

function toHome() {
  router.replace({ path: "/" })
}
</script>

<template>
  <div class="container">
    <button @click="toHome">&#11013; Back</button>
    <TypeNew />
  </div>
</template>
