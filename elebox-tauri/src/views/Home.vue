<script setup lang="ts">
import router from '../router.js';
import PartList from "../components/PartList.vue";
import DbPath from "../components/DbPath.vue";

function toPart() {
  router.replace({ path: "/part" })
}

function toCategory() {
  router.replace({ path: "/category" })
}
</script>

<template>
  <div class="container">
    <DbPath />
    <hr>
    <button @click="toPart">New Part</button>
    <button @click="toCategory">New Category</button>
    <hr>
    <PartList />
  </div>
</template>
