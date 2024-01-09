<script setup lang="ts">
import router from '../router.js';
import PartList from "../components/PartList.vue";

function toNewPart() {
  router.replace({ path: "/new-part" })
}

function toNewType() {
  router.replace({ path: "/new-type" })
}
</script>

<template>
  <div class="container">
    <button @click="toNewPart">New part</button>
    <button @click="toNewType">New type</button>
    <PartList />
  </div>
</template>
