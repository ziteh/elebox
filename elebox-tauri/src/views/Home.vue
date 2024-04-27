<script setup lang="ts">
import router from '../router.js';
import PartList from "../components/PartList.vue";

function toPart() {
  router.replace({ path: "/part" })
}

function toCategory() {
  router.replace({ path: "/category" })
}

function toConfig() {
  router.replace({ path: "/config" })
}
</script>

<template>
  <div class="container">
    <hr>
    <button @click="toPart">New Part</button>
    <button @click="toCategory">New Category</button>
    <button @click="toConfig">Config</button>
    <hr>
    <PartList />
  </div>
</template>
